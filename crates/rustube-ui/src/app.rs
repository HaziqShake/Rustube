use std::collections::HashMap;
use iced::widget::{column, row};
use iced::{Element, Length, Task};
use rustube_core::player::PlayerController;
use rustube_core::youtube::client::YouTubeClient;
use rustube_core::youtube::models::{VideoDetails, VideoItem};

use crate::message::Message;
use crate::pages::{home, search, watch};
use crate::router::{Page, Router};
use crate::widgets::{sidebar, top_bar};

pub struct RustubeApp {
    router: Router,
    search_input: String,
    sidebar_expanded: bool,
    trending_videos: Vec<VideoItem>,
    search_results: Vec<VideoItem>,
    active_video: Option<VideoDetails>,
    thumbnail_cache: HashMap<String, Vec<u8>>,
    youtube: YouTubeClient,
    player: PlayerController,
}

impl RustubeApp {
    pub fn new() -> (Self, Task<Message>) {
        let youtube = YouTubeClient::new().expect("Failed to initialize YouTube client");
        let player = PlayerController::new();

        let initial_arg = std::env::args().nth(1);
        let (router, initial_task) = if let Some(arg) = initial_arg {
            let video_id = if arg.contains("v=") {
                arg.split("v=").nth(1).and_then(|s| s.split('&').next()).unwrap_or(&arg).to_string()
            } else {
                arg
            };
            let mut r = Router::new();
            r.navigate_to(Page::Watch(video_id.clone()));
            let _ = player.play_embedded_at(&video_id, 24, 78, 940, 510);
            let yt_clone = youtube.clone();
            (
                r,
                Task::perform(
                    async move {
                        yt_clone
                            .get_video_details(&video_id)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::VideoDetailsLoaded,
                ),
            )
        } else {
            let yt_clone = youtube.clone();
            (
                Router::new(),
                Task::perform(
                    async move {
                        yt_clone
                            .search("Trending", &Default::default())
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::TrendingLoaded,
                ),
            )
        };

        (
            Self {
                router,
                search_input: String::new(),
                sidebar_expanded: true,
                trending_videos: Vec::new(),
                search_results: Vec::new(),
                active_video: None,
                thumbnail_cache: HashMap::new(),
                youtube,
                player,
            },
            initial_task,
        )
    }

    pub fn title(&self) -> String {
        String::from("Rustube - Native YouTube Client")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(page) => {
                // If moving away from Watch page, stop MPV and clean up child window
                if matches!(self.router.current, Page::Watch(_)) && !matches!(page, Page::Watch(_)) {
                    self.player.stop();
                }
                self.router.navigate_to(page);
                Task::none()
            }
            Message::GoBack => {
                if matches!(self.router.current, Page::Watch(_)) {
                    self.player.stop();
                }
                self.router.go_back();
                Task::none()
            }
            Message::GoForward => {
                if matches!(self.router.current, Page::Watch(_)) {
                    self.player.stop();
                }
                self.router.go_forward();
                Task::none()
            }
            Message::ToggleSidebar => {
                self.sidebar_expanded = !self.sidebar_expanded;
                Task::none()
            }
            Message::SearchInputChanged(query) => {
                self.search_input = query;
                Task::none()
            }
            Message::SubmitSearch => {
                if matches!(self.router.current, Page::Watch(_)) {
                    self.player.stop();
                }
                let query = self.search_input.clone();
                let yt = self.youtube.clone();
                self.router.navigate_to(Page::Search(query.clone()));
                Task::perform(
                    async move {
                        yt.search(&query, &Default::default())
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::SearchResultsLoaded,
                )
            }
            Message::SearchResultsLoaded(Ok(results)) => {
                self.search_results = results;
                // Fetch thumbnails for search results
                let tasks: Vec<Task<Message>> = self.search_results.iter().map(|v| {
                    let url = v.thumbnail_url.clone();
                    Task::perform(
                        async move {
                            if url.is_empty() {
                                return Err("empty url".to_string());
                            }
                            let bytes = reqwest::get(&url).await.map_err(|e| e.to_string())?
                                .bytes().await.map_err(|e| e.to_string())?
                                .to_vec();
                            Ok((url, bytes))
                        },
                        |res| match res {
                            Ok((url, bytes)) => Message::ThumbnailLoaded(url, bytes),
                            Err(_) => Message::Noop,
                        },
                    )
                }).collect();
                Task::batch(tasks)
            }
            Message::SearchResultsLoaded(Err(err)) => {
                tracing::error!("Search error: {}", err);
                Task::none()
            }
            Message::TrendingLoaded(Ok(videos)) => {
                self.trending_videos = videos;
                // Fetch thumbnails for trending videos
                let tasks: Vec<Task<Message>> = self.trending_videos.iter().map(|v| {
                    let url = v.thumbnail_url.clone();
                    Task::perform(
                        async move {
                            if url.is_empty() {
                                return Err("empty url".to_string());
                            }
                            let bytes = reqwest::get(&url).await.map_err(|e| e.to_string())?
                                .bytes().await.map_err(|e| e.to_string())?
                                .to_vec();
                            Ok((url, bytes))
                        },
                        |res| match res {
                            Ok((url, bytes)) => Message::ThumbnailLoaded(url, bytes),
                            Err(_) => Message::Noop,
                        },
                    )
                }).collect();
                Task::batch(tasks)
            }
            Message::TrendingLoaded(Err(err)) => {
                tracing::error!("Trending load error: {}", err);
                Task::none()
            }
            Message::ThumbnailLoaded(url, bytes) => {
                self.thumbnail_cache.insert(url, bytes);
                Task::none()
            }
            Message::OpenVideo(id) => {
                self.router.navigate_to(Page::Watch(id.clone()));
                self.active_video = None;
                // Play embedded video at standard player coordinates (x=24, y=78, w=940, h=510)
                let _ = self.player.play_embedded_at(&id, 24, 78, 940, 510);
                let yt = self.youtube.clone();
                Task::perform(
                    async move {
                        yt.get_video_details(&id)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::VideoDetailsLoaded,
                )
            }
            Message::VideoDetailsLoaded(Ok(details)) => {
                self.active_video = Some(details);
                Task::none()
            }
            Message::VideoDetailsLoaded(Err(err)) => {
                tracing::error!("Failed to fetch video details: {}", err);
                Task::none()
            }
            Message::PlayPauseToggle => {
                self.player.toggle_pause();
                Task::none()
            }
            Message::SeekRelative(seconds) => {
                self.player.seek_relative(seconds);
                Task::none()
            }
            Message::SeekPercent(percent) => {
                self.player.seek_percent(percent);
                Task::none()
            }
            Message::SetVolume(vol) => {
                self.player.set_volume(vol);
                Task::none()
            }
            Message::PlayerToggleFullscreen => {
                self.player.toggle_fullscreen();
                Task::none()
            }
            Message::SubscribeToggle => {
                tracing::info!("Toggle subscribe clicked");
                Task::none()
            }
            Message::LikeToggle => {
                tracing::info!("Toggle like clicked");
                Task::none()
            }
            Message::FilterCategorySelected(category) => {
                let yt = self.youtube.clone();
                let query = if category == "All" { "Trending".to_string() } else { category };
                Task::perform(
                    async move {
                        yt.search(&query, &Default::default())
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::TrendingLoaded,
                )
            }
            Message::Noop => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let top = top_bar::view(
            &self.search_input,
            self.router.can_go_back(),
            self.router.can_go_forward(),
        );

        let main_view: Element<Message> = match &self.router.current {
            Page::Home => home::view(&self.trending_videos, &self.thumbnail_cache),
            Page::Search(q) => search::view(q, &self.search_results, &self.thumbnail_cache),
            Page::Watch(_) => watch::view(self.active_video.as_ref(), &self.trending_videos, &self.thumbnail_cache),
            _ => home::view(&self.trending_videos, &self.thumbnail_cache),
        };

        let is_watch_page = matches!(self.router.current, Page::Watch(_));
        let body: Element<Message> = if is_watch_page {
            main_view
        } else {
            row![sidebar::view(self.sidebar_expanded), main_view]
                .spacing(12)
                .width(Length::Fill)
                .into()
        };

        column![top, body]
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

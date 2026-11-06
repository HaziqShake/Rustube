use std::collections::HashMap;
use iced::widget::{column, row};
use iced::{Element, Length, Size, Subscription, Task};
use rustube_core::player::PlayerController;
use rustube_core::youtube::client::YouTubeClient;
use rustube_core::youtube::models::{VideoDetails, VideoItem};

use crate::message::Message;
use crate::pages::{home, search, watch};
use crate::router::{Page, Router};
use crate::utils::images;
use crate::widgets::{sidebar, top_bar};

/// Geometry of the embedded MPV stage on the watch page, derived from the
/// window size so the video always sits exactly where the UI draws it.
/// Watch page: 24px left padding, 80px top (56px masthead + 24px page padding),
/// primary column is 70% of available width up to 1280px, stage is 16:9.
pub fn player_stage_geometry(window: Size) -> (i32, i32, u32, u32) {
    let available_w = (window.width - 72.0).max(800.0);
    let primary_w = (available_w * 0.70).min(1280.0);
    let stage_h = primary_w * 9.0 / 16.0;
    (24, 80, primary_w as u32, stage_h as u32)
}

pub struct RustubeApp {
    router: Router,
    search_input: String,
    sidebar_expanded: bool,
    active_category: String,
    window_size: Size,
    trending_videos: Vec<VideoItem>,
    search_results: Vec<VideoItem>,
    active_video: Option<VideoDetails>,
    /// Thumbnail URL of the video open on the watch page, so the stage
    /// can show its thumbnail while MPV spins up. (The thumbnail cache
    /// is keyed by URL, and `VideoDetails` carries no thumbnail.)
    active_thumb_url: Option<String>,
    /// Decoded thumbnail handles, keyed by thumbnail URL. Handles are
    /// created once when the image arrives and cloned (cheap) in `view`,
    /// instead of re-decoding the bytes on every frame.
    thumbnail_cache: HashMap<String, iced::widget::image::Handle>,
    /// Circular-cropped channel avatars, keyed by avatar URL.
    avatar_cache: HashMap<String, iced::widget::image::Handle>,
    youtube: YouTubeClient,
    player: PlayerController,
    player_position: f64,
    player_duration: f64,
    player_paused: bool,
    volume: u32,
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
            let (x, y, w, h) = player_stage_geometry(Size::new(1440.0, 900.0));
            let player_clone = player.clone();
            let vid = video_id.clone();
            let start_player = Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || {
                        let _ = player_clone.play_embedded_at(&vid, x, y, w, h);
                    })
                    .await
                    .ok();
                },
                |_| Message::Noop,
            );
            let yt_clone = youtube.clone();
            let yt_clone2 = youtube.clone();
            (
                r,
                Task::batch([
                    start_player,
                    Task::perform(
                        async move {
                            yt_clone
                                .get_video_details(&video_id)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::VideoDetailsLoaded,
                    ),
                    Task::perform(
                        async move {
                            yt_clone2
                                .search("Trending", &Default::default())
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::TrendingLoaded,
                    ),
                ]),
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
                active_category: "All".to_string(),
                window_size: Size::new(1440.0, 900.0),
                trending_videos: Vec::new(),
                search_results: Vec::new(),
                active_video: None,
                active_thumb_url: None,
                thumbnail_cache: HashMap::new(),
                avatar_cache: HashMap::new(),
                youtube,
                player,
                player_position: 0.0,
                player_duration: 0.0,
                player_paused: false,
                volume: 100,
            },
            initial_task,
        )
    }

    pub fn title(&self) -> String {
        String::from("Rustube - Native YouTube Client")
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::window::resize_events()
                .map(|(_id, size)| Message::WindowResized(size)),
            iced::time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::PlayerTick),
        ])
    }

    /// Fetch thumbnails (rounded 12px) and channel avatars (circular) for a
    /// list of videos. Images are processed once here, then stored as
    /// handles; nothing is decoded during `view`.
    fn fetch_images(&self, videos: &[VideoItem]) -> Task<Message> {
        let mut tasks: Vec<Task<Message>> = Vec::new();

        for v in videos {
            let url = v.thumbnail_url.clone();
            if !url.is_empty() && !self.thumbnail_cache.contains_key(&url) {
                tasks.push(Task::perform(
                    async move {
                        let bytes = reqwest::get(&url)
                            .await
                            .map_err(|e| e.to_string())?
                            .bytes()
                            .await
                            .map_err(|e| e.to_string())?
                            .to_vec();
                        let processed = images::rounded(&bytes, 12)
                            .ok_or_else(|| "thumbnail decode failed".to_string())?;
                        Ok::<(String, Vec<u8>), String>((url, processed))
                    },
                    |res| match res {
                        Ok((url, bytes)) => Message::ThumbnailLoaded(url, bytes),
                        Err(_) => Message::Noop,
                    },
                ));
            }
        }

        // Avatars, deduplicated by URL.
        let mut seen: Vec<String> = Vec::new();
        for v in videos {
            if let Some(url) = &v.channel.avatar_url {
                if !url.is_empty()
                    && !self.avatar_cache.contains_key(url)
                    && !seen.contains(url)
                {
                    seen.push(url.clone());
                    let url = url.clone();
                    tasks.push(Task::perform(
                        async move {
                            let bytes = reqwest::get(&url)
                                .await
                                .map_err(|e| e.to_string())?
                                .bytes()
                                .await
                                .map_err(|e| e.to_string())?
                                .to_vec();
                            let processed = images::circle(&bytes)
                                .ok_or_else(|| "avatar decode failed".to_string())?;
                            Ok::<(String, Vec<u8>), String>((url, processed))
                        },
                        |res| match res {
                            Ok((url, bytes)) => Message::AvatarLoaded(url, bytes),
                            Err(_) => Message::Noop,
                        },
                    ));
                }
            }
        }

        Task::batch(tasks)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(page) => {
                if matches!(self.router.current, Page::Watch(_)) && !matches!(page, Page::Watch(_)) {
                    self.player.stop();
                }
                self.router.navigate_to(page);
                Task::none()
            }
            Message::ToggleSidebar => {
                self.sidebar_expanded = !self.sidebar_expanded;
                Task::none()
            }
            Message::WindowResized(size) => {
                self.window_size = size;
                if matches!(self.router.current, Page::Watch(_)) {
                    let (x, y, w, h) = player_stage_geometry(size);
                    self.player.update_geometry(x, y, w, h);
                }
                Task::none()
            }
            Message::PlayerTick => {
                if matches!(self.router.current, Page::Watch(_)) {
                    let (pos, dur, paused) = self.player.query_state();
                    if let Some(pos) = pos {
                        self.player_position = pos;
                    }
                    if let Some(dur) = dur {
                        self.player_duration = dur;
                    }
                    if let Some(paused) = paused {
                        self.player_paused = paused;
                    }
                }
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
                self.fetch_images(&self.search_results)
            }
            Message::SearchResultsLoaded(Err(err)) => {
                tracing::error!("Search error: {}", err);
                Task::none()
            }
            Message::TrendingLoaded(Ok(videos)) => {
                self.trending_videos = videos;
                self.fetch_images(&self.trending_videos)
            }
            Message::TrendingLoaded(Err(err)) => {
                tracing::error!("Trending load error: {}", err);
                Task::none()
            }
            Message::ThumbnailLoaded(url, bytes) => {
                self.thumbnail_cache
                    .insert(url, iced::widget::image::Handle::from_bytes(bytes));
                Task::none()
            }
            Message::AvatarLoaded(url, bytes) => {
                self.avatar_cache
                    .insert(url, iced::widget::image::Handle::from_bytes(bytes));
                Task::none()
            }
            Message::OpenVideo(id) => {
                self.active_thumb_url = self
                    .trending_videos
                    .iter()
                    .chain(self.search_results.iter())
                    .find(|v| v.id == id)
                    .map(|v| v.thumbnail_url.clone());
                self.router.navigate_to(Page::Watch(id.clone()));
                self.active_video = None;
                self.player_position = 0.0;
                self.player_duration = 0.0;
                self.player_paused = false;
                let (x, y, w, h) = player_stage_geometry(self.window_size);
                let player = self.player.clone();
                let vid = id.clone();
                let start_player = Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let _ = player.play_embedded_at(&vid, x, y, w, h);
                        })
                        .await
                        .ok();
                    },
                    |_| Message::Noop,
                );
                let yt = self.youtube.clone();
                let fetch_details = Task::perform(
                    async move {
                        yt.get_video_details(&id)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    Message::VideoDetailsLoaded,
                );
                if self.trending_videos.is_empty() {
                    let yt2 = self.youtube.clone();
                    let fetch_trending = Task::perform(
                        async move {
                            yt2.search("Trending", &Default::default())
                                .await
                                .map_err(|e| e.to_string())
                        },
                        Message::TrendingLoaded,
                    );
                    Task::batch([start_player, fetch_details, fetch_trending])
                } else {
                    Task::batch([start_player, fetch_details])
                }
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
                self.player_paused = !self.player_paused;
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
                self.volume = vol;
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
                self.active_category = category.clone();
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
        let top = top_bar::view(&self.search_input);

        let main_view: Element<Message> = match &self.router.current {
            Page::Home => home::view(
                &self.trending_videos,
                &self.thumbnail_cache,
                &self.avatar_cache,
                self.window_size.width,
                self.sidebar_expanded,
                &self.active_category,
            ),
            Page::Search(q) => search::view(
                q,
                &self.search_results,
                &self.thumbnail_cache,
                &self.avatar_cache,
            ),
            Page::Watch(_) => watch::view(
                self.active_video.as_ref(),
                &self.trending_videos,
                &self.thumbnail_cache,
                self.active_thumb_url
                    .as_ref()
                    .and_then(|url| self.thumbnail_cache.get(url)),
                self.window_size.width,
                watch::PlayerViewState {
                    position: self.player_position,
                    duration: self.player_duration,
                    paused: self.player_paused,
                    volume: self.volume,
                },
            ),
            _ => home::view(
                &self.trending_videos,
                &self.thumbnail_cache,
                &self.avatar_cache,
                self.window_size.width,
                self.sidebar_expanded,
                &self.active_category,
            ),
        };

        // YouTube's masthead, sidebar and content are flush: no gaps.
        let is_watch_page = matches!(self.router.current, Page::Watch(_));
        let body: Element<Message> = if is_watch_page {
            main_view
        } else {
            row![sidebar::view(self.sidebar_expanded), main_view]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        column![top, body]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

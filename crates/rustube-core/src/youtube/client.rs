use std::sync::Arc;
use rustypipe::client::RustyPipe;
use rustypipe::model::{Thumbnail, YouTubeItem};
use thiserror::Error;

use super::models::*;

/// Pick the smallest thumbnail that is at least `min_width` wide.
/// YouTube serves the same image at 120 (default), 320 (mqdefault),
/// 480 (hqdefault), 640 (sddefault) and 1280 (maxres) px. Taking the
/// first entry used to grab the 120px one and upscale it ~3x, which is
/// why thumbnails looked blurry. Grid cards are ~360px wide, so 320px
/// (mqdefault) is the right, lightweight choice; fall back to the
/// largest available when nothing meets the minimum.
pub fn pick_thumbnail(thumbs: &[Thumbnail], min_width: u32) -> String {
    let mut best: Option<&Thumbnail> = None;
    for t in thumbs {
        if t.width >= min_width && best.map_or(true, |b| t.width < b.width) {
            best = Some(t);
        }
    }
    if let Some(t) = best {
        return t.url.clone();
    }
    thumbs
        .iter()
        .max_by_key(|t| t.width)
        .map(|t| t.url.clone())
        .unwrap_or_default()
}

#[derive(Error, Debug)]
pub enum YouTubeError {
    #[error("RustyPipe error: {0}")]
    RustyPipe(#[from] rustypipe::error::Error),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Video not found: {0}")]
    NotFound(String),
    #[error("Custom error: {0}")]
    Other(String),
}

#[derive(Clone)]
pub struct YouTubeClient {
    client: Arc<RustyPipe>,
}

impl YouTubeClient {
    pub fn new() -> Result<Self, YouTubeError> {
        let client = RustyPipe::new();
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn inner(&self) -> &RustyPipe {
        &self.client
    }

    pub async fn search(&self, query: &str, _filter: &SearchFilter) -> Result<Vec<VideoItem>, YouTubeError> {
        let search_res = self.client.query().search(query).await?;
        let mut results = Vec::new();

        for item in search_res.items.items {
            if let YouTubeItem::Video(v) = item {
                let thumbnail_url = pick_thumbnail(&v.thumbnail, 320);
                let (chan_id, chan_name, avatar_url) = if let Some(ch) = v.channel {
                    let av = ch.avatar.first().map(|a| a.url.clone());
                    (ch.id, ch.name, av)
                } else {
                    (String::new(), String::new(), None)
                };

                results.push(VideoItem {
                    id: v.id,
                    title: v.name,
                    channel: ChannelBadge {
                        id: chan_id,
                        name: chan_name,
                        avatar_url,
                        is_verified: false,
                    },
                    thumbnail_url,
                    duration: v.duration.map(|secs| std::time::Duration::from_secs(secs as u64)),
                    view_count: v.view_count,
                    published_text: v.publish_date_txt.unwrap_or_default(),
                    is_live: false,
                });
            }
        }

        Ok(results)
    }

    pub async fn get_video_details(&self, id: &str) -> Result<VideoDetails, YouTubeError> {
        let player = self.client.query().player(id).await?;
        let mut video_details = VideoDetails {
            id: player.details.id.clone(),
            title: player.details.name.clone().unwrap_or_default(),
            description: player.details.description.clone().unwrap_or_default(),
            channel: ChannelBadge {
                id: player.details.channel_id.clone(),
                name: player.details.channel_name.clone().unwrap_or_default(),
                avatar_url: None,
                is_verified: false,
            },
            view_count: player.details.view_count.unwrap_or(0),
            like_count: None,
            subscriber_count: None,
            published_date: None,
            tags: player.details.keywords.clone(),
            chapters: Vec::new(),
            streams: player
                .video_streams
                .iter()
                .chain(player.video_only_streams.iter())
                .map(|s| StreamInfo {
                    url: s.url.clone(),
                    quality: s.quality.clone(),
                    mime_type: s.mime.clone(),
                    bitrate: s.bitrate as u64,
                    is_audio_only: false,
                    codec: format!("{:?}", s.codec),
                    content_length: s.size,
                })
                .chain(player.audio_streams.iter().map(|s| StreamInfo {
                    url: s.url.clone(),
                    quality: format!("{:?}", s.format),
                    mime_type: s.mime.clone(),
                    bitrate: s.bitrate as u64,
                    is_audio_only: true,
                    codec: format!("{:?}", s.codec),
                    content_length: Some(s.size),
                }))
                .collect(),
            captions: player
                .subtitles
                .iter()
                .map(|c| CaptionTrack {
                    language: c.lang.clone(),
                    url: c.url.clone(),
                    is_auto_generated: false,
                })
                .collect(),
        };

        // Best effort: show the real subscriber count instead of a made-up
        // number. A hidden count (or a failed lookup) just hides the line.
        if let Ok(ch) = self
            .client
            .query()
            .channel_info(&video_details.channel.id)
            .await
        {
            video_details.subscriber_count = ch.subscriber_count;
        }

        Ok(video_details)
    }

    pub async fn get_channel(&self, id: &str) -> Result<ChannelInfo, YouTubeError> {
        let ch = self.client.query().channel_info(id).await?;
        Ok(ChannelInfo {
            id: ch.id,
            name: ch.url,
            description: ch.description,
            avatar_url: None,
            banner_url: None,
            subscriber_count: ch.subscriber_count,
            video_count: ch.video_count,
        })
    }

    pub async fn get_search_suggestions(&self, query: &str) -> Result<Vec<String>, YouTubeError> {
        let suggestions = self.client.query().search_suggestion(query).await?;
        Ok(suggestions)
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoItem {
    pub id: String,
    pub title: String,
    pub channel: ChannelBadge,
    pub thumbnail_url: String,
    pub duration: Option<Duration>,
    pub view_count: Option<u64>,
    pub published_text: String,
    pub is_live: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelBadge {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetails {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel: ChannelBadge,
    pub view_count: u64,
    pub like_count: Option<u64>,
    pub published_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub chapters: Vec<Chapter>,
    pub streams: Vec<StreamInfo>,
    pub captions: Vec<CaptionTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub url: String,
    pub quality: String,
    pub mime_type: String,
    pub bitrate: u64,
    pub is_audio_only: bool,
    pub codec: String,
    pub content_length: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub start_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionTrack {
    pub language: String,
    pub url: String,
    pub is_auto_generated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub author_avatar: Option<String>,
    pub text: String,
    pub like_count: u64,
    pub published_text: String,
    pub reply_count: u64,
    pub replies: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub subscriber_count: Option<u64>,
    pub video_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub channel: Option<ChannelBadge>,
    pub video_count: usize,
    pub thumbnail_url: Option<String>,
    pub videos: Vec<VideoItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilter {
    pub sort_by: Option<String>,
    pub upload_date: Option<String>,
    pub video_type: Option<String>,
    pub duration_filter: Option<String>,
}

//! # rustube-core
//!
//! Core library for the Rustube YouTube desktop client.
//!
//! This crate provides the backend functionality including:
//! - YouTube API integration via `rustypipe`
//! - MPV player control via IPC
//! - SQLite storage for subscriptions, history, and bookmarks
//! - Thumbnail disk cache with LRU eviction
//! - TOML-based configuration
//! - Subscription management with OPML import/export
//! - Download manager with format handling
//! - OAuth2 authentication stubs

pub mod auth;
pub mod download;
pub mod player;
pub mod storage;
pub mod subscription;
pub mod youtube;

/// Re-export commonly used types.
pub mod prelude {
    pub use crate::youtube::client::YouTubeClient;
    pub use crate::youtube::models::*;

    pub use crate::player::controller::{PlayerController, PlayerEvent, PlayerState};
    pub use crate::player::sponsorblock::SponsorBlockClient;

    pub use crate::storage::cache::ThumbnailCache;
    pub use crate::storage::config::AppConfig;
    pub use crate::storage::database::Database;

    pub use crate::subscription::manager::SubscriptionManager;

    pub use crate::download::manager::{DownloadFormat, DownloadManager, DownloadStatus, DownloadTask};
}

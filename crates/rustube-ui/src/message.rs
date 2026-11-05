use rustube_core::youtube::models::{VideoDetails, VideoItem};
use crate::router::Page;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    GoBack,
    GoForward,
    ToggleSidebar,

    SearchInputChanged(String),
    SubmitSearch,
    SearchResultsLoaded(Result<Vec<VideoItem>, String>),

    TrendingLoaded(Result<Vec<VideoItem>, String>),
    OpenVideo(String),
    VideoDetailsLoaded(Result<VideoDetails, String>),

    ThumbnailLoaded(String, Vec<u8>),

    PlayPauseToggle,
    SeekRelative(f64),
    SeekPercent(f64),
    SetVolume(u32),
    PlayerToggleFullscreen,

    SubscribeToggle,
    LikeToggle,
    FilterCategorySelected(String),

    Noop,
}

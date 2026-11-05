use iced::Size;
use rustube_core::youtube::models::{VideoDetails, VideoItem};
use crate::router::Page;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    ToggleSidebar,

    SearchInputChanged(String),
    SubmitSearch,
    SearchResultsLoaded(Result<Vec<VideoItem>, String>),

    TrendingLoaded(Result<Vec<VideoItem>, String>),
    OpenVideo(String),
    VideoDetailsLoaded(Result<VideoDetails, String>),

    ThumbnailLoaded(String, Vec<u8>),
    AvatarLoaded(String, Vec<u8>),

    WindowResized(Size),
    PlayerTick,

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

use crate::youtube::models::VideoItem;

pub struct FeedBuilder;

impl FeedBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn sort_by_date(&self, mut videos: Vec<VideoItem>) -> Vec<VideoItem> {
        videos.sort_by(|a, b| b.published_text.cmp(&a.published_text));
        videos
    }
}

impl Default for FeedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

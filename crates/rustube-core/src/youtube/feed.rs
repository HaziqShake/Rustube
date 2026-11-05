use super::client::{YouTubeClient, YouTubeError};
use super::models::VideoItem;

pub async fn get_trending(client: &YouTubeClient) -> Result<Vec<VideoItem>, YouTubeError> {
    client.search("Trending", &Default::default()).await
}

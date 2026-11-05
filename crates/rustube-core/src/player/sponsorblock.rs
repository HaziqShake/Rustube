use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SponsorSegment {
    pub category: String,
    pub action_type: String,
    pub segment: (f64, f64),
    pub uuid: String,
}

pub struct SponsorBlockClient {
    client: reqwest::Client,
    base_url: String,
}

impl SponsorBlockClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://sponsor.ajay.app/api".to_string(),
        }
    }

    pub async fn get_segments(&self, video_id: &str) -> anyhow::Result<Vec<SponsorSegment>> {
        let url = format!("{}/skipSegments?videoID={}", self.base_url, video_id);
        let resp = self.client.get(&url).send().await?;
        if resp.status().is_success() {
            let segments: Vec<SponsorSegment> = resp.json().await?;
            Ok(segments)
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for SponsorBlockClient {
    fn default() -> Self {
        Self::new()
    }
}

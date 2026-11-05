use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadFormat {
    Mp4,
    Mp3,
    WebM,
    M4a,
    Opus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Muxing,
    Complete,
    Failed(String),
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub video_id: String,
    pub title: String,
    pub format: DownloadFormat,
    pub quality: String,
    pub status: DownloadStatus,
    pub progress: f32,
    pub file_path: PathBuf,
}

pub struct DownloadManager {
    tasks: Arc<Mutex<Vec<DownloadTask>>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn queue_download(&self, video_id: &str, title: &str, format: DownloadFormat, quality: &str, dest_dir: PathBuf) -> DownloadTask {
        let ext = match format {
            DownloadFormat::Mp4 => "mp4",
            DownloadFormat::Mp3 => "mp3",
            DownloadFormat::WebM => "webm",
            DownloadFormat::M4a => "m4a",
            DownloadFormat::Opus => "opus",
        };
        let file_path = dest_dir.join(format!("{}.{}", title, ext));
        let task = DownloadTask {
            id: format!("{}_{}", video_id, chrono::Utc::now().timestamp()),
            video_id: video_id.to_string(),
            title: title.to_string(),
            format,
            quality: quality.to_string(),
            status: DownloadStatus::Pending,
            progress: 0.0,
            file_path,
        };

        let mut lock = self.tasks.lock().unwrap();
        lock.push(task.clone());
        task
    }

    pub fn get_tasks(&self) -> Vec<DownloadTask> {
        self.tasks.lock().unwrap().clone()
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub default_quality: String,
    pub sponsorblock_enabled: bool,
    pub hover_preview_enabled: bool,
    pub download_path: String,
    pub cache_size_mb: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            default_quality: "1080p".to_string(),
            sponsorblock_enabled: true,
            hover_preview_enabled: false,
            download_path: "~/Downloads/Rustube".to_string(),
            cache_size_mb: 200,
        }
    }
}

impl AppConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

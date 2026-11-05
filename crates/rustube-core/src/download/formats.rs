use super::manager::DownloadFormat;

pub fn get_extension(format: DownloadFormat) -> &'static str {
    match format {
        DownloadFormat::Mp4 => "mp4",
        DownloadFormat::Mp3 => "mp3",
        DownloadFormat::WebM => "webm",
        DownloadFormat::M4a => "m4a",
        DownloadFormat::Opus => "opus",
    }
}

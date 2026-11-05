use super::models::StreamInfo;

pub fn select_best_video_audio(
    streams: &[StreamInfo],
    _preferred_quality: &str,
) -> (Option<StreamInfo>, Option<StreamInfo>) {
    let best_video = streams
        .iter()
        .filter(|s| !s.is_audio_only)
        .max_by_key(|s| s.bitrate)
        .cloned();

    let best_audio = streams
        .iter()
        .filter(|s| s.is_audio_only)
        .max_by_key(|s| s.bitrate)
        .cloned();

    (best_video, best_audio)
}

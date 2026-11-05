use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}

pub fn format_view_count(views: u64) -> String {
    if views >= 1_000_000_000 {
        format!("{:.1}B views", views as f64 / 1_000_000_000.0)
    } else if views >= 1_000_000 {
        format!("{:.1}M views", views as f64 / 1_000_000.0)
    } else if views >= 1_000 {
        format!("{:.1}K views", views as f64 / 1_000.0)
    } else {
        format!("{} views", views)
    }
}

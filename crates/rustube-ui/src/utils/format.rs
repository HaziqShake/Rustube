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
    format!("{} views", format_count(views))
}

/// Compact count without a suffix, e.g. `1.8M` for likes,
/// `4.56M subscribers` is built from this in the watch page.
pub fn format_count(count: u64) -> String {
    if count >= 1_000_000_000 {
        trim_float(count as f64 / 1_000_000_000.0, "B")
    } else if count >= 1_000_000 {
        trim_float(count as f64 / 1_000_000.0, "M")
    } else if count >= 1_000 {
        trim_float(count as f64 / 1_000.0, "K")
    } else {
        format!("{}", count)
    }
}

fn trim_float(value: f64, suffix: &str) -> String {
    let s = format!("{:.2}", value);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    format!("{}{}", s, suffix)
}

pub fn format_subscriber_count(count: u64) -> String {
    format!("{} subscribers", format_count(count))
}

/// Clean title/text to remove emojis and non-standard symbols that the bundled Roboto font lacks,
/// preventing ugly missing-glyph boxes like `[]`.
pub fn clean_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let code = ch as u32;
        let is_emoji_or_missing = (0x1F300..=0x1FAFF).contains(&code)
            || (0x2600..=0x27BF).contains(&code)
            || (0xFE00..=0xFE0F).contains(&code)
            || (0x200D == code)
            || (0x2300..=0x23FF).contains(&code)
            || (0x2B50..=0x2B55).contains(&code);
        if !is_emoji_or_missing && !ch.is_control() {
            out.push(ch);
        }
    }
    let mut cleaned = String::with_capacity(out.len());
    let mut last_was_space = false;
    for ch in out.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                cleaned.push(' ');
                last_was_space = true;
            }
        } else {
            cleaned.push(ch);
            last_was_space = false;
        }
    }
    cleaned.trim().to_string()
}

/// "3 days ago" style relative time for the watch page description.
/// Returns None when the date is missing.
pub fn format_relative(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let secs = (now - date).num_seconds().max(0);
    if secs < 60 {
        return "just now".to_string();
    }
    let mins = secs / 60;
    if mins < 60 {
        return format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" });
    }
    let hours = mins / 60;
    if hours < 24 {
        return format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" });
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{} day{} ago", days, if days == 1 { "" } else { "s" });
    }
    let weeks = days / 7;
    if weeks < 5 {
        return format!("{} week{} ago", weeks, if weeks == 1 { "" } else { "s" });
    }
    let months = days / 30;
    if months < 12 {
        return format!("{} month{} ago", months, if months == 1 { "" } else { "s" });
    }
    let years = days / 365;
    format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
}

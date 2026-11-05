use std::collections::HashMap;
use iced::widget::{column, scrollable, text};
use iced::Element;
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::widgets::video_card;

pub fn view<'a>(
    query: &'a str,
    results: &'a [VideoItem],
    thumbnails: &'a HashMap<String, Vec<u8>>,
) -> Element<'a, Message> {
    if results.is_empty() {
        return text(format!("No search results found for: {}", query))
            .size(16)
            .into();
    }

    let mut col = column![text(format!("Results for \"{}\"", query)).size(18)].spacing(14);
    for v in results {
        let thumb = thumbnails.get(&v.thumbnail_url);
        col = col.push(video_card::view(v, thumb));
    }

    scrollable(col).into()
}

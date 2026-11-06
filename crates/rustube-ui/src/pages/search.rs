use std::collections::HashMap;
use iced::widget::{button, column, container, image, row, scrollable, stack, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::theme;
use crate::utils::format::{clean_text, format_duration, format_view_count};
use crate::widgets::video_card;

/// YouTube search results: horizontal rows, 360x202 thumbnail on the
/// left, 18px title, channel row with avatar, and 12px metadata on the right.
pub fn view<'a>(
    query: &'a str,
    results: &'a [VideoItem],
    thumbnails: &'a HashMap<String, iced::widget::image::Handle>,
    avatars: &'a HashMap<String, iced::widget::image::Handle>,
) -> Element<'a, Message> {
    if results.is_empty() {
        return container(
            text(format!("No results found for \"{}\"", query))
                .size(16)
                .color(theme::TEXT_SECONDARY),
        )
        .padding(32)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into();
    }

    let mut col = column![].spacing(16).width(Length::Fill);
    for v in results {
        let thumb = thumbnails.get(&v.thumbnail_url);
        let avatar = v
            .channel
            .avatar_url
            .as_ref()
            .and_then(|url| avatars.get(url));
        col = col.push(search_row(v, thumb, avatar));
    }

    scrollable(
        container(col)
            .padding(iced::Padding {
                top: 16.0,
                right: 24.0,
                bottom: 32.0,
                left: 24.0,
            })
            .width(Length::Fill)
            .max_width(1280.0),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn search_row<'a>(
    video: &'a VideoItem,
    thumbnail: Option<&'a image::Handle>,
    avatar_handle: Option<&'a image::Handle>,
) -> Element<'a, Message> {
    let dur_text = video
        .duration
        .map(format_duration)
        .unwrap_or_else(|| "LIVE".to_string());

    let thumb_elem: Element<'a, Message> = if let Some(handle) = thumbnail {
        container(
            image(handle.clone())
                .width(Length::Fixed(360.0))
                .height(Length::Fixed(202.0)),
        )
        .width(Length::Fixed(360.0))
        .height(Length::Fixed(202.0))
        .into()
    } else {
        container(text("").size(12))
            .width(Length::Fixed(360.0))
            .height(Length::Fixed(202.0))
            .style(theme::solid_container(theme::THUMB_PLACEHOLDER, 12.0))
            .into()
    };

    let duration_badge = container(
        text(dur_text)
            .size(12)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            }),
    )
    .padding([3, 4])
    .style(theme::solid_container(
        if video.is_live { theme::RED } else { theme::BADGE_BG },
        4.0,
    ));

    let thumb_view: Element<'a, Message> = stack![
        thumb_elem,
        container(duration_badge)
            .width(Length::Fixed(360.0))
            .height(Length::Fixed(202.0))
            .align_x(Alignment::End)
            .align_y(Alignment::End)
            .padding(iced::Padding {
                top: 0.0,
                right: 6.0,
                bottom: 6.0,
                left: 0.0,
            }),
    ]
    .width(Length::Fixed(360.0))
    .height(Length::Fixed(202.0))
    .into();

    let views_text = video
        .view_count
        .map(format_view_count)
        .unwrap_or_default();
    let meta_text = format!("{} • {}", views_text, video.published_text);
    let clean_title = clean_text(&video.title);

    let details = column![
        text(clean_title)
            .size(18)
            .line_height(iced::Pixels(26.0))
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
        text(meta_text).size(12).color(theme::TEXT_SECONDARY),
        row![
            video_card::avatar(&video.channel.name, avatar_handle, 24.0),
            text(&video.channel.name).size(12).color(theme::TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    ]
    .spacing(8)
    .width(Length::Fill);

    let content = row![thumb_view, details]
        .spacing(16)
        .align_y(Alignment::Start);

    button(content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(theme::ghost_button())
        .padding(0)
        .width(Length::Fill)
        .into()
}

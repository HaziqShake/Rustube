use iced::widget::{button, column, container, image, row, stack, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::theme;
use crate::utils::format::{format_duration, format_view_count};

/// YouTube clamps titles to two lines. Iced has no line clamp, so
/// truncate by character count as an approximation (16px Roboto in a
/// ~340px card fits ~55 chars per line).
fn truncate(title: &str, max_chars: usize) -> String {
    if title.chars().count() <= max_chars {
        return title.to_string();
    }
    let mut s: String = title.chars().take(max_chars.saturating_sub(3)).collect();
    s.push_str("...");
    s
}

fn duration_badge(dur_text: String, live: bool) -> Element<'static, Message> {
    let bg = if live { theme::RED } else { theme::BADGE_BG };
    container(
        text(dur_text)
            .size(12)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            }),
    )
    .padding([3, 4])
    .style(theme::solid_container(bg, 4.0))
    .into()
}

/// 36px channel avatar: the real photo when downloaded, otherwise the
/// initial-letter circle (same fallback YouTube shows for channels
/// without a custom avatar).
pub fn avatar<'a>(
    channel_name: &str,
    avatar_handle: Option<&'a image::Handle>,
    size: f32,
) -> Element<'a, Message> {
    if let Some(handle) = avatar_handle {
        container(
            image(handle.clone())
                .width(Length::Fixed(size))
                .height(Length::Fixed(size)),
        )
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .into()
    } else {
        container(
            text(channel_name.chars().next().unwrap_or('?').to_string())
                .size((size * 0.4) as u16)
                .color(iced::Color::WHITE)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                }),
        )
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(theme::solid_container(theme::AVATAR_BG, size / 2.0))
        .into()
    }
}

/// Standard YouTube grid video card (home feed).
/// `thumb_height` is 16:9 of the card width, computed by the home page
/// from the window size so the grid stays responsive.
pub fn view<'a>(
    video: &'a VideoItem,
    thumbnail: Option<&'a image::Handle>,
    avatar_handle: Option<&'a image::Handle>,
    thumb_height: f32,
) -> Element<'a, Message> {
    let dur_text = video
        .duration
        .map(format_duration)
        .unwrap_or_else(|| "LIVE".to_string());

    let views_text = video
        .view_count
        .map(format_view_count)
        .unwrap_or_default();

    let meta_text = format!("{} • {}", views_text, video.published_text);

    // Thumbnail: the image itself is pre-rounded (12px) at download time.
    // The duration badge is stacked on top, bottom-right, 4px from the
    // edges, instead of sitting underneath the image.
    let thumb_elem: Element<'a, Message> = if let Some(handle) = thumbnail {
        container(
            image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fixed(thumb_height)),
        )
        .width(Length::Fill)
        .height(Length::Fixed(thumb_height))
        .into()
    } else {
        container(text("").size(12))
            .width(Length::Fill)
            .height(Length::Fixed(thumb_height))
            .style(theme::solid_container(theme::THUMB_PLACEHOLDER, 12.0))
            .into()
    };

    let thumbnail_view: Element<'a, Message> = stack![
        thumb_elem,
        container(duration_badge(dur_text, video.is_live))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::End)
            .align_y(Alignment::End)
            .padding(iced::Padding {
                top: 0.0,
                right: 4.0,
                bottom: 4.0,
                left: 0.0,
            }),
    ]
    .width(Length::Fill)
    .height(Length::Fixed(thumb_height))
    .into();

    let text_meta = column![
        text(truncate(&video.title, 95))
            .size(16)
            .line_height(iced::Pixels(22.0))
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
        text(&video.channel.name).size(14).color(theme::TEXT_SECONDARY),
        text(meta_text).size(14).color(theme::TEXT_SECONDARY),
    ]
    .spacing(4)
    .width(Length::Fill);

    let card_content = column![
        thumbnail_view,
        row![avatar(&video.channel.name, avatar_handle, 36.0), text_meta]
            .spacing(12)
            .align_y(Alignment::Start),
    ]
    .spacing(12);

    button(card_content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(theme::ghost_button())
        .padding(0)
        .width(Length::Fill)
        .into()
}

/// Watch-page sidebar compact card: 168x94 thumbnail, no avatar,
/// title 14px Medium, meta 12px. Matches YouTube's "Up next" list.
pub fn compact_view<'a>(
    video: &'a VideoItem,
    thumbnail: Option<&'a image::Handle>,
) -> Element<'a, Message> {
    let dur_text = video
        .duration
        .map(format_duration)
        .unwrap_or_else(|| "LIVE".to_string());

    let views_text = video
        .view_count
        .map(format_view_count)
        .unwrap_or_default();

    let meta_text = format!("{} • {}", views_text, video.published_text);

    let thumb_elem: Element<'a, Message> = if let Some(handle) = thumbnail {
        container(
            image(handle.clone())
                .width(Length::Fixed(168.0))
                .height(Length::Fixed(94.0)),
        )
        .width(Length::Fixed(168.0))
        .height(Length::Fixed(94.0))
        .into()
    } else {
        container(text("").size(11))
            .width(Length::Fixed(168.0))
            .height(Length::Fixed(94.0))
            .style(theme::solid_container(theme::THUMB_PLACEHOLDER, 8.0))
            .into()
    };

    let thumb_view: Element<'a, Message> = stack![
        thumb_elem,
        container(duration_badge(dur_text, video.is_live))
            .width(Length::Fixed(168.0))
            .height(Length::Fixed(94.0))
            .align_x(Alignment::End)
            .align_y(Alignment::End)
            .padding(iced::Padding {
                top: 0.0,
                right: 4.0,
                bottom: 4.0,
                left: 0.0,
            }),
    ]
    .width(Length::Fixed(168.0))
    .height(Length::Fixed(94.0))
    .into();

    let details = column![
        text(truncate(&video.title, 60))
            .size(14)
            .line_height(iced::Pixels(20.0))
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
        text(&video.channel.name).size(12).color(theme::TEXT_SECONDARY),
        text(meta_text).size(12).color(theme::TEXT_SECONDARY),
    ]
    .spacing(4)
    .width(Length::Fill);

    let content = row![thumb_view, details]
        .spacing(8)
        .align_y(Alignment::Start);

    button(content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(theme::ghost_button())
        .padding(0)
        .width(Length::Fill)
        .into()
}

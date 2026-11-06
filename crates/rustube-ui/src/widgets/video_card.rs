use iced::widget::{button, column, container, image, row, stack, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::theme;
use crate::utils::format::{clean_text, format_duration, format_view_count};

/// Truncate text cleanly with ellipsis.
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

/// Channel avatar: real image when available, otherwise clean initial-letter circle.
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
        let initial = channel_name.chars().next().unwrap_or('?').to_uppercase().to_string();
        container(
            text(initial)
                .size((size * 0.42) as u16)
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

/// Standard YouTube grid video card (home & search feed).
/// `thumb_height` is 16:9 of the card width so aspect ratio is preserved.
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

    // 16:9 Thumbnail pre-cropped with 12px rounded corners.
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
                right: 6.0,
                bottom: 6.0,
                left: 0.0,
            }),
    ]
    .width(Length::Fill)
    .height(Length::Fixed(thumb_height))
    .into();

    let clean_title = clean_text(&video.title);
    let title_display = truncate(&clean_title, 55);

    let title_container = container(
        text(title_display)
            .size(15)
            .line_height(iced::Pixels(20.0))
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
    )
    .height(Length::Fixed(42.0))
    .width(Length::Fill);

    let text_meta = column![
        title_container,
        text(&video.channel.name)
            .size(13)
            .color(theme::TEXT_SECONDARY),
        text(meta_text)
            .size(13)
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(3)
    .width(Length::Fill);

    let card_content = column![
        thumbnail_view,
        row![
            container(avatar(&video.channel.name, avatar_handle, 36.0))
                .padding(iced::Padding {
                    top: 2.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                }),
            text_meta,
        ]
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

    let clean_title = clean_text(&video.title);
    let title_display = truncate(&clean_title, 48);

    let title_container = container(
        text(title_display)
            .size(14)
            .line_height(iced::Pixels(19.0))
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
    )
    .height(Length::Fixed(40.0))
    .width(Length::Fill);

    let details = column![
        title_container,
        text(&video.channel.name)
            .size(12)
            .color(theme::TEXT_SECONDARY),
        text(meta_text)
            .size(12)
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(3)
    .width(Length::Fill);

    let content = row![thumb_view, details]
        .spacing(10)
        .align_y(Alignment::Start);

    button(content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(theme::ghost_button())
        .padding(0)
        .width(Length::Fill)
        .into()
}

/// High-fidelity YouTube grid skeleton card for loading states.
/// Preserves exact 16:9 ratio, 12px rounded thumbnail, 36px circular avatar,
/// and realistic 2-line title + channel/meta skeleton bars.
pub fn skeleton<'a>(thumb_height: f32) -> Element<'a, Message> {
    // 16:9 Thumbnail placeholder with 12px rounded corners
    let thumb = container(text("").size(1))
        .width(Length::Fill)
        .height(Length::Fixed(thumb_height))
        .style(theme::solid_container(theme::THUMB_PLACEHOLDER, 12.0));

    // Circular avatar placeholder (36x36, 18px radius)
    let avatar_skeleton = container(text("").size(1))
        .width(Length::Fixed(36.0))
        .height(Length::Fixed(36.0))
        .style(theme::solid_container(theme::CHIP_BG, 18.0));

    // Title line 1: ~90% width, 14px high, 4px radius
    let title_line1 = container(text("").size(1))
        .width(Length::FillPortion(9))
        .height(Length::Fixed(14.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    // Title line 2: ~60% width, 14px high, 4px radius
    let title_line2 = container(text("").size(1))
        .width(Length::FillPortion(6))
        .height(Length::Fixed(14.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    // Channel name placeholder: ~45% width, 12px high, 4px radius
    let channel_line = container(text("").size(1))
        .width(Length::FillPortion(4))
        .height(Length::Fixed(12.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    // Views / meta placeholder: ~30% width, 12px high, 4px radius
    let meta_line = container(text("").size(1))
        .width(Length::FillPortion(3))
        .height(Length::Fixed(12.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    let title_block = column![
        row![title_line1, container(text("").size(1)).width(Length::FillPortion(1))],
        row![title_line2, container(text("").size(1)).width(Length::FillPortion(4))],
    ]
    .spacing(6)
    .width(Length::Fill);

    let text_meta = column![
        title_block,
        row![channel_line, container(text("").size(1)).width(Length::FillPortion(5))],
        row![meta_line, container(text("").size(1)).width(Length::FillPortion(7))],
    ]
    .spacing(6)
    .width(Length::Fill);

    let content = column![
        thumb,
        row![
            container(avatar_skeleton).padding(iced::Padding {
                top: 2.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            }),
            text_meta,
        ]
        .spacing(12)
        .align_y(Alignment::Start),
    ]
    .spacing(12);

    container(content)
        .width(Length::Fill)
        .into()
}

/// Compact skeleton card for watch page sidebar recommendations.
pub fn compact_skeleton<'a>() -> Element<'a, Message> {
    let thumb = container(text("").size(1))
        .width(Length::Fixed(168.0))
        .height(Length::Fixed(94.0))
        .style(theme::solid_container(theme::THUMB_PLACEHOLDER, 8.0));

    let title_line1 = container(text("").size(1))
        .width(Length::FillPortion(9))
        .height(Length::Fixed(13.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    let title_line2 = container(text("").size(1))
        .width(Length::FillPortion(5))
        .height(Length::Fixed(13.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    let channel_line = container(text("").size(1))
        .width(Length::FillPortion(4))
        .height(Length::Fixed(11.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    let meta_line = container(text("").size(1))
        .width(Length::FillPortion(3))
        .height(Length::Fixed(11.0))
        .style(theme::solid_container(theme::CHIP_BG, 4.0));

    let details = column![
        column![
            row![title_line1, container(text("").size(1)).width(Length::FillPortion(1))],
            row![title_line2, container(text("").size(1)).width(Length::FillPortion(5))],
        ]
        .spacing(5),
        row![channel_line, container(text("").size(1)).width(Length::FillPortion(5))],
        row![meta_line, container(text("").size(1)).width(Length::FillPortion(6))],
    ]
    .spacing(5)
    .width(Length::Fill);

    container(
        row![thumb, details]
            .spacing(10)
            .align_y(Alignment::Start),
    )
    .width(Length::Fill)
    .into()
}

use std::collections::HashMap;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::theme;
use crate::widgets::video_card;

/// YouTube home: 56px filter-chip bar, then a responsive grid.
/// Columns come from the window width (cards ~310px minimum, max 5),
/// the same way youtube.com reflows. Gaps are 16px horizontal and
/// 36px vertical; content padding is 24px on the sides.
pub fn view<'a>(
    videos: &'a [VideoItem],
    thumbnails: &'a HashMap<String, iced::widget::image::Handle>,
    avatars: &'a HashMap<String, iced::widget::image::Handle>,
    window_width: f32,
    sidebar_expanded: bool,
    active_category: &str,
) -> Element<'a, Message> {
    if videos.is_empty() {
        return container(
            text("Loading YouTube home feed...")
                .size(16)
                .color(theme::TEXT_SECONDARY),
        )
        .padding(24)
        .into();
    }

    let sidebar_w = if sidebar_expanded { 240.0 } else { 72.0 };
    let content_w = (window_width - sidebar_w - 48.0).max(320.0);
    let columns = ((content_w / 310.0).floor() as usize).clamp(1, 5);
    let card_w = (content_w - (columns as f32 - 1.0) * 16.0) / columns as f32;
    let thumb_h = card_w * 9.0 / 16.0;

    let categories = [
        "All",
        "Gaming",
        "Music",
        "News",
        "Podcasts",
        "Live",
        "Coding",
        "Recently uploaded",
        "Watched",
    ];
    let mut chips = row![].spacing(12).align_y(iced::Alignment::Center);
    for cat in categories {
        chips = chips.push(pill(cat, cat == active_category));
    }
    let filter_bar = container(chips)
        .padding([0, 24])
        .height(Length::Fixed(56.0))
        .width(Length::Fill);

    let mut rows = column![].spacing(36).width(Length::Fill);
    let mut current_row = row![].spacing(16).width(Length::Fill);
    let mut count = 0;

    for video in videos {
        let thumb = thumbnails.get(&video.thumbnail_url);
        let avatar = video
            .channel
            .avatar_url
            .as_ref()
            .and_then(|url| avatars.get(url));
        current_row = current_row.push(
            container(video_card::view(video, thumb, avatar, thumb_h))
                .width(Length::FillPortion(1)),
        );
        count += 1;
        if count % columns == 0 {
            rows = rows.push(current_row);
            current_row = row![].spacing(16).width(Length::Fill);
        }
    }

    if count % columns != 0 {
        rows = rows.push(current_row);
    }

    let page_content = column![
        filter_bar,
        container(rows)
            .padding(iced::Padding {
                top: 16.0,
                right: 24.0,
                bottom: 24.0,
                left: 24.0,
            })
            .width(Length::Fill),
    ]
    .spacing(0)
    .width(Length::Fill);

    scrollable(page_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Filter chip: 32px tall, 8px radius, 14px label. Inactive is `#272727`
/// (hover `#3d3d3d`); active is white with black text.
fn pill<'a>(label: &'static str, active: bool) -> Element<'a, Message> {
    let (bg, hover_bg, fg, weight) = if active {
        (
            theme::WHITE,
            theme::WHITE,
            iced::Color::BLACK,
            iced::font::Weight::Medium,
        )
    } else {
        (
            theme::CHIP_BG,
            theme::HOVER_BG,
            theme::TEXT,
            iced::font::Weight::Normal,
        )
    };

    button(
        container(text(label).size(14).color(fg).font(iced::Font {
            weight,
            ..Default::default()
        }))
        .padding([0, 12])
        .height(Length::Fixed(32.0))
        .style(move |_theme: &iced::Theme| {
            container::Style {
                background: Some(bg.into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }),
    )
    .on_press(Message::FilterCategorySelected(label.to_string()))
    .style(move |_theme: &iced::Theme, status: button::Status| {
        // Hover state lives on the button; the container paints the base.
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => hover_bg,
            _ => bg,
        };
        button::Style {
            background: Some(background.into()),
            text_color: fg,
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .padding(0)
    .into()
}

use std::collections::HashMap;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::theme;
use crate::widgets::video_card;

/// YouTube home: 56px sticky filter-chip bar, then a responsive grid.
/// Columns come from the window width (cards ~330px minimum, max 4),
/// matching youtube.com reflow. Gaps are 16px horizontal and 40px vertical.
pub fn view<'a>(
    videos: &'a [VideoItem],
    thumbnails: &'a HashMap<String, iced::widget::image::Handle>,
    avatars: &'a HashMap<String, iced::widget::image::Handle>,
    window_width: f32,
    sidebar_expanded: bool,
    active_category: &str,
) -> Element<'a, Message> {
    let sidebar_w = if sidebar_expanded { 240.0 } else { 72.0 };
    let content_w = (window_width - sidebar_w - 48.0).max(320.0);
    let columns = ((content_w / 330.0).floor() as usize).clamp(1, 4);
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
        "Trending",
        "New to you",
    ];
    let mut chips = row![].spacing(10).align_y(iced::Alignment::Center);
    for cat in categories {
        chips = chips.push(pill(cat, cat == active_category));
    }
    let filter_bar = container(
        scrollable(chips)
            .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::new()))
            .width(Length::Fill),
    )
    .padding([8, 24])
    .height(Length::Fixed(52.0))
    .align_y(iced::Alignment::Center)
    .width(Length::Fill);

    // If videos are loading, show high-fidelity YouTube skeleton cards
    if videos.is_empty() {
        let skeleton_count = columns * 3; // 3 full rows of skeleton cards
        let mut rows = column![].spacing(40).width(Length::Fill);
        let mut current_row = row![].spacing(16).width(Length::Fill);

        for i in 0..skeleton_count {
            current_row = current_row.push(
                container(video_card::skeleton(thumb_h))
                    .width(Length::FillPortion(1)),
            );
            if (i + 1) % columns == 0 {
                rows = rows.push(current_row);
                current_row = row![].spacing(16).width(Length::Fill);
            }
        }

        let page_content = column![
            filter_bar,
            container(rows)
                .padding(iced::Padding {
                    top: 12.0,
                    right: 24.0,
                    bottom: 32.0,
                    left: 24.0,
                })
                .width(Length::Fill),
        ]
        .spacing(0)
        .width(Length::Fill);

        return scrollable(page_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
    }

    let mut rows = column![].spacing(40).width(Length::Fill);
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

    // Preserve equal column widths on the final row by filling remaining slots with dummy spacers
    if count % columns != 0 {
        let remaining = columns - (count % columns);
        for _ in 0..remaining {
            current_row = current_row.push(
                container(Space::new(Length::FillPortion(1), Length::Shrink))
                    .width(Length::FillPortion(1)),
            );
        }
        rows = rows.push(current_row);
    }

    let page_content = column![
        filter_bar,
        container(rows)
            .padding(iced::Padding {
                top: 12.0,
                right: 24.0,
                bottom: 32.0,
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
        container(
            text(label)
                .size(14)
                .color(fg)
                .font(iced::Font {
                    weight,
                    ..Default::default()
                }),
        )
        .padding([0, 12])
        .height(Length::Fixed(32.0))
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center),
    )
    .on_press(Message::FilterCategorySelected(label.to_string()))
    .style(theme::pill_button(bg, hover_bg, 8.0))
    .padding(0)
    .into()
}

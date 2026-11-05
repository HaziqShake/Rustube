use std::collections::HashMap;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::widgets::video_card;

pub fn view<'a>(
    videos: &'a [VideoItem],
    thumbnails: &'a HashMap<String, Vec<u8>>,
) -> Element<'a, Message> {
    if videos.is_empty() {
        return container(text("Loading YouTube home feed...").size(16))
            .padding(24)
            .into();
    }

    // Exact YouTube Category Filter Pills Bar
    let filter_pills = row![
        pill("All", true),
        pill("Gaming", false),
        pill("Music", false),
        pill("News", false),
        pill("Podcasts", false),
        pill("Live", false),
        pill("Coding", false),
        pill("Recently uploaded", false),
        pill("Watched", false),
    ]
    .spacing(10)
    .padding([0, 16]);

    // Responsive 3-column auto-filling grid
    let mut rows = column![].spacing(28).width(Length::Fill);
    let mut current_row = row![].spacing(18).width(Length::Fill);
    let mut count = 0;

    for video in videos {
        let thumb = thumbnails.get(&video.thumbnail_url);
        current_row = current_row.push(
            container(video_card::view(video, thumb))
                .width(Length::FillPortion(1))
        );
        count += 1;
        if count % 3 == 0 {
            rows = rows.push(current_row);
            current_row = row![].spacing(18).width(Length::Fill);
        }
    }

    if count % 3 != 0 {
        rows = rows.push(current_row);
    }

    let page_content = column![
        filter_pills,
        container(rows).padding([8, 16]).width(Length::Fill),
    ]
    .spacing(16)
    .width(Length::Fill);

    scrollable(page_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn pill<'a>(label: &'static str, active: bool) -> Element<'a, Message> {
    let (bg, fg) = if active {
        (iced::Color::WHITE, iced::Color::BLACK)
    } else {
        (iced::Color::from_rgb8(0x27, 0x27, 0x27), iced::Color::WHITE)
    };

    button(
        container(
            text(label)
                .size(13)
                .color(fg)
                .font(iced::Font {
                    weight: if active { iced::font::Weight::Medium } else { iced::font::Weight::Normal },
                    ..Default::default()
                }),
        )
        .padding([6, 14])
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
    .style(button::text)
    .padding(0)
    .into()
}

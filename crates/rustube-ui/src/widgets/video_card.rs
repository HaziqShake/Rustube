use iced::widget::{button, column, container, image, row, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::VideoItem;
use crate::message::Message;
use crate::utils::format::{format_duration, format_view_count};

/// Standard YouTube Grid Video Card (Used in Home Feed & Search)
pub fn view<'a>(video: &'a VideoItem, thumbnail_bytes: Option<&'a Vec<u8>>) -> Element<'a, Message> {
    let dur_text = video
        .duration
        .map(format_duration)
        .unwrap_or_else(|| "LIVE".to_string());

    let views_text = video
        .view_count
        .map(format_view_count)
        .unwrap_or_default();

    let meta_text = format!("{} • {}", views_text, video.published_text);

    // Duration badge overlay
    let dur_badge = container(
        text(dur_text)
            .size(12)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            }),
    )
    .padding([3, 6])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.82).into()),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    // 16:9 Thumbnail container with rounded corners (YouTube 12px border radius)
    let thumbnail_view: Element<'a, Message> = if let Some(bytes) = thumbnail_bytes {
        container(
            column![
                image(image::Handle::from_bytes(bytes.clone()))
                    .width(Length::Fill)
                    .height(Length::Fixed(188.0)),
                container(dur_badge)
                    .width(Length::Fill)
                    .align_x(Alignment::End)
                    .padding([0, 6]),
            ]
            .spacing(4),
        )
        .width(Length::Fill)
        .height(Length::Fixed(216.0))
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::from_rgb8(0x18, 0x18, 0x18).into()),
                border: iced::Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
    } else {
        container(
            container(text("").size(12))
                .width(Length::Fill)
                .height(Length::Fixed(188.0))
                .style(|_theme: &iced::Theme| {
                    container::Style {
                        background: Some(iced::Color::from_rgb8(0x22, 0x22, 0x22).into()),
                        border: iced::Border {
                            radius: 12.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(216.0))
        .into()
    };

    // Channel avatar circle
    let avatar_circle = container(
        text(video.channel.name.chars().next().unwrap_or('?').to_string())
            .size(13)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    )
    .padding([8, 12])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x38, 0x38, 0x38).into()),
            border: iced::Border {
                radius: 18.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let text_meta = column![
        text(&video.title)
            .size(15)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
        text(&video.channel.name)
            .size(13)
            .color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
        text(meta_text)
            .size(12)
            .color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
    ]
    .spacing(4)
    .width(Length::Fill);

    let card_content = column![
        thumbnail_view,
        row![avatar_circle, text_meta]
            .spacing(12)
            .padding([6, 2])
            .align_y(Alignment::Start),
    ]
    .spacing(6);

    button(card_content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(button::text)
        .padding(6)
        .width(Length::Fill)
        .into()
}

/// YouTube Watch Page Sidebar Compact Video Card (Horizontal Layout)
pub fn compact_view<'a>(video: &'a VideoItem, thumbnail_bytes: Option<&'a Vec<u8>>) -> Element<'a, Message> {
    let dur_text = video
        .duration
        .map(format_duration)
        .unwrap_or_else(|| "LIVE".to_string());

    let views_text = video
        .view_count
        .map(format_view_count)
        .unwrap_or_default();

    let meta_text = format!("{} • {}", views_text, video.published_text);

    let dur_badge = container(
        text(dur_text)
            .size(11)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            }),
    )
    .padding([2, 4])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.85).into()),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let thumb_elem: Element<'a, Message> = if let Some(bytes) = thumbnail_bytes {
        container(
            column![
                image(image::Handle::from_bytes(bytes.clone()))
                    .width(Length::Fixed(168.0))
                    .height(Length::Fixed(94.0)),
                container(dur_badge)
                    .width(Length::Fixed(168.0))
                    .align_x(Alignment::End)
                    .padding([0, 4]),
            ]
            .spacing(2),
        )
        .width(Length::Fixed(168.0))
        .height(Length::Fixed(114.0))
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::from_rgb8(0x18, 0x18, 0x18).into()),
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
    } else {
        container(text("").size(11))
            .width(Length::Fixed(168.0))
            .height(Length::Fixed(94.0))
            .style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Color::from_rgb8(0x22, 0x22, 0x22).into()),
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .into()
    };

    let details = column![
        text(&video.title)
            .size(13)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
        text(&video.channel.name)
            .size(12)
            .color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
        text(meta_text)
            .size(11)
            .color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
    ]
    .spacing(4)
    .width(Length::Fill);

    let content = row![thumb_elem, details]
        .spacing(10)
        .align_y(Alignment::Start);

    button(content)
        .on_press(Message::OpenVideo(video.id.clone()))
        .style(button::text)
        .padding(4)
        .width(Length::Fill)
        .into()
}

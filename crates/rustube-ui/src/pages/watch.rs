use std::collections::HashMap;
use iced::widget::{button, column, container, image, row, scrollable, slider, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::{VideoDetails, VideoItem};
use crate::message::Message;
use crate::utils::icons::icon;
use crate::widgets::video_card;

pub fn view<'a>(
    details: Option<&'a VideoDetails>,
    recommended: &'a [VideoItem],
    thumbnails: &'a HashMap<String, Vec<u8>>,
) -> Element<'a, Message> {
    let Some(info) = details else {
        return container(
            column![
                icon("play_center", 64),
                text("Loading YouTube watch page...").size(16).color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
            ]
            .spacing(16)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .padding(32)
        .into();
    };

    let thumb_bytes = thumbnails.get(&info.id);

    // 1. YouTube Integrated Video Player Stage (16:9 Aspect Ratio)
    let player_view: Element<'a, Message> = if let Some(bytes) = thumb_bytes {
        container(
            image(image::Handle::from_bytes(bytes.clone()))
                .width(Length::Fill)
                .height(Length::Fixed(510.0))
        )
        .width(Length::Fill)
        .height(Length::Fixed(510.0))
        .into()
    } else {
        container(
            column![
                icon("play_center", 64),
                text("Embedded YouTube Player Active")
                    .size(15)
                    .color(iced::Color::from_rgb8(0xee, 0xee, 0xee)),
            ]
            .spacing(12)
            .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fixed(510.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::BLACK.into()),
                ..Default::default()
            }
        })
        .into()
    };

    // YouTube Red Scrubber Progress Line
    let scrubber_line = container(
        slider(0.0..=100.0, 35.0, Message::SeekPercent)
            .step(0.1)
            .style(|_theme: &iced::Theme, _status: slider::Status| {
                slider::Style {
                    rail: slider::Rail {
                        backgrounds: (
                            iced::Background::Color(iced::Color::from_rgb8(0xff, 0x00, 0x00)),
                            iced::Background::Color(iced::Color::from_rgb8(0x44, 0x44, 0x44)),
                        ),
                        width: 3.0,
                        border: iced::Border {
                            radius: 1.5.into(),
                            ..Default::default()
                        },
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius: 6.0 },
                        background: iced::Background::Color(iced::Color::from_rgb8(0xff, 0x00, 0x00)),
                        border_width: 0.0,
                        border_color: iced::Color::TRANSPARENT,
                    },
                }
            })
    )
    .width(Length::Fill)
    .padding([0, 0]);

    // Bottom On-Screen Controls Bar (Exact YouTube controls bar)
    let left_controls = row![
        button(icon("play", 20)).style(button::text).padding(6).on_press(Message::PlayPauseToggle),
        button(icon("forward", 18)).style(button::text).padding(6).on_press(Message::SeekRelative(10.0)),
        button(icon("volume", 20)).style(button::text).padding(6).on_press(Message::SetVolume(100)),
        text("0:45 / 3:33").size(13).color(iced::Color::from_rgb8(0xdd, 0xdd, 0xdd)),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let right_controls = row![
        button(icon("subtitles_cc", 20)).style(button::text).padding(6),
        button(icon("settings", 18)).style(button::text).padding(6),
        button(icon("pip", 18)).style(button::text).padding(6),
        button(icon("fullscreen", 20)).style(button::text).padding(6).on_press(Message::PlayerToggleFullscreen),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let controls_row = row![
        left_controls,
        container(right_controls).width(Length::Fill).align_x(Alignment::End),
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .padding([4, 12]);

    let complete_player_stage = container(
        column![
            player_view,
            scrubber_line,
            controls_row,
        ]
        .spacing(0),
    )
    .width(Length::Fill)
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::BLACK.into()),
            border: iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    // 2. Video Title (YouTube 20px Bold)
    let title_text = text(&info.title)
        .size(20)
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1))
        .width(Length::Fill);

    // 3. Channel Info & Actions Bar (Exact YouTube Watch Metadata Row)
    let channel_avatar = container(
        text(info.channel.name.chars().next().unwrap_or('?').to_string())
            .size(16)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    )
    .padding([10, 15])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x44, 0x44, 0x44).into()),
            border: iced::Border {
                radius: 20.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let channel_details = column![
        text(&info.channel.name)
            .size(16)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
        text("4.56M subscribers")
            .size(12)
            .color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)),
    ]
    .spacing(2);

    let subscribe_btn = button(
        container(
            text("Subscribe")
                .size(14)
                .color(iced::Color::BLACK)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..Default::default()
                }),
        )
        .padding([8, 18])
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::WHITE.into()),
                border: iced::Border {
                    radius: 18.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }),
    )
    .on_press(Message::SubscribeToggle)
    .style(button::text)
    .padding(0);

    let channel_box = row![channel_avatar, channel_details, subscribe_btn]
        .spacing(16)
        .align_y(Alignment::Center);

    // Action buttons: Like/Dislike segmented pill, Share, Download
    let like_dislike_pill = container(
        row![
            button(
                row![
                    icon("like", 18),
                    text("1.8M").size(13).color(iced::Color::WHITE),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            )
            .on_press(Message::LikeToggle)
            .style(button::text)
            .padding([8, 14]),
            text("|").size(14).color(iced::Color::from_rgb8(0x44, 0x44, 0x44)),
            button(icon("dislike", 18))
                .on_press(Message::LikeToggle)
                .style(button::text)
                .padding([8, 14]),
        ]
        .spacing(2)
        .align_y(Alignment::Center),
    )
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x27, 0x27, 0x27).into()),
            border: iced::Border {
                radius: 18.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let share_btn = button(
        container(
            row![icon("share", 18), text("Share").size(13).color(iced::Color::WHITE)]
                .spacing(6)
                .align_y(Alignment::Center),
        )
        .padding([8, 16])
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::from_rgb8(0x27, 0x27, 0x27).into()),
                border: iced::Border {
                    radius: 18.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }),
    )
    .style(button::text)
    .padding(0);

    let download_btn = button(
        container(
            row![icon("downloads", 18), text("Download").size(13).color(iced::Color::WHITE)]
                .spacing(6)
                .align_y(Alignment::Center),
        )
        .padding([8, 16])
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Color::from_rgb8(0x27, 0x27, 0x27).into()),
                border: iced::Border {
                    radius: 18.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }),
    )
    .style(button::text)
    .padding(0);

    let actions_row = row![like_dislike_pill, share_btn, download_btn]
        .spacing(10)
        .align_y(Alignment::Center);

    let meta_bar = row![channel_box, container(actions_row).width(Length::Fill).align_x(Alignment::End)]
        .spacing(16)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    // 4. Description Box (YouTube #272727 Rounded Box)
    let views_formatted = format!("{} views", info.view_count);
    let desc_container = container(
        column![
            text(format!("{} • Published on YouTube", views_formatted))
                .size(14)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
            text(&info.description).size(13).color(iced::Color::from_rgb8(0xee, 0xee, 0xee)),
        ]
        .spacing(8),
    )
    .padding(16)
    .width(Length::Fill)
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x22, 0x22, 0x22).into()),
            border: iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let primary_column = column![
        complete_player_stage,
        title_text,
        meta_bar,
        desc_container,
    ]
    .spacing(16)
    .width(Length::FillPortion(7));

    // 5. Right Sidebar ("Up next" with Compact Horizontal Cards)
    let rec_chips = row![
        sidebar_chip("All", true),
        sidebar_chip("From this channel", false),
        sidebar_chip("Related", false),
    ]
    .spacing(8);

    let mut rec_col = column![
        rec_chips,
    ]
    .spacing(12)
    .width(Length::FillPortion(3));

    for item in recommended.iter().take(10) {
        let thumb = thumbnails.get(&item.thumbnail_url);
        rec_col = rec_col.push(video_card::compact_view(item, thumb));
    }

    let two_column_layout = row![
        primary_column,
        rec_col,
    ]
    .spacing(24)
    .padding(20)
    .width(Length::Fill);

    scrollable(two_column_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn sidebar_chip<'a>(label: &'static str, active: bool) -> Element<'a, Message> {
    let (bg, fg) = if active {
        (iced::Color::WHITE, iced::Color::BLACK)
    } else {
        (iced::Color::from_rgb8(0x27, 0x27, 0x27), iced::Color::WHITE)
    };

    button(
        container(text(label).size(12).color(fg))
            .padding([6, 12])
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
    .style(button::text)
    .padding(0)
    .into()
}

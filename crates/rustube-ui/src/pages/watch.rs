use std::collections::HashMap;
use std::time::Duration;
use iced::widget::{button, column, container, image, row, scrollable, slider, text};
use iced::{Alignment, Element, Length};
use rustube_core::youtube::models::{VideoDetails, VideoItem};
use crate::message::Message;
use crate::theme;
use crate::utils::format::{
    clean_text, format_count, format_duration, format_relative, format_subscriber_count,
};
use crate::utils::icons::icon;
use crate::widgets::video_card;

/// Live playback state polled from MPV (see `PlayerController::query_state`).
#[derive(Debug, Clone, Copy)]
pub struct PlayerViewState {
    pub position: f64,
    pub duration: f64,
    pub paused: bool,
    pub volume: u32,
}

/// Watch page: 16:9 player stage, video title, channel row with subscribe pill,
/// segmented like/dislike pill, `#272727` description box, and "Up next" recommendation column.
pub fn view<'a>(
    details: Option<&'a VideoDetails>,
    recommended: &'a [VideoItem],
    thumbnails: &'a HashMap<String, iced::widget::image::Handle>,
    stage_thumb: Option<&'a iced::widget::image::Handle>,
    window_width: f32,
    player: PlayerViewState,
) -> Element<'a, Message> {
    let Some(info) = details else {
        return container(
            column![
                icon("play_center", 64),
                text("Loading YouTube watch page...")
                    .size(16)
                    .color(theme::TEXT_SECONDARY),
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

    // 1. Player stage, 16:9 of the primary column width.
    let available_w = (window_width - 72.0).max(800.0);
    let primary_w = (available_w * 0.70).min(1280.0);
    let stage_h = primary_w * 9.0 / 16.0;

    let thumb_bytes = stage_thumb;
    let player_view: Element<'a, Message> = if let Some(handle) = thumb_bytes {
        container(
            image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fixed(stage_h)),
        )
        .width(Length::Fill)
        .height(Length::Fixed(stage_h))
        .into()
    } else {
        container(text("").size(1))
            .width(Length::Fill)
            .height(Length::Fixed(stage_h))
            .style(theme::solid_container(iced::Color::BLACK, 0.0))
            .into()
    };

    // Scrubber: real position/duration from MPV, red rail.
    let duration = player.duration.max(1.0);
    let scrubber = slider(0.0..=duration, player.position.min(duration), move |v| {
        Message::SeekPercent(v / duration * 100.0)
    })
    .step(0.1)
    .style(|_theme: &iced::Theme, _status: slider::Status| slider::Style {
        rail: slider::Rail {
            backgrounds: (
                iced::Background::Color(theme::RED),
                iced::Background::Color(iced::Color::from_rgb8(0x44, 0x44, 0x44)),
            ),
            width: 4.0,
            border: iced::Border {
                radius: 2.0.into(),
                ..Default::default()
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 6.0 },
            background: iced::Background::Color(theme::RED),
            border_width: 0.0,
            border_color: iced::Color::TRANSPARENT,
        },
    });

    let time_text = format!(
        "{} / {}",
        format_duration(Duration::from_secs_f64(player.position)),
        format_duration(Duration::from_secs_f64(player.duration)),
    );

    let play_icon = if player.paused { "play" } else { "pause" };
    let left_controls = row![
        button(icon(play_icon, 20))
            .style(theme::icon_button())
            .padding(6)
            .on_press(Message::PlayPauseToggle),
        button(icon("back", 18))
            .style(theme::icon_button())
            .padding(6)
            .on_press(Message::SeekRelative(-10.0)),
        button(icon("forward", 18))
            .style(theme::icon_button())
            .padding(6)
            .on_press(Message::SeekRelative(10.0)),
        button(icon("volume", 20))
            .style(theme::icon_button())
            .padding(6),
        slider(0.0..=100.0, player.volume as f64, |v| {
            Message::SetVolume(v as u32)
        })
        .step(1.0)
        .width(Length::Fixed(80.0))
        .style(|_theme: &iced::Theme, _status: slider::Status| slider::Style {
            rail: slider::Rail {
                backgrounds: (
                    iced::Background::Color(iced::Color::WHITE),
                    iced::Background::Color(iced::Color::from_rgb8(0x66, 0x66, 0x66)),
                ),
                width: 4.0,
                border: iced::Border {
                    radius: 2.0.into(),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 6.0 },
                background: iced::Background::Color(iced::Color::WHITE),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
            },
        }),
        text(time_text)
            .size(13)
            .color(iced::Color::from_rgb8(0xdd, 0xdd, 0xdd)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let right_controls = row![
        button(icon("subtitles_cc", 20)).style(theme::icon_button()).padding(6),
        button(icon("settings", 18)).style(theme::icon_button()).padding(6),
        button(icon("pip", 18)).style(theme::icon_button()).padding(6),
        button(icon("fullscreen", 20))
            .style(theme::icon_button())
            .padding(6)
            .on_press(Message::PlayerToggleFullscreen),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let controls_row = row![
        left_controls,
        container(right_controls)
            .width(Length::Fill)
            .align_x(Alignment::End),
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .padding([4, 12]);

    let complete_player_stage = container(
        column![player_view, scrubber, controls_row].spacing(0),
    )
    .width(Length::Fill)
    .style(theme::solid_container(iced::Color::BLACK, 12.0));

    // 2. Video title, 20px Bold.
    let clean_title = clean_text(&info.title);
    let title_text = text(clean_title)
        .size(20)
        .line_height(iced::Pixels(28.0))
        .font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        })
        .color(theme::TEXT)
        .width(Length::Fill);

    // 3. Channel row + actions.
    let initial = info.channel.name.chars().next().unwrap_or('?').to_uppercase().to_string();
    let channel_avatar = container(
        text(initial)
            .size(16)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(theme::solid_container(theme::AVATAR_BG, 20.0));

    let subs_text = info
        .subscriber_count
        .map(format_subscriber_count)
        .unwrap_or_default();
    let channel_details = column![
        text(&info.channel.name)
            .size(16)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
        text(subs_text).size(12).color(theme::TEXT_SECONDARY),
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
        .padding([8, 16])
        .height(Length::Fixed(36.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(theme::solid_container(theme::WHITE, 18.0)),
    )
    .on_press(Message::SubscribeToggle)
    .style(theme::ghost_button())
    .padding(0);

    let channel_box = row![channel_avatar, channel_details, subscribe_btn]
        .spacing(12)
        .align_y(Alignment::Center);

    let like_label = info
        .like_count
        .map(format_count)
        .unwrap_or_default();
    let like_dislike_pill = container(
        row![
            button(
                row![icon("like", 18), text(like_label).size(14).color(theme::TEXT)]
                    .spacing(6)
                    .align_y(Alignment::Center),
            )
            .on_press(Message::LikeToggle)
            .style(theme::ghost_button())
            .padding([6, 12]),
            container(text("").size(1))
                .width(Length::Fixed(1.0))
                .height(Length::Fixed(20.0))
                .style(theme::solid_container(
                    iced::Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                    0.0,
                )),
            button(icon("dislike", 18))
                .on_press(Message::LikeToggle)
                .style(theme::ghost_button())
                .padding([6, 12]),
        ]
        .spacing(0)
        .align_y(Alignment::Center),
    )
    .height(Length::Fixed(36.0))
    .style(theme::solid_container(theme::CHIP_BG, 18.0));

    let action_pill = |icon_name: &'static str, label: &'static str| {
        button(
            container(
                row![icon(icon_name, 18), text(label).size(14).color(theme::TEXT)]
                    .spacing(6)
                    .align_y(Alignment::Center),
            )
            .padding([6, 16])
            .height(Length::Fixed(36.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(theme::solid_container(theme::CHIP_BG, 18.0)),
        )
        .style(theme::ghost_button())
        .padding(0)
    };

    let actions_row = row![
        like_dislike_pill,
        action_pill("share", "Share"),
        action_pill("downloads", "Download"),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let meta_bar = row![
        channel_box,
        container(actions_row)
            .width(Length::Fill)
            .align_x(Alignment::End),
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    // 4. Description box, `#272727`, 12px radius.
    let meta_line = match info.published_date {
        Some(d) => format!(
            "{} views • {}",
            format_count(info.view_count),
            format_relative(d)
        ),
        None => format!("{} views", format_count(info.view_count)),
    };
    let clean_desc = clean_text(&info.description);
    let desc_container = container(
        column![
            text(meta_line)
                .size(14)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..Default::default()
                })
                .color(theme::TEXT),
            text(clean_desc)
                .size(14)
                .line_height(iced::Pixels(20.0))
                .color(theme::TEXT),
        ]
        .spacing(8),
    )
    .padding(14)
    .width(Length::Fill)
    .style(theme::solid_container(theme::CHIP_BG, 12.0));

    let primary_column = column![
        complete_player_stage,
        title_text,
        meta_bar,
        desc_container,
    ]
    .spacing(14)
    .width(Length::FillPortion(7));

    // 5. Right column: "Up next" chips + compact cards.
    let rec_chips = row![
        sidebar_chip("All", true),
        sidebar_chip("From this channel", false),
        sidebar_chip("Related", false),
    ]
    .spacing(8);

    let mut rec_col = column![rec_chips,].spacing(12).width(Length::FillPortion(3));

    for item in recommended.iter().take(12) {
        let thumb = thumbnails.get(&item.thumbnail_url);
        rec_col = rec_col.push(video_card::compact_view(item, thumb));
    }

    let two_column_layout = row![primary_column, rec_col]
        .spacing(24)
        .padding(24)
        .width(Length::Fill);

    scrollable(two_column_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn sidebar_chip<'a>(label: &'static str, active: bool) -> Element<'a, Message> {
    let (bg, fg, weight) = if active {
        (
            theme::WHITE,
            iced::Color::BLACK,
            iced::font::Weight::Medium,
        )
    } else {
        (theme::CHIP_BG, theme::TEXT, iced::font::Weight::Normal)
    };

    button(
        container(text(label).size(14).color(fg).font(iced::Font {
            weight,
            ..Default::default()
        }))
        .padding([0, 12])
        .height(Length::Fixed(32.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
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
    .style(theme::pill_button(bg, theme::HOVER_BG, 8.0))
    .padding(0)
    .into()
}

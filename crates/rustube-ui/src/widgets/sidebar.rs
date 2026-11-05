use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use crate::message::Message;
use crate::router::Page;
use crate::utils::icons::icon;

pub fn view<'a>(expanded: bool) -> Element<'a, Message> {
    if !expanded {
        return container(
            column![
                sidebar_item_mini("home", "Home", Message::NavigateTo(Page::Home)),
                sidebar_item_mini("shorts", "Shorts", Message::NavigateTo(Page::Home)),
                sidebar_item_mini("subscriptions", "Subscriptions", Message::NavigateTo(Page::Subscriptions)),
                sidebar_item_mini("library", "You", Message::NavigateTo(Page::History)),
                sidebar_item_mini("downloads", "Downloads", Message::NavigateTo(Page::Downloads)),
            ]
            .spacing(18)
            .align_x(Alignment::Center)
            .width(Length::Fixed(72.0)),
        )
        .padding([12, 0])
        .into();
    }

    container(
        column![
            sidebar_item_full("home", "Home", Message::NavigateTo(Page::Home), true),
            sidebar_item_full("shorts", "Shorts", Message::NavigateTo(Page::Home), false),
            sidebar_item_full("subscriptions", "Subscriptions", Message::NavigateTo(Page::Subscriptions), false),
            divider(),
            section_header("You >"),
            sidebar_item_full("history", "History", Message::NavigateTo(Page::History), false),
            sidebar_item_full("library", "Playlists", Message::NavigateTo(Page::History), false),
            sidebar_item_full("downloads", "Downloads", Message::NavigateTo(Page::Downloads), false),
            divider(),
            section_header("Explore"),
            sidebar_item_full("logo", "Trending", Message::NavigateTo(Page::Home), false),
            sidebar_item_full("volume", "Music", Message::NavigateTo(Page::Home), false),
            sidebar_item_full("play", "Gaming", Message::NavigateTo(Page::Home), false),
            sidebar_item_full("subtitles_cc", "News", Message::NavigateTo(Page::Home), false),
            divider(),
            sidebar_item_full("settings", "Settings", Message::NavigateTo(Page::Settings), false),
        ]
        .spacing(3)
        .width(Length::Fixed(220.0)),
    )
    .padding([8, 12])
    .into()
}

fn section_header<'a>(title: &'static str) -> Element<'a, Message> {
    container(
        text(title)
            .size(14)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
    )
    .padding([6, 12])
    .into()
}

fn sidebar_item_full<'a>(icon_name: &'static str, label: &'static str, msg: Message, active: bool) -> Element<'a, Message> {
    let (bg, font_weight) = if active {
        (Some(iced::Color::from_rgb8(0x27, 0x27, 0x27).into()), iced::font::Weight::Bold)
    } else {
        (None, iced::font::Weight::Normal)
    };

    button(
        container(
            row![
                icon(icon_name, 22),
                text(label)
                    .size(14)
                    .font(iced::Font {
                        weight: font_weight,
                        ..Default::default()
                    })
                    .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
            ]
            .spacing(22)
            .align_y(Alignment::Center),
        )
        .padding([8, 12])
        .style(move |_theme: &iced::Theme| {
            container::Style {
                background: bg,
                border: iced::Border {
                    radius: 10.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .width(Length::Fill),
    )
    .on_press(msg)
    .style(button::text)
    .padding(0)
    .width(Length::Fill)
    .into()
}

fn sidebar_item_mini<'a>(icon_name: &'static str, label: &'static str, msg: Message) -> Element<'a, Message> {
    button(
        container(
            column![
                icon(icon_name, 22),
                text(label).size(10).color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .padding([12, 4])
        .style(|_theme: &iced::Theme| {
            container::Style {
                border: iced::Border {
                    radius: 10.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .width(Length::Fill),
    )
    .on_press(msg)
    .style(button::text)
    .padding(0)
    .width(Length::Fill)
    .into()
}

fn divider<'a>() -> Element<'a, Message> {
    container(
        container(text("").size(1))
            .height(Length::Fixed(1.0))
            .width(Length::Fill)
            .style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Color::from_rgb8(0x27, 0x27, 0x27).into()),
                    ..Default::default()
                }
            }),
    )
    .padding([8, 0])
    .into()
}

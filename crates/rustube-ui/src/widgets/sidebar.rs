use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use crate::message::Message;
use crate::router::Page;
use crate::theme;
use crate::utils::icons::icon;

/// YouTube sidebar, expanded: 240px wide, items 40px tall, 24px icons,
/// 24px icon-to-label gap, 14px labels, 10px corner radius.
/// Hover and active both use `#272727`; active labels are Medium weight.
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
            .spacing(0)
            .align_x(Alignment::Center)
            .width(Length::Fixed(72.0)),
        )
        .padding([4, 0])
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
        .spacing(0)
        .width(Length::Fixed(240.0)),
    )
    .padding([12, 12])
    .into()
}

fn section_header<'a>(title: &'static str) -> Element<'a, Message> {
    container(
        text(title)
            .size(16)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..Default::default()
            })
            .color(theme::TEXT),
    )
    .padding(iced::Padding {
        top: 8.0,
        right: 12.0,
        bottom: 4.0,
        left: 12.0,
    })
    .into()
}

fn item_style(
    active: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status: button::Status| {
        let background = if active {
            Some(theme::CHIP_BG.into())
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(theme::CHIP_BG.into())
                }
                _ => None,
            }
        };
        button::Style {
            background,
            text_color: theme::TEXT,
            border: iced::Border {
                radius: 10.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn sidebar_item_full<'a>(
    icon_name: &'static str,
    label: &'static str,
    msg: Message,
    active: bool,
) -> Element<'a, Message> {
    let weight = if active {
        iced::font::Weight::Medium
    } else {
        iced::font::Weight::Normal
    };

    button(
        container(
            row![
                icon(icon_name, 24),
                text(label)
                    .size(14)
                    .font(iced::Font {
                        weight,
                        ..Default::default()
                    })
                    .color(theme::TEXT),
            ]
            .spacing(24)
            .align_y(Alignment::Center),
        )
        .padding([0, 12])
        .height(Length::Fixed(40.0))
        .width(Length::Fill),
    )
    .on_press(msg)
    .style(item_style(active))
    .padding(0)
    .width(Length::Fill)
    .into()
}

fn sidebar_item_mini<'a>(icon_name: &'static str, label: &'static str, msg: Message) -> Element<'a, Message> {
    button(
        container(
            column![
                icon(icon_name, 24),
                text(label).size(10).color(theme::TEXT),
            ]
            .spacing(6)
            .align_x(Alignment::Center),
        )
        .padding(iced::Padding {
            top: 16.0,
            right: 0.0,
            bottom: 14.0,
            left: 0.0,
        })
        .width(Length::Fill),
    )
    .on_press(msg)
    .style(item_style(false))
    .padding(0)
    .width(Length::Fill)
    .into()
}

fn divider<'a>() -> Element<'a, Message> {
    container(
        container(text("").size(1))
            .height(Length::Fixed(1.0))
            .width(Length::Fill)
            .style(theme::solid_container(theme::DIVIDER, 0.0)),
    )
    .padding([12, 12])
    .into()
}

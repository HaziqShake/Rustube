use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Element, Length};
use crate::message::Message;
use crate::utils::icons::icon;

pub fn view<'a>(
    search_query: &'a str,
    can_back: bool,
    can_forward: bool,
) -> Element<'a, Message> {
    // Left: hamburger + back/forward + YouTube Brand
    let menu_btn = button(icon("menu", 20))
        .on_press(Message::ToggleSidebar)
        .style(button::text)
        .padding(8);

    let back_btn = if can_back {
        button(icon("back", 18)).on_press(Message::GoBack).style(button::text).padding(8)
    } else {
        button(icon("back", 18)).style(button::text).padding(8)
    };

    let fwd_btn = if can_forward {
        button(icon("forward", 18)).on_press(Message::GoForward).style(button::text).padding(8)
    } else {
        button(icon("forward", 18)).style(button::text).padding(8)
    };

    let logo_badge = button(
        row![
            icon("logo", 30),
            text("Rustube")
                .size(19)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(iced::Color::from_rgb8(0xf1, 0xf1, 0xf1)),
            container(text("IN").size(10).color(iced::Color::from_rgb8(0xaa, 0xaa, 0xaa)))
                .padding([0, 2]),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    )
    .on_press(Message::NavigateTo(crate::router::Page::Home))
    .style(button::text)
    .padding(4);

    let left_section = row![menu_btn, back_btn, fwd_btn, logo_badge]
        .spacing(6)
        .align_y(Alignment::Center);

    // Center: search input + search submit + mic
    let search_input_widget = text_input("Search", search_query)
        .on_input(Message::SearchInputChanged)
        .on_submit(Message::SubmitSearch)
        .padding([9, 16])
        .width(Length::Fixed(480.0));

    let search_submit = button(icon("search", 19))
        .on_press(Message::SubmitSearch)
        .style(button::secondary)
        .padding([9, 20]);

    let search_pill = container(
        row![search_input_widget, search_submit]
            .spacing(0)
            .align_y(Alignment::Center),
    )
    .style(|_theme: &iced::Theme| {
        container::Style {
            border: iced::Border {
                color: iced::Color::from_rgb8(0x30, 0x30, 0x30),
                width: 1.0,
                radius: 20.0.into(),
            },
            ..Default::default()
        }
    });

    let mic_btn = button(
        container(icon("mic", 19))
            .padding(10)
            .style(|_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Color::from_rgb8(0x22, 0x22, 0x22).into()),
                    border: iced::Border {
                        radius: 20.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }),
    )
    .style(button::text)
    .padding(0);

    let center_section = row![search_pill, mic_btn]
        .spacing(12)
        .align_y(Alignment::Center);

    // Right: Bell + User avatar
    let notif_btn = button(
        container(icon("bell", 20))
            .padding(8)
            .style(|_theme: &iced::Theme| {
                container::Style {
                    border: iced::Border {
                        radius: 20.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }),
    )
    .style(button::text)
    .padding(0);

    let avatar_btn = container(
        text("U")
            .size(14)
            .color(iced::Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    )
    .padding([8, 12])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x06, 0x5f, 0xd4).into()),
            border: iced::Border {
                radius: 18.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    let right_section = row![notif_btn, avatar_btn]
        .spacing(14)
        .align_y(Alignment::Center);

    container(
        row![
            left_section,
            container(center_section).width(Length::Fill).align_x(Alignment::Center),
            right_section,
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding([8, 18])
    .style(|_theme: &iced::Theme| {
        container::Style {
            background: Some(iced::Color::from_rgb8(0x0f, 0x0f, 0x0f).into()),
            border: iced::Border {
                color: iced::Color::from_rgb8(0x27, 0x27, 0x27),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .width(Length::Fill)
    .into()
}

use iced::widget::{button, container, row, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};
use crate::message::Message;
use crate::theme;
use crate::utils::icons::{icon, logo};

/// YouTube masthead: 56px tall, flat `#0f0f0f`, exact YouTube desktop layout.
pub fn view<'a>(search_query: &'a str) -> Element<'a, Message> {
    // Left: hamburger + brand lockup (red badge + wordmark + "IN").
    let menu_btn = button(icon("menu", 24))
        .on_press(Message::ToggleSidebar)
        .style(theme::icon_button())
        .padding(8)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0));

    let logo_badge = button(
        row![
            logo(),
            text("Rustube")
                .size(19)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..Default::default()
                })
                .color(theme::TEXT),
            container(
                text("IN")
                    .size(10)
                    .color(theme::TEXT_SECONDARY),
            )
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: 6.0,
                left: 0.0,
            }),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::NavigateTo(crate::router::Page::Home))
    .style(theme::ghost_button())
    .padding([4, 8]);

    let left_section = row![menu_btn, logo_badge]
        .spacing(12)
        .align_y(Alignment::Center);

    // Center: YouTube search pill with left input and right submit button.
    let search_input_widget = text_input("Search", search_query)
        .on_input(Message::SearchInputChanged)
        .on_submit(Message::SubmitSearch)
        .padding([0, 16])
        .size(16)
        .width(Length::Fill)
        .style(|_theme: &iced::Theme, status: text_input::Status| {
            let border_color = match status {
                text_input::Status::Focused => theme::LINK,
                _ => theme::SEARCH_BORDER,
            };
            text_input::Style {
                background: Background::Color(theme::SEARCH_BG),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: iced::border::Radius {
                        top_left: 20.0,
                        bottom_left: 20.0,
                        top_right: 0.0,
                        bottom_right: 0.0,
                    },
                },
                icon: theme::TEXT,
                placeholder: theme::TEXT_PLACEHOLDER,
                value: theme::TEXT,
                selection: theme::LINK,
            }
        });

    let search_submit = button(
        container(icon("search", 20))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::SubmitSearch)
    .style(|_theme: &iced::Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered | button::Status::Pressed => theme::HOVER_BG,
            _ => theme::SEARCH_BTN_BG,
        };
        button::Style {
            background: Some(bg.into()),
            text_color: theme::TEXT,
            border: Border {
                color: theme::SEARCH_BORDER,
                width: 1.0,
                radius: iced::border::Radius {
                    top_left: 0.0,
                    top_right: 20.0,
                    bottom_right: 20.0,
                    bottom_left: 0.0,
                },
            },
            ..Default::default()
        }
    })
    .padding(0)
    .width(Length::Fixed(64.0))
    .height(Length::Fixed(40.0));

    let search_pill = container(
        row![search_input_widget, search_submit]
            .spacing(0)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .max_width(640.0)
    .height(Length::Fixed(40.0));

    let mic_btn = button(icon("mic", 20))
        .style(|_theme: &iced::Theme, status: button::Status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => theme::HOVER_BG,
                _ => theme::MIC_BG,
            };
            button::Style {
                background: Some(bg.into()),
                text_color: theme::TEXT,
                border: Border {
                    radius: 20.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .padding(10)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0));

    let center_section = row![search_pill, mic_btn]
        .spacing(12)
        .align_y(Alignment::Center);

    // Right: notification bell + user avatar circle (32px).
    let notif_btn = button(icon("bell", 20))
        .style(theme::icon_button())
        .padding(10)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0));

    let avatar_btn = container(
        text("U")
            .size(14)
            .color(Color::WHITE)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
    )
    .padding(0)
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(theme::solid_container(
        Color::from_rgb8(0x06, 0x5f, 0xd4),
        16.0,
    ));

    let right_section = row![notif_btn, avatar_btn]
        .spacing(12)
        .align_y(Alignment::Center);

    container(
        row![
            container(left_section).width(Length::FillPortion(1)),
            container(center_section)
                .width(Length::FillPortion(2))
                .align_x(Alignment::Center),
            container(right_section)
                .width(Length::FillPortion(1))
                .align_x(Alignment::End),
        ]
        .spacing(16)
        .align_y(Alignment::Center)
        .width(Length::Fill),
    )
    .padding([0, 16])
    .height(Length::Fixed(56.0))
    .style(theme::solid_container(theme::BG, 0.0))
    .width(Length::Fill)
    .into()
}

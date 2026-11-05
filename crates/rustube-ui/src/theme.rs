//! YouTube dark-theme design tokens, measured from youtube.com (dark mode).
//!
//! Every widget in the app must use these constants instead of hard-coded
//! hex values, so the whole UI stays in lock-step with YouTube.

use iced::widget::{button, container, text_input};
use iced::{Background, Border, Color, Theme};

/// `Color::from_rgb8` is not a `const fn` in iced 0.13, so plain
/// `pub const` tokens need these helpers instead.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
}

/// Page background, masthead, sidebar. `#0f0f0f`
pub const BG: Color = rgb(0x0f, 0x0f, 0x0f);
/// Search input background. `#121212`
pub const SEARCH_BG: Color = rgb(0x12, 0x12, 0x12);
/// Search input/button border. `#303030`
pub const SEARCH_BORDER: Color = rgb(0x30, 0x30, 0x30);
/// Search submit button + voice-search circle. `#222222` / `#181818`
pub const SEARCH_BTN_BG: Color = rgb(0x22, 0x22, 0x22);
pub const MIC_BG: Color = rgb(0x18, 0x18, 0x18);
/// Chips, sidebar hover/active, action pills, description box. `#272727`
pub const CHIP_BG: Color = rgb(0x27, 0x27, 0x27);
/// Hover state for chips and icon buttons. `#3d3d3d`
pub const HOVER_BG: Color = rgb(0x3d, 0x3d, 0x3d);
/// Dividers. `#272727`
pub const DIVIDER: Color = rgb(0x27, 0x27, 0x27);
/// Primary text. `#f1f1f1`
pub const TEXT: Color = rgb(0xf1, 0xf1, 0xf1);
/// Secondary text (channel names, views, timestamps). `#aaaaaa`
pub const TEXT_SECONDARY: Color = rgb(0xaa, 0xaa, 0xaa);
/// Placeholder text in the search box. `#888888`
pub const TEXT_PLACEHOLDER: Color = rgb(0x88, 0x88, 0x88);
/// YouTube red, used for the logo badge, scrubber and live badges. `#ff0000`
pub const RED: Color = rgb(0xff, 0x00, 0x00);
/// Active chip / subscribe button background. `#f1f1f1`
pub const WHITE: Color = rgb(0xf1, 0xf1, 0xf1);
/// Link blue. `#3ea6ff`
pub const LINK: Color = rgb(0x3e, 0xa6, 0xff);
/// Letter-avatar fallback background. `#383838`
pub const AVATAR_BG: Color = rgb(0x38, 0x38, 0x38);
/// Thumbnail placeholder while the image loads. `#212121`
pub const THUMB_PLACEHOLDER: Color = rgb(0x21, 0x21, 0x21);
/// Duration badge background, 80% black.
pub const BADGE_BG: Color = rgba(0, 0, 0, 0.8);

/// Legacy palette struct, kept so old code keeps compiling.
/// New code should use the constants above.
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent_red: Color,
    pub link_blue: Color,
    pub border: Color,
}

pub fn dark_palette() -> Palette {
    Palette {
        background: BG,
        surface: CHIP_BG,
        elevated: HOVER_BG,
        text_primary: TEXT,
        text_secondary: TEXT_SECONDARY,
        accent_red: RED,
        link_blue: LINK,
        border: DIVIDER,
    }
}

fn border(radius: f32) -> Border {
    Border {
        radius: radius.into(),
        ..Default::default()
    }
}

/// A flat button with a solid background that lightens on hover.
/// Used for chips, action pills and the subscribe button.
pub fn pill_button(
    bg: Color,
    hover_bg: Color,
    radius: f32,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status: button::Status| {
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => hover_bg,
            _ => bg,
        };
        button::Style {
            background: Some(Background::Color(background)),
            text_color: TEXT,
            border: border(radius),
            ..Default::default()
        }
    }
}

/// A circular icon button (hamburger, bell, mic). Transparent until hovered.
pub fn icon_button() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme: &Theme, status: button::Status| {
        let background = match status {
            button::Status::Hovered | button::Status::Pressed => Some(Background::Color(HOVER_BG)),
            _ => None,
        };
        button::Style {
            background,
            text_color: TEXT,
            border: border(20.0),
            ..Default::default()
        }
    }
}

/// Fully transparent button (video cards, sidebar rows get their own
/// container background instead).
pub fn ghost_button() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme: &Theme, _status: button::Status| button::Style {
        background: None,
        text_color: TEXT,
        ..Default::default()
    }
}

/// The search text field: dark input, no visible border of its own
/// (the surrounding pill container draws the border).
pub fn search_input_style(
    _theme: &Theme,
    _status: text_input::Status,
) -> text_input::Style {
    text_input::Style {
        background: Background::Color(SEARCH_BG),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: TEXT,
        placeholder: TEXT_PLACEHOLDER,
        value: TEXT,
        selection: LINK,
    }
}

/// A rounded container with a solid background, e.g. pills and badges.
pub fn solid_container(bg: Color, radius: f32) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        background: Some(Background::Color(bg)),
        border: border(radius),
        ..Default::default()
    }
}

/// A rounded container with a solid background and a 1px border.
pub fn bordered_container(
    bg: Color,
    border_color: Color,
    radius: f32,
) -> impl Fn(&Theme) -> container::Style {
    move |_theme: &Theme| container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: radius.into(),
        },
        ..Default::default()
    }
}

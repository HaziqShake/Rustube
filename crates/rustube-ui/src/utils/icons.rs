use iced::widget::svg::{Handle, Svg};
use iced::widget::svg;
use iced::Length;

pub fn icon<'a>(name: &str, size: u16) -> Svg<'a> {
    let path = format!("assets/icons/{}.svg", name);
    svg(Handle::from_path(path))
        .width(Length::Fixed(size as f32))
        .height(Length::Fixed(size as f32))
}

/// The YouTube play badge, rendered at its real aspect ratio (28x20).
/// The old code rendered it through `icon("logo", 30)`, which forced the
/// wide badge into a 30x30 square and squashed it.
pub fn logo<'a>() -> Svg<'a> {
    svg(Handle::from_path("assets/icons/logo.svg"))
        .width(Length::Fixed(30.0))
        .height(Length::Fixed(21.0))
}

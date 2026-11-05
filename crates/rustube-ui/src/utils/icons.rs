use iced::widget::svg::{Handle, Svg};
use iced::widget::svg;
use iced::Length;

pub fn icon<'a>(name: &str, size: u16) -> Svg<'a> {
    let path = format!("assets/icons/{}.svg", name);
    svg(Handle::from_path(path))
        .width(Length::Fixed(size as f32))
        .height(Length::Fixed(size as f32))
}

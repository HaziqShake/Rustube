use iced::Color;

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
        background: Color::from_rgb8(0x0f, 0x0f, 0x0f),
        surface: Color::from_rgb8(0x1a, 0x1a, 0x1a),
        elevated: Color::from_rgb8(0x27, 0x27, 0x27),
        text_primary: Color::from_rgb8(0xf1, 0xf1, 0xf1),
        text_secondary: Color::from_rgb8(0xaa, 0xaa, 0xaa),
        accent_red: Color::from_rgb8(0xff, 0x44, 0x44),
        link_blue: Color::from_rgb8(0x3e, 0xa6, 0xff),
        border: Color::from_rgb8(0x33, 0x33, 0x33),
    }
}

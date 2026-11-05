pub mod app;
pub mod message;
pub mod pages;
pub mod router;
pub mod theme;
pub mod utils;
pub mod widgets;

use app::RustubeApp;
use iced::{Font, Theme};

fn main() -> iced::Result {
    // If DISPLAY is available on Linux, prioritize X11 backend so MPV can embed directly
    // into an X11 child subwindow right inside the application window layout.
    if std::env::var("DISPLAY").is_ok() && std::env::var("RUSTUBE_FORCE_WAYLAND").is_err() {
        unsafe {
            std::env::remove_var("WAYLAND_DISPLAY");
        }
    }

    tracing_subscriber::fmt::init();

    iced::application(
        RustubeApp::title,
        RustubeApp::update,
        RustubeApp::view,
    )
    // Roboto is YouTube's font. Bundled (assets/fonts) so the UI looks
    // identical on machines without Roboto installed; weights resolve
    // through fontdb by their usWeightClass (400/500/700).
    .font(include_bytes!("../../../assets/fonts/Roboto-Regular.ttf"))
    .font(include_bytes!("../../../assets/fonts/Roboto-Medium.ttf"))
    .font(include_bytes!("../../../assets/fonts/Roboto-Bold.ttf"))
    .default_font(Font::with_name("Roboto"))
    .theme(|_| Theme::Dark)
    .subscription(RustubeApp::subscription)
    .window(iced::window::Settings {
        size: iced::Size::new(1440.0, 900.0),
        min_size: Some(iced::Size::new(960.0, 600.0)),
        ..Default::default()
    })
    .run_with(RustubeApp::new)
}

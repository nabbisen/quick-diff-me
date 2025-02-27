use iced::Font;

mod app;
use app::{update, view};
mod core;
use core::consts::{APP_THEME, APP_TITLE};
use core::font::app_font;

/// app entry point
pub fn main() -> iced::Result {
    let app = iced::application(APP_TITLE, update, view)
        .default_font(Font::with_name(app_font()))
        .theme(|_state| APP_THEME);
    app.run()
}

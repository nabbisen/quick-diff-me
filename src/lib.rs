#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::{subscription, update, view};
use iced::Font;
mod core;
use core::{
    consts::{APP_THEME, APP_TITLE},
    font::app_default_font,
};

/// app entry point
pub fn run() -> iced::Result {
    let app = iced::application(APP_TITLE, update::handle, view::handle)
        .default_font(Font::with_name(app_default_font()))
        .subscription(subscription::handle)
        .theme(|_state| APP_THEME);
    app.run()
}

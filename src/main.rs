#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::{subscription, update, view};
mod core;
use core::consts::{APP_THEME, APP_TITLE};

/// app entry point
pub fn main() -> iced::Result {
    let app = iced::application(APP_TITLE, update, view)
        .subscription(subscription)
        .theme(|_state| APP_THEME);
    app.run()
}

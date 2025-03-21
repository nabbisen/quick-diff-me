use arboard::Clipboard;

use super::{message::Message, state::State};
use crate::core::consts::DIFF_TO_CLIPBOARD_CLICKED;
use crate::core::diff::diff;
use crate::core::utils::file_dialog;

/// iced update handler
pub fn handle(state: &mut State, message: Message) {
    let file_dialog = file_dialog(state, &message);

    match message {
        Message::EventOccurred(event) => match event {
            iced::Event::Window(event) => match event {
                iced::window::Event::FileDropped(path) => {
                    if !path.extension().is_some_and(|x| x.to_str() == Some("xlsx")) {
                        return;
                    }

                    if state.old_filepath.is_empty() {
                        state.old_filepath = path.to_string_lossy().to_string();
                    } else if state.new_filepath.is_empty() {
                        state.new_filepath = path.to_string_lossy().to_string();
                    } else {
                        state.old_filepath = path.to_string_lossy().to_string();
                        state.new_filepath = String::new();
                    }
                    diff(state);
                }
                _ => (),
            },
            _ => (),
        },
        Message::OldFileSelect => {
            let selected = file_dialog
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.old_filepath = selected;
                diff(state);
            }
        }
        Message::NewFileSelect => {
            let selected = file_dialog
                .pick_file()
                .map(|file| file.display().to_string());
            if let Some(selected) = selected {
                state.new_filepath = selected;
                diff(state);
            }
        }
        Message::Clear => state.reset(),
        Message::DiffToClipboard => {
            state.copy_to_clipboard_button_label = DIFF_TO_CLIPBOARD_CLICKED.to_owned();

            let mut clipboard = Clipboard::new().unwrap();
            let formatted_unified_diff =
                format!("{}", state.formatted_unified_diff.clone().unwrap());
            clipboard.set_text(formatted_unified_diff.as_str()).unwrap();
        }
    }
}

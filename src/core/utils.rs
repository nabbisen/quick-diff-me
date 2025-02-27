use std::env;
use std::path::{Path, PathBuf};

use rfd::FileDialog;

use super::types::{Message, State};

/// get dialog to choose file
pub fn file_dialog(state: &State, message: &Message) -> FileDialog {
    let default_directory_base_filepath =
        if !state.old_filepath.is_empty() && !state.new_filepath.is_empty() {
            match message {
                Message::NewFileSelect => Some(state.new_filepath.as_str()),
                _ => Some(state.old_filepath.as_str()),
            }
        } else if !state.old_filepath.is_empty() {
            Some(state.old_filepath.as_str())
        } else if !state.new_filepath.is_empty() {
            Some(state.new_filepath.as_str())
        } else {
            None
        };
    let default_directory = if let Some(filepath) = default_directory_base_filepath {
        Path::new(filepath).parent().unwrap().to_path_buf()
    } else {
        desktop_path()
    };

    let file_dialog = FileDialog::new()
        .add_filter("Excel", &["xlsx"])
        .add_filter("All files", &["*"])
        .set_directory(default_directory);
    file_dialog
}

/// get full path of user desktop
fn desktop_path() -> PathBuf {
    let home = env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .unwrap();
    PathBuf::from(home).join("Desktop")
}

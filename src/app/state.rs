use sheets_diff::core::diff::UnifiedDiff;

use crate::core::consts::DIFF_TO_CLIPBOARD_DEFAULT;

/// iced state
pub struct State {
    pub old_filepath: String,
    pub new_filepath: String,
    pub unified_diff: Option<UnifiedDiff>,
    pub copy_to_clipboard_button_label: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            old_filepath: String::new(),
            new_filepath: String::new(),
            unified_diff: None,
            copy_to_clipboard_button_label: DIFF_TO_CLIPBOARD_DEFAULT.to_owned(),
        }
    }
}

impl State {
    pub fn reset(&mut self) {
        *self = State::default();
    }
}

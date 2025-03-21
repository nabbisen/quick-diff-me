use sheets_diff::core::unified_format::FormattedUnifiedDiff;

use crate::core::consts::DIFF_TO_CLIPBOARD_DEFAULT;

/// iced state
pub struct State {
    pub old_filepath: String,
    pub new_filepath: String,
    pub formatted_unified_diff: Option<FormattedUnifiedDiff>,
    pub copy_to_clipboard_button_label: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            old_filepath: String::new(),
            new_filepath: String::new(),
            formatted_unified_diff: None,
            copy_to_clipboard_button_label: DIFF_TO_CLIPBOARD_DEFAULT.to_owned(),
        }
    }
}

impl State {
    pub fn reset(&mut self) {
        *self = State::default();
    }
}

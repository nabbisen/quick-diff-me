use crate::app::state::State;
use sheets_diff::core::diff::Diff;

use super::consts::DIFF_TO_CLIPBOARD_DEFAULT;

/// set diff in state
pub fn diff(state: &mut State) {
    let validated = !state.old_filepath.is_empty()
        && !state.new_filepath.is_empty()
        && state.old_filepath != state.new_filepath;
    if !validated {
        return;
    }

    let unified_diff =
        Diff::new(state.old_filepath.as_str(), state.new_filepath.as_str()).unified_diff();
    state.unified_diff = Some(unified_diff);

    state.copy_to_clipboard_button_label = DIFF_TO_CLIPBOARD_DEFAULT.to_owned();
}

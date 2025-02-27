use super::types::State;
use sheets_diff::core::diff::Diff;

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
    state.unified_diff = Some(unified_diff)
}

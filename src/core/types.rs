use iced::Event;
use sheets_diff::core::diff::UnifiedDiff;

/// iced messages
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    OldFileSelect,
    NewFileSelect,
}

/// iced state
#[derive(Default)]
pub struct State {
    pub old_filepath: String,
    pub new_filepath: String,
    pub unified_diff: Option<UnifiedDiff>,
}

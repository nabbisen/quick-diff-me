use iced::Event;

/// iced messages
#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    OldFileSelect,
    NewFileSelect,
    Clear,
    DiffToClipboard,
}

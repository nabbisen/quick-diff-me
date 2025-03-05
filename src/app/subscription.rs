use iced::{event, Subscription};

use crate::core::types::{Message, State};

/// iced subscription handler
pub fn handle(_: &State) -> Subscription<Message> {
    event::listen().map(Message::EventOccurred)
}

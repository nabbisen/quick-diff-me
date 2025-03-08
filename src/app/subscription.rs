use iced::{event, Subscription};

use super::{message::Message, state::State};

/// iced subscription handler
pub fn handle(_: &State) -> Subscription<Message> {
    event::listen().map(Message::EventOccurred)
}

use serde::Serialize;

use super::{MessageToCreate, RichMediaMessage, SendType};
use crate::models::message::{Keyboard, KeyboardPayload};

/// API message envelope used by group and C2C message APIs.
#[derive(Debug, Clone, PartialEq)]
pub enum ApiMessage {
    /// Regular message create payload
    Message(Box<MessageToCreate>),
    /// Rich media payload
    RichMedia(RichMediaMessage),
}

impl ApiMessage {
    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        match self {
            Self::Message(message) => message.send_type(),
            Self::RichMedia(message) => message.send_type(),
        }
    }

    /// Returns the event ID used for passive replies.
    pub fn event_id(&self) -> &str {
        match self {
            Self::Message(message) => message.event_id(),
            Self::RichMedia(message) => message.event_id(),
        }
    }
}

impl Serialize for ApiMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Message(message) => message.serialize(serializer),
            Self::RichMedia(message) => message.serialize(serializer),
        }
    }
}

impl From<MessageToCreate> for ApiMessage {
    fn from(message: MessageToCreate) -> Self {
        Self::Message(Box::new(message))
    }
}

impl From<RichMediaMessage> for ApiMessage {
    fn from(message: RichMediaMessage) -> Self {
        Self::RichMedia(message)
    }
}

impl From<KeyboardPayload> for Keyboard {
    fn from(payload: KeyboardPayload) -> Self {
        serde_json::from_value(payload.content).unwrap_or_default()
    }
}

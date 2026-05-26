use serde::Serialize;

use super::{MessageToCreate, RichMediaMessage, SendType};
use crate::models::message::{Keyboard, KeyboardPayload, Reference};

/// Message interface.
pub trait APIMessage: Serialize {
    /// Returns the event ID used for passive replies.
    #[allow(non_snake_case)]
    fn GetEventID(&self) -> &str;

    /// Returns the route family for this message payload.
    #[allow(non_snake_case)]
    fn GetSendType(&self) -> SendType;
}

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

    /// Event ID accessor.
    #[allow(non_snake_case)]
    pub fn GetEventID(&self) -> &str {
        match self {
            Self::Message(message) => message.GetEventID(),
            Self::RichMedia(message) => message.GetEventID(),
        }
    }

    /// Send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

impl APIMessage for MessageToCreate {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
    }
}

impl APIMessage for RichMediaMessage {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
    }
}

impl APIMessage for ApiMessage {
    fn GetEventID(&self) -> &str {
        self.GetEventID()
    }

    fn GetSendType(&self) -> SendType {
        self.GetSendType()
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

impl APIMessage for Reference {
    fn GetEventID(&self) -> &str {
        self.message_id.as_deref().unwrap_or("")
    }

    fn GetSendType(&self) -> SendType {
        SendType::Text
    }
}

impl From<KeyboardPayload> for Keyboard {
    fn from(payload: KeyboardPayload) -> Self {
        serde_json::from_value(payload.content).unwrap_or_default()
    }
}

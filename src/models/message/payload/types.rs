use serde::{Deserialize, Serialize};

pub(crate) fn option_message_type_is_none_or_zero(value: &Option<MessageCreateType>) -> bool {
    value.as_ref().is_none_or(|value| u32::from(*value) == 0)
}

/// Message send type used to select API routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum SendType {
    /// Regular text/message route
    Text = 1,
    /// Rich media file route
    RichMedia = 2,
    /// Unknown send type
    Unknown(u8),
}

wire_enum!(SendType, u8, Unknown, {
    Text = 1,
    RichMedia = 2,
});

/// Message type used by the message create APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum MessageCreateType {
    /// Text message
    Text = 0,
    /// Markdown message
    Markdown = 2,
    /// Ark message
    Ark = 3,
    /// Embed message
    Embed = 4,
    /// Mention message
    At = 5,
    /// Input status notification
    InputNotify = 6,
    /// Rich media message
    RichMedia = 7,
    /// Unknown message type
    Unknown(u32),
}

wire_enum!(MessageCreateType, u32, Unknown, {
    Text = 0,
    Markdown = 2,
    Ark = 3,
    Embed = 4,
    At = 5,
    InputNotify = 6,
    RichMedia = 7,
});

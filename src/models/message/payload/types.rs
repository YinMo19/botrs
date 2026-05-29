use serde::{Deserialize, Serialize};

pub(crate) fn option_message_type_is_none_or_zero(value: &Option<MessageCreateType>) -> bool {
    value.as_ref().is_none_or(|value| u32::from(*value) == 0)
}

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
    RichMedia = 7,
});

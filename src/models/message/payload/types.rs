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

impl From<u8> for SendType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Text,
            2 => Self::RichMedia,
            other => Self::Unknown(other),
        }
    }
}

impl From<SendType> for u8 {
    fn from(send_type: SendType) -> Self {
        match send_type {
            SendType::Text => 1,
            SendType::RichMedia => 2,
            SendType::Unknown(value) => value,
        }
    }
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
    /// Mention message
    At = 5,
    /// Input status notification
    InputNotify = 6,
    /// Rich media message
    RichMedia = 7,
    /// Unknown message type
    Unknown(u32),
}

#[allow(non_upper_case_globals)]
pub const TextMsg: MessageCreateType = MessageCreateType::Text;
#[allow(non_upper_case_globals)]
pub const MarkdownMsg: MessageCreateType = MessageCreateType::Markdown;
#[allow(non_upper_case_globals)]
pub const ArkMsg: MessageCreateType = MessageCreateType::Ark;
#[allow(non_upper_case_globals)]
pub const EmbedMsg: MessageCreateType = MessageCreateType::Embed;
#[allow(non_upper_case_globals)]
pub const ATMsg: MessageCreateType = MessageCreateType::At;
#[allow(non_upper_case_globals)]
pub const InputNotifyMsg: MessageCreateType = MessageCreateType::InputNotify;
#[allow(non_upper_case_globals)]
pub const RichMediaMsg: MessageCreateType = MessageCreateType::RichMedia;
#[allow(non_upper_case_globals)]
pub const RichMedia: SendType = SendType::RichMedia;

impl From<u32> for MessageCreateType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Text,
            2 => Self::Markdown,
            3 => Self::Ark,
            4 => Self::Embed,
            5 => Self::At,
            6 => Self::InputNotify,
            7 => Self::RichMedia,
            other => Self::Unknown(other),
        }
    }
}

impl From<MessageCreateType> for u32 {
    fn from(message_type: MessageCreateType) -> Self {
        match message_type {
            MessageCreateType::Text => 0,
            MessageCreateType::Markdown => 2,
            MessageCreateType::Ark => 3,
            MessageCreateType::Embed => 4,
            MessageCreateType::At => 5,
            MessageCreateType::InputNotify => 6,
            MessageCreateType::RichMedia => 7,
            MessageCreateType::Unknown(value) => value,
        }
    }
}

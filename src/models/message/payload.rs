use crate::models::serde_helpers::option_is_none_or_default;
use base64::Engine;
use serde::{Deserialize, Serialize};

use super::{Ark, Embed, Keyboard, KeyboardPayload, MarkdownPayload, Media, Reference};

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

/// Input status notification payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InputNotify {
    /// Input status type
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_type: Option<i32>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub input_second: Option<i32>,
}

/// Rich media info used after uploading media.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaInfo {
    /// Uploaded rich media file info
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub file_info: Option<String>,
}

impl From<Media> for MediaInfo {
    fn from(media: Media) -> Self {
        Self {
            file_info: media.file_info,
        }
    }
}

impl From<MediaInfo> for Media {
    fn from(media: MediaInfo) -> Self {
        Self {
            file_info: media.file_info,
            ttl: None,
        }
    }
}

/// Streamed message fragment metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Stream {
    /// Stream state
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub state: Option<i32>,
    /// Stream ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub id: Option<String>,
    /// Fragment index
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub index: Option<i32>,
    /// Whether to reset an unfinished stream
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub reset: Option<bool>,
}

/// Prompt keyboard wrapper used by message extension areas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PromptKeyboard {
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
}

/// Message action button configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ActionButton {
    /// Action bar template ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub template_id: Option<i32>,
    /// Callback payload
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub callback_data: Option<String>,
    /// Feedback button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub feedback: Option<bool>,
    /// TTS button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub tts: Option<bool>,
    /// Regenerate button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub re_generate: Option<bool>,
    /// Stop generation button
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub stop_generate: Option<bool>,
}

/// Channel/direct message create payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageToCreate {
    /// Message content
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content: Option<String>,
    /// Message type
    #[serde(skip_serializing_if = "option_message_type_is_none_or_zero")]
    pub msg_type: Option<MessageCreateType>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark: Option<Ark>,
    /// Image URL
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub image: Option<String>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub msg_id: Option<String>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<Reference>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<Keyboard>,
    /// Event ID to reply to
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub event_id: Option<String>,
    /// Deprecated timestamp field kept for backwards compatibility
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub timestamp: Option<i64>,
    /// Message sequence number
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub msg_seq: Option<u32>,
    /// Subscription ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub subscribe_id: Option<String>,
    /// Input status notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_notify: Option<InputNotify>,
    /// Rich media info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaInfo>,
    /// Prompt keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_keyboard: Option<PromptKeyboard>,
    /// Action button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_button: Option<ActionButton>,
    /// Stream info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
    /// Feature control ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub feature_id: Option<u32>,
    /// Base64 encoded file image, supported by the legacy Rust API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_image: Option<String>,
}

impl MessageToCreate {
    /// Creates a text message payload.
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }

    /// Sets file image data, automatically encoding to base64.
    pub fn with_file_image(mut self, data: &[u8]) -> Self {
        self.file_image = Some(base64::engine::general_purpose::STANDARD.encode(data));
        self
    }

    /// Sets the message ID to reply to.
    pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
        self.msg_id = Some(message_id.into());
        self
    }

    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        SendType::Text
    }

    /// Event ID accessor.
    #[allow(non_snake_case)]
    pub fn GetEventID(&self) -> &str {
        self.event_id.as_deref().unwrap_or("")
    }

    /// Send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

/// Rich media upload/direct-send payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RichMediaMessage {
    /// Deprecated event ID field, kept for API compatibility
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub event_id: Option<String>,
    /// File type, 1=image, 2=video, 3=audio
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub file_type: Option<u64>,
    /// Rich media URL
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub url: Option<String>,
    /// Whether the server should send the message directly
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub srv_send_msg: Option<bool>,
    /// Optional text content
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub content: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub msg_seq: Option<i64>,
}

impl RichMediaMessage {
    /// Creates a rich media payload with file type and URL.
    pub fn new(file_type: u64, url: impl Into<String>) -> Self {
        Self {
            file_type: Some(file_type),
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Returns the send type for route selection.
    pub const fn send_type(&self) -> SendType {
        SendType::RichMedia
    }

    /// Event ID accessor. Botgo intentionally returns
    /// an empty value for rich media payloads.
    #[allow(non_snake_case)]
    pub const fn GetEventID(&self) -> &str {
        ""
    }

    /// Send type accessor.
    #[allow(non_snake_case)]
    pub const fn GetSendType(&self) -> SendType {
        self.send_type()
    }
}

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

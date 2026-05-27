use crate::models::serde_helpers::option_is_none_or_default;
use base64::Engine;
use serde::{Deserialize, Serialize};

use super::{
    ActionButton, InputNotify, MediaInfo, MessageCreateType, PromptKeyboard, SendType, Stream,
    option_message_type_is_none_or_zero,
};
use crate::models::message::{Ark, Embed, Keyboard, MarkdownPayload, Reference};

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

    /// Returns the event ID used for passive replies.
    pub fn event_id(&self) -> &str {
        self.event_id.as_deref().unwrap_or("")
    }
}

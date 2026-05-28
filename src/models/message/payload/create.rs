use crate::models::serde_helpers::option_is_none_or_default;
use serde::{Deserialize, Serialize};

use super::{MediaInfo, MessageCreateType, option_message_type_is_none_or_zero};
use crate::models::message::{
    ActionButton, Ark, Embed, InputNotify, Keyboard, MarkdownPayload, PromptKeyboard, Reference,
    Stream,
};

/// Channel/direct message create payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct MessageToCreate {
    /// Message content
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) content: Option<String>,
    /// Message type
    #[serde(skip_serializing_if = "option_message_type_is_none_or_zero")]
    pub(crate) msg_type: Option<MessageCreateType>,
    /// Message embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) embed: Option<Embed>,
    /// Ark template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ark: Option<Ark>,
    /// Image URL
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) image: Option<String>,
    /// Message ID to reply to
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) msg_id: Option<String>,
    /// Message reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) message_reference: Option<Reference>,
    /// Markdown payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) markdown: Option<MarkdownPayload>,
    /// Keyboard payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) keyboard: Option<Keyboard>,
    /// Event ID to reply to
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) event_id: Option<String>,
    /// Message sequence number
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) msg_seq: Option<u32>,
    /// Rich media info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) media: Option<MediaInfo>,
    /// Subscribe message template ID.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) subscribe_id: Option<String>,
    /// Input notification payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_notify: Option<InputNotify>,
    /// Prompt keyboard payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) prompt_keyboard: Option<PromptKeyboard>,
    /// Message action button payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_button: Option<ActionButton>,
    /// Streaming message metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<Stream>,
    /// Feature ID controlling message send behavior.
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub(crate) feature_id: Option<u32>,
}

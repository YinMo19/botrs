use crate::models::serde_helpers::{is_zero_u32, option_is_none_or_default};
use base64::Engine;
use serde::{Deserialize, Serialize};

use super::{
    option_message_type_is_none_or_zero, ActionButton, Ark, Embed, InputNotify, Keyboard,
    KeyboardPayload, MarkdownPayload, Media, MediaInfo, MessageCreateType, MessageToCreate,
    PromptKeyboard, Reference, Stream,
};

macro_rules! channel_like_message_params_struct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
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
            /// Message reference
            #[serde(skip_serializing_if = "Option::is_none")]
            pub message_reference: Option<Reference>,
            /// Image URL
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub image: Option<String>,
            /// Base64 encoded file image
            #[serde(skip_serializing_if = "Option::is_none")]
            pub file_image: Option<String>,
            /// Message ID to reply to
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub msg_id: Option<String>,
            /// Event ID
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub event_id: Option<String>,
            /// Markdown payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub markdown: Option<MarkdownPayload>,
            /// Keyboard payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub keyboard: Option<Keyboard>,
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
        }
    };
}

macro_rules! open_message_params_struct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            /// Message type (0=text, 1=rich text, 2=markdown, 3=ark, 4=embed, 7=media)
            #[serde(skip_serializing_if = "is_zero_u32")]
            pub msg_type: u32,
            /// Message content
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub content: Option<String>,
            /// Message embed
            #[serde(skip_serializing_if = "Option::is_none")]
            pub embed: Option<Embed>,
            /// Ark template
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ark: Option<Ark>,
            /// Message reference
            #[serde(skip_serializing_if = "Option::is_none")]
            pub message_reference: Option<Reference>,
            /// Media attachment
            #[serde(skip_serializing_if = "Option::is_none")]
            pub media: Option<Media>,
            /// Message ID to reply to
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub msg_id: Option<String>,
            /// Message sequence number
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub msg_seq: Option<u32>,
            /// Event ID
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub event_id: Option<String>,
            /// Markdown payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub markdown: Option<MarkdownPayload>,
            /// Keyboard payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub keyboard: Option<KeyboardPayload>,
            /// Deprecated timestamp field kept for backwards compatibility
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub timestamp: Option<i64>,
            /// Subscription ID
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub subscribe_id: Option<String>,
            /// Input status notification
            #[serde(skip_serializing_if = "Option::is_none")]
            pub input_notify: Option<InputNotify>,
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
        }
    };
}

channel_like_message_params_struct! {
    /// Parameters for sending a message to a channel.
    pub struct MessageParams;
}

open_message_params_struct! {
    /// Parameters for sending a group message.
    pub struct GroupMessageParams;
}

open_message_params_struct! {
    /// Parameters for sending a C2C (client-to-client) message.
    pub struct C2CMessageParams;
}

channel_like_message_params_struct! {
    /// Parameters for sending a direct message.
    pub struct DirectMessageParams;
}

macro_rules! impl_channel_like_message_params {
    ($name:ident) => {
        impl $name {
            /// Creates a new parameter object with text content.
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

            /// Sets the message reference for replying.
            pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
                self.msg_id = Some(message_id.into());
                self
            }

            /// Converts this payload into the message create body.
            pub fn into_message_to_create(self) -> MessageToCreate {
                self.into()
            }
        }

        impl From<$name> for MessageToCreate {
            fn from(params: $name) -> Self {
                Self {
                    content: params.content,
                    msg_type: params.msg_type,
                    embed: params.embed,
                    ark: params.ark,
                    image: params.image,
                    msg_id: params.msg_id,
                    message_reference: params.message_reference,
                    markdown: params.markdown,
                    keyboard: params.keyboard,
                    event_id: params.event_id,
                    timestamp: params.timestamp,
                    msg_seq: params.msg_seq,
                    subscribe_id: params.subscribe_id,
                    input_notify: params.input_notify,
                    media: params.media,
                    prompt_keyboard: params.prompt_keyboard,
                    action_button: params.action_button,
                    stream: params.stream,
                    feature_id: params.feature_id,
                    file_image: params.file_image,
                }
            }
        }
    };
}

macro_rules! impl_open_message_params {
    ($name:ident) => {
        impl $name {
            /// Creates a new parameter object with text content.
            pub fn new_text(content: impl Into<String>) -> Self {
                Self {
                    msg_type: 0,
                    content: Some(content.into()),
                    ..Default::default()
                }
            }

            /// Sets the message reference for replying.
            pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
                self.msg_id = Some(message_id.into());
                self
            }

            /// Converts this payload into the message create body.
            pub fn into_message_to_create(self) -> MessageToCreate {
                self.into()
            }
        }

        impl From<$name> for MessageToCreate {
            fn from(params: $name) -> Self {
                Self {
                    content: params.content,
                    msg_type: Some(MessageCreateType::from(params.msg_type)),
                    embed: params.embed,
                    ark: params.ark,
                    msg_id: params.msg_id,
                    message_reference: params.message_reference,
                    markdown: params.markdown,
                    keyboard: params.keyboard.map(Into::into),
                    event_id: params.event_id,
                    timestamp: params.timestamp,
                    msg_seq: params.msg_seq,
                    subscribe_id: params.subscribe_id,
                    input_notify: params.input_notify,
                    media: params.media.map(Into::into),
                    prompt_keyboard: params.prompt_keyboard,
                    action_button: params.action_button,
                    stream: params.stream,
                    feature_id: params.feature_id,
                    ..Default::default()
                }
            }
        }
    };
}

impl_channel_like_message_params!(MessageParams);
impl_open_message_params!(GroupMessageParams);
impl_open_message_params!(C2CMessageParams);
impl_channel_like_message_params!(DirectMessageParams);

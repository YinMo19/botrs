use crate::models::serde_helpers::{is_zero_u32, option_is_none_or_default};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::payload::option_message_type_is_none_or_zero;
use super::{
    ActionButton, Ark, Embed, InputNotify, Keyboard, KeyboardPayload, MarkdownPayload, Media,
    MessageCreateType, MessageToCreate, PromptKeyboard, Reference, Stream,
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
            /// Message sequence number
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub msg_seq: Option<u32>,
            /// Subscribe message template ID
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub subscribe_id: Option<String>,
            /// Input notification payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub input_notify: Option<InputNotify>,
            /// Prompt keyboard payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub prompt_keyboard: Option<PromptKeyboard>,
            /// Message action button payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub action_button: Option<ActionButton>,
            /// Streaming message metadata
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream: Option<Stream>,
            /// Feature ID controlling message send behavior
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
            /// Subscribe message template ID
            #[serde(skip_serializing_if = "option_is_none_or_default")]
            pub subscribe_id: Option<String>,
            /// Input notification payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub input_notify: Option<InputNotify>,
            /// Prompt keyboard payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub prompt_keyboard: Option<PromptKeyboard>,
            /// Message action button payload
            #[serde(skip_serializing_if = "Option::is_none")]
            pub action_button: Option<ActionButton>,
            /// Streaming message metadata
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream: Option<Stream>,
            /// Feature ID controlling message send behavior
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

            /// Creates a new parameter object with raw markdown content.
            pub fn new_markdown(content: impl Into<String>) -> Self {
                Self {
                    msg_type: Some(MessageCreateType::Markdown),
                    markdown: Some(MarkdownPayload {
                        content: Some(content.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            }

            /// Creates a new parameter object with an Ark payload.
            pub fn new_ark(ark: Ark) -> Self {
                Self {
                    msg_type: Some(MessageCreateType::Ark),
                    ark: Some(ark),
                    ..Default::default()
                }
            }

            /// Creates a new parameter object with an embed payload.
            pub fn new_embed(embed: Embed) -> Self {
                Self {
                    msg_type: Some(MessageCreateType::Embed),
                    embed: Some(embed),
                    ..Default::default()
                }
            }

            /// Creates a new markdown message with a keyboard payload.
            pub fn new_keyboard(content: impl Into<String>, keyboard: Keyboard) -> Self {
                Self {
                    msg_type: Some(MessageCreateType::Markdown),
                    markdown: Some(MarkdownPayload {
                        content: Some(content.into()),
                        ..Default::default()
                    }),
                    keyboard: Some(keyboard),
                    ..Default::default()
                }
            }

            /// Sets the message reference for replying.
            pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
                self.msg_id = Some(message_id.into());
                self
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
                    msg_seq: params.msg_seq,
                    media: None,
                    subscribe_id: params.subscribe_id,
                    input_notify: params.input_notify,
                    prompt_keyboard: params.prompt_keyboard,
                    action_button: params.action_button,
                    stream: params.stream,
                    feature_id: params.feature_id,
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

            /// Creates a new parameter object with raw markdown content.
            pub fn new_markdown(content: impl Into<String>) -> Self {
                Self {
                    msg_type: 2,
                    markdown: Some(MarkdownPayload {
                        content: Some(content.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            }

            /// Creates a new parameter object with an Ark payload.
            pub fn new_ark(ark: Ark) -> Self {
                Self {
                    msg_type: 3,
                    ark: Some(ark),
                    ..Default::default()
                }
            }

            /// Creates a new parameter object with an embed payload.
            pub fn new_embed(embed: Embed) -> Self {
                Self {
                    msg_type: 4,
                    embed: Some(embed),
                    ..Default::default()
                }
            }

            /// Creates a new parameter object with an uploaded media payload.
            pub fn new_media(media: Media) -> Self {
                Self {
                    msg_type: 7,
                    media: Some(media),
                    ..Default::default()
                }
            }

            /// Creates a new markdown message with a keyboard payload.
            pub fn new_keyboard(content: impl Into<String>, keyboard: KeyboardPayload) -> Self {
                Self {
                    msg_type: 2,
                    markdown: Some(MarkdownPayload {
                        content: Some(content.into()),
                        ..Default::default()
                    }),
                    keyboard: Some(keyboard),
                    ..Default::default()
                }
            }

            /// Sets the message reference for replying.
            pub fn with_reply(mut self, message_id: impl Into<String>) -> Self {
                self.msg_id = Some(message_id.into());
                self
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
                    msg_seq: params.msg_seq,
                    media: params.media.map(Into::into),
                    subscribe_id: params.subscribe_id,
                    input_notify: params.input_notify,
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

/// Direction used when listing channel messages around an anchor message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessagePagerType {
    /// Fetch messages around the anchor message.
    Around,
    /// Fetch messages before the anchor message.
    Before,
    /// Fetch messages after the anchor message.
    After,
}

impl MessagePagerType {
    fn as_query_key(self) -> &'static str {
        match self {
            Self::Around => "around",
            Self::Before => "before",
            Self::After => "after",
        }
    }
}

/// Query parameters for listing channel messages.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessagesPager {
    /// Anchor mode and message ID.
    pub anchor: Option<(MessagePagerType, String)>,
    /// Maximum number of messages to return.
    pub limit: Option<u32>,
}

impl MessagesPager {
    /// Creates a pager that only sets the limit.
    pub fn new(limit: impl Into<Option<u32>>) -> Self {
        Self {
            limit: limit.into(),
            ..Default::default()
        }
    }

    /// Fetches messages before the provided message ID.
    pub fn before(message_id: impl Into<String>, limit: impl Into<Option<u32>>) -> Self {
        Self {
            anchor: Some((MessagePagerType::Before, message_id.into())),
            limit: limit.into(),
        }
    }

    /// Fetches messages after the provided message ID.
    pub fn after(message_id: impl Into<String>, limit: impl Into<Option<u32>>) -> Self {
        Self {
            anchor: Some((MessagePagerType::After, message_id.into())),
            limit: limit.into(),
        }
    }

    /// Fetches messages around the provided message ID.
    pub fn around(message_id: impl Into<String>, limit: impl Into<Option<u32>>) -> Self {
        Self {
            anchor: Some((MessagePagerType::Around, message_id.into())),
            limit: limit.into(),
        }
    }

    pub(crate) fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit
            && limit != 0
        {
            query.insert("limit", limit.to_string());
        }
        if let Some((pager_type, message_id)) = &self.anchor
            && !message_id.is_empty()
        {
            query.insert(pager_type.as_query_key(), message_id.clone());
        }
        query
    }
}

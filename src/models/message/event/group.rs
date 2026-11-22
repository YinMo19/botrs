use super::{GroupMessageUser, MessageAttachment, MessageReference, MessageScene};
use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// A received structured card. QQ chooses the keys in `fields` by card type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ReceivedArk {
    pub prompt: String,
    pub ark_type: String,
    pub ark_name: String,
    pub fields: Map<String, Value>,
}

/// One element supplied by QQ, including nested records and quoted messages.
/// Its presence alone does not identify the group's receive mode.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupMessageElement {
    pub msg_idx: Option<String>,
    pub message_type: Option<u32>,
    pub content: String,
    pub attachments: Vec<MessageAttachment>,
    pub ark_data: Option<ReceivedArk>,
    pub msg_elements: Vec<Self>,
    /// Additional fields supplied with this element.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Represents a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMessage {
    /// The message's unique ID
    pub id: Snowflake,
    /// The message content
    #[serde(default)]
    pub content: String,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    #[serde(default)]
    pub mentions: Vec<GroupMessageUser>,
    /// Attachments in this message
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub msg_seq: Option<u64>,
    /// When this message was sent
    pub timestamp: Timestamp,
    /// The author of this message
    pub author: GroupMessageUser,
    /// Group ID
    #[serde(default)]
    pub group_id: Snowflake,
    /// Group OpenID
    #[serde(default)]
    pub group_openid: String,
    /// Message scene information
    #[serde(default)]
    pub message_scene: MessageScene,
    /// Open-message type code.
    #[serde(default)]
    pub message_type: u32,
    /// Related messages as delivered, in wire order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub msg_elements: Vec<GroupMessageElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ark_data: Option<Box<ReceivedArk>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_group_openid: Option<String>,
    /// Additional fields, including context extensions when supplied by QQ.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl GroupMessage {
    /// Visible quote for this incoming message. QQ supplies its index in
    /// `message_scene.ext`; `ref_msg_idx` refers to an earlier quoted message.
    /// The session supplies the separate passive reply ID when sending.
    pub fn quote_reference(&self) -> Option<crate::models::message::Reference> {
        self.message_scene
            .ext
            .iter()
            .filter_map(|field| field.strip_prefix("msg_idx="))
            .map(str::trim)
            .find(|index| !index.is_empty())
            .map(|index| crate::models::message::Reference {
                message_id: Some(index.to_owned()),
                ignore_get_message_error: Some(true),
            })
    }

    /// Current text with an explicitly marked leading mention of this bot removed.
    /// QQ already strips this prefix from mention-only deliveries. Mentions of
    /// other members and `<@all>` are preserved.
    pub fn content_without_bot_mention(&self) -> &str {
        let content = self.content.trim_start();
        if let Some((id, rest)) = content.strip_prefix("<@").and_then(|s| s.split_once('>'))
            && !id.is_empty()
            && id != "all"
            && self
                .mentions
                .iter()
                .any(|m| m.is_you && (m.member_openid == id || m.id == id))
        {
            return rest.trim_start();
        }
        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn mention_context_is_ordered_and_only_the_current_text_is_a_command() {
        let elements: Vec<_> = (0..10)
            .map(|i| json!({"content":format!("/command {i}")}))
            .collect();
        let mut message: GroupMessage = serde_json::from_value(json!({
            "id":"now", "author":{"member_openid":"sender"}, "timestamp":"2026-07-21T09:00:00+08:00",
            "content":" question", "msg_elements":elements, "mentions":[{"id":"other","bot":true}]
        })).unwrap();
        assert_eq!(message.content_without_bot_mention(), "question");
        assert!(
            message
                .msg_elements
                .iter()
                .enumerate()
                .all(|(i, e)| e.content == format!("/command {i}"))
        );
        for text in ["<@other> /ping", "<@all> /ping", "<@> /ping"] {
            message.content = text.into();
            assert_eq!(message.content_without_bot_mention(), text);
        }
    }
}

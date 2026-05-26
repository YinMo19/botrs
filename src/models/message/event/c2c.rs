use super::{C2CMessageUser, MessageAttachment, MessageReference};
use crate::models::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::message::C2CMessageParams;

/// Represents a C2C (client-to-client) message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct C2CMessage {
    /// The message's unique ID
    pub id: Option<String>,
    /// The message content
    pub content: Option<String>,
    /// Referenced message information
    pub message_reference: Option<MessageReference>,
    /// Users mentioned in this message
    #[serde(default)]
    pub mentions: Vec<C2CMessageUser>,
    /// Attachments in this message
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    /// Global message sequence number
    pub msg_seq: Option<u64>,
    /// When this message was sent
    pub timestamp: Option<Timestamp>,
    /// The author of this message
    pub author: Option<C2CMessageUser>,
    /// Message scene information
    pub message_scene: Option<Value>,
    /// Event ID from the gateway
    pub event_id: Option<String>,
}

impl C2CMessage {
    /// Creates a new C2C message.
    pub fn new() -> Self {
        Self {
            id: None,
            content: None,
            message_reference: None,
            mentions: Vec::new(),
            attachments: Vec::new(),
            msg_seq: None,
            timestamp: None,
            author: None,
            message_scene: None,
            event_id: None,
        }
    }

    /// Creates a new C2C message from API data.
    pub fn from_data(_api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let mut message: Self = serde_json::from_value(data).unwrap_or_default();
        message.event_id = Some(event_id);
        message
    }

    /// Reply to this C2C message
    pub async fn reply(
        &self,
        api: &crate::api::BotApi,
        token: &crate::token::Token,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(user_openid), Some(msg_id)) = (
            self.author.as_ref().and_then(|a| a.user_openid.as_ref()),
            &self.id,
        ) {
            let params = C2CMessageParams {
                msg_type: 0,
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                msg_seq: Some(1),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.post_c2c_message_with_params(token, user_openid, params)
                .await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing user_openid or message_id for C2C reply".to_string(),
            ))
        }
    }
}

impl Default for C2CMessage {
    fn default() -> Self {
        Self::new()
    }
}

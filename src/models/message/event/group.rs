use super::{GroupMessageUser, MessageAttachment, MessageReference};
use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

use crate::models::message::GroupMessageParams;

/// Represents a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMessage {
    /// The message's unique ID
    pub id: Option<Snowflake>,
    /// The message content
    pub content: Option<String>,
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
    pub timestamp: Option<Timestamp>,
    /// The author of this message
    pub author: Option<GroupMessageUser>,
    /// Group OpenID
    pub group_openid: Option<String>,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl GroupMessage {
    /// Reply to this group message
    pub async fn reply(
        &self,
        api: &crate::api_impl::BotApi,
        content: &str,
    ) -> Result<crate::models::api::MessageResponse, crate::error::BotError> {
        if let (Some(group_openid), Some(msg_id)) = (&self.group_openid, &self.id) {
            let params = GroupMessageParams {
                msg_type: 0,
                content: Some(content.to_string()),
                msg_id: Some(msg_id.clone()),
                event_id: self.event_id.clone(),
                ..Default::default()
            };
            api.send_group_message(group_openid, params).await
        } else {
            Err(crate::error::BotError::InvalidData(
                "Missing group_openid or message_id for reply".to_string(),
            ))
        }
    }
}

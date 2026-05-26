use super::Message;
use serde::{Deserialize, Serialize};

/// Message delete event payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDelete {
    /// Deleted message
    pub message: Message,
    /// User who performed the operation
    pub op_user: crate::models::User,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

impl MessageDelete {
    /// Creates a message delete payload from gateway data.
    pub fn from_data(api: crate::api::BotApi, event_id: String, data: serde_json::Value) -> Self {
        let message_data = data.get("message").cloned().unwrap_or_else(|| data.clone());
        let op_user = data
            .get("op_user")
            .cloned()
            .map(crate::models::User::from_data)
            .unwrap_or_default();
        Self {
            message: Message::from_data(api, event_id.clone(), message_data),
            op_user,
            event_id: Some(event_id),
        }
    }
}

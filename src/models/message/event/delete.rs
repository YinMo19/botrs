use super::Message;
use serde::{Deserialize, Serialize};

/// Message delete event payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageDelete {
    /// Deleted message
    pub message: Message,
    /// User who performed the operation
    pub op_user: crate::models::user::User,
    /// Event ID from the gateway
    #[serde(skip)]
    pub event_id: Option<String>,
}

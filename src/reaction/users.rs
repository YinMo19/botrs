use crate::models::user::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type ReactionUser = User;

/// Reaction users response structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactionUsers {
    /// List of users who reacted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
    /// Pagination cookie for next page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// Whether this is the last page
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_end: bool,
}

pub type MessageReactionUsers = ReactionUsers;

impl ReactionUsers {
    /// Create a new ReactionUsers instance from JSON data
    pub fn new(data: &Value) -> crate::Result<Self> {
        Ok(serde_json::from_value(data.clone())?)
    }

    /// Check if there are more pages available
    pub fn has_more_pages(&self) -> bool {
        !self.is_end
    }

    /// Get the number of users in this page
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

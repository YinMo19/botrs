use crate::models::user::User;
use serde::{Deserialize, Serialize};

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

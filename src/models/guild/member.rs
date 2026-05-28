use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a member of a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Member {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The user information
    #[serde(default)]
    pub user: Option<crate::models::user::User>,
    /// The member's nickname in the guild
    #[serde(default)]
    pub nick: String,
    /// The member's roles in the guild
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// When the member joined the guild
    #[serde(default)]
    pub joined_at: Timestamp,
    /// Operator user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub op_user_id: Snowflake,
}

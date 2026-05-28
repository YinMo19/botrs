use crate::models::{Snowflake, Timestamp, channel::Channel};
use serde::{Deserialize, Serialize};

/// Response returned by the guild role members endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildRoleMembers {
    /// Role members in the current page
    #[serde(default)]
    pub data: Vec<Member>,
    /// Cursor for the next page
    #[serde(default)]
    pub next: String,
}

/// Body used when adding or deleting a member role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberAddRoleBody {
    /// Channel object for channel administrator roles.
    pub channel: Option<Channel>,
}

/// Additional options for deleting a guild member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MemberDeleteOptions {
    /// Whether to add the member to the guild blacklist
    pub add_blacklist: bool,
    /// How many days of history to retract
    pub delete_history_msg_days: i32,
}

/// Normalizes history deletion days to the official supported values.
pub fn normalize_delete_history_msg_days(days: i32) -> i32 {
    match days {
        3 | 7 | 15 | 30 | -1 => days,
        _ => 0,
    }
}

/// Represents a member of a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Member {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The user information
    #[serde(default)]
    pub user: Option<crate::models::User>,
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

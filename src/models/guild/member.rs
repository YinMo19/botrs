use crate::models::{HasId, Snowflake, Timestamp, channel::Channel};
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

/// Supported history deletion windows when removing a guild member.
pub type DeleteHistoryMsgDay = i32;

pub const NO_DELETE: DeleteHistoryMsgDay = 0;
pub const DELETE_THREE_DAYS: DeleteHistoryMsgDay = 3;
pub const DELETE_SEVEN_DAYS: DeleteHistoryMsgDay = 7;
pub const DELETE_FIFTEEN_DAYS: DeleteHistoryMsgDay = 15;
pub const DELETE_THIRTY_DAYS: DeleteHistoryMsgDay = 30;
pub const DELETE_ALL: DeleteHistoryMsgDay = -1;

/// Additional options for deleting a guild member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberDeleteOptions {
    /// Whether to add the member to the guild blacklist
    pub add_blacklist: bool,
    /// How many days of history to retract
    pub delete_history_msg_days: DeleteHistoryMsgDay,
}

impl Default for MemberDeleteOptions {
    fn default() -> Self {
        Self {
            add_blacklist: false,
            delete_history_msg_days: NO_DELETE,
        }
    }
}

/// Normalizes history deletion days to the official supported values.
pub fn normalize_delete_history_msg_days(days: DeleteHistoryMsgDay) -> DeleteHistoryMsgDay {
    match days {
        DELETE_THREE_DAYS | DELETE_SEVEN_DAYS | DELETE_FIFTEEN_DAYS | DELETE_THIRTY_DAYS
        | DELETE_ALL => days,
        _ => NO_DELETE,
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

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        self.user.as_ref().map(|user| &user.id)
    }
}

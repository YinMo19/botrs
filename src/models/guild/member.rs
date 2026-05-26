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

impl MemberAddRoleBody {
    /// Creates a body without a channel.
    pub fn new() -> Self {
        Self { channel: None }
    }

    /// Creates a body for a channel-specific role.
    pub fn with_channel_id(channel_id: impl Into<String>) -> Self {
        let mut channel = Channel::new();
        channel.id = channel_id.into();
        Self {
            channel: Some(channel),
        }
    }
}

/// Supported history deletion windows when removing a guild member.
pub type DeleteHistoryMsgDay = i32;

pub const NO_DELETE: DeleteHistoryMsgDay = 0;
pub const DELETE_THREE_DAYS: DeleteHistoryMsgDay = 3;
pub const DELETE_SEVEN_DAYS: DeleteHistoryMsgDay = 7;
pub const DELETE_FIFTEEN_DAYS: DeleteHistoryMsgDay = 15;
pub const DELETE_THIRTY_DAYS: DeleteHistoryMsgDay = 30;
pub const DELETE_ALL: DeleteHistoryMsgDay = -1;
#[allow(non_upper_case_globals)]
pub const NoDelete: DeleteHistoryMsgDay = NO_DELETE;
#[allow(non_upper_case_globals)]
pub const DeleteThreeDays: DeleteHistoryMsgDay = DELETE_THREE_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteSevenDays: DeleteHistoryMsgDay = DELETE_SEVEN_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteFifteenDays: DeleteHistoryMsgDay = DELETE_FIFTEEN_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteThirtyDays: DeleteHistoryMsgDay = DELETE_THIRTY_DAYS;
#[allow(non_upper_case_globals)]
pub const DeleteAll: DeleteHistoryMsgDay = DELETE_ALL;

pub type MemberDeleteOpts = MemberDeleteOptions;

/// Additional options for deleting a guild member.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberDeleteOptions {
    /// Whether to add the member to the guild blacklist
    pub add_blacklist: bool,
    /// How many days of history to retract
    pub delete_history_msg_days: DeleteHistoryMsgDay,
}

impl MemberDeleteOptions {
    /// Creates delete options with official defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether the member should also be added to the blacklist.
    pub fn with_add_blacklist(mut self, add_blacklist: bool) -> Self {
        self.add_blacklist = add_blacklist;
        self
    }

    /// Sets the history deletion window.
    pub fn with_delete_history_msg_days(mut self, days: DeleteHistoryMsgDay) -> Self {
        self.delete_history_msg_days = days;
        self
    }
}

impl Default for MemberDeleteOptions {
    fn default() -> Self {
        Self {
            add_blacklist: false,
            delete_history_msg_days: NO_DELETE,
        }
    }
}

pub type MemberDeleteOption = Box<dyn FnOnce(&mut MemberDeleteOptions) + Send>;

#[allow(non_snake_case)]
pub fn WithAddBlackList(add_blacklist: bool) -> MemberDeleteOption {
    Box::new(move |options| {
        options.add_blacklist = add_blacklist;
    })
}

#[allow(non_snake_case)]
pub fn WithDeleteHistoryMsg(days: DeleteHistoryMsgDay) -> MemberDeleteOption {
    Box::new(move |options| {
        options.delete_history_msg_days = days;
    })
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

impl Member {
    /// Creates a new member.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the member's display name (nickname or username).
    pub fn display_name(&self) -> Option<&str> {
        (!self.nick.is_empty())
            .then_some(self.nick.as_str())
            .or_else(|| self.user.as_ref().map(|u| u.username.as_str()))
    }

    /// Gets the member's username.
    pub fn username(&self) -> Option<&str> {
        self.user.as_ref().map(|u| u.username.as_str())
    }

    /// Gets the member's user ID.
    pub fn user_id(&self) -> Option<&Snowflake> {
        self.user.as_ref().map(|u| &u.id)
    }

    /// Returns true if the member is a bot.
    pub fn is_bot(&self) -> bool {
        self.user.as_ref().is_some_and(|u| u.is_bot())
    }

    /// Gets the member's roles.
    pub fn role_ids(&self) -> &[Snowflake] {
        &self.roles
    }

    /// Returns true if the member has a specific role.
    pub fn has_role(&self, role_id: &str) -> bool {
        self.role_ids().iter().any(|id| id == role_id)
    }
}

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        self.user_id()
    }
}

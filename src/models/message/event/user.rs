use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// User information in a regular message.
/// Represents a user mentioned in a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageUser {
    /// The user's ID
    pub id: Snowflake,
    /// The user's username
    pub username: String,
    /// Whether the user is a bot
    pub bot: bool,
    /// The user's avatar hash
    #[serde(default)]
    pub avatar: String,
}
/// User information in a group message.
/// Represents a user in a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMessageUser {
    /// The user's ID
    pub id: String,
    /// The member's OpenID in the group
    pub member_openid: String,
    /// The union OpenID
    pub union_openid: String,
    /// The user's username
    pub username: String,
    /// Whether the user is a bot
    pub bot: bool,
}
/// User information in a C2C message.
/// Represents a user in a C2C message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct C2CMessageUser {
    /// The user's ID
    pub id: String,
    /// The user's union openid
    pub union_openid: String,
    /// The user's openid
    pub user_openid: String,
}
/// Member information in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageMember {
    /// The member's nickname
    #[serde(default)]
    pub nick: String,
    /// The member's roles
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// When the member joined the guild
    #[serde(default)]
    pub joined_at: Timestamp,
}

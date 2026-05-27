use crate::models::{Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// User information in a regular message.
/// Represents a user mentioned in a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageUser {
    /// The user's ID
    pub id: Option<Snowflake>,
    /// The user's username
    pub username: Option<String>,
    /// Whether the user is a bot
    pub bot: Option<bool>,
    /// The user's avatar hash
    pub avatar: Option<String>,
}
/// User information in a direct message.
/// Represents a user in a direct message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DirectMessageUser {
    /// The user's ID
    pub id: Option<Snowflake>,
    /// The user's username
    pub username: Option<String>,
    /// The user's avatar hash
    pub avatar: Option<String>,
}
/// User information in a group message.
/// Represents a user in a group message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMessageUser {
    /// The user's ID
    pub id: Option<String>,
    /// The member's OpenID in the group
    pub member_openid: Option<String>,
    /// The union OpenID
    pub union_openid: Option<String>,
}
/// User information in a C2C message.
/// Represents a user in a C2C message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct C2CMessageUser {
    /// The user's ID
    pub id: Option<String>,
    /// The user's union openid
    pub union_openid: Option<String>,
    /// The user's openid
    pub user_openid: Option<String>,
}
/// Member information in a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MessageMember {
    /// The member's nickname
    pub nick: Option<String>,
    /// The member's roles
    pub roles: Option<Vec<Snowflake>>,
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

/// Member information in a direct message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectMessageMember {
    /// When the member joined the guild
    pub joined_at: Option<Timestamp>,
}

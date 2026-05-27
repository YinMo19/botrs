use super::User;
use crate::models::{HasId, Snowflake, Timestamp};
use serde::{Deserialize, Serialize};

/// Represents a guild member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    /// The underlying user object
    #[serde(flatten)]
    pub user: User,
    /// The member's nickname in the guild
    pub nick: Option<String>,
    /// Array of role IDs
    pub roles: Vec<Snowflake>,
    /// When the user joined the guild
    pub joined_at: Timestamp,
    /// Whether the user is deafened in voice channels
    #[serde(default)]
    pub deaf: bool,
    /// Whether the user is muted in voice channels
    #[serde(default)]
    pub mute: bool,
}

impl Member {
    /// Gets the member's mention string.
    pub fn mention(&self) -> String {
        self.user.mention()
    }

    /// Gets the member's avatar URL.
    pub fn avatar_url(&self) -> Option<String> {
        self.user.avatar_url()
    }
}

impl HasId for Member {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.user.id)
    }
}

impl std::ops::Deref for Member {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

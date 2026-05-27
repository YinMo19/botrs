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
    /// Creates a new member from a user.
    pub fn new(user: User, joined_at: Timestamp) -> Self {
        Self {
            user,
            nick: None,
            roles: Vec::new(),
            joined_at,
            deaf: false,
            mute: false,
        }
    }

    /// Gets the member's display name (nickname if set, otherwise username).
    pub fn display_name(&self) -> &str {
        self.nick.as_deref().unwrap_or(&self.user.username)
    }

    /// Gets the member's mention string.
    pub fn mention(&self) -> String {
        self.user.mention()
    }

    /// Returns true if the member has the specified role.
    pub fn has_role(&self, role_id: &Snowflake) -> bool {
        self.roles.contains(role_id)
    }

    /// Returns true if the member has any of the specified roles.
    pub fn has_any_role(&self, role_ids: &[Snowflake]) -> bool {
        role_ids.iter().any(|role_id| self.has_role(role_id))
    }

    /// Returns true if the member has all of the specified roles.
    pub fn has_all_roles(&self, role_ids: &[Snowflake]) -> bool {
        role_ids.iter().all(|role_id| self.has_role(role_id))
    }

    /// Gets the member's avatar URL.
    pub fn avatar_url(&self) -> Option<String> {
        self.user.avatar_url()
    }

    /// Returns true if this member is a bot.
    pub fn is_bot(&self) -> bool {
        self.user.is_bot()
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

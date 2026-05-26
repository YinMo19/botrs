use crate::models::{HasId, HasName, Snowflake, Timestamp, channel::Channel};
use serde::{Deserialize, Serialize};

/// Represents a guild (server) in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Guild {
    /// The guild's unique ID
    #[serde(default)]
    pub id: Snowflake,
    /// The guild's name
    #[serde(default)]
    pub name: String,
    /// The guild's icon hash
    #[serde(default)]
    pub icon: String,
    /// The ID of the guild owner
    #[serde(default)]
    pub owner_id: Snowflake,
    /// Whether the current user is the owner of this guild
    #[serde(rename = "owner")]
    #[serde(default)]
    pub is_owner: bool,
    /// The number of members in this guild
    #[serde(default)]
    pub member_count: i32,
    /// The maximum number of members for this guild
    #[serde(default)]
    pub max_members: i64,
    /// The guild's description
    #[serde(default)]
    pub description: String,
    /// When the current user joined this guild
    #[serde(default)]
    pub joined_at: Timestamp,
    /// Channels contained in this guild when included by gateway payloads
    #[serde(default)]
    pub channels: Vec<Channel>,
    /// Bound game world/server ID
    #[serde(default)]
    pub union_world_id: String,
    /// Bound game organization/team ID
    #[serde(default)]
    pub union_org_id: String,
    /// Operator user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub op_user_id: Snowflake,
}

impl Guild {
    /// Creates a new guild.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new guild from API data.
    pub fn from_data(_api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        let mut guild = serde_json::from_value::<Self>(data).unwrap_or_default();
        if guild.id.is_empty() {
            guild.id = id;
        }
        guild
    }

    /// Gets the guild's icon URL if it has one.
    pub fn icon_url(&self) -> Option<String> {
        (!self.icon.is_empty()).then(|| {
            format!(
                "https://groupprofile.qq.com/groupicon/{}/{}",
                self.id, self.icon
            )
        })
    }

    /// Returns true if the current user owns this guild.
    pub fn is_owned_by_current_user(&self) -> bool {
        self.is_owner
    }

    /// Gets the guild's member count.
    pub fn get_member_count(&self) -> i32 {
        self.member_count
    }

    /// Gets the guild's maximum member count.
    pub fn get_max_members(&self) -> i64 {
        self.max_members
    }

    /// Returns true if the guild has reached its member limit.
    pub fn is_at_member_limit(&self) -> bool {
        self.max_members > 0 && i64::from(self.member_count) >= self.max_members
    }

    /// Gets the guild's display name (same as name for guilds).
    pub fn display_name(&self) -> Option<&str> {
        (!self.name.is_empty()).then_some(self.name.as_str())
    }

    /// Returns true if the guild has a description.
    pub fn has_description(&self) -> bool {
        !self.description.is_empty()
    }
}

impl HasId for Guild {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for Guild {
    fn name(&self) -> &str {
        &self.name
    }
}

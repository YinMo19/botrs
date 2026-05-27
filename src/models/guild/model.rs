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

use crate::models::Snowflake;
use crate::models::serde_helpers::is_zero_u32;
use serde::{Deserialize, Serialize};

/// Represents a role in a guild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRole {
    /// The role's unique ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: Snowflake,
    /// The role's name
    #[serde(default)]
    pub name: String,
    /// The role's color (ARGB hex as decimal)
    #[serde(default)]
    pub color: u32,
    /// Whether this role is displayed separately in the member list
    #[serde(default)]
    pub hoist: u32,
    /// The number of members with this role
    #[serde(default, rename = "number", skip_serializing_if = "is_zero_u32")]
    pub member_count: u32,
    /// The number of online members with this role
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub member_limit: u32,
}

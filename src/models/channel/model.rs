use super::{ChannelSubType, ChannelType, PrivateType, SpeakPermission};
use crate::models::Snowflake;
use crate::models::serde_helpers::is_default;
use serde::{Deserialize, Serialize};

/// Represents a channel in a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Channel {
    /// The channel's unique ID
    #[serde(default)]
    pub id: Snowflake,
    /// The guild ID this channel belongs to
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The channel's name
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The type of channel
    #[serde(default, rename = "type", skip_serializing_if = "is_default")]
    pub channel_type: ChannelType,
    /// The subtype of channel
    #[serde(default, skip_serializing_if = "is_default")]
    pub sub_type: ChannelSubType,
    /// The position of this channel in the channel list
    #[serde(default, skip_serializing_if = "is_default")]
    pub position: i64,
    /// The ID of the parent category
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent_id: Snowflake,
    /// The ID of the channel owner
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub owner_id: Snowflake,
    /// The private type of the channel
    #[serde(default, skip_serializing_if = "is_default")]
    pub private_type: PrivateType,
    /// User IDs included when creating a private channel
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub private_user_ids: Vec<String>,
    /// The speak permission setting
    #[serde(default, skip_serializing_if = "is_default")]
    pub speak_permission: SpeakPermission,
    /// The application ID for application channels
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub application_id: Snowflake,
    /// The permissions string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub permissions: String,
    /// The operator user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub op_user_id: Snowflake,
}

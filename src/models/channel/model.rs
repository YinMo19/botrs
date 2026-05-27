use super::{ChannelSubType, ChannelType, PrivateType, SpeakPermission};
use crate::models::serde_helpers::is_default;
use crate::models::{HasId, HasName, Snowflake};
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

impl Channel {
    /// Creates a new channel.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the channel's mention string.
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

    /// Returns true if this is a text channel.
    pub fn is_text(&self) -> bool {
        self.channel_type == ChannelType::Text
    }

    /// Returns true if this is a voice channel.
    pub fn is_voice(&self) -> bool {
        self.channel_type == ChannelType::Voice
    }

    /// Returns true if this is a group channel (category).
    pub fn is_group(&self) -> bool {
        self.channel_type == ChannelType::Category
    }

    /// Returns true if this is a live channel.
    pub fn is_live(&self) -> bool {
        self.channel_type == ChannelType::Live
    }

    /// Returns true if this is an application channel.
    pub fn is_application(&self) -> bool {
        self.channel_type == ChannelType::Application
    }

    /// Returns true if this is a discussion (forum) channel.
    pub fn is_discussion(&self) -> bool {
        self.channel_type == ChannelType::Forum
    }

    /// Returns true if the channel is public.
    pub fn is_public(&self) -> bool {
        self.private_type == PrivateType::Public
    }

    /// Returns true if the channel is private (admin only).
    pub fn is_admin_only(&self) -> bool {
        self.private_type == PrivateType::OnlyAdmin
    }

    /// Returns true if the channel is for specified users only.
    pub fn is_specified_users_only(&self) -> bool {
        self.private_type == PrivateType::AdminAndMember
    }

    /// Returns true if everyone can speak in this channel.
    pub fn everyone_can_speak(&self) -> bool {
        self.speak_permission == SpeakPermission::Public
    }

    /// Returns true if only admins can speak in this channel.
    pub fn admin_only_speak(&self) -> bool {
        self.speak_permission == SpeakPermission::AdminAndMember
    }

    /// Gets the channel's display name (same as name for channels).
    pub fn display_name(&self) -> Option<&str> {
        (!self.name.is_empty()).then_some(self.name.as_str())
    }
}

impl HasId for Channel {
    fn id(&self) -> Option<&Snowflake> {
        (!self.id.is_empty()).then_some(&self.id)
    }
}

impl HasName for Channel {
    fn name(&self) -> &str {
        &self.name
    }
}

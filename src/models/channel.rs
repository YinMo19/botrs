//! Channel-related data models for the QQ Guild Bot API.
//!
//! This module contains channel types that correspond to the Python botpy implementation.

use crate::models::{HasId, HasName, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents a channel in a guild.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    /// The channel's unique ID
    pub id: Option<Snowflake>,
    /// The guild ID this channel belongs to
    pub guild_id: Option<Snowflake>,
    /// The channel's name
    pub name: Option<String>,
    /// The type of channel
    #[serde(rename = "type")]
    pub channel_type: Option<ChannelType>,
    /// The subtype of channel
    pub sub_type: Option<ChannelSubType>,
    /// The position of this channel in the channel list
    pub position: Option<i32>,
    /// The ID of the parent category
    pub parent_id: Option<Snowflake>,
    /// The ID of the channel owner
    pub owner_id: Option<Snowflake>,
    /// The private type of the channel
    pub private_type: Option<PrivateType>,
    /// The speak permission setting
    pub speak_permission: Option<SpeakPermission>,
    /// The application ID for application channels
    pub application_id: Option<Snowflake>,
    /// The permissions string
    pub permissions: Option<String>,
}

impl Channel {
    /// Creates a new channel.
    pub fn new() -> Self {
        Self {
            id: None,
            guild_id: None,
            name: None,
            channel_type: None,
            sub_type: None,
            position: None,
            parent_id: None,
            owner_id: None,
            private_type: None,
            speak_permission: None,
            application_id: None,
            permissions: None,
        }
    }

    /// Creates a new channel from API data.
    pub fn from_data(_api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        Self {
            id: Some(id),
            guild_id: data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            name: data.get("name").and_then(|v| v.as_str()).map(String::from),
            channel_type: data
                .get("type")
                .and_then(|v| v.as_u64())
                .and_then(|v| ChannelType::from_u8(v as u8)),
            sub_type: data
                .get("sub_type")
                .and_then(|v| v.as_u64())
                .and_then(|v| ChannelSubType::from_u8(v as u8)),
            position: data
                .get("position")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            parent_id: data
                .get("parent_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            owner_id: data
                .get("owner_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            private_type: data
                .get("private_type")
                .and_then(|v| v.as_u64())
                .and_then(|v| PrivateType::from_u8(v as u8)),
            speak_permission: data
                .get("speak_permission")
                .and_then(|v| v.as_u64())
                .and_then(|v| SpeakPermission::from_u8(v as u8)),
            application_id: data
                .get("application_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            permissions: data
                .get("permissions")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }

    /// Gets the channel's mention string.
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id.as_ref().unwrap_or(&String::new()))
    }

    /// Returns true if this is a text channel.
    pub fn is_text(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Text))
    }

    /// Returns true if this is a voice channel.
    pub fn is_voice(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Voice))
    }

    /// Returns true if this is a group channel (category).
    pub fn is_group(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Group))
    }

    /// Returns true if this is a live channel.
    pub fn is_live(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Live))
    }

    /// Returns true if this is an application channel.
    pub fn is_application(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Application))
    }

    /// Returns true if this is a discussion (forum) channel.
    pub fn is_discussion(&self) -> bool {
        matches!(self.channel_type, Some(ChannelType::Discussion))
    }

    /// Returns true if the channel is public.
    pub fn is_public(&self) -> bool {
        matches!(self.private_type, Some(PrivateType::Public) | None)
    }

    /// Returns true if the channel is private (admin only).
    pub fn is_admin_only(&self) -> bool {
        matches!(self.private_type, Some(PrivateType::AdminOnly))
    }

    /// Returns true if the channel is for specified users only.
    pub fn is_specified_users_only(&self) -> bool {
        matches!(
            self.private_type,
            Some(PrivateType::AdminAndSpecifiedMembers)
        )
    }

    /// Returns true if everyone can speak in this channel.
    pub fn everyone_can_speak(&self) -> bool {
        matches!(self.speak_permission, Some(SpeakPermission::Everyone))
    }

    /// Returns true if only admins can speak in this channel.
    pub fn admin_only_speak(&self) -> bool {
        matches!(
            self.speak_permission,
            Some(SpeakPermission::AdminAndSpecifiedMembers)
        )
    }

    /// Gets the channel's display name (same as name for channels).
    pub fn display_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::new()
    }
}

impl HasId for Channel {
    fn id(&self) -> Option<&Snowflake> {
        self.id.as_ref()
    }
}

impl HasName for Channel {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
}

/// Channel type enumeration based on Python botpy implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum ChannelType {
    /// Text channel (0)
    Text = 0,
    /// Voice channel (2)
    Voice = 2,
    /// Group channel/Category (4)
    Group = 4,
    /// Live channel (10005)
    Live = 10005,
    /// Application channel (10006)
    Application = 10006,
    /// Discussion/Forum channel (10007)
    Discussion = 10007,
    /// Unknown channel type
    Unknown(u32),
}

impl From<u32> for ChannelType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Text,
            2 => Self::Voice,
            4 => Self::Group,
            10005 => Self::Live,
            10006 => Self::Application,
            10007 => Self::Discussion,
            other => Self::Unknown(other),
        }
    }
}

impl ChannelType {
    /// Create ChannelType from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(Self::from(value as u32))
    }
}

impl From<ChannelType> for u32 {
    fn from(channel_type: ChannelType) -> Self {
        match channel_type {
            ChannelType::Text => 0,
            ChannelType::Voice => 2,
            ChannelType::Group => 4,
            ChannelType::Live => 10005,
            ChannelType::Application => 10006,
            ChannelType::Discussion => 10007,
            ChannelType::Unknown(value) => value,
        }
    }
}

/// Channel subtype enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum ChannelSubType {
    /// Talk/Chat (0)
    Talk = 0,
    /// Post/Announcement (1)
    Post = 1,
    /// Cheat/Guide (2)
    Cheat = 2,
    /// Black/Gaming (3)
    Black = 3,
    /// Unknown subtype
    Unknown(u32),
}

impl From<u32> for ChannelSubType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Talk,
            1 => Self::Post,
            2 => Self::Cheat,
            3 => Self::Black,
            other => Self::Unknown(other),
        }
    }
}

impl ChannelSubType {
    /// Create ChannelSubType from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(Self::from(value as u32))
    }
}

impl From<ChannelSubType> for u32 {
    fn from(subtype: ChannelSubType) -> Self {
        match subtype {
            ChannelSubType::Talk => 0,
            ChannelSubType::Post => 1,
            ChannelSubType::Cheat => 2,
            ChannelSubType::Black => 3,
            ChannelSubType::Unknown(value) => value,
        }
    }
}

/// Private type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum PrivateType {
    /// Public channel (0)
    Public = 0,
    /// Admin and owner only (1)
    AdminOnly = 1,
    /// Admin and specified members (2)
    AdminAndSpecifiedMembers = 2,
    /// Unknown private type
    Unknown(u8),
}

impl From<u8> for PrivateType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Public,
            1 => Self::AdminOnly,
            2 => Self::AdminAndSpecifiedMembers,
            other => Self::Unknown(other),
        }
    }
}

impl From<PrivateType> for u8 {
    fn from(private_type: PrivateType) -> Self {
        match private_type {
            PrivateType::Public => 0,
            PrivateType::AdminOnly => 1,
            PrivateType::AdminAndSpecifiedMembers => 2,
            PrivateType::Unknown(other) => other,
        }
    }
}

impl PrivateType {
    /// Create PrivateType from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(Self::from(value))
    }
}

impl From<PrivateType> for u32 {
    fn from(private_type: PrivateType) -> Self {
        match private_type {
            PrivateType::Public => 0,
            PrivateType::AdminOnly => 1,
            PrivateType::AdminAndSpecifiedMembers => 2,
            PrivateType::Unknown(value) => value as u32,
        }
    }
}

/// Speak permission enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum SpeakPermission {
    /// Invalid (0)
    Invalid = 0,
    /// Everyone can speak (1)
    Everyone = 1,
    /// Only admin and specified members (2)
    AdminAndSpecifiedMembers = 2,
    /// Unknown speak permission
    Unknown(u8),
}

impl From<u8> for SpeakPermission {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Invalid,
            1 => Self::Everyone,
            2 => Self::AdminAndSpecifiedMembers,
            other => Self::Unknown(other),
        }
    }
}

impl From<SpeakPermission> for u8 {
    fn from(speak_permission: SpeakPermission) -> Self {
        match speak_permission {
            SpeakPermission::Invalid => 0,
            SpeakPermission::Everyone => 1,
            SpeakPermission::AdminAndSpecifiedMembers => 2,
            SpeakPermission::Unknown(other) => other,
        }
    }
}

impl SpeakPermission {
    /// Create SpeakPermission from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(Self::from(value))
    }
}

impl From<SpeakPermission> for u32 {
    fn from(speak_permission: SpeakPermission) -> Self {
        match speak_permission {
            SpeakPermission::Invalid => 0,
            SpeakPermission::Everyone => 1,
            SpeakPermission::AdminAndSpecifiedMembers => 2,
            SpeakPermission::Unknown(value) => value as u32,
        }
    }
}

/// Channel permissions for a user or role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelPermissions {
    /// The channel ID
    pub channel_id: Option<Snowflake>,
    /// The user ID (if this is for a user)
    pub user_id: Option<Snowflake>,
    /// The role ID (if this is for a role)
    pub role_id: Option<Snowflake>,
    /// The permissions string
    pub permissions: Option<String>,
}

impl ChannelPermissions {
    /// Creates new channel permissions.
    pub fn new() -> Self {
        Self {
            channel_id: None,
            user_id: None,
            role_id: None,
            permissions: None,
        }
    }

    /// Returns true if this is for a user.
    pub fn is_user_permission(&self) -> bool {
        self.user_id.is_some()
    }

    /// Returns true if this is for a role.
    pub fn is_role_permission(&self) -> bool {
        self.role_id.is_some()
    }
}

impl Default for ChannelPermissions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let channel = Channel::new();
        assert!(channel.id.is_none());
        assert!(channel.name.is_none());
        assert!(channel.is_public()); // Default should be public
    }

    #[test]
    fn test_channel_types() {
        let mut channel = Channel::new();

        channel.channel_type = Some(ChannelType::Text);
        assert!(channel.is_text());
        assert!(!channel.is_voice());

        channel.channel_type = Some(ChannelType::Voice);
        assert!(channel.is_voice());
        assert!(!channel.is_text());

        channel.channel_type = Some(ChannelType::Group);
        assert!(channel.is_group());
    }

    #[test]
    fn test_channel_type_conversion() {
        assert_eq!(ChannelType::from(0), ChannelType::Text);
        assert_eq!(u32::from(ChannelType::Text), 0);

        assert_eq!(ChannelType::from(10005), ChannelType::Live);
        assert_eq!(u32::from(ChannelType::Live), 10005);

        assert_eq!(ChannelType::from(99999), ChannelType::Unknown(99999));
        assert_eq!(u32::from(ChannelType::Unknown(99999)), 99999);
    }

    #[test]
    fn test_private_types() {
        let mut channel = Channel::new();

        channel.private_type = Some(PrivateType::Public);
        assert!(channel.is_public());
        assert!(!channel.is_admin_only());

        channel.private_type = Some(PrivateType::AdminOnly);
        assert!(!channel.is_public());
        assert!(channel.is_admin_only());

        channel.private_type = Some(PrivateType::AdminAndSpecifiedMembers);
        assert!(channel.is_specified_users_only());
    }

    #[test]
    fn test_speak_permissions() {
        let mut channel = Channel::new();

        channel.speak_permission = Some(SpeakPermission::Everyone);
        assert!(channel.everyone_can_speak());
        assert!(!channel.admin_only_speak());

        channel.speak_permission = Some(SpeakPermission::AdminAndSpecifiedMembers);
        assert!(!channel.everyone_can_speak());
        assert!(channel.admin_only_speak());
    }

    #[test]
    fn test_channel_mention() {
        let mut channel = Channel::new();
        channel.id = Some("123456789".to_string());
        assert_eq!(channel.mention(), "<#123456789>");
    }

    #[test]
    fn test_channel_permissions() {
        let mut perms = ChannelPermissions::new();
        assert!(!perms.is_user_permission());
        assert!(!perms.is_role_permission());

        perms.user_id = Some("user123".to_string());
        assert!(perms.is_user_permission());
        assert!(!perms.is_role_permission());

        perms.user_id = None;
        perms.role_id = Some("role123".to_string());
        assert!(!perms.is_user_permission());
        assert!(perms.is_role_permission());
    }
}

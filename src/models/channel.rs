//! Channel-related data models for the QQ Guild Bot API.
//!
//! This module contains channel types for the QQ Bot Open API.

use crate::models::{HasId, HasName, Snowflake};
use serde::{Deserialize, Serialize};

fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq,
{
    value == &T::default()
}

fn option_is_none_or_default<T>(value: &Option<T>) -> bool
where
    T: Default + PartialEq,
{
    value.as_ref().is_none_or(is_default)
}

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

    /// Creates a new channel from API data.
    pub fn from_data(_api: crate::api::BotApi, id: String, data: serde_json::Value) -> Self {
        let mut channel = serde_json::from_value::<Self>(data).unwrap_or_default();
        channel.id = id;
        channel
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

/// Channel value object used when creating or modifying a channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelValueObject {
    /// Channel name
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub name: Option<String>,
    /// Channel type
    #[serde(rename = "type", skip_serializing_if = "option_is_none_or_default")]
    pub channel_type: Option<ChannelType>,
    /// Sort position
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub position: Option<i64>,
    /// Parent channel ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub parent_id: Option<Snowflake>,
    /// Owner ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub owner_id: Option<Snowflake>,
    /// Channel subtype
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub sub_type: Option<ChannelSubType>,
    /// Channel visibility type
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub private_type: Option<PrivateType>,
    /// Private channel member IDs
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub private_user_ids: Option<Vec<String>>,
    /// Speak permission
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub speak_permission: Option<SpeakPermission>,
    /// Application ID for application channels
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub application_id: Option<Snowflake>,
    /// Channel permissions
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub permissions: Option<String>,
    /// Operator user ID
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub op_user_id: Option<Snowflake>,
}

impl ChannelValueObject {
    /// Creates a channel value object with the required create-channel fields.
    pub fn new(
        name: impl Into<String>,
        channel_type: ChannelType,
        sub_type: ChannelSubType,
    ) -> Self {
        Self {
            name: Some(name.into()),
            channel_type: Some(channel_type),
            sub_type: Some(sub_type),
            ..Default::default()
        }
    }
}

/// Channel type enumeration for the QQ Bot Open API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum ChannelType {
    /// Text channel (0)
    #[default]
    Text = 0,
    /// Voice channel (2)
    Voice = 2,
    /// Group channel/Category (4)
    Category = 4,
    /// Live channel (10005)
    Live = 10005,
    /// Application channel (10006)
    Application = 10006,
    /// Discussion/Forum channel (10007)
    Forum = 10007,
    /// Unknown channel type
    Unknown(u32),
}

pub const CHANNEL_TYPE_TEXT: ChannelType = ChannelType::Text;
pub const CHANNEL_TYPE_VOICE: ChannelType = ChannelType::Voice;
pub const CHANNEL_TYPE_CATEGORY: ChannelType = ChannelType::Category;
pub const CHANNEL_TYPE_LIVE: ChannelType = ChannelType::Live;
pub const CHANNEL_TYPE_APPLICATION: ChannelType = ChannelType::Application;
pub const CHANNEL_TYPE_FORUM: ChannelType = ChannelType::Forum;
#[allow(non_upper_case_globals)]
pub const ChannelTypeText: ChannelType = CHANNEL_TYPE_TEXT;
#[allow(non_upper_case_globals)]
pub const ChannelTypeVoice: ChannelType = CHANNEL_TYPE_VOICE;
#[allow(non_upper_case_globals)]
pub const ChannelTypeCategory: ChannelType = CHANNEL_TYPE_CATEGORY;
#[allow(non_upper_case_globals)]
pub const ChannelTypeLive: ChannelType = CHANNEL_TYPE_LIVE;
#[allow(non_upper_case_globals)]
pub const ChannelTypeApplication: ChannelType = CHANNEL_TYPE_APPLICATION;
#[allow(non_upper_case_globals)]
pub const ChannelTypeForum: ChannelType = CHANNEL_TYPE_FORUM;

impl From<u32> for ChannelType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Text,
            2 => Self::Voice,
            4 => Self::Category,
            10005 => Self::Live,
            10006 => Self::Application,
            10007 => Self::Forum,
            other => Self::Unknown(other),
        }
    }
}

impl ChannelType {
    /// Create ChannelType from u8 value
    pub fn from_u8(value: u8) -> Option<Self> {
        Some(Self::from(value as u32))
    }

    /// Create ChannelType from the raw integer value.
    pub fn from_u32(value: u32) -> Self {
        Self::from(value)
    }
}

impl From<ChannelType> for u32 {
    fn from(channel_type: ChannelType) -> Self {
        match channel_type {
            ChannelType::Text => 0,
            ChannelType::Voice => 2,
            ChannelType::Category => 4,
            ChannelType::Live => 10005,
            ChannelType::Application => 10006,
            ChannelType::Forum => 10007,
            ChannelType::Unknown(value) => value,
        }
    }
}

/// Channel subtype enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u32", into = "u32")]
#[repr(u32)]
pub enum ChannelSubType {
    /// Chat channel (0)
    #[default]
    Chat = 0,
    /// Notice channel (1)
    Notice = 1,
    /// Guide channel (2)
    Guide = 2,
    /// Team game channel (3)
    TeamGame = 3,
    /// Unknown subtype
    Unknown(u32),
}

pub const CHANNEL_SUB_TYPE_CHAT: ChannelSubType = ChannelSubType::Chat;
pub const CHANNEL_SUB_TYPE_NOTICE: ChannelSubType = ChannelSubType::Notice;
pub const CHANNEL_SUB_TYPE_GUIDE: ChannelSubType = ChannelSubType::Guide;
pub const CHANNEL_SUB_TYPE_TEAM_GAME: ChannelSubType = ChannelSubType::TeamGame;
#[allow(non_upper_case_globals)]
pub const ChannelSubTypeChat: ChannelSubType = CHANNEL_SUB_TYPE_CHAT;
#[allow(non_upper_case_globals)]
pub const ChannelSubTypeNotice: ChannelSubType = CHANNEL_SUB_TYPE_NOTICE;
#[allow(non_upper_case_globals)]
pub const ChannelSubTypeGuide: ChannelSubType = CHANNEL_SUB_TYPE_GUIDE;
#[allow(non_upper_case_globals)]
pub const ChannelSubTypeTeamGame: ChannelSubType = CHANNEL_SUB_TYPE_TEAM_GAME;

impl From<u32> for ChannelSubType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Chat,
            1 => Self::Notice,
            2 => Self::Guide,
            3 => Self::TeamGame,
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
            ChannelSubType::Chat => 0,
            ChannelSubType::Notice => 1,
            ChannelSubType::Guide => 2,
            ChannelSubType::TeamGame => 3,
            ChannelSubType::Unknown(value) => value,
        }
    }
}

/// Private type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum PrivateType {
    /// Public channel (0)
    #[default]
    Public = 0,
    /// Admin and owner only (1)
    OnlyAdmin = 1,
    /// Admin and specified members (2)
    AdminAndMember = 2,
    /// Unknown private type
    Unknown(u8),
}

pub type ChannelPrivateType = PrivateType;
pub const CHANNEL_PRIVATE_TYPE_PUBLIC: ChannelPrivateType = PrivateType::Public;
pub const CHANNEL_PRIVATE_TYPE_ONLY_ADMIN: ChannelPrivateType = PrivateType::OnlyAdmin;
pub const CHANNEL_PRIVATE_TYPE_ADMIN_AND_MEMBER: ChannelPrivateType = PrivateType::AdminAndMember;
#[allow(non_upper_case_globals)]
pub const ChannelPrivateTypePublic: ChannelPrivateType = CHANNEL_PRIVATE_TYPE_PUBLIC;
#[allow(non_upper_case_globals)]
pub const ChannelPrivateTypeOnlyAdmin: ChannelPrivateType = CHANNEL_PRIVATE_TYPE_ONLY_ADMIN;
#[allow(non_upper_case_globals)]
pub const ChannelPrivateTypeAdminAndMember: ChannelPrivateType =
    CHANNEL_PRIVATE_TYPE_ADMIN_AND_MEMBER;

impl From<u8> for PrivateType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Public,
            1 => Self::OnlyAdmin,
            2 => Self::AdminAndMember,
            other => Self::Unknown(other),
        }
    }
}

impl From<PrivateType> for u8 {
    fn from(private_type: PrivateType) -> Self {
        match private_type {
            PrivateType::Public => 0,
            PrivateType::OnlyAdmin => 1,
            PrivateType::AdminAndMember => 2,
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
            PrivateType::OnlyAdmin => 1,
            PrivateType::AdminAndMember => 2,
            PrivateType::Unknown(value) => value as u32,
        }
    }
}

/// Speak permission enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(from = "u8", into = "u8")]
#[repr(u8)]
pub enum SpeakPermission {
    /// Invalid (0)
    #[default]
    Invalid = 0,
    /// Public speak permission (1)
    Public = 1,
    /// Only admin and specified members (2)
    AdminAndMember = 2,
    /// Unknown speak permission
    Unknown(u8),
}

pub type SpeakPermissionType = SpeakPermission;
pub const SPEAK_PERMISSION_TYPE_PUBLIC: SpeakPermissionType = SpeakPermission::Public;
pub const SPEAK_PERMISSION_TYPE_ADMIN_AND_MEMBER: SpeakPermissionType =
    SpeakPermission::AdminAndMember;
#[allow(non_upper_case_globals)]
pub const SpeakPermissionTypePublic: SpeakPermissionType = SPEAK_PERMISSION_TYPE_PUBLIC;
#[allow(non_upper_case_globals)]
pub const SpeakPermissionTypeAdminAndMember: SpeakPermissionType =
    SPEAK_PERMISSION_TYPE_ADMIN_AND_MEMBER;

impl From<u8> for SpeakPermission {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Invalid,
            1 => Self::Public,
            2 => Self::AdminAndMember,
            other => Self::Unknown(other),
        }
    }
}

impl From<SpeakPermission> for u8 {
    fn from(speak_permission: SpeakPermission) -> Self {
        match speak_permission {
            SpeakPermission::Invalid => 0,
            SpeakPermission::Public => 1,
            SpeakPermission::AdminAndMember => 2,
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
            SpeakPermission::Public => 1,
            SpeakPermission::AdminAndMember => 2,
            SpeakPermission::Unknown(value) => value as u32,
        }
    }
}

/// Channel permissions for a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelPermissions {
    /// The channel ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel_id: Snowflake,
    /// The user ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_id: Snowflake,
    /// The permissions string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub permissions: String,
}

impl ChannelPermissions {
    /// Creates new channel permissions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true if this is for a user.
    pub fn is_user_permission(&self) -> bool {
        !self.user_id.is_empty()
    }
}

/// Channel role permissions response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChannelRolesPermissions {
    /// The channel ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub channel_id: Snowflake,
    /// The role ID
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub role_id: Snowflake,
    /// The permissions string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub permissions: String,
}

impl ChannelRolesPermissions {
    /// Returns true if this is for a role.
    pub fn is_role_permission(&self) -> bool {
        !self.role_id.is_empty()
    }
}

/// Body for updating channel user or role permissions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateChannelPermissions {
    /// Permissions to add
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub add: Option<String>,
    /// Permissions to remove
    #[serde(skip_serializing_if = "option_is_none_or_default")]
    pub remove: Option<String>,
}

impl UpdateChannelPermissions {
    /// Creates update-channel-permissions parameters.
    pub fn new(add: Option<impl ToString>, remove: Option<impl ToString>) -> Self {
        Self {
            add: add.map(|value| value.to_string()),
            remove: remove.map(|value| value.to_string()),
        }
    }

    /// Validates that the permission strings can be parsed as unsigned integers.
    pub fn validate(&self) -> crate::error::Result<()> {
        if let Some(add) = self.add.as_deref() {
            add.parse::<u64>().map_err(|err| {
                crate::error::BotError::invalid_data(format!("invalid parameter add: {err}"))
            })?;
        }
        if let Some(remove) = self.remove.as_deref() {
            remove.parse::<u64>().map_err(|err| {
                crate::error::BotError::invalid_data(format!("invalid parameter remove: {err}"))
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let channel = Channel::new();
        assert!(channel.id.is_empty());
        assert!(channel.name.is_empty());
        assert!(channel.is_public()); // Default should be public
    }

    #[test]
    fn test_channel_types() {
        let mut channel = Channel::new();

        channel.channel_type = ChannelType::Text;
        assert!(channel.is_text());
        assert!(!channel.is_voice());

        channel.channel_type = ChannelType::Voice;
        assert!(channel.is_voice());
        assert!(!channel.is_text());

        channel.channel_type = ChannelType::Category;
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

        channel.private_type = PrivateType::Public;
        assert!(channel.is_public());
        assert!(!channel.is_admin_only());

        channel.private_type = PrivateType::OnlyAdmin;
        assert!(!channel.is_public());
        assert!(channel.is_admin_only());

        channel.private_type = PrivateType::AdminAndMember;
        assert!(channel.is_specified_users_only());
    }

    #[test]
    fn test_speak_permissions() {
        let mut channel = Channel::new();

        channel.speak_permission = SpeakPermission::Public;
        assert!(channel.everyone_can_speak());
        assert!(!channel.admin_only_speak());

        channel.speak_permission = SpeakPermission::AdminAndMember;
        assert!(!channel.everyone_can_speak());
        assert!(channel.admin_only_speak());
    }

    #[test]
    fn test_channel_mention() {
        let mut channel = Channel::new();
        channel.id = "123456789".to_string();
        assert_eq!(channel.mention(), "<#123456789>");
    }

    #[test]
    fn test_channel_permissions() {
        let mut perms = ChannelPermissions::new();
        assert!(!perms.is_user_permission());

        perms.user_id = "user123".to_string();
        assert!(perms.is_user_permission());

        let role_perms = ChannelRolesPermissions {
            role_id: "role123".to_string(),
            ..Default::default()
        };
        assert!(role_perms.is_role_permission());
    }

    #[test]
    fn channel_uses_zero_values_for_missing_fields() {
        let channel: Channel = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(channel.id, "");
        assert_eq!(channel.guild_id, "");
        assert_eq!(channel.name, "");
        assert_eq!(channel.channel_type, ChannelType::Text);
        assert_eq!(channel.sub_type, ChannelSubType::Chat);
        assert_eq!(channel.private_type, PrivateType::Public);
        assert_eq!(channel.speak_permission, SpeakPermission::Invalid);
        assert!(channel.private_user_ids.is_empty());

        let value = serde_json::to_value(&channel).unwrap();
        assert_eq!(value["id"], serde_json::json!(""));
        assert_eq!(value["guild_id"], serde_json::json!(""));
        assert!(value.get("name").is_none());
        assert!(value.get("type").is_none());
        assert!(value.get("sub_type").is_none());
        assert!(value.get("position").is_none());
        assert!(value.get("private_user_ids").is_none());
    }

    #[test]
    fn channel_decodes_large_type_values() {
        let channel: Channel = serde_json::from_value(serde_json::json!({
            "id": "channel-1",
            "guild_id": "guild-1",
            "name": "live",
            "type": 10005,
            "sub_type": 3,
            "private_type": 2,
            "speak_permission": 1,
            "private_user_ids": ["user-1"],
            "permissions": "2048"
        }))
        .unwrap();

        assert_eq!(channel.id, "channel-1");
        assert_eq!(channel.guild_id, "guild-1");
        assert_eq!(channel.channel_type, ChannelType::Live);
        assert_eq!(channel.sub_type, ChannelSubType::TeamGame);
        assert_eq!(channel.private_type, PrivateType::AdminAndMember);
        assert_eq!(channel.speak_permission, SpeakPermission::Public);
        assert_eq!(channel.private_user_ids, ["user-1"]);
        assert_eq!(channel.permissions, "2048");
    }

    #[test]
    fn channel_permissions_are_separate_dtos() {
        let user_permissions: ChannelPermissions = serde_json::from_value(serde_json::json!({
            "channel_id": "channel-1",
            "user_id": "user-1",
            "permissions": "1024"
        }))
        .unwrap();
        let role_permissions: ChannelRolesPermissions = serde_json::from_value(serde_json::json!({
            "channel_id": "channel-1",
            "role_id": "role-1",
            "permissions": "2048"
        }))
        .unwrap();

        assert_eq!(user_permissions.user_id, "user-1");
        assert_eq!(user_permissions.permissions, "1024");
        assert_eq!(role_permissions.role_id, "role-1");
        assert_eq!(role_permissions.permissions, "2048");
    }

    #[test]
    fn channel_value_object_omits_go_zero_values() {
        let value = ChannelValueObject {
            name: Some(String::new()),
            channel_type: Some(ChannelType::Text),
            position: Some(0),
            parent_id: Some(String::new()),
            owner_id: Some(String::new()),
            sub_type: Some(ChannelSubType::Chat),
            private_type: Some(PrivateType::Public),
            private_user_ids: Some(Vec::new()),
            speak_permission: Some(SpeakPermission::Invalid),
            application_id: Some(String::new()),
            permissions: Some(String::new()),
            op_user_id: Some(String::new()),
        };

        assert_eq!(serde_json::to_value(&value).unwrap(), serde_json::json!({}));
    }

    #[test]
    fn channel_value_object_keeps_non_zero_values() {
        let value = ChannelValueObject {
            name: Some("name".to_string()),
            channel_type: Some(ChannelType::Voice),
            position: Some(1),
            parent_id: Some("parent".to_string()),
            owner_id: Some("owner".to_string()),
            sub_type: Some(ChannelSubType::Notice),
            private_type: Some(PrivateType::AdminAndMember),
            private_user_ids: Some(vec!["user".to_string()]),
            speak_permission: Some(SpeakPermission::Public),
            application_id: Some("app".to_string()),
            permissions: Some("1".to_string()),
            op_user_id: Some("op".to_string()),
        };

        assert_eq!(
            serde_json::to_value(&value).unwrap(),
            serde_json::json!({
                "name": "name",
                "type": 2,
                "position": 1,
                "parent_id": "parent",
                "owner_id": "owner",
                "sub_type": 1,
                "private_type": 2,
                "private_user_ids": ["user"],
                "speak_permission": 1,
                "application_id": "app",
                "permissions": "1",
                "op_user_id": "op"
            })
        );
    }

    #[test]
    fn channel_permissions_omit_empty_fields() {
        let user_permissions = ChannelPermissions::default();
        let role_permissions = ChannelRolesPermissions::default();
        let update_permissions = UpdateChannelPermissions {
            add: Some(String::new()),
            remove: Some(String::new()),
        };

        assert_eq!(
            serde_json::to_value(&user_permissions).unwrap(),
            serde_json::json!({})
        );
        assert_eq!(
            serde_json::to_value(&role_permissions).unwrap(),
            serde_json::json!({})
        );
        assert_eq!(
            serde_json::to_value(&update_permissions).unwrap(),
            serde_json::json!({})
        );
    }
}

use serde::{Deserialize, Serialize};

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

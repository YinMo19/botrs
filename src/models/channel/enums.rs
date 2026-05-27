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

wire_enum!(ChannelType, u32, Unknown, {
    Text = 0,
    Voice = 2,
    Category = 4,
    Live = 10005,
    Application = 10006,
    Forum = 10007,
});

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

wire_enum!(ChannelSubType, u32, Unknown, {
    Chat = 0,
    Notice = 1,
    Guide = 2,
    TeamGame = 3,
});

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

wire_enum!(PrivateType, u8, Unknown, {
    Public = 0,
    OnlyAdmin = 1,
    AdminAndMember = 2,
});

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

wire_enum!(SpeakPermission, u8, Unknown, {
    Invalid = 0,
    Public = 1,
    AdminAndMember = 2,
});

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

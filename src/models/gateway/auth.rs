use crate::intents::Intents;
use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Hello payload from the gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// Heartbeat interval in milliseconds
    pub heartbeat_interval: u64,
}

/// Identify payload for gateway authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identify {
    /// Bot token
    pub token: String,
    /// Intent flags
    pub intents: u32,
    /// Shard information
    pub shard: Option<[u32; 2]>,
    /// Properties
    pub properties: IdentifyProperties,
}

/// Properties for identify payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifyProperties {
    /// Operating system
    #[serde(rename = "$os", default, skip_serializing_if = "String::is_empty")]
    pub os: String,
    /// Browser/library name
    #[serde(rename = "$browser", default, skip_serializing_if = "String::is_empty")]
    pub browser: String,
    /// Device name
    #[serde(rename = "$device", default, skip_serializing_if = "String::is_empty")]
    pub device: String,
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            os: String::new(),
            browser: String::new(),
            device: String::new(),
        }
    }
}

/// Identify payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSIdentityData {
    pub token: String,
    pub intents: u32,
    pub shard: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<IdentifyProperties>,
}

impl From<Identify> for WSIdentityData {
    fn from(identify: Identify) -> Self {
        Self {
            token: identify.token,
            intents: identify.intents,
            shard: identify.shard.map(Vec::from).unwrap_or_default(),
            properties: Some(identify.properties),
        }
    }
}

impl From<WSIdentityData> for Identify {
    fn from(data: WSIdentityData) -> Self {
        Self {
            token: data.token,
            intents: data.intents,
            shard: (data.shard.len() == 2).then(|| [data.shard[0], data.shard[1]]),
            properties: data.properties.unwrap_or_default(),
        }
    }
}

impl WSIdentityData {
    pub fn new(token: impl Into<String>, intents: Intents, shard: Option<[u32; 2]>) -> Self {
        Self {
            token: token.into(),
            intents: intents.bits,
            shard: shard.map(Vec::from).unwrap_or_default(),
            properties: Some(IdentifyProperties::default()),
        }
    }
}

/// Resume payload for gateway reconnection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resume {
    /// Bot token
    pub token: String,
    /// Session ID
    pub session_id: String,
    /// Last sequence number
    pub seq: u64,
}

/// Resume payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSResumeData {
    pub token: String,
    pub session_id: String,
    pub seq: u32,
}

impl From<Resume> for WSResumeData {
    fn from(resume: Resume) -> Self {
        Self {
            token: resume.token,
            session_id: resume.session_id,
            seq: resume.seq as u32,
        }
    }
}

impl From<WSResumeData> for Resume {
    fn from(data: WSResumeData) -> Self {
        Self {
            token: data.token,
            session_id: data.session_id,
            seq: u64::from(data.seq),
        }
    }
}

/// Hello payload alias.
pub type WSHelloData = Hello;

/// Ready user object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSUser {
    pub id: Snowflake,
    pub username: String,
    #[serde(default)]
    pub bot: bool,
}

/// Ready event data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ready {
    /// Gateway version
    pub version: u32,
    /// Session ID
    pub session_id: String,
    /// Bot information
    pub user: crate::models::robot::Robot,
    /// Shard information
    pub shard: Option<[u32; 2]>,
}

/// Ready payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSReadyData {
    pub version: u32,
    pub session_id: String,
    pub user: WSUser,
    pub shard: Vec<u32>,
}

impl From<Ready> for WSReadyData {
    fn from(ready: Ready) -> Self {
        Self {
            version: ready.version,
            session_id: ready.session_id,
            user: WSUser {
                id: ready.user.id,
                username: ready.user.username,
                bot: ready.user.bot,
            },
            shard: ready.shard.map(Vec::from).unwrap_or_default(),
        }
    }
}

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
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

/// Ready event data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ready {
    /// Gateway version
    pub version: u32,
    /// Session ID
    pub session_id: String,
    /// Bot information
    pub user: crate::models::User,
    /// Shard information
    pub shard: Option<[u32; 2]>,
}

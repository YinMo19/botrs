//! Gateway event models for the QQ Guild Bot API.

// use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Gateway event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayEvent {
    /// The event type
    #[serde(rename = "t")]
    pub event_type: Option<String>,
    /// The event data
    #[serde(rename = "d")]
    pub data: Option<serde_json::Value>,
    /// The sequence number
    #[serde(rename = "s")]
    pub sequence: Option<u64>,
    /// The opcode
    #[serde(rename = "op")]
    pub opcode: u8,
}

/// Gateway opcode constants.
pub mod opcodes {
    /// Dispatch event
    pub const DISPATCH: u8 = 0;
    /// Heartbeat
    pub const HEARTBEAT: u8 = 1;
    /// Identify
    pub const IDENTIFY: u8 = 2;
    /// Resume
    pub const RESUME: u8 = 6;
    /// Reconnect
    pub const RECONNECT: u8 = 7;
    /// Invalid session
    pub const INVALID_SESSION: u8 = 9;
    /// Hello
    pub const HELLO: u8 = 10;
    /// Heartbeat ACK
    pub const HEARTBEAT_ACK: u8 = 11;
}

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
    #[serde(rename = "$os")]
    pub os: String,
    /// Browser/library name
    #[serde(rename = "$browser")]
    pub browser: String,
    /// Device name
    #[serde(rename = "$device")]
    pub device: String,
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            browser: "botrs".to_string(),
            device: "botrs".to_string(),
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

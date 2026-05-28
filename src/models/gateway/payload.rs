use serde::{Deserialize, Serialize};

/// Gateway event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayEvent {
    /// The gateway event ID (used as passive event context ID)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The event type
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The event data
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// The sequence number
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
    /// The opcode
    #[serde(rename = "op")]
    pub opcode: u8,
}

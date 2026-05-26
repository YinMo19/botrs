use super::{EventType, OpCode};
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

/// Websocket payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSPayload {
    #[serde(flatten)]
    pub base: WSPayloadBase,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip)]
    pub raw_message: Option<Vec<u8>>,
    #[serde(skip)]
    pub session: Option<crate::session_manager::Session>,
}

impl PartialEq for WSPayload {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.data == other.data && self.raw_message == other.raw_message
    }
}

/// Websocket payload base.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WSPayloadBase {
    #[serde(rename = "op")]
    pub op_code: OpCode,
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u32>,
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

impl From<GatewayEvent> for WSPayload {
    fn from(event: GatewayEvent) -> Self {
        Self {
            base: WSPayloadBase {
                op_code: event.opcode,
                seq: event.sequence.map(|seq| seq as u32),
                event_type: event.event_type,
                event_id: event.id,
            },
            data: event.data,
            raw_message: None,
            session: None,
        }
    }
}

impl From<WSPayload> for GatewayEvent {
    fn from(payload: WSPayload) -> Self {
        Self {
            id: payload.base.event_id,
            event_type: payload.base.event_type,
            data: payload.data,
            sequence: payload.base.seq.map(u64::from),
            opcode: payload.base.op_code,
        }
    }
}

//! HTTP webhook session models.

use crate::models::api::BotInfo;
use serde::{Deserialize, Serialize};

/// Identity payload for creating an HTTP webhook session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpIdentity {
    /// Gateway intents bitmask
    pub intents: u64,
    /// Shard tuple: `[shard_id, num_shards]`
    pub shards: [u32; 2],
    /// Callback URL
    pub callback_url: String,
}

impl HttpIdentity {
    /// Creates a new HTTP identity payload.
    pub fn new(intents: u64, shards: [u32; 2], callback_url: impl Into<String>) -> Self {
        Self {
            intents,
            shards,
            callback_url: callback_url.into(),
        }
    }
}

pub type HTTPIdentity = HttpIdentity;

/// Ready response returned after creating an HTTP webhook session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpReady {
    /// Gateway version
    pub version: i32,
    /// Session ID
    pub session_id: String,
    /// Bot information
    pub bot: BotInfo,
    /// Shard tuple
    pub shard: [u32; 2],
}

pub type HTTPReady = HttpReady;

/// HTTP webhook session object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpSession {
    /// App ID
    pub app_id: i64,
    /// Session ID
    pub session_id: String,
    /// Callback URL
    pub callback_url: String,
    /// Environment
    pub env: String,
    /// Gateway intents bitmask
    pub intents: i64,
    /// Last heartbeat time
    pub last_heartbeat_time: String,
    /// Session state
    pub state: String,
    /// Shard tuple
    pub shards: [i64; 2],
}

pub type HTTPSession = HttpSession;

/// Webhook callback validation request data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WebhookValidationRequest {
    /// Plain token from the validation request
    pub plain_token: String,
    /// Event timestamp
    pub event_ts: String,
}

pub type WHValidationReq = WebhookValidationRequest;

/// Webhook callback validation response data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WebhookValidationResponse {
    /// Plain token from the validation request
    pub plain_token: String,
    /// Signature generated with the app secret
    pub signature: String,
    /// Data format version
    pub data_version: String,
}

pub type WHValidationRsp = WebhookValidationResponse;

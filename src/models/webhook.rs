//! HTTP webhook session models.

use serde::{Deserialize, Serialize};

/// Identity payload for creating an HTTP webhook session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpIdentity {
    /// Gateway intents bitmask
    pub intents: u32,
    /// Shard tuple: `[shard_id, num_shards]`
    pub shards: [u32; 2],
    /// Callback URL
    pub callback_url: String,
}

impl HttpIdentity {
    /// Creates a new HTTP identity payload.
    pub fn new(intents: u32, shards: [u32; 2], callback_url: impl Into<String>) -> Self {
        Self {
            intents,
            shards,
            callback_url: callback_url.into(),
        }
    }
}

/// Bot information embedded in an HTTP webhook ready response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpBot {
    /// Bot ID
    pub id: String,
    /// Bot username
    pub username: String,
}

/// Ready response returned after creating an HTTP webhook session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpReady {
    /// Gateway version
    pub version: i32,
    /// Session ID
    pub session_id: String,
    /// Bot information
    pub bot: HttpBot,
    /// Shard tuple
    pub shard: [u32; 2],
}

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

/// Webhook callback validation request data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WebhookValidationRequest {
    /// Plain token from the validation request
    pub plain_token: String,
    /// Event timestamp
    pub event_ts: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_identity_keeps_official_json_shape() {
        let identity = HttpIdentity::new(1 << 25, [0, 2], "https://example.com/callback");
        let value = serde_json::to_value(&identity).unwrap();

        assert_eq!(value["intents"], 1 << 25);
        assert_eq!(value["shards"], serde_json::json!([0, 2]));
        assert_eq!(value["callback_url"], "https://example.com/callback");
    }

    #[test]
    fn http_ready_bot_uses_embedded_shape() {
        let ready: HttpReady = serde_json::from_value(serde_json::json!({
            "version": 1,
            "session_id": "session-1",
            "bot": {
                "id": "bot-1",
                "username": "bot"
            },
            "shard": [0, 1]
        }))
        .unwrap();

        assert_eq!(ready.version, 1);
        assert_eq!(ready.session_id, "session-1");
        assert_eq!(ready.bot.id, "bot-1");
        assert_eq!(ready.bot.username, "bot");
        assert_eq!(ready.shard, [0, 1]);

        let value = serde_json::to_value(&ready).unwrap();
        assert_eq!(value["bot"]["id"], "bot-1");
        assert_eq!(value["bot"]["username"], "bot");
        assert!(value["bot"].get("avatar").is_none());
        assert!(value["bot"].get("bot").is_none());
    }
}

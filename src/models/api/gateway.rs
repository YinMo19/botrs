use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Gateway URL response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GatewayResponse {
    /// The WebSocket gateway URL
    pub url: String,
    /// The number of shards to use
    pub shards: u32,
    /// Session start limit information
    pub session_start_limit: SessionStartLimit,
}

pub type WebsocketAP = GatewayResponse;

/// Session start limit information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts allowed
    pub total: u32,
    /// Number of session starts remaining
    pub remaining: u32,
    /// Time after which the limit resets (in milliseconds)
    pub reset_after: u32,
    /// Maximum number of concurrent sessions
    pub max_concurrency: u32,
}

/// Shard configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShardConfig {
    pub shard_id: u32,
    pub shard_count: u32,
}

/// Bot information response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotInfo {
    /// The bot's ID
    #[serde(default)]
    pub id: Snowflake,
    /// The bot's username
    #[serde(default)]
    pub username: String,
    /// The bot's avatar hash
    #[serde(default)]
    pub avatar: String,
    /// Whether this is a bot account
    #[serde(default)]
    pub bot: bool,
    /// The bot's union openid.
    #[serde(default)]
    pub union_openid: String,
    /// The bot's union user account.
    #[serde(default)]
    pub union_user_account: String,
    /// Robot share URL returned by the current-user endpoint.
    #[serde(default)]
    pub share_url: String,
    /// Robot welcome message returned by the current-user endpoint.
    #[serde(default)]
    pub welcome_msg: String,
}

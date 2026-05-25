//! API response models for the QQ Guild Bot API.

use crate::models::Snowflake;
use serde::{Deserialize, Serialize};

/// Standard API response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The response data
    #[serde(flatten)]
    pub data: T,
    /// Error code if the request failed
    pub code: Option<u32>,
    /// Error message if the request failed
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful response.
    pub fn success(data: T) -> Self {
        Self {
            data,
            code: None,
            message: None,
        }
    }

    /// Creates an error response.
    pub fn error(code: u32, message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            data: T::default(),
            code: Some(code),
            message: Some(message.into()),
        }
    }

    /// Returns true if the response indicates success.
    pub fn is_success(&self) -> bool {
        self.code.is_none()
    }

    /// Returns true if the response indicates an error.
    pub fn is_error(&self) -> bool {
        self.code.is_some()
    }

    /// Converts this response into a Result.
    pub fn into_result(self) -> crate::Result<T> {
        if let Some(code) = self.code {
            let message = self.message.unwrap_or_else(|| format!("API error {code}"));
            Err(crate::BotError::api(code, message))
        } else {
            Ok(self.data)
        }
    }
}

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

/// Botgo-compatible shard configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ShardConfig {
    pub shard_id: u32,
    pub shard_count: u32,
}

/// Bot information response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotInfo {
    /// The bot's ID
    pub id: Snowflake,
    /// The bot's username
    pub username: String,
    /// The bot's avatar hash
    pub avatar: Option<String>,
    /// Whether this is a bot account
    #[serde(default)]
    pub bot: bool,
}

/// Pagination information for list responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    /// The current page number
    pub page: u32,
    /// The number of items per page
    pub per_page: u32,
    /// The total number of items
    pub total: u32,
    /// The total number of pages
    pub total_pages: u32,
    /// Whether there is a next page
    pub has_next: bool,
    /// Whether there is a previous page
    pub has_prev: bool,
}

impl Pagination {
    /// Creates a new pagination info.
    pub fn new(page: u32, per_page: u32, total: u32) -> Self {
        let total_pages = total.div_ceil(per_page); // Ceiling division
        Self {
            page,
            per_page,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

/// Paginated list response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The list of items
    pub items: Vec<T>,
    /// Pagination information
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response.
    pub fn new(items: Vec<T>, pagination: Pagination) -> Self {
        Self { items, pagination }
    }

    /// Returns true if there are more pages.
    pub fn has_more(&self) -> bool {
        self.pagination.has_next
    }

    /// Gets the next page number if available.
    pub fn next_page(&self) -> Option<u32> {
        if self.pagination.has_next {
            Some(self.pagination.page + 1)
        } else {
            None
        }
    }

    /// Gets the previous page number if available.
    pub fn prev_page(&self) -> Option<u32> {
        if self.pagination.has_prev {
            Some(self.pagination.page - 1)
        } else {
            None
        }
    }
}

/// Rate limit information from API headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimit {
    /// The rate limit bucket
    pub bucket: Option<String>,
    /// The number of requests allowed per window
    pub limit: u32,
    /// The number of requests remaining in the current window
    pub remaining: u32,
    /// The time when the rate limit resets (Unix timestamp)
    pub reset: u64,
    /// The time after which to retry (in seconds)
    pub retry_after: Option<u64>,
}

impl RateLimit {
    /// Returns true if the rate limit has been exceeded.
    pub fn is_exceeded(&self) -> bool {
        self.remaining == 0
    }

    /// Returns the time until the rate limit resets (in seconds).
    pub fn reset_in(&self) -> u64 {
        let now = chrono::Utc::now().timestamp() as u64;
        self.reset.saturating_sub(now)
    }
}

/// Error response from the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: u32,
    /// Error message
    pub message: String,
    /// Additional error details
    pub errors: Option<serde_json::Value>,
    /// Request trace ID for debugging
    pub trace_id: Option<String>,
}

impl ApiError {
    /// Creates a new API error.
    pub fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            errors: None,
            trace_id: None,
        }
    }

    /// Checks if this is a rate limit error.
    pub fn is_rate_limit(&self) -> bool {
        self.code == 429
    }

    /// Checks if this is an authentication error.
    pub fn is_auth_error(&self) -> bool {
        self.code == 401 || self.code == 403
    }

    /// Checks if this is a not found error.
    pub fn is_not_found(&self) -> bool {
        self.code == 404
    }

    /// Checks if this is a server error.
    pub fn is_server_error(&self) -> bool {
        self.code >= 500
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// Audio action data structure for audio events
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioAction {
    /// Guild ID where the audio event occurred
    #[serde(default)]
    pub guild_id: String,
    /// Channel ID where the audio event occurred
    #[serde(default)]
    pub channel_id: String,
    /// URL of the audio file
    #[serde(default)]
    pub audio_url: String,
    /// Text description of the audio
    #[serde(default)]
    pub text: String,
}

impl AudioAction {
    pub(crate) fn from_value(value: &serde_json::Value) -> Self {
        Self {
            guild_id: string_field(value, "guild_id"),
            channel_id: string_field(value, "channel_id"),
            audio_url: string_field(value, "audio_url"),
            text: string_field(value, "text"),
        }
    }
}

fn string_field(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_string()
}

/// Response from message sending operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageResponse {
    /// The ID of the sent message
    pub id: Option<Snowflake>,
    /// The timestamp when the message was sent
    pub timestamp: Option<String>,
    /// Additional response data
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Pinned messages response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PinsMessage {
    /// Guild ID
    #[serde(default)]
    pub guild_id: Snowflake,
    /// Channel ID
    #[serde(default)]
    pub channel_id: Snowflake,
    /// Pinned message IDs
    #[serde(default)]
    pub message_ids: Vec<Snowflake>,
}

impl MessageResponse {
    /// Creates a new message response
    pub fn new(id: impl Into<Snowflake>) -> Self {
        Self {
            id: Some(id.into()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            extra: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response() {
        let success: ApiResponse<String> = ApiResponse::success("test".to_string());
        assert!(success.is_success());
        assert!(!success.is_error());
        assert!(success.into_result().is_ok());

        let error: ApiResponse<String> = ApiResponse::error(404, "Not found");
        assert!(!error.is_success());
        assert!(error.is_error());
        assert!(error.into_result().is_err());
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(2, 10, 25);
        assert_eq!(pagination.total_pages, 3);
        assert!(pagination.has_prev);
        assert!(pagination.has_next);

        let last_page = Pagination::new(3, 10, 25);
        assert!(!last_page.has_next);
        assert!(last_page.has_prev);
    }

    #[test]
    fn test_rate_limit() {
        let rate_limit = RateLimit {
            bucket: Some("global".to_string()),
            limit: 100,
            remaining: 0,
            reset: chrono::Utc::now().timestamp() as u64 + 60,
            retry_after: Some(60),
        };

        assert!(rate_limit.is_exceeded());
        assert!(rate_limit.reset_in() > 0);
    }

    #[test]
    fn test_api_error() {
        let error = ApiError::new(429, "Rate limited");
        assert!(error.is_rate_limit());
        assert!(!error.is_auth_error());
        assert!(!error.is_not_found());
        assert!(!error.is_server_error());

        let auth_error = ApiError::new(401, "Unauthorized");
        assert!(auth_error.is_auth_error());
    }

    #[test]
    fn botgo_websocket_ap_keeps_official_json_shape() {
        let ap: WebsocketAP = serde_json::from_value(serde_json::json!({
            "url": "wss://api.sgroup.qq.com/websocket",
            "shards": 2,
            "session_start_limit": {
                "total": 10,
                "remaining": 9,
                "reset_after": 1000,
                "max_concurrency": 1
            }
        }))
        .unwrap();

        assert_eq!(ap.url, "wss://api.sgroup.qq.com/websocket");
        assert_eq!(ap.shards, 2);
        assert_eq!(ap.session_start_limit.total, 10);
        assert_eq!(ap.session_start_limit.remaining, 9);
        assert_eq!(ap.session_start_limit.reset_after, 1000);
        assert_eq!(ap.session_start_limit.max_concurrency, 1);

        let value = serde_json::to_value(&ap).unwrap();
        assert_eq!(value["session_start_limit"]["reset_after"], 1000);
    }

    #[test]
    fn botgo_audio_action_uses_required_zero_value_fields() {
        let action: AudioAction = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(action.guild_id, "");
        assert_eq!(action.channel_id, "");
        assert_eq!(action.audio_url, "");
        assert_eq!(action.text, "");
    }

    #[test]
    fn botgo_audio_action_keeps_official_json_shape() {
        let action = AudioAction {
            guild_id: "guild-1".to_string(),
            channel_id: "channel-1".to_string(),
            audio_url: "https://example.com/audio.mp3".to_string(),
            text: "now playing".to_string(),
        };
        let value = serde_json::to_value(&action).unwrap();

        assert_eq!(value["guild_id"], "guild-1");
        assert_eq!(value["channel_id"], "channel-1");
        assert_eq!(value["audio_url"], "https://example.com/audio.mp3");
        assert_eq!(value["text"], "now playing");
    }

    #[test]
    fn botgo_audio_action_from_value_tolerates_missing_fields() {
        let action = AudioAction::from_value(&serde_json::json!({
            "guild_id": "guild-1",
            "channel_id": 123,
        }));

        assert_eq!(action.guild_id, "guild-1");
        assert_eq!(action.channel_id, "");
        assert_eq!(action.audio_url, "");
        assert_eq!(action.text, "");
    }

    #[test]
    fn botgo_pins_message_uses_required_zero_value_fields() {
        let pins: PinsMessage = serde_json::from_value(serde_json::json!({})).unwrap();

        assert!(pins.guild_id.is_empty());
        assert!(pins.channel_id.is_empty());
        assert!(pins.message_ids.is_empty());
    }

    #[test]
    fn botgo_pins_message_keeps_official_json_shape() {
        let pins = PinsMessage {
            guild_id: "guild-1".to_string(),
            channel_id: "channel-1".to_string(),
            message_ids: vec!["message-1".to_string(), "message-2".to_string()],
        };
        let value = serde_json::to_value(&pins).unwrap();

        assert_eq!(value["guild_id"], "guild-1");
        assert_eq!(value["channel_id"], "channel-1");
        assert_eq!(value["message_ids"][0], "message-1");
    }
}

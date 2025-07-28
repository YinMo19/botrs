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
            let message = self
                .message
                .unwrap_or_else(|| format!("API error {code}"));
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

/// Session start limit information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts allowed
    pub total: u32,
    /// Number of session starts remaining
    pub remaining: u32,
    /// Time after which the limit resets (in milliseconds)
    pub reset_after: u64,
    /// Maximum number of concurrent sessions
    pub max_concurrency: u32,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioAction {
    /// Guild ID where the audio event occurred
    pub guild_id: Option<String>,
    /// Channel ID where the audio event occurred
    pub channel_id: Option<String>,
    /// URL of the audio file
    pub audio_url: Option<String>,
    /// Text description of the audio
    pub text: Option<String>,
}

/// Response from message sending operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub struct MessageResponse {
    /// The ID of the sent message
    pub id: Option<Snowflake>,
    /// The timestamp when the message was sent
    pub timestamp: Option<String>,
    /// Additional response data
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
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
}

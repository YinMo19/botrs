use serde::{Deserialize, Serialize};

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

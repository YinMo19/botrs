use super::{HttpFailure, SdkError};

/// The main error type for BotRS operations.
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    /// SDK errors.
    #[error("{0}")]
    Sdk(#[from] SdkError),

    /// HTTP client errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// WebSocket connection errors
    #[error("WebSocket error: {0}")]
    WebSocket(Box<tokio_tungstenite::tungstenite::Error>),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing errors
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// API response errors
    #[error("API error: {code} - {message}")]
    Api {
        code: u32,
        message: String,
        response: Option<Box<HttpFailure>>,
    },

    /// Authentication failed (401)
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(Box<HttpFailure>),

    /// Not found (404)
    #[error("Not found: {0}")]
    NotFound(Box<HttpFailure>),

    /// Method not allowed (405)
    #[error("Method not allowed: {0}")]
    MethodNotAllowed(Box<HttpFailure>),

    /// Forbidden (403)
    #[error("Forbidden: {0}")]
    Forbidden(Box<HttpFailure>),

    /// Sequence number error (429)
    #[error("Sequence number error: {0}")]
    SequenceNumber(String),

    /// Server error (500, 504)
    #[error("Server error: {0}")]
    Server(Box<HttpFailure>),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Connection errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Rate limiting errors
    #[error("Rate limited: retry after {retry_after} seconds")]
    RateLimit {
        retry_after: u64,
        response: Option<Box<HttpFailure>>,
    },

    /// Invalid configuration errors
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Invalid data format errors
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Network timeout errors
    #[error("Network timeout")]
    Timeout,

    /// Gateway errors
    #[error("Gateway error: {0}")]
    Gateway(String),

    /// Session errors
    #[error("Session error: {0}")]
    Session(String),

    /// Generic errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl BotError {
    /// Creates a new API error.
    pub fn api(code: u32, message: impl Into<String>) -> Self {
        Self::Api {
            code,
            message: message.into(),
            response: None,
        }
    }

    /// Creates a new authentication error.
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }

    /// Creates a new connection error.
    pub fn connection(message: impl Into<String>) -> Self {
        Self::Connection(message.into())
    }

    /// Creates a new configuration error.
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// Creates a new invalid data error.
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::InvalidData(message.into())
    }

    /// Creates a new gateway error.
    pub fn gateway(message: impl Into<String>) -> Self {
        Self::Gateway(message.into())
    }

    /// Creates a new session error.
    pub fn session(message: impl Into<String>) -> Self {
        Self::Session(message.into())
    }

    /// Creates a new internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Creates a new rate limit error.
    pub fn rate_limit(retry_after: u64) -> Self {
        Self::RateLimit {
            retry_after,
            response: None,
        }
    }

    /// Returns the original HTTP failure for OpenAPI response errors.
    pub fn response(&self) -> Option<&HttpFailure> {
        match self {
            Self::Api { response, .. } | Self::RateLimit { response, .. } => response.as_deref(),
            Self::Forbidden(err)
            | Self::NotFound(err)
            | Self::AuthenticationFailed(err)
            | Self::Server(err)
            | Self::MethodNotAllowed(err) => Some(err),
            _ => None,
        }
    }

    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            BotError::Http(e) => e.is_timeout() || e.is_connect(),
            BotError::WebSocket(_) => true,
            BotError::Connection(_) => true,
            BotError::Timeout => true,
            BotError::Gateway(_) => true,
            BotError::RateLimit { .. } => true,
            _ => false,
        }
    }

    /// Returns the retry delay in seconds if this error is retryable.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            BotError::RateLimit { retry_after, .. } => Some(*retry_after),
            BotError::Connection(_) => Some(5),
            BotError::Gateway(_) => Some(1),
            BotError::Timeout => Some(3),
            _ if self.is_retryable() => Some(1),
            _ => None,
        }
    }
}

// Manual From implementation for boxing WebSocket error
impl From<tokio_tungstenite::tungstenite::Error> for BotError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        BotError::WebSocket(Box::new(err))
    }
}

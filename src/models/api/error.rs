use serde::{Deserialize, Deserializer, Serialize};

/// Error response from the API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiError {
    /// Error code
    pub code: u32,
    /// Current OpenAPI error code (`err_code`); `code` is kept for backwards compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_code: Option<u32>,
    /// Error message
    pub message: String,
    /// Additional error details
    pub errors: Option<serde_json::Value>,
    /// Request trace ID for debugging
    pub trace_id: Option<String>,
}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ApiErrorWire {
            code: Option<u32>,
            err_code: Option<u32>,
            message: String,
            errors: Option<serde_json::Value>,
            trace_id: Option<String>,
        }

        let wire = ApiErrorWire::deserialize(deserializer)?;
        let code = wire
            .err_code
            .or(wire.code)
            .ok_or_else(|| serde::de::Error::custom("missing field `code` or `err_code`"))?;

        Ok(Self {
            code,
            err_code: wire.err_code,
            message: wire.message,
            errors: wire.errors,
            trace_id: wire.trace_id,
        })
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

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

use super::BotError;
use std::fmt;

/// Diagnostic information from one failed QQ OpenAPI response.
#[derive(Debug, Clone)]
pub struct HttpFailure {
    pub status: reqwest::StatusCode,
    pub platform_code: Option<u32>,
    pub message: String,
    pub trace_id: Option<String>,
    /// Original response text, including additional platform data such as an audit ID.
    pub body: String,
}

impl fmt::Display for HttpFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.status)?;
        if let Some(code) = self.platform_code {
            write!(f, "; API error {code}")?;
        }
        write!(f, ": {}", self.message)?;
        if let Some(trace) = &self.trace_id {
            write!(f, "; trace_id={trace}")?;
        }
        Ok(())
    }
}

impl HttpFailure {
    pub(crate) fn into_error(self, retry_after: Option<u64>) -> BotError {
        let status = self.status.as_u16();
        let failure = Box::new(self);
        match status {
            401 => BotError::AuthenticationFailed(failure),
            403 => BotError::Forbidden(failure),
            404 => BotError::NotFound(failure),
            405 => BotError::MethodNotAllowed(failure),
            429 => BotError::RateLimit {
                retry_after: retry_after.unwrap_or(60),
                response: Some(failure),
            },
            500..=599 => BotError::Server(failure),
            _ => BotError::Api {
                code: failure.platform_code.unwrap_or(u32::from(status)),
                message: failure.message.clone(),
                response: Some(failure),
            },
        }
    }
}

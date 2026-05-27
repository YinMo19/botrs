use std::fmt;

use super::{BotError, CODE_CONN_CLOSE_CANT_IDENTIFY, CODE_CONN_CLOSE_CANT_RESUME};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkError {
    code: i32,
    message: String,
    trace_id: String,
}

impl SdkError {
    pub fn new(code: i32, message: impl Into<String>, trace_id: Option<impl Into<String>>) -> Self {
        Self {
            code,
            message: message.into(),
            trace_id: trace_id.map(Into::into).unwrap_or_default(),
        }
    }

    pub const fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }
}

impl std::error::Error for SdkError {}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code:{}, text:{}, traceID:{}",
            self.code, self.message, self.trace_id
        )
    }
}

pub fn sdk_error(code: i32, message: impl Into<String>) -> SdkError {
    SdkError::new(code, message, None::<String>)
}

pub fn sdk_error_from_error(err: &(dyn std::error::Error + 'static)) -> SdkError {
    if let Some(err) = err.downcast_ref::<SdkError>() {
        return err.clone();
    }
    if let Some(BotError::Sdk(err)) = err.downcast_ref::<BotError>() {
        return err.clone();
    }
    SdkError::new(9999, err.to_string(), None::<String>)
}

pub fn invalid_session_error() -> SdkError {
    SdkError::new(
        CODE_CONN_CLOSE_CANT_RESUME,
        "invalid session",
        None::<String>,
    )
}

pub fn session_limit_error() -> SdkError {
    SdkError::new(
        CODE_CONN_CLOSE_CANT_IDENTIFY,
        "session num limit",
        None::<String>,
    )
}

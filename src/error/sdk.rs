use std::fmt;

use super::{
    BotError, CodeConnCloseCantIdentify, CodeConnCloseCantResume, CodeNeedReConnect,
    CodeNotFoundOpenAPI, CodePagerIsNil,
};

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

pub fn sdk_error_with_trace(
    code: i32,
    message: impl Into<String>,
    trace_id: impl Into<String>,
) -> SdkError {
    SdkError::new(code, message, Some(trace_id))
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

pub fn need_reconnect_error() -> SdkError {
    SdkError::new(CodeNeedReConnect, "need reconnect", None::<String>)
}

pub fn invalid_session_error() -> SdkError {
    SdkError::new(CodeConnCloseCantResume, "invalid session", None::<String>)
}

pub fn invalid_url_error() -> SdkError {
    SdkError::new(
        CodeConnCloseCantIdentify,
        "ws ap url is invalid",
        None::<String>,
    )
}

pub fn session_limit_error() -> SdkError {
    SdkError::new(
        CodeConnCloseCantIdentify,
        "session num limit",
        None::<String>,
    )
}

pub fn openapi_not_found_error() -> SdkError {
    SdkError::new(
        CodeNotFoundOpenAPI,
        "not found openapi version",
        None::<String>,
    )
}

pub fn missing_pager_error() -> SdkError {
    SdkError::new(CodePagerIsNil, "pager is nil", None::<String>)
}

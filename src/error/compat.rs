use std::fmt;
use std::sync::LazyLock;

use super::{
    BotError, CodeConnCloseCantIdentify, CodeConnCloseCantResume, CodeNeedReConnect,
    CodeNotFoundOpenAPI, CodePagerIsNil,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Err {
    code: i32,
    text: String,
    trace: String,
}

impl Err {
    pub fn new(code: i32, text: impl Into<String>, trace: Option<impl Into<String>>) -> Self {
        Self {
            code,
            text: text.into(),
            trace: trace.map(Into::into).unwrap_or_default(),
        }
    }

    pub const fn code(&self) -> i32 {
        self.code
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn trace(&self) -> &str {
        &self.trace
    }

    #[allow(non_snake_case)]
    pub const fn Code(&self) -> i32 {
        self.code()
    }

    #[allow(non_snake_case)]
    pub fn Text(&self) -> &str {
        self.text()
    }

    #[allow(non_snake_case)]
    pub fn Trace(&self) -> &str {
        self.trace()
    }
}

#[allow(non_snake_case)]
pub fn Code(err: &(dyn std::error::Error + 'static)) -> i32 {
    Error(err).Code()
}

impl std::error::Error for Err {}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code:{}, text:{}, traceID:{}",
            self.code, self.text, self.trace
        )
    }
}

#[allow(non_snake_case)]
pub fn New(code: i32, text: impl Into<String>) -> Err {
    Err::new(code, text, None::<String>)
}

#[allow(non_snake_case)]
pub fn Error(err: &(dyn std::error::Error + 'static)) -> Err {
    if let Some(err) = err.downcast_ref::<Err>() {
        return err.clone();
    }
    if let Some(BotError::Sdk(err)) = err.downcast_ref::<BotError>() {
        return err.clone();
    }
    Err::new(9999, err.to_string(), None::<String>)
}

pub fn err_need_reconnect() -> Err {
    Err::new(CodeNeedReConnect, "need reconnect", None::<String>)
}

pub fn err_invalid_session() -> Err {
    Err::new(CodeConnCloseCantResume, "invalid session", None::<String>)
}

pub fn err_url_invalid() -> Err {
    Err::new(
        CodeConnCloseCantIdentify,
        "ws ap url is invalid",
        None::<String>,
    )
}

pub fn err_session_limit() -> Err {
    Err::new(
        CodeConnCloseCantIdentify,
        "session num limit",
        None::<String>,
    )
}

pub fn err_not_found_openapi() -> Err {
    Err::new(
        CodeNotFoundOpenAPI,
        "not found openapi version",
        None::<String>,
    )
}

pub fn err_pager_is_nil() -> Err {
    Err::new(CodePagerIsNil, "pager is nil", None::<String>)
}

pub static ErrNeedReConnect: LazyLock<Err> = LazyLock::new(err_need_reconnect);
pub static ErrInvalidSession: LazyLock<Err> = LazyLock::new(err_invalid_session);
pub static ErrURLInvalid: LazyLock<Err> = LazyLock::new(err_url_invalid);
pub static ErrSessionLimit: LazyLock<Err> = LazyLock::new(err_session_limit);
pub static ErrNotFoundOpenAPI: LazyLock<Err> = LazyLock::new(err_not_found_openapi);
pub static ErrPagerIsNil: LazyLock<Err> = LazyLock::new(err_pager_is_nil);

//! Error types for the BotRS library.
//!
//! This module defines all the error types that can occur when using the BotRS framework.

#![allow(non_upper_case_globals)]

mod bot_error;
mod codes;
mod compat;
mod http;

/// A specialized Result type for BotRS operations.
pub type Result<T> = std::result::Result<T, BotError>;

pub use bot_error::{BotError, IntoBotError};
pub use codes::*;
pub use compat::{
    Code, Err, ErrInvalidSession, ErrNeedReConnect, ErrNotFoundOpenAPI, ErrPagerIsNil,
    ErrSessionLimit, ErrURLInvalid, Error, New, err_invalid_session, err_need_reconnect,
    err_not_found_openapi, err_pager_is_nil, err_session_limit, err_url_invalid,
};
pub use http::http_error_from_status;

#[cfg(test)]
mod tests;

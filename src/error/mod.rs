//! Error types for the BotRS library.
//!
//! This module defines all the error types that can occur when using the BotRS framework.

#![allow(non_upper_case_globals)]

mod bot_error;
mod codes;
mod http;
mod sdk;

/// A specialized Result type for BotRS operations.
pub type Result<T> = std::result::Result<T, BotError>;

pub use bot_error::BotError;
pub use codes::*;
pub use http::HttpFailure;
pub use sdk::SdkError;
pub(crate) use sdk::{invalid_session_error, sdk_error, session_limit_error};

#[cfg(test)]
mod tests;

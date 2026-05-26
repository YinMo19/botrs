//! Authentication token management for QQ Guild Bot API.
//!
//! This module provides the `Token` struct for managing bot authentication
//! credentials including app ID and secret, with access token management.

#![allow(non_upper_case_globals)]

mod credentials;
mod display;
mod refresh;
mod token;

pub use credentials::{
    NewQQBotTokenSource, QQBotCredentials, QQBotTokenSource, TypeBearer, TypeQQBot,
};
pub use refresh::StartRefreshAccessToken;
pub use token::Token;

#[cfg(test)]
use refresh::get_refresh_millis;
use refresh::parse_expires_in;

#[cfg(test)]
mod tests;

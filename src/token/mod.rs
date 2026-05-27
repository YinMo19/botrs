//! Authentication token management for QQ Guild Bot API.
//!
//! This module provides the `Token` struct for managing bot authentication
//! credentials including app ID and secret, with access token management.

mod core;
mod display;
mod refresh;

pub use core::Token;
pub use refresh::start_access_token_refresh;

#[cfg(test)]
use refresh::get_refresh_millis;
use refresh::parse_expires_in;

#[cfg(test)]
mod tests;

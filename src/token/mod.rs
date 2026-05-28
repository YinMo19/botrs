//! Authentication token management for QQ Guild Bot API.
//!
//! This module provides the `Token` struct for managing bot authentication
//! credentials including app ID and secret, with access token management.

mod core;
mod display;

pub use core::Token;

pub(super) fn parse_expires_in(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
}

#[cfg(test)]
mod tests;

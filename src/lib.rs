// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

#[path = "api/mod.rs"]
mod api_impl;
pub mod audio;
pub mod client;
mod constant;
pub mod error;
mod event;
pub mod forum;
pub mod gateway;
pub mod http;
pub mod intents;
pub mod interaction;
pub mod manage;
pub mod models;
pub mod reaction;
pub mod session_manager;
pub mod signature;
#[path = "token/mod.rs"]
mod token_impl;
pub mod webhook;

// Re-export main types for convenience
pub use api_impl::BotApi;
pub use client::{Client, Context, EventHandler};
pub use error::{BotError, Result};
pub use intents::Intents;
pub use models::gateway::Ready;
pub use models::*;
pub use signature::{HEADER_SIGNATURE, HEADER_TIMESTAMP, generate, verify};
pub use token_impl::{Token, start_access_token_refresh};
pub use webhook::{dispatch_ack, handle_http_callback, heartbeat_ack, validation_ack};

/// The current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default timeout for HTTP requests in seconds
pub const DEFAULT_TIMEOUT: u64 = 30;

/// Default WebSocket URL for QQ Guild API
pub const DEFAULT_WS_URL: &str = "wss://api.sgroup.qq.com/websocket";

/// Default API base URL for QQ Guild API
pub const DEFAULT_API_URL: &str = "https://api.sgroup.qq.com";

/// Sandbox API base URL for testing
pub const SANDBOX_API_URL: &str = "https://sandbox.api.sgroup.qq.com";

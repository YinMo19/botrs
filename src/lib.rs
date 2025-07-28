// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]

//! # BotRS - Rust QQ Bot Framework
//!
//! BotRS is a Rust implementation of the QQ Guild Bot API framework, inspired by the Python botpy library.
//! It provides an easy-to-use, efficient, and type-safe way to create QQ Guild bots.
//!
//! ## Features
//!
//! - Async/await support with Tokio
//! - WebSocket connection management
//! - HTTP API client
//! - Event-driven architecture
//! - Type-safe message handling
//! - Intent-based event filtering
//!
//! ## Quick Start
//!
/// ```rust,no_run
/// use botrs::{Client, Token, Intents, EventHandler};
///
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl EventHandler for MyHandler {}
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let token = Token::new("your_app_id", "your_secret");
///     let intents = Intents::default();
///     let handler = MyHandler;
///     let mut client = Client::new(token, intents, handler, false)?;
///     client.start().await?;
///     Ok(())
/// }
/// ```
pub mod api;
pub mod audio;
pub mod client;
pub mod connection;
pub mod error;
pub mod forum;
pub mod gateway;
pub mod http;
pub mod intents;
pub mod interaction;
pub mod manage;
pub mod models;
pub mod reaction;
pub mod token;

// Re-export main types for convenience
pub use api::BotApi;
pub use audio::{Audio, AudioControl, AudioStatus, PublicAudio, PublicAudioType};
pub use client::{Client, Context, EventHandler};
pub use connection::{ConnectionSession, ConnectionState, Session};
pub use error::{BotError, Result};
pub use forum::{Content, Format, OpenThread, Thread, ThreadInfo, Title};
pub use intents::Intents;
pub use interaction::{Interaction, InteractionData, InteractionDataType, InteractionType};
pub use manage::{C2CManageEvent, GroupManageEvent, ManageEventType};
pub use models::gateway::Ready;
pub use models::*;
pub use reaction::{Reaction, ReactionTarget, ReactionTargetType, ReactionUsers};
pub use token::Token;

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

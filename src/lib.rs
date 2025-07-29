// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

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

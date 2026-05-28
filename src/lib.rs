// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

#[path = "api/mod.rs"]
mod api_impl;
mod audio;
pub mod client;
mod constant;
pub mod error;
pub mod forum;
mod gateway;
pub mod http;
pub mod intents;
pub mod interaction;
pub mod manage;
pub mod models;
mod reaction;
mod session_manager;
#[path = "token/mod.rs"]
mod token_impl;

// Re-export main types for convenience
pub use api_impl::BotApi;
pub use audio::{Audio, PublicAudio, PublicAudioType};
pub use client::{Client, Context, EventHandler};
pub use error::{BotError, Result};
pub use intents::Intents;
pub use models::api::{BotInfo, MessageResponse};
pub use models::channel::Channel;
pub use models::gateway::{GatewayEvent, Ready};
pub use models::guild::{Guild, GuildRole, Member};
pub use models::message::{
    C2CMessage, DirectMessage, DirectMessageToCreate, GroupMessage, Message, MessageDelete,
    MessageParams,
};
pub use models::robot::Robot;
pub use models::schedule::Schedule;
pub use models::user::User;
pub use reaction::{
    Emoji as ReactionEmoji, MessageReaction, MessageReactionPager, Reaction, ReactionTarget,
    ReactionTargetType, ReactionUsers,
};
pub use token_impl::Token;

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

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
mod forum;
mod gateway;
pub mod http;
pub mod intents;
mod interaction;
mod manage;
pub mod models;
mod reaction;
mod session_manager;
#[path = "token/mod.rs"]
mod token_impl;

// Public runtime and API entry points. Data models live under `botrs::models`.
pub use api_impl::BotApi;
pub use api_impl::session::{
    AudioSession, C2CManageSession, C2CReplySession, ChannelReplySession, ChannelSession,
    DirectReplySession, EnterAioSession, EventSession, ForumAuditSession, ForumReplySession,
    GroupManageSession, GroupReplySession, GuildSession, InteractionSession, MemberSession,
    MessageAuditSession, MessageDeleteSession, OpenForumSession, PostSession, PublicAudioSession,
    ReactionSession, ReadySession, ResumeSession, SubscribeMessageStatusSession, ThreadSession,
    UnknownEventSession,
};
pub use client::{Client, EventHandler};
pub use error::{BotError, Result};
pub use intents::Intents;
pub use token_impl::Token;

/// The current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default timeout for HTTP requests in seconds
pub const DEFAULT_TIMEOUT: u64 = 30;

/// Default API base URL for QQ Guild API
pub const DEFAULT_API_URL: &str = "https://api.sgroup.qq.com";

/// Sandbox API base URL for testing
pub const SANDBOX_API_URL: &str = "https://sandbox.api.sgroup.qq.com";

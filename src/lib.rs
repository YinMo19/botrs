// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

pub mod api;
pub mod audio;
pub mod client;
pub mod constant;
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
pub mod token;
pub mod webhook;

// Re-export main types for convenience
pub use api::BotApi;
pub use audio::{Audio, AudioControl, AudioStatus, PublicAudio, PublicAudioType};
pub use client::{Client, Context, EventHandler};
pub use constant::*;
pub use error::{BotError, Result};
pub use forum::{
    Content, Format, ForumAuditResult, ForumRsp, OpenThread, Post, PostInfo, PostThreadRsp, Reply,
    ReplyInfo, Thread, ThreadInfo, ThreadToCreate, Title,
};
pub use intents::Intents;
pub use interaction::{
    Interaction, InteractionData, InteractionDataType, InteractionType, SearchInputResolved,
    SearchLayout, SearchRecord, SearchRsp,
};
pub use manage::{
    C2CManageEvent, EnterAioEvent, GroupManageEvent, ManageEventType, SubscribeMessageStatusData,
    SubscribeMsgTemplateResult,
};
pub use models::gateway::Ready;
pub use models::*;
pub use reaction::{
    Emoji as ReactionEmoji, MessageReaction, MessageReactionPager, Reaction, ReactionTarget,
    ReactionTargetType, ReactionUsers,
};
pub use session_manager::{
    CANNOT_IDENTIFY_ERROR_CODES, CANNOT_RESUME_ERROR_CODES, ChanManager, Session, SessionManager,
    can_not_identify, can_not_resume, check_session_limit, new_session_manager,
    set_session_manager_factory,
};
pub use signature::{HEADER_SIGNATURE, HEADER_TIMESTAMP, generate, verify};
pub use token::{Token, start_access_token_refresh};
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

//! Main client implementation for the QQ Guild Bot API.
//!
//! This module provides the main `Client` struct that serves as the entry point
//! for bot applications, handling connections, events, and API interactions.

mod context;
mod debug;
mod dispatch;
mod handler;
mod lifecycle;

pub(crate) use context::Context;
pub use handler::EventHandler;

#[allow(unused_imports)]
mod prelude {
    pub(super) use crate::Reaction;
    pub(super) use crate::api_impl::BotApi;
    pub(super) use crate::api_impl::session::{
        AudioSession, C2CManageSession, C2CReplySession, ChannelReplySession, ChannelSession,
        DirectReplySession, EnterAioSession, EventSession, ForumAuditSession, ForumReplySession,
        GroupManageSession, GroupReplySession, GuildSession, InteractionSession, MemberSession,
        MessageAuditSession, MessageDeleteSession, OpenForumSession, PostSession,
        PublicAudioSession, ReactionSession, ReadySession, ResumeSession,
        SubscribeMessageStatusSession, ThreadSession, UnknownEventSession,
    };
    pub(super) use crate::audio::{Audio, PublicAudio};
    pub(super) use crate::error::{BotError, Result};
    pub(super) use crate::forum::{ForumAuditResult, OpenThread, Post, Reply, Thread};
    pub(super) use crate::intents::Intents;
    pub(super) use crate::interaction::Interaction;
    pub(super) use crate::manage::{
        C2CManageEvent, EnterAioEvent, GroupManageEvent, SubscribeMessageStatusData,
    };
    pub(super) use crate::models::api::AudioAction;
    pub(super) use crate::models::gateway::GatewayEvent;
    pub(super) use crate::models::*;
    pub(super) use crate::session_manager::{check_session_limit, new_session_manager};
    pub(super) use crate::token_impl::Token;
    pub(super) use std::sync::Arc;
    pub(super) use tokio::sync::mpsc;
    pub(super) use tracing::{debug, error, info};
}

use prelude::{Arc, BotApi, Intents};

/// Main client for the QQ Guild Bot API.
pub struct Client<H: EventHandler> {
    /// Intent flags
    intents: Intents,
    /// API client
    api: Arc<BotApi>,
    /// Event handler
    handler: Arc<H>,
}

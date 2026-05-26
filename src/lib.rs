// the api indeed have a lot of arguments
#![allow(clippy::too_many_arguments)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

pub mod api;
pub mod audio;
pub mod client;
pub mod connection;
pub mod constant;
pub mod error;
pub mod event;
pub mod facade;
pub mod forum;
pub mod gateway;
pub mod http;
pub mod intents;
pub mod interaction;
pub mod log;
pub mod manage;
pub mod models;
pub mod openapi;
pub mod options;
pub mod reaction;
pub mod remote;
pub mod search;
pub mod session_manager;
pub mod signature;
pub mod token;
pub mod version;
pub mod webhook;
pub mod websocket;

// Re-export main types for convenience
pub use api::BotApi;
pub use audio::{Audio, AudioControl, AudioStatus, PublicAudio, PublicAudioType};
pub use client::{Client, Context, EventHandler};
pub use connection::{ConnectionSession, ConnectionState, Session};
pub use constant::*;
pub use error::{BotError, Result};
pub use event::{
    ATMessageEventHandler, AudioEventHandler, C2CFriendEventHandler, C2CMessageEventHandler,
    ChannelEventHandler, DirectMessageDeleteEventHandler, DirectMessageEventHandler,
    EnterAIOEventHandler, ErrorNotifyHandler, ForumAuditEventHandler, GroupATMessageEventHandler,
    GuildEventHandler, GuildMemberEventHandler, InteractionEventHandler, MessageAuditEventHandler,
    MessageDeleteEventHandler, MessageEventHandler, MessageReactionEventHandler, ParseAndHandle,
    ParseData, PlainEventHandler, PostEventHandler, PublicMessageDeleteEventHandler, ReadyHandler,
    RegisterHandler, RegisterHandlers, ReplyEventHandler, SubscribeMsgStatusEventHandler,
    ThreadEventHandler,
};
pub use facade::{
    DefaultImpl, NewOpenAPI, NewSandboxOpenAPI, RegisterDispatchEventHandler, SelectOpenAPIVersion,
    SetLogger, SetOpenAPIClient, SetSessionManager, SetWebsocketClient,
};
pub use forum::{
    Content, Format, ForumAuditResult, OpenThread, Post, PostInfo, Reply, ReplyInfo, Thread,
    ThreadInfo, Title,
};
pub use intents::Intents;
pub use interaction::{
    Interaction, InteractionData, InteractionDataType, InteractionType, SearchInputResolved,
    SearchLayout, SearchRecord, SearchRsp,
};
pub use log::{
    Debug, Debugf, DefaultLogger, Error as LogError, Errorf, Info, Infof, Logger, Sync, Warn, Warnf,
};
pub use manage::{
    C2CManageEvent, EnterAioEvent, GroupManageEvent, ManageEventType, SubscribeMessageStatusData,
    SubscribeMsgTemplateResult,
};
pub use models::gateway::Ready;
pub use models::*;
pub use openapi::{
    APIPermissionsAPI, AnnouncesAPI, AudioAPI, Base, ChannelAPI, ChannelPermissionsAPI,
    DirectMessageAPI, DoReqFilterChains, DoRespFilterChains, FilterContext, GuildAPI, HTTPFilter,
    InteractionAPI, IsSuccessStatus, MemberAPI, MessageAPI, MessageReactionAPI, MessageSettingAPI,
    OpenAPI, PinsAPI, Register as RegisterOpenAPI, RegisterReqFilter, RegisterRespFilter, RoleAPI,
    ScheduleAPI, UserAPI, WebhookAPI, WebsocketAPI,
};
pub use options::{OpenApiOption, Options, WithHideTip, WithURL};
pub use reaction::{
    Emoji as ReactionEmoji, MessageReaction, MessageReactionPager, MessageReactionUsers, Reaction,
    ReactionTarget, ReactionTargetType, ReactionUsers,
};
pub use remote::{
    ErrGotLockFailed, ErrProduceFailed, ErrSessionMarshalFailed, ErrorNotOk, Lock,
    Option as RemoteOption, RedisManager, WithClusterKey,
};
pub use search::{Config as SearchConfig, SimulateSearch};
pub use session_manager::{
    CalcInterval, CanNotIdentify, CanNotIdentifyErrSet, CanNotResume, CanNotResumeErrSet,
    ChanManager, CheckSessionLimit, NewSessionManager, Session as ManagedSession, SessionManager,
    calc_interval, new_session_manager, set_session_manager_factory,
};
pub use signature::{Generate, HeaderSig, HeaderTimestamp, Verify};
pub use token::{
    NewQQBotTokenSource, QQBotCredentials, QQBotTokenSource, StartRefreshAccessToken, Token,
    TypeBearer, TypeQQBot,
};
pub use version::version_string;
pub use webhook::{
    DefaultGetSecretFunc, GenDispatchACK, GenHeartbeatACK, GenValidationACK, HTTPHandler,
};
pub use websocket::{
    ClientImpl, DefaultQueueSize, PanicBufLen, PanicHandler, Register as RegisterWebSocket,
    RegisterHandlers as RegisterWebSocketHandlers, RegisterResumeSignal, ResumeSignal, WebSocket,
};

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

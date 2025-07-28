//! Connection state management for QQ Bot
//!
//! This module provides connection session management, state handling, and event parsing
//! for the websocket connections to QQ's gateway.

use crate::api::BotApi;
use crate::audio::{Audio, PublicAudio};
use crate::forum::{OpenThread, Thread};
use crate::interaction::Interaction;
use crate::manage::{C2CManageEvent, GroupManageEvent};
use crate::models::{channel::Channel, guild::Guild, message::*, robot::Robot, user::Member};
use crate::reaction::Reaction;
use futures_util::stream::{SplitSink, SplitStream};
// use futures_util::StreamExt;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

/// Type alias for websocket sink
pub type WsSink =
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>;

/// Type alias for websocket stream
pub type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Session information for websocket connections
#[derive(Debug, Clone)]
pub struct Session {
    /// Session ID
    pub session_id: String,
    /// Shard information
    pub shard: (u32, u32),
    /// Gateway URL
    pub url: String,
    /// Whether this session needs reconnection
    pub needs_reconnect: bool,
}

impl Session {
    /// Create a new session
    pub fn new(session_id: String, shard: (u32, u32), url: String) -> Self {
        Self {
            session_id,
            shard,
            url,
            needs_reconnect: false,
        }
    }

    /// Mark this session as needing reconnection
    pub fn mark_for_reconnect(&mut self) {
        self.session_id = String::new();
        self.needs_reconnect = true;
    }
}

/// Connection session pool for managing multiple websocket sessions
pub struct ConnectionSession {
    /// Maximum concurrent connections
    max_async: usize,
    /// Connection function
    connect_fn: Box<
        dyn Fn(
                Session,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<(), crate::error::BotError>> + Send>,
            > + Send
            + Sync,
    >,
    /// Event dispatcher
    dispatch_fn: Box<dyn Fn(&str, Value) + Send + Sync>,
    /// Session list
    sessions: Vec<Session>,
    /// Connection state
    state: Arc<Mutex<ConnectionState>>,
}

impl ConnectionSession {
    /// Create a new connection session
    pub fn new<F, D>(max_async: usize, connect_fn: F, dispatch_fn: D, api: BotApi) -> Self
    where
        F: Fn(
                Session,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<(), crate::error::BotError>> + Send>,
            > + Send
            + Sync
            + 'static,
        D: Fn(&str, Value) + Send + Sync + 'static,
    {
        Self {
            max_async,
            connect_fn: Box::new(connect_fn),
            dispatch_fn: Box::new(dispatch_fn),
            sessions: Vec::new(),
            state: Arc::new(Mutex::new(ConnectionState::new(api))),
        }
    }

    /// Add a session to the connection pool
    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    /// Run multiple sessions with specified interval
    pub async fn multi_run(mut self, session_interval: u64) -> Result<(), crate::error::BotError> {
        if self.sessions.is_empty() {
            return Ok(());
        }

        let mut index = 0;
        let mut tasks = Vec::new();

        while !self.sessions.is_empty() {
            debug!("Session list loop running");
            let time_interval = session_interval * (index + 1);
            info!(
                "Max concurrent connections: {}, Starting sessions: {}",
                self.max_async,
                self.sessions.len()
            );

            for _ in 0..self.max_async {
                if self.sessions.is_empty() {
                    break;
                }

                let session = self.sessions.remove(0);

                tasks.push(tokio::spawn(async move {
                    // For now, we'll skip the actual connection logic
                    // TODO: Implement proper connection handling
                    debug!("Would connect session: {:?}", session);
                    sleep(Duration::from_secs(time_interval)).await;
                }));
            }

            index += self.max_async as u64;
        }

        // Wait for all tasks to complete
        for task in tasks {
            if let Err(e) = task.await {
                error!("Task execution failed: {:?}", e);
            }
        }

        Ok(())
    }

    /// Get the connection state
    pub fn state(&self) -> Arc<Mutex<ConnectionState>> {
        self.state.clone()
    }
}

/// Connection state for handling websocket events
pub struct ConnectionState {
    /// Robot information
    pub robot: Option<Robot>,
    /// API client
    api: BotApi,
    /// Event parsers
    parsers: HashMap<String, fn(&ConnectionState, &Value) -> Option<(&'static str, Value)>>,
}

impl ConnectionState {
    /// Create a new connection state
    pub fn new(api: BotApi) -> Self {
        let mut state = Self {
            robot: None,
            api,
            parsers: HashMap::new(),
        };

        state.register_parsers();
        state
    }

    /// Register all event parsers
    fn register_parsers(&mut self) {
        self.parsers.insert("ready".to_string(), Self::parse_ready);
        self.parsers
            .insert("resumed".to_string(), Self::parse_resumed);

        // Guild events
        self.parsers
            .insert("guild_create".to_string(), Self::parse_guild_create);
        self.parsers
            .insert("guild_update".to_string(), Self::parse_guild_update);
        self.parsers
            .insert("guild_delete".to_string(), Self::parse_guild_delete);

        // Channel events
        self.parsers
            .insert("channel_create".to_string(), Self::parse_channel_create);
        self.parsers
            .insert("channel_update".to_string(), Self::parse_channel_update);
        self.parsers
            .insert("channel_delete".to_string(), Self::parse_channel_delete);

        // Member events
        self.parsers
            .insert("guild_member_add".to_string(), Self::parse_guild_member_add);
        self.parsers.insert(
            "guild_member_update".to_string(),
            Self::parse_guild_member_update,
        );
        self.parsers.insert(
            "guild_member_remove".to_string(),
            Self::parse_guild_member_remove,
        );

        // Message events
        self.parsers
            .insert("message_create".to_string(), Self::parse_message_create);
        self.parsers
            .insert("message_delete".to_string(), Self::parse_message_delete);
        self.parsers.insert(
            "at_message_create".to_string(),
            Self::parse_at_message_create,
        );
        self.parsers.insert(
            "public_message_delete".to_string(),
            Self::parse_public_message_delete,
        );

        // Direct message events
        self.parsers.insert(
            "direct_message_create".to_string(),
            Self::parse_direct_message_create,
        );
        self.parsers.insert(
            "direct_message_delete".to_string(),
            Self::parse_direct_message_delete,
        );

        // Reaction events
        self.parsers.insert(
            "message_reaction_add".to_string(),
            Self::parse_message_reaction_add,
        );
        self.parsers.insert(
            "message_reaction_remove".to_string(),
            Self::parse_message_reaction_remove,
        );

        // Interaction events
        self.parsers.insert(
            "interaction_create".to_string(),
            Self::parse_interaction_create,
        );

        // Audio events
        self.parsers
            .insert("audio_start".to_string(), Self::parse_audio_start);
        self.parsers
            .insert("audio_finish".to_string(), Self::parse_audio_finish);
        self.parsers
            .insert("on_mic".to_string(), Self::parse_on_mic);
        self.parsers
            .insert("off_mic".to_string(), Self::parse_off_mic);

        // Public audio events
        self.parsers.insert(
            "audio_or_live_channel_member_enter".to_string(),
            Self::parse_audio_or_live_channel_member_enter,
        );
        self.parsers.insert(
            "audio_or_live_channel_member_exit".to_string(),
            Self::parse_audio_or_live_channel_member_exit,
        );

        // Forum events
        self.parsers.insert(
            "forum_thread_create".to_string(),
            Self::parse_forum_thread_create,
        );
        self.parsers.insert(
            "forum_thread_update".to_string(),
            Self::parse_forum_thread_update,
        );
        self.parsers.insert(
            "forum_thread_delete".to_string(),
            Self::parse_forum_thread_delete,
        );
        self.parsers.insert(
            "forum_post_create".to_string(),
            Self::parse_forum_post_create,
        );
        self.parsers.insert(
            "forum_post_delete".to_string(),
            Self::parse_forum_post_delete,
        );
        self.parsers.insert(
            "forum_reply_create".to_string(),
            Self::parse_forum_reply_create,
        );
        self.parsers.insert(
            "forum_reply_delete".to_string(),
            Self::parse_forum_reply_delete,
        );
        self.parsers.insert(
            "forum_publish_audit_result".to_string(),
            Self::parse_forum_publish_audit_result,
        );

        // Open forum events
        self.parsers.insert(
            "open_forum_thread_create".to_string(),
            Self::parse_open_forum_thread_create,
        );
        self.parsers.insert(
            "open_forum_thread_update".to_string(),
            Self::parse_open_forum_thread_update,
        );
        self.parsers.insert(
            "open_forum_thread_delete".to_string(),
            Self::parse_open_forum_thread_delete,
        );
        self.parsers.insert(
            "open_forum_post_create".to_string(),
            Self::parse_open_forum_post_create,
        );
        self.parsers.insert(
            "open_forum_post_delete".to_string(),
            Self::parse_open_forum_post_delete,
        );
        self.parsers.insert(
            "open_forum_reply_create".to_string(),
            Self::parse_open_forum_reply_create,
        );
        self.parsers.insert(
            "open_forum_reply_delete".to_string(),
            Self::parse_open_forum_reply_delete,
        );

        // Group and C2C events
        self.parsers.insert(
            "group_at_message_create".to_string(),
            Self::parse_group_at_message_create,
        );
        self.parsers.insert(
            "c2c_message_create".to_string(),
            Self::parse_c2c_message_create,
        );
        self.parsers
            .insert("group_add_robot".to_string(), Self::parse_group_add_robot);
        self.parsers
            .insert("group_del_robot".to_string(), Self::parse_group_del_robot);
        self.parsers
            .insert("group_msg_reject".to_string(), Self::parse_group_msg_reject);
        self.parsers.insert(
            "group_msg_receive".to_string(),
            Self::parse_group_msg_receive,
        );
        self.parsers
            .insert("friend_add".to_string(), Self::parse_friend_add);
        self.parsers
            .insert("friend_del".to_string(), Self::parse_friend_del);
        self.parsers
            .insert("c2c_msg_reject".to_string(), Self::parse_c2c_msg_reject);
        self.parsers
            .insert("c2c_msg_receive".to_string(), Self::parse_c2c_msg_receive);

        // Message audit events
        self.parsers.insert(
            "message_audit_pass".to_string(),
            Self::parse_message_audit_pass,
        );
        self.parsers.insert(
            "message_audit_reject".to_string(),
            Self::parse_message_audit_reject,
        );
    }

    /// Parse an event and return the event name and data for dispatching
    pub fn parse_event(&self, event_type: &str, payload: &Value) -> Option<(&'static str, Value)> {
        if let Some(parser) = self.parsers.get(event_type) {
            parser(self, payload)
        } else {
            warn!("Unknown event type: {}", event_type);
            None
        }
    }

    // Event parsers
    fn parse_ready(_state: &ConnectionState, _payload: &Value) -> Option<(&'static str, Value)> {
        Some(("ready", Value::Null))
    }

    fn parse_resumed(_state: &ConnectionState, _payload: &Value) -> Option<(&'static str, Value)> {
        Some(("resumed", Value::Null))
    }

    fn parse_guild_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let guild_id = payload.get("id").and_then(|v| v.as_str())?;
        let guild_data = payload.get("d")?;
        let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
        Some(("guild_create", serde_json::to_value(guild).ok()?))
    }

    fn parse_guild_update(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let guild_id = payload.get("id").and_then(|v| v.as_str())?;
        let guild_data = payload.get("d")?;
        let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
        Some(("guild_update", serde_json::to_value(guild).ok()?))
    }

    fn parse_guild_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let guild_id = payload.get("id").and_then(|v| v.as_str())?;
        let guild_data = payload.get("d")?;
        let guild = Guild::from_data(state.api.clone(), guild_id.to_string(), guild_data.clone());
        Some(("guild_delete", serde_json::to_value(guild).ok()?))
    }

    fn parse_channel_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let channel_id = payload.get("id").and_then(|v| v.as_str())?;
        let channel_data = payload.get("d")?;
        let channel = Channel::from_data(
            state.api.clone(),
            channel_id.to_string(),
            channel_data.clone(),
        );
        Some(("channel_create", serde_json::to_value(channel).ok()?))
    }

    fn parse_channel_update(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let channel_id = payload.get("id").and_then(|v| v.as_str())?;
        let channel_data = payload.get("d")?;
        let channel = Channel::from_data(
            state.api.clone(),
            channel_id.to_string(),
            channel_data.clone(),
        );
        Some(("channel_update", serde_json::to_value(channel).ok()?))
    }

    fn parse_channel_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let channel_id = payload.get("id").and_then(|v| v.as_str())?;
        let channel_data = payload.get("d")?;
        let channel = Channel::from_data(
            state.api.clone(),
            channel_id.to_string(),
            channel_data.clone(),
        );
        Some(("channel_delete", serde_json::to_value(channel).ok()?))
    }

    fn parse_guild_member_add(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let _member_id = payload.get("id").and_then(|v| v.as_str())?;
        let member_data = payload.get("d")?;
        let member = Member::from_data(member_data.clone());
        Some(("guild_member_add", serde_json::to_value(member).ok()?))
    }

    fn parse_guild_member_update(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let _member_id = payload.get("id").and_then(|v| v.as_str())?;
        let member_data = payload.get("d")?;
        let member = Member::from_data(member_data.clone());
        Some(("guild_member_update", serde_json::to_value(member).ok()?))
    }

    fn parse_guild_member_remove(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let _member_id = payload.get("id").and_then(|v| v.as_str())?;
        let member_data = payload.get("d")?;
        let member = Member::from_data(member_data.clone());
        Some(("guild_member_remove", serde_json::to_value(member).ok()?))
    }

    fn parse_message_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = Message::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("message_create", serde_json::to_value(message).ok()?))
    }

    fn parse_message_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = Message::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("message_delete", serde_json::to_value(message).ok()?))
    }

    fn parse_at_message_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = Message::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("at_message_create", serde_json::to_value(message).ok()?))
    }

    fn parse_public_message_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = Message::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("public_message_delete", serde_json::to_value(message).ok()?))
    }

    fn parse_direct_message_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = DirectMessage::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("direct_message_create", serde_json::to_value(message).ok()?))
    }

    fn parse_direct_message_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = DirectMessage::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("direct_message_delete", serde_json::to_value(message).ok()?))
    }

    fn parse_message_reaction_add(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
        let reaction_data = payload.get("d")?;
        let reaction = Reaction::new(
            state.api.clone(),
            Some(reaction_id.to_string()),
            reaction_data,
        );
        Some(("message_reaction_add", serde_json::to_value(reaction).ok()?))
    }

    fn parse_message_reaction_remove(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reaction_id = payload.get("id").and_then(|v| v.as_str())?;
        let reaction_data = payload.get("d")?;
        let reaction = Reaction::new(
            state.api.clone(),
            Some(reaction_id.to_string()),
            reaction_data,
        );
        Some((
            "message_reaction_remove",
            serde_json::to_value(reaction).ok()?,
        ))
    }

    fn parse_interaction_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let interaction_id = payload.get("id").and_then(|v| v.as_str())?;
        let interaction_data = payload.get("d")?;
        let interaction = Interaction::new(
            state.api.clone(),
            Some(interaction_id.to_string()),
            interaction_data,
        );
        Some((
            "interaction_create",
            serde_json::to_value(interaction).ok()?,
        ))
    }

    fn parse_audio_start(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let audio_id = payload.get("id").and_then(|v| v.as_str())?;
        let audio_data = payload.get("d")?;
        // Convert to AudioAction for Audio::new
        let audio_action = crate::models::api::AudioAction {
            guild_id: audio_data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: audio_data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            audio_url: audio_data
                .get("audio_url")
                .and_then(|v| v.as_str())
                .map(String::from),
            text: audio_data
                .get("text")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
        Some(("audio_start", serde_json::to_value(audio).ok()?))
    }

    fn parse_audio_finish(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let audio_id = payload.get("id").and_then(|v| v.as_str())?;
        let audio_data = payload.get("d")?;
        let audio_action = crate::models::api::AudioAction {
            guild_id: audio_data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: audio_data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            audio_url: audio_data
                .get("audio_url")
                .and_then(|v| v.as_str())
                .map(String::from),
            text: audio_data
                .get("text")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
        Some(("audio_finish", serde_json::to_value(audio).ok()?))
    }

    fn parse_on_mic(state: &ConnectionState, payload: &Value) -> Option<(&'static str, Value)> {
        let audio_id = payload.get("id").and_then(|v| v.as_str())?;
        let audio_data = payload.get("d")?;
        let audio_action = crate::models::api::AudioAction {
            guild_id: audio_data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: audio_data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            audio_url: audio_data
                .get("audio_url")
                .and_then(|v| v.as_str())
                .map(String::from),
            text: audio_data
                .get("text")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
        Some(("on_mic", serde_json::to_value(audio).ok()?))
    }

    fn parse_off_mic(state: &ConnectionState, payload: &Value) -> Option<(&'static str, Value)> {
        let audio_id = payload.get("id").and_then(|v| v.as_str())?;
        let audio_data = payload.get("d")?;
        let audio_action = crate::models::api::AudioAction {
            guild_id: audio_data
                .get("guild_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_id: audio_data
                .get("channel_id")
                .and_then(|v| v.as_str())
                .map(String::from),
            audio_url: audio_data
                .get("audio_url")
                .and_then(|v| v.as_str())
                .map(String::from),
            text: audio_data
                .get("text")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        let audio = Audio::new(state.api.clone(), Some(audio_id.to_string()), audio_action);
        Some(("off_mic", serde_json::to_value(audio).ok()?))
    }

    fn parse_audio_or_live_channel_member_enter(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let audio_data = payload.get("d")?;
        let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
        Some((
            "audio_or_live_channel_member_enter",
            serde_json::to_value(public_audio).ok()?,
        ))
    }

    fn parse_audio_or_live_channel_member_exit(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let audio_data = payload.get("d")?;
        let public_audio = PublicAudio::new(state.api.clone(), audio_data.clone());
        Some((
            "audio_or_live_channel_member_exit",
            serde_json::to_value(public_audio).ok()?,
        ))
    }

    fn parse_forum_thread_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_id = payload.get("id").and_then(|v| v.as_str())?;
        let thread_data = payload.get("d")?;
        let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
        Some(("forum_thread_create", serde_json::to_value(thread).ok()?))
    }

    fn parse_forum_thread_update(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_id = payload.get("id").and_then(|v| v.as_str())?;
        let thread_data = payload.get("d")?;
        let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
        Some(("forum_thread_update", serde_json::to_value(thread).ok()?))
    }

    fn parse_forum_thread_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_id = payload.get("id").and_then(|v| v.as_str())?;
        let thread_data = payload.get("d")?;
        let thread = Thread::new(state.api.clone(), Some(thread_id.to_string()), thread_data);
        Some(("forum_thread_delete", serde_json::to_value(thread).ok()?))
    }

    fn parse_forum_post_create(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let post_data = payload.get("d")?;
        Some(("forum_post_create", post_data.clone()))
    }

    fn parse_forum_post_delete(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let post_data = payload.get("d")?;
        Some(("forum_post_delete", post_data.clone()))
    }

    fn parse_forum_reply_create(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reply_data = payload.get("d")?;
        Some(("forum_reply_create", reply_data.clone()))
    }

    fn parse_forum_reply_delete(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reply_data = payload.get("d")?;
        Some(("forum_reply_delete", reply_data.clone()))
    }

    fn parse_forum_publish_audit_result(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let audit_data = payload.get("d")?;
        Some(("forum_publish_audit_result", audit_data.clone()))
    }

    fn parse_open_forum_thread_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_data = payload.get("d")?;
        let thread = OpenThread::new(state.api.clone(), thread_data);
        Some((
            "open_forum_thread_create",
            serde_json::to_value(thread).ok()?,
        ))
    }

    fn parse_open_forum_thread_update(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_data = payload.get("d")?;
        let thread = OpenThread::new(state.api.clone(), thread_data);
        Some((
            "open_forum_thread_update",
            serde_json::to_value(thread).ok()?,
        ))
    }

    fn parse_open_forum_thread_delete(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let thread_data = payload.get("d")?;
        let thread = OpenThread::new(state.api.clone(), thread_data);
        Some((
            "open_forum_thread_delete",
            serde_json::to_value(thread).ok()?,
        ))
    }

    fn parse_open_forum_post_create(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let post_data = payload.get("d")?;
        Some(("open_forum_post_create", post_data.clone()))
    }

    fn parse_open_forum_post_delete(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let post_data = payload.get("d")?;
        Some(("open_forum_post_delete", post_data.clone()))
    }

    fn parse_open_forum_reply_create(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reply_data = payload.get("d")?;
        Some(("open_forum_reply_create", reply_data.clone()))
    }

    fn parse_open_forum_reply_delete(
        _state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let reply_data = payload.get("d")?;
        Some(("open_forum_reply_delete", reply_data.clone()))
    }

    fn parse_group_at_message_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = GroupMessage::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some((
            "group_at_message_create",
            serde_json::to_value(message).ok()?,
        ))
    }

    fn parse_c2c_message_create(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = C2CMessage::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("c2c_message_create", serde_json::to_value(message).ok()?))
    }

    fn parse_group_add_robot(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("group_add_robot", serde_json::to_value(event).ok()?))
    }

    fn parse_group_del_robot(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("group_del_robot", serde_json::to_value(event).ok()?))
    }

    fn parse_group_msg_reject(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("group_msg_reject", serde_json::to_value(event).ok()?))
    }

    fn parse_group_msg_receive(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = GroupManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("group_msg_receive", serde_json::to_value(event).ok()?))
    }

    fn parse_friend_add(state: &ConnectionState, payload: &Value) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("friend_add", serde_json::to_value(event).ok()?))
    }

    fn parse_friend_del(state: &ConnectionState, payload: &Value) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("friend_del", serde_json::to_value(event).ok()?))
    }

    fn parse_c2c_msg_reject(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("c2c_msg_reject", serde_json::to_value(event).ok()?))
    }

    fn parse_c2c_msg_receive(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let event_id = payload.get("id").and_then(|v| v.as_str())?;
        let event_data = payload.get("d")?;
        let mut data_map = std::collections::HashMap::new();
        if let Value::Object(obj) = event_data {
            for (k, v) in obj {
                data_map.insert(k.clone(), v.clone());
            }
        }
        let event = C2CManageEvent::new(state.api.clone(), Some(event_id.to_string()), &data_map);
        Some(("c2c_msg_receive", serde_json::to_value(event).ok()?))
    }

    fn parse_message_audit_pass(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = MessageAudit::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("message_audit_pass", serde_json::to_value(message).ok()?))
    }

    fn parse_message_audit_reject(
        state: &ConnectionState,
        payload: &Value,
    ) -> Option<(&'static str, Value)> {
        let message_id = payload.get("id").and_then(|v| v.as_str())?;
        let message_data = payload.get("d")?;
        let message = MessageAudit::from_data(
            state.api.clone(),
            message_id.to_string(),
            message_data.clone(),
        );
        Some(("message_audit_reject", serde_json::to_value(message).ok()?))
    }
}

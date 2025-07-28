//! Main client implementation for the QQ Guild Bot API.
//!
//! This module provides the main `Client` struct that serves as the entry point
//! for bot applications, handling connections, events, and API interactions.

use crate::api::BotApi;
use crate::error::{BotError, Result};
use crate::gateway::Gateway;
use crate::http::HttpClient;
use crate::intents::Intents;
use crate::models::gateway::GatewayEvent;
use crate::models::*;
use crate::token::Token;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Event handler trait for processing gateway events.
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, _ready: Ready) {}

    /// Called when a message is created (@ mentions).
    async fn message_create(&self, _ctx: Context, _message: Message) {}

    /// Called when a direct message is created.
    async fn direct_message_create(&self, _ctx: Context, _message: DirectMessage) {}

    /// Called when a group message is created.
    async fn group_message_create(&self, _ctx: Context, _message: GroupMessage) {}

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, _ctx: Context, _message: C2CMessage) {}

    /// Called when a message is deleted.
    async fn message_delete(&self, _ctx: Context, _message: Message) {}

    /// Called when a guild is created (bot joins).
    async fn guild_create(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is updated.
    async fn guild_update(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a guild is deleted (bot leaves).
    async fn guild_delete(&self, _ctx: Context, _guild: Guild) {}

    /// Called when a channel is created.
    async fn channel_create(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is updated.
    async fn channel_update(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a channel is deleted.
    async fn channel_delete(&self, _ctx: Context, _channel: Channel) {}

    /// Called when a guild member is added.
    async fn guild_member_add(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is updated.
    async fn guild_member_update(&self, _ctx: Context, _member: Member) {}

    /// Called when a guild member is removed.
    async fn guild_member_remove(&self, _ctx: Context, _member: Member) {}

    /// Called when a message audit passes.
    async fn message_audit_pass(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called when a message audit is rejected.
    async fn message_audit_reject(&self, _ctx: Context, _audit: MessageAudit) {}

    /// Called for any unhandled events.
    async fn unknown_event(&self, _ctx: Context, _event: GatewayEvent) {}

    /// Called when an error occurs during event processing.
    async fn error(&self, _error: BotError) {
        error!("Event handler error: {}", _error);
    }
}

/// Context passed to event handlers containing API access and bot information.
#[derive(Clone)]
pub struct Context {
    /// API client for making requests
    pub api: Arc<BotApi>,
    /// Bot information
    pub bot_info: Option<BotInfo>,
}

impl Context {
    /// Creates a new context.
    pub fn new(api: Arc<BotApi>) -> Self {
        Self {
            api,
            bot_info: None,
        }
    }

    /// Sets the bot information.
    pub fn with_bot_info(mut self, bot_info: BotInfo) -> Self {
        self.bot_info = Some(bot_info);
        self
    }
}

/// Main client for the QQ Guild Bot API.
pub struct Client<H: EventHandler> {
    /// Authentication token
    token: Token,
    /// Intent flags
    intents: Intents,
    /// HTTP client
    http: HttpClient,
    /// API client
    api: Arc<BotApi>,
    /// Event handler
    handler: Arc<H>,
    /// Whether to use sandbox environment
    is_sandbox: bool,
    /// Request timeout in seconds
    timeout: u64,
}

impl<H: EventHandler + 'static> Client<H> {
    /// Creates a new client.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `intents` - Intent flags for events to receive
    /// * `handler` - Event handler implementation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use botrs::{Client, Token, Intents, EventHandler, Context};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl EventHandler for MyHandler {
    ///     async fn message_create(&self, ctx: Context, message: botrs::Message) {
    ///         println!("Received message: {:?}", message.content);
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let client = Client::new(token, intents, handler)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(token: Token, intents: Intents, handler: H) -> Result<Self> {
        let timeout = crate::DEFAULT_TIMEOUT;
        let is_sandbox = false;

        let http = HttpClient::new(timeout, is_sandbox)?;
        let api = Arc::new(BotApi::new(http.clone()));

        Ok(Self {
            token,
            intents,
            http,
            api,
            handler: Arc::new(handler),
            is_sandbox,
            timeout,
        })
    }

    /// Creates a new client with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `intents` - Intent flags for events to receive
    /// * `handler` - Event handler implementation
    /// * `timeout` - Request timeout in seconds
    /// * `is_sandbox` - Whether to use sandbox environment
    ///
    /// # Returns
    ///
    /// A new client instance.
    pub fn with_config(
        token: Token,
        intents: Intents,
        handler: H,
        timeout: u64,
        is_sandbox: bool,
    ) -> Result<Self> {
        let http = HttpClient::new(timeout, is_sandbox)?;
        let api = Arc::new(BotApi::new(http.clone()));

        Ok(Self {
            token,
            intents,
            http,
            api,
            handler: Arc::new(handler),
            is_sandbox,
            timeout,
        })
    }

    /// Starts the bot and connects to the gateway.
    ///
    /// This method will block until the bot is stopped or an error occurs.
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    ///
    /// # Examples
    ///
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
    ///     let token = Token::new("app_id", "secret");
    ///     let intents = Intents::default();
    ///     let handler = MyHandler;
    ///     let mut client = Client::new(token, intents, handler)?;
    ///     client.start().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting bot client");

        // Validate token
        self.token.validate()?;

        // Get bot information
        let bot_info = self.api.get_bot_info(&self.token).await?;
        info!("Bot info: {} ({})", bot_info.username, bot_info.id);

        // Get gateway information
        let gateway_info = self.api.get_gateway(&self.token).await?;
        info!("Gateway URL: {}", gateway_info.url);

        // Create context
        let ctx = Context::new(self.api.clone()).with_bot_info(bot_info);

        // Set up event channel
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

        // Create and connect gateway
        let gateway = Gateway::new(
            gateway_info.url,
            self.token.clone(),
            self.intents,
            None, // TODO: Implement sharding
        );

        // Start gateway connection in a separate task
        let gateway_task = {
            let mut gateway_clone = gateway;
            async move {
                if let Err(e) = gateway_clone.connect(event_sender).await {
                    error!("Gateway connection error: {}", e);
                }
            }
        };

        tokio::spawn(gateway_task);

        // Main event processing loop
        while let Some(event) = event_receiver.recv().await {
            if let Err(e) = self.handle_event(ctx.clone(), event).await {
                self.handler.error(e).await;
            }
        }

        info!("Bot client stopped");
        Ok(())
    }

    /// Handles a gateway event by dispatching it to the appropriate handler method.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Event context
    /// * `event` - Gateway event to handle
    ///
    /// # Returns
    ///
    /// Result indicating success or failure.
    async fn handle_event(&self, ctx: Context, event: GatewayEvent) -> Result<()> {
        debug!("Handling event: {:?}", event.event_type);

        match event.event_type.as_deref() {
            Some("READY") => {
                if let Ok(ready) = serde_json::from_value::<Ready>(event.data) {
                    info!("Bot is ready! Session ID: {}", ready.session_id);
                    self.handler.ready(ctx, ready).await;
                }
            }
            Some("AT_MESSAGE_CREATE") => {
                if let Ok(message) = serde_json::from_value::<Message>(event.data) {
                    self.handler.message_create(ctx, message).await;
                }
            }
            Some("DIRECT_MESSAGE_CREATE") => {
                if let Ok(message) = serde_json::from_value::<DirectMessage>(event.data) {
                    self.handler.direct_message_create(ctx, message).await;
                }
            }
            Some("GROUP_AT_MESSAGE_CREATE") => {
                if let Ok(message) = serde_json::from_value::<GroupMessage>(event.data) {
                    self.handler.group_message_create(ctx, message).await;
                }
            }
            Some("C2C_MESSAGE_CREATE") => {
                if let Ok(message) = serde_json::from_value::<C2CMessage>(event.data) {
                    self.handler.c2c_message_create(ctx, message).await;
                }
            }
            Some("PUBLIC_MESSAGE_DELETE") => {
                if let Ok(message) = serde_json::from_value::<Message>(event.data) {
                    self.handler.message_delete(ctx, message).await;
                }
            }
            Some("GUILD_CREATE") => {
                if let Ok(guild) = serde_json::from_value::<Guild>(event.data) {
                    self.handler.guild_create(ctx, guild).await;
                }
            }
            Some("GUILD_UPDATE") => {
                if let Ok(guild) = serde_json::from_value::<Guild>(event.data) {
                    self.handler.guild_update(ctx, guild).await;
                }
            }
            Some("GUILD_DELETE") => {
                if let Ok(guild) = serde_json::from_value::<Guild>(event.data) {
                    self.handler.guild_delete(ctx, guild).await;
                }
            }
            Some("CHANNEL_CREATE") => {
                if let Ok(channel) = serde_json::from_value::<Channel>(event.data) {
                    self.handler.channel_create(ctx, channel).await;
                }
            }
            Some("CHANNEL_UPDATE") => {
                if let Ok(channel) = serde_json::from_value::<Channel>(event.data) {
                    self.handler.channel_update(ctx, channel).await;
                }
            }
            Some("CHANNEL_DELETE") => {
                if let Ok(channel) = serde_json::from_value::<Channel>(event.data) {
                    self.handler.channel_delete(ctx, channel).await;
                }
            }
            Some("GUILD_MEMBER_ADD") => {
                if let Ok(member) = serde_json::from_value::<Member>(event.data) {
                    self.handler.guild_member_add(ctx, member).await;
                }
            }
            Some("GUILD_MEMBER_UPDATE") => {
                if let Ok(member) = serde_json::from_value::<Member>(event.data) {
                    self.handler.guild_member_update(ctx, member).await;
                }
            }
            Some("GUILD_MEMBER_REMOVE") => {
                if let Ok(member) = serde_json::from_value::<Member>(event.data) {
                    self.handler.guild_member_remove(ctx, member).await;
                }
            }
            Some("MESSAGE_AUDIT_PASS") => {
                if let Ok(audit) = serde_json::from_value::<MessageAudit>(event.data) {
                    self.handler.message_audit_pass(ctx, audit).await;
                }
            }
            Some("MESSAGE_AUDIT_REJECT") => {
                if let Ok(audit) = serde_json::from_value::<MessageAudit>(event.data) {
                    self.handler.message_audit_reject(ctx, audit).await;
                }
            }
            _ => {
                debug!("Unknown event type: {:?}", event.event_type);
                self.handler.unknown_event(ctx, event).await;
            }
        }

        Ok(())
    }

    /// Gets a reference to the API client.
    pub fn api(&self) -> &BotApi {
        &self.api
    }

    /// Gets a reference to the HTTP client.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Gets the intents being used.
    pub fn intents(&self) -> Intents {
        self.intents
    }

    /// Returns true if using sandbox environment.
    pub fn is_sandbox(&self) -> bool {
        self.is_sandbox
    }

    /// Shuts down the client and cleans up resources.
    pub async fn shutdown(&self) {
        info!("Shutting down bot client");
        self.api.close().await;
    }
}

impl<H: EventHandler> std::fmt::Debug for Client<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("intents", &self.intents)
            .field("is_sandbox", &self.is_sandbox)
            .field("timeout", &self.timeout)
            .finish()
    }
}

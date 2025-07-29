//! Demo: C2C Manage Event
//!
//! This example demonstrates how to create a bot that handles C2C management events.
//! It's equivalent to the Python demo_c2c_manage_event.py example.

mod common;

use botrs::{C2CManageEvent, Client, Context, EventHandler, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to C2C management events.
struct C2CManageEventHandler;

#[async_trait::async_trait]
impl EventHandler for C2CManageEventHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a friend is added.
    async fn friend_add(&self, ctx: Context, event: C2CManageEvent) {
        info!("用户添加机器人：{}", event);

        // Get user OpenID
        let openid = match &event.openid {
            Some(openid) => openid,
            None => {
                warn!("Friend add event has no openid");
                return;
            }
        };

        // Send welcome message (equivalent to self.api.post_c2c_message)
        let params = botrs::models::message::C2CMessageParams {
            msg_type: 0,
            content: Some("hello".to_string()),
            event_id: event.event_id.clone(),
            ..Default::default()
        };

        match ctx
            .api
            .post_c2c_message_with_params(&ctx.token, openid, params)
            .await
        {
            Ok(response) => {
                info!("Successfully sent welcome message");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send welcome message: {}", e),
        }
    }

    /// Called when a friend is deleted.
    async fn friend_del(&self, _ctx: Context, event: C2CManageEvent) {
        info!("用户删除机器人：{}", event);
    }

    /// Called when C2C message is rejected.
    async fn c2c_msg_reject(&self, _ctx: Context, event: C2CManageEvent) {
        info!("用户关闭机器人主动消息：{}", event);
    }

    /// Called when C2C message is received.
    async fn c2c_msg_receive(&self, _ctx: Context, event: C2CManageEvent) {
        info!("用户打开机器人主动消息：{}", event);
    }

    /// Called when an error occurs during event processing.
    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting C2C manage event demo...");

    // Load configuration with multiple fallback options
    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1), // app_id from command line
        env::args().nth(2), // secret from command line
    )?;

    info!("Configuration loaded successfully");

    // Create token
    let token = Token::new(config.bot.app_id, config.bot.secret);

    // Validate token
    if let Err(e) = token.validate() {
        panic!("Invalid token: {}", e);
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public messages (C2C management events)
    // This is equivalent to: intents = botpy.Intents(public_messages=True)
    let intents = Intents::default().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = C2CManageEventHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

//! Demo: Group Manage Event
//!
//! This example demonstrates how to create a bot that handles group management events.
//! It's equivalent to the Python demo_group_manage_event.py example.

mod common;

use botrs::{Client, Context, EventHandler, GroupManageEvent, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to group management events.
struct GroupManageEventHandler;

#[async_trait::async_trait]
impl EventHandler for GroupManageEventHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when robot is added to group.
    async fn group_add_robot(&self, ctx: Context, event: GroupManageEvent) {
        info!("机器人被添加到群聊：{}", event);

        // Get group OpenID
        let group_openid = match &event.group_openid {
            Some(openid) => openid,
            None => {
                warn!("Group add robot event has no group_openid");
                return;
            }
        };

        // Send welcome message (equivalent to self.api.post_group_message)
        let params = botrs::models::message::GroupMessageParams {
            msg_type: 0,
            content: Some("hello".to_string()),
            event_id: event.event_id.clone(),
            ..Default::default()
        };

        match ctx
            .api
            .post_group_message_with_params(&ctx.token, group_openid, params)
            .await
        {
            Ok(response) => {
                info!("Successfully sent welcome message to group");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send welcome message to group: {}", e),
        }
    }

    /// Called when robot is deleted from group.
    async fn group_del_robot(&self, _ctx: Context, event: GroupManageEvent) {
        info!("机器人被移除群聊：{}", event);
    }

    /// Called when group message is rejected.
    async fn group_msg_reject(&self, _ctx: Context, event: GroupManageEvent) {
        info!("群聊关闭机器人主动消息：{}", event);
    }

    /// Called when group message is received.
    async fn group_msg_receive(&self, _ctx: Context, event: GroupManageEvent) {
        info!("群聊打开机器人主动消息：{}", event);
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

    info!("Starting group manage event demo...");

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
        panic!("Invalid token: {e}");
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public messages (group management events)
    // This is equivalent to: intents = botpy.Intents(public_messages=True)
    let intents = Intents::default().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = GroupManageEventHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

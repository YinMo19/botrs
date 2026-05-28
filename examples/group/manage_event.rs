//! Group Manage Event
//!
//! This example demonstrates how to create a bot that handles group management events.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::GroupMessageParams;
use botrs::{Client, EventHandler, GroupManageSession, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to group management events.
struct GroupManageEventHandler;

#[async_trait::async_trait]
impl EventHandler for GroupManageEventHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when robot is added to group.
    async fn group_add_robot(&self, mut session: GroupManageSession) {
        let event = session.event();
        info!("机器人被添加到群聊：{:?}", event);

        let params = GroupMessageParams::new_text("hello");

        match session.send_message(params).await {
            Ok(response) => {
                info!("Successfully sent welcome message to group");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send welcome message to group: {}", e),
        }
    }

    /// Called when robot is deleted from group.
    async fn group_del_robot(&self, session: GroupManageSession) {
        info!("机器人被移除群聊：{:?}", session.event());
    }

    /// Called when group message is rejected.
    async fn group_msg_reject(&self, session: GroupManageSession) {
        info!("群聊关闭机器人主动消息：{:?}", session.event());
    }

    /// Called when group message is received.
    async fn group_msg_receive(&self, session: GroupManageSession) {
        info!("群聊打开机器人主动消息：{:?}", session.event());
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

    info!("Starting group manage event example...");

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
    let intents = Intents::new().with_public_messages();

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

//! Guild Pins Message
//!
//! This example demonstrates how to create a bot that manages pinned messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that manages pinned messages.
struct PinsMessageHandler;

#[async_trait::async_trait]
impl EventHandler for PinsMessageHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = &message.content;

        // Get channel ID for operations
        let channel_id = &message.channel_id;

        // Get message ID for operations
        let message_id = &message.id;

        // Get bot name from the bot info if available
        let bot_name = session
            .bot_info()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Reply to the message first
        match session.reply(reply_content).await {
            Ok(_) => info!("Successfully replied to message"),
            Err(e) => warn!("Failed to reply to message: {}", e),
        }

        // Handle different pin-related commands
        if content.contains("/获取精华列表") {
            // Get pins message list (equivalent to self.api.get_pins)
            match session.get_pins(channel_id).await {
                Ok(pins_message) => {
                    info!("Pins message list: {:?}", pins_message);
                }
                Err(e) => {
                    warn!("Failed to get pins: {}", e);
                }
            }
        }

        if content.contains("/创建精华消息") {
            // Create pin message (equivalent to self.api.put_pin)
            match session.put_pin(channel_id, message_id).await {
                Ok(pins_message) => {
                    info!("Created pin message: {:?}", pins_message);
                }
                Err(e) => {
                    warn!("Failed to create pin: {}", e);
                }
            }
        }

        if content.contains("/删除精华消息") {
            // Delete pin message (equivalent to self.api.delete_pin)
            match session.delete_pin(channel_id, message_id).await {
                Ok(result) => {
                    info!("Deleted pin message: {:?}", result);
                }
                Err(e) => {
                    warn!("Failed to delete pin: {}", e);
                }
            }
        }
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

    info!("Starting guild pins message example...");

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

    // Set up intents - we want to receive public guild messages (@ mentions)
    let intents = Intents::new().with_public_guild_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = PinsMessageHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

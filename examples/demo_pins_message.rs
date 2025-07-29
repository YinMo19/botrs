//! Demo: Pins Message
//!
//! This example demonstrates how to create a bot that manages pinned messages.
//! It's equivalent to the Python demo_pins_message.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that manages pinned messages.
struct PinsMessageHandler;

#[async_trait::async_trait]
impl EventHandler for PinsMessageHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, ctx: Context, message: Message) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        // Get channel ID for operations
        let channel_id = match &message.channel_id {
            Some(channel_id) => channel_id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        // Get message ID for operations
        let message_id = match &message.id {
            Some(message_id) => message_id,
            None => {
                warn!("Message has no message_id");
                return;
            }
        };

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{}收到你的@消息了: {}", bot_name, content);

        // Reply to the message first
        match message.reply(&ctx.api, &ctx.token, &reply_content).await {
            Ok(_) => info!("Successfully replied to message"),
            Err(e) => warn!("Failed to reply to message: {}", e),
        }

        // Handle different pin-related commands
        if content.contains("/获取精华列表") {
            // Get pins message list (equivalent to self.api.get_pins)
            match ctx.api.get_pins(&ctx.token, channel_id).await {
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
            match ctx.api.put_pin(&ctx.token, channel_id, message_id).await {
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
            match ctx.api.delete_pin(&ctx.token, channel_id, message_id).await {
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

    info!("Starting pins message demo...");

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

    // Set up intents - we want to receive public guild messages (@ mentions)
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

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

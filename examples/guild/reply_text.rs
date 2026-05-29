//! Guild Reply Text
//!
//! This example demonstrates how to create a bot that responds to @ mentions.

#[path = "../common/mod.rs"]
mod common;

use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions.
struct AtReplyHandler;

#[async_trait::async_trait]
impl EventHandler for AtReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        // Log user avatar and username.
        if !message.author.avatar.is_empty() {
            info!("User avatar: {}", message.author.avatar);
        }
        info!("Username: {}", message.author.username);

        // Get message content
        let content = &message.content;

        // Handle "sleep" command (similar to Python asyncio.sleep)
        if content.contains("sleep") {
            info!("Received sleep command, waiting 10 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

        // Get bot name from the bot info if available
        let bot_name = session
            .bot_info()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Reply to the message
        match session.reply(reply_content).await {
            Ok(_) => info!("Successfully replied to message"),
            Err(e) => warn!("Failed to reply to message: {}", e),
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

    info!("Starting guild reply text example...");

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
    let handler = AtReplyHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

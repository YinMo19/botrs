//! Demo: AT Reply
//!
//! This example demonstrates how to create a bot that responds to @ mentions.
//! It's equivalent to the Python demo_at_reply.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{init_logging, Config};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions.
struct AtReplyHandler;

#[async_trait::async_trait]
impl EventHandler for AtReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, ctx: Context, message: Message) {
        // Log user avatar and username (similar to Python version)
        if let Some(author) = &message.author {
            if let Some(avatar) = &author.avatar {
                info!("User avatar: {}", avatar);
            }
            if let Some(username) = &author.username {
                info!("Username: {}", username);
            }
        }

        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        // Handle "sleep" command (similar to Python asyncio.sleep)
        if content.contains("sleep") {
            info!("Received sleep command, waiting 10 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Reply to the message
        match message.reply(&ctx.api, &ctx.token, &reply_content).await {
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

    info!("Starting AT reply demo...");

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
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

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

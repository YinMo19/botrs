//! Simple bot example demonstrating basic usage of the BotRS framework.
//!
//! This example shows how to create a basic QQ Guild bot that responds to messages.

use botrs::{api::BotApi, Client, Context, EventHandler, Intents, Message, Ready, Token};

use tracing::{info, warn};

/// Simple event handler that responds to messages.
struct SimpleHandler;

#[async_trait::async_trait]
impl EventHandler for SimpleHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot is ready! Logged in as: {}", ready.user.username);
        info!("Session ID: {}", ready.session_id);
    }

    /// Called when a message is created (@ mentions).
    async fn message_create(&self, ctx: Context, message: Message) {
        // Ignore messages from bots
        if message.is_from_bot() {
            return;
        }

        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!(
            "Received message from {}: {}",
            message
                .author
                .as_ref()
                .map(|a| a.username.as_deref().unwrap_or("Unknown"))
                .unwrap_or("Unknown"),
            content
        );

        // Respond to specific commands
        let response = match content.trim().to_lowercase().as_str() {
            "!ping" => Some("Pong! 🏓".to_string()),
            "!hello" => Some("Hello there! 👋".to_string()),
            "!help" => Some(
                "Available commands:\n• !ping - Test bot responsiveness\n• !hello - Get a greeting\n• !info - Get bot information"
                    .to_string(),
            ),
            "!info" => Some(
                "I'm a simple QQ Guild bot built with BotRS! 🤖\nWritten in Rust for performance and safety."
                    .to_string(),
            ),
            _ => {
                // Echo back messages that mention the bot
                if content.contains("bot") || content.contains("机器人") {
                    Some(format!("You mentioned me! You said: {}", content))
                } else {
                    None
                }
            }
        };

        // Send response if we have one
        if let Some(response_text) = response {
            // Try to reply using the message's reply method
            match message.reply(&ctx.api, &ctx.token, &response_text).await {
                Ok(_) => info!("Successfully sent reply"),
                Err(e) => warn!("Failed to send reply: {}", e),
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
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,simple_bot=info")
        .init();

    info!("Starting simple bot example...");

    // Get credentials from environment variables or command line args
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .or_else(|_| std::env::args().nth(1).ok_or("Missing app_id"))
        .expect("Please provide QQ_BOT_APP_ID environment variable or as first argument");

    let secret = std::env::var("QQ_BOT_SECRET")
        .or_else(|_| std::env::args().nth(2).ok_or("Missing secret"))
        .expect("Please provide QQ_BOT_SECRET environment variable or as second argument");

    // Create token
    let token = Token::new(app_id, secret);

    // Validate token
    if let Err(e) = token.validate() {
        panic!("Invalid token: {}", e);
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive guild messages and direct messages
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = SimpleHandler;

    // Create client
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

//! C2C Reply Text
//!
//! This example demonstrates how to create a bot that responds to C2C (client-to-client) messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::{C2CReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to C2C messages.
struct C2CReplyHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received C2C message: {}", content);

        // Create reply content.
        let reply_content = format!("我收到了你的消息：{content}");

        match session.reply(reply_content).await {
            Ok(response) => {
                info!("Successfully sent C2C message reply");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send C2C message reply: {}", e),
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

    info!("Starting C2C reply text example...");

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

    // Set up intents - we want to receive public messages (C2C messages)
    let intents = Intents::new().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = C2CReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

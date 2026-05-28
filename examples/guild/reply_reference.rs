//! Guild Reply Reference
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with message references (reply to specific messages).

#[path = "../common/mod.rs"]
mod common;

use botrs::{
    ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token,
    models::message::Reference,
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with message references.
struct ReferenceReplyHandler;

#[async_trait::async_trait]
impl EventHandler for ReferenceReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received message: {}", content);

        let message_id = match &message.id {
            Some(id) => id,
            None => {
                warn!("Message has no id");
                return;
            }
        };

        // Create message reference.
        let message_reference = Reference {
            message_id: Some(message_id.clone()),
            ignore_get_message_error: None,
        };

        // Send message with reference.
        let params = botrs::models::message::MessageParams {
            content: Some("<emoji:4>这是一条引用消息".to_string()),
            message_reference: Some(message_reference),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(_) => info!("Successfully sent message with reference"),
            Err(e) => warn!("Failed to send message with reference: {}", e),
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

    info!("Starting guild reply reference example...");

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
    let handler = ReferenceReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

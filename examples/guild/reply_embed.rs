//! Guild Reply Embed
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with embed messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::{
    ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token,
    models::message::{Embed, EmbedField},
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with embed messages.
struct EmbedReplyHandler;

#[async_trait::async_trait]
impl EventHandler for EmbedReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = &message.content;

        info!("Received message: {}", content);

        // Create an embed message.
        let embed = Embed {
            title: Some("embed消息".to_string()),
            prompt: "消息透传显示".to_string(),
            fields: Some(vec![
                EmbedField {
                    name: Some("<@!1234>hello world".to_string()),
                    ..Default::default()
                },
                EmbedField {
                    name: Some("<@!1234>hello world".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        match session.send_embed_message(embed).await {
            Ok(_) => info!("Successfully sent embed message"),
            Err(e) => warn!("Failed to send embed message: {}", e),
        }

        // For embeds, use the params path because `session.reply` sends text.
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

    info!("Starting guild reply embed example...");

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
    let handler = EmbedReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

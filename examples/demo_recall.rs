//! Demo: Message Recall
//!
//! This example demonstrates how to create a bot that sends a message and then
//! immediately recalls (deletes) it. It's equivalent to the Python demo_recall.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{init_logging, Config};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions and recalls messages.
struct RecallHandler;

#[async_trait::async_trait]
impl EventHandler for RecallHandler {
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

        info!("Received message: {}", content);

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Send a reply message first (equivalent to message.reply in Python)
        let reply_result = message.reply(&ctx.api, &ctx.token, &reply_content).await;

        match reply_result {
            Ok(response) => {
                info!("Successfully sent reply message");

                // Extract message ID from response
                if let Some(message_id) = response.id {
                    // Get channel ID
                    let channel_id = match &message.channel_id {
                        Some(id) => id,
                        None => {
                            warn!("Original message has no channel_id");
                            return;
                        }
                    };

                    info!(
                        "Attempting to recall message {} in channel {}",
                        message_id, channel_id
                    );

                    // Recall (delete) the message we just sent (equivalent to api.recall_message)
                    match ctx
                        .recall_message(channel_id, &message_id, true) // hidetip=True
                        .await
                    {
                        Ok(_) => info!("Successfully recalled message"),
                        Err(e) => warn!("Failed to recall message: {}", e),
                    }
                } else {
                    warn!("Reply response did not contain message ID");
                }
            }
            Err(e) => warn!("Failed to send reply message: {}", e),
        }
    }

    /// Called when a message is deleted.
    async fn message_delete(&self, _ctx: Context, message: Message) {
        info!("Message deleted: {:?}", message.id);
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

    info!("Starting message recall demo...");

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
    let handler = RecallHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

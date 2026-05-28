//! Direct Reply
//!
//! This example demonstrates how to create a bot that responds to direct messages
//! and can create DM sessions.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::DirectMessageToCreate;
use botrs::{
    ChannelReplySession, Client, DirectReplySession, EventHandler, Intents, ReadySession, Token,
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to direct messages and can create DM sessions.
struct DmsReplyHandler;

#[async_trait::async_trait]
impl EventHandler for DmsReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a direct message is created.
    async fn direct_message_create(&self, mut session: DirectReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received direct message: {}", content);

        // Get bot name from the bot info if available
        let bot_name = session
            .bot_info()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的私信了: {content}");

        match session.reply(reply_content).await {
            Ok(_) => info!("Successfully replied to direct message"),
            Err(e) => warn!("Failed to reply to direct message: {}", e),
        }
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, session: ChannelReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received @ message: {}", content);

        // Check if the message contains "/私信" to trigger DM creation
        if content.contains("/私信") {
            // Get required IDs
            let guild_id = match &message.guild_id {
                Some(id) => id,
                None => {
                    warn!("Message has no guild_id");
                    return;
                }
            };

            let user_id = match &message.author {
                Some(author) => match &author.id {
                    Some(id) => id,
                    None => {
                        warn!("Message author has no id");
                        return;
                    }
                },
                None => {
                    warn!("Message has no author");
                    return;
                }
            };

            info!(
                "Creating DM session for user {} in guild {}",
                user_id, guild_id
            );

            let dm = DirectMessageToCreate::new(guild_id, user_id);
            match session.create_direct_message(&dm).await {
                Ok(dms_payload) => {
                    info!("Successfully created DM session");
                    info!("DMS Payload: {:?}", dms_payload);

                    let dm_guild_id = if dms_payload.guild_id.is_empty() {
                        guild_id
                    } else {
                        dms_payload.guild_id.as_str()
                    };

                    // Send a DM using the created session
                    let params = botrs::models::message::DirectMessageParams::new_text("hello");

                    match session.send_direct_message(dm_guild_id, params).await {
                        Ok(_) => info!("Successfully sent DM via created session"),
                        Err(e) => warn!("Failed to send DM via created session: {}", e),
                    }
                }
                Err(e) => warn!("Failed to create DM session: {}", e),
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

    info!("Starting direct reply example...");

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

    // Set up intents - we want both direct messages and public guild messages
    let intents = Intents::new()
        .with_direct_message()
        .with_public_guild_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = DmsReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

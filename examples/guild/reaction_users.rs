//! Guild Reaction Users
//!
//! This example demonstrates how to create a bot that gets users who reacted to a message.

#[path = "../common/mod.rs"]
mod common;

use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that gets reaction users when receiving @ messages.
struct GetReactionUsersHandler;

#[async_trait::async_trait]
impl EventHandler for GetReactionUsersHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, session: ChannelReplySession) {
        let mut users: Vec<botrs::User> = Vec::new();
        let mut cookie = String::new();

        // Example channel_id and message_id - these would need to be actual values
        let channel_id = "CHANNEL_ID_XXXXXX";
        let message_id = "MESSAGE_ID_XXXXXX";
        let reaction_type = botrs::models::emoji::EmojiType::System; // System emoji
        let emoji_id = "4"; // Emoji ID

        loop {
            // Get reaction users (equivalent to self.api.get_reaction_users)
            let cookie_param = if cookie.is_empty() {
                None
            } else {
                Some(cookie.as_str())
            };

            match session
                .get_reaction_users(
                    channel_id,
                    message_id,
                    reaction_type,
                    emoji_id,
                    cookie_param,
                    None, // Use default limit of 20
                )
                .await
            {
                Ok(reaction_users) => {
                    if reaction_users.users.is_empty() {
                        break;
                    }

                    // Extend users list
                    users.extend(reaction_users.users.clone());

                    // Check if we've reached the end
                    if reaction_users.is_end {
                        break;
                    } else {
                        cookie = reaction_users.cookie.unwrap_or_default();
                    }
                }
                Err(e) => {
                    warn!("Failed to get reaction users: {}", e);
                    break;
                }
            }
        }

        // Log results.
        info!("Total users found: {}", users.len());
        for user in users {
            info!("User: {}", user.username);
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

    info!("Starting guild reaction users example...");

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
    let handler = GetReactionUsersHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

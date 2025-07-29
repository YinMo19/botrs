//! Demo: Get Reaction Users
//!
//! This example demonstrates how to create a bot that gets users who reacted to a message.
//! It's equivalent to the Python demo_get_reaction_users.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that gets reaction users when receiving @ messages.
struct GetReactionUsersHandler;

#[async_trait::async_trait]
impl EventHandler for GetReactionUsersHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, _ctx: Context, _message: Message) {
        let _users: Vec<botrs::models::User> = Vec::new();
        let _cookie = String::new();

        // Example channel_id and message_id - these would need to be actual values
        let _channel_id = "2568610";
        let _message_id = "088de19cbeb883e7e97110a2e39c0138d80d48acfc879406";
        let _reaction_type = 1; // Reaction type
        let _emoji_id = "4"; // Emoji ID

        // TODO: Get reaction users (equivalent to self.api.get_reaction_users)
        // This API is not yet implemented in the Rust version
        warn!("get_reaction_users API is not yet implemented");
        /*
        loop {
            // Get reaction users (equivalent to self.api.get_reaction_users)
            let cookie_param = if cookie.is_empty() {
                None
            } else {
                Some(&cookie)
            };

            match ctx
                .api
                .get_reaction_users(
                    &ctx.token,
                    channel_id,
                    message_id,
                    reaction_type,
                    emoji_id,
                    cookie_param,
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

        // Log results (equivalent to Python print statements)
        info!("Total users found: {}", users.len());
        for user in users {
            if let Some(username) = user.username {
                info!("User: {}", username);
            }
        }
        */
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

    info!("Starting get reaction users demo...");

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
    let handler = GetReactionUsersHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

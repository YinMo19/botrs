//! Demo: Announce
//!
//! This example demonstrates how to create a bot that manages announcements.
//! It's equivalent to the Python demo_announce.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that manages announcements.
struct AnnounceHandler;

#[async_trait::async_trait]
impl EventHandler for AnnounceHandler {
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

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        info!("{}receive message {}", bot_name, content);

        // Get channel ID for operations
        let channel_id = match &message.channel_id {
            Some(channel_id) => channel_id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        // Send acknowledgment message first (equivalent to self.api.post_message)
        let ack_content = format!("command received: {content}");
        let params = botrs::models::message::MessageParams {
            content: Some(ack_content),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Successfully sent acknowledgment message"),
            Err(e) => warn!("Failed to send acknowledgment message: {}", e),
        }

        // Get guild ID for announcement operations
        let _guild_id = match &message.guild_id {
            Some(guild_id) => guild_id,
            None => {
                warn!("Message has no guild_id");
                return;
            }
        };

        // Handle referenced message for announcement operations
        let _referenced_message_id = match &message.message_reference {
            Some(reference) => match &reference.message_id {
                Some(message_id) => message_id,
                None => {
                    warn!("Message reference has no message_id");
                    return;
                }
            },
            None => {
                warn!("No message reference found for announcement operation");
                return;
            }
        };

        // Handle different announcement commands
        if content.contains("/建公告") {
            // Create announcement (equivalent to self.api.create_announce)
            match ctx
                .api
                .create_announce(&ctx.token, _guild_id, channel_id, _referenced_message_id)
                .await
            {
                Ok(result) => {
                    info!("Successfully created announcement: {:?}", result);
                }
                Err(e) => {
                    warn!("Failed to create announcement: {}", e);
                }
            }
        } else if content.contains("/删公告") {
            // Delete announcement (equivalent to self.api.delete_announce)
            match ctx
                .api
                .delete_announce(&ctx.token, _guild_id, _referenced_message_id)
                .await
            {
                Ok(result) => {
                    info!("Successfully deleted announcement: {:?}", result);
                }
                Err(e) => {
                    warn!("Failed to delete announcement: {}", e);
                }
            }
        } else if content.contains("/设置推荐子频道") {
            // Create recommended channel announcement (equivalent to self.api.create_recommend_announce)
            let channel_list = vec![botrs::models::announce::RecommendChannel::new(
                channel_id.clone(),
                Some("introduce".to_string()),
            )];

            match ctx
                .api
                .create_recommend_announce(
                    &ctx.token,
                    _guild_id,
                    botrs::models::announce::AnnouncesType::Member,
                    channel_list,
                )
                .await
            {
                Ok(result) => {
                    info!("Successfully created recommend announcement: {:?}", result);
                }
                Err(e) => {
                    warn!("Failed to create recommend announcement: {}", e);
                }
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

    info!("Starting announce demo...");

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
    let handler = AnnounceHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

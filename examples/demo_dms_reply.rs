//! Demo: DMS Reply
//!
//! This example demonstrates how to create a bot that responds to direct messages
//! and can create DM sessions. It's equivalent to the Python demo_dms_reply.py example.

mod common;

use botrs::{Client, Context, DirectMessage, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to direct messages and can create DM sessions.
struct DmsReplyHandler;

#[async_trait::async_trait]
impl EventHandler for DmsReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a direct message is created.
    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received direct message: {}", content);

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        // Get guild ID for DM session
        let guild_id = match &message.guild_id {
            Some(id) => id,
            None => {
                warn!("Direct message has no guild_id");
                return;
            }
        };

        let reply_content = format!("机器人{}收到你的私信了: {}", bot_name, content);

        // Reply to the direct message using new API
        let params = botrs::models::message::DirectMessageParams {
            content: Some(reply_content),
            msg_id: message.id.clone(),
            ..Default::default()
        };

        match ctx
            .api
            .post_dms_with_params(&ctx.token, guild_id, params)
            .await
        {
            Ok(_) => info!("Successfully replied to direct message"),
            Err(e) => warn!("Failed to reply to direct message: {}", e),
        }
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, ctx: Context, message: Message) {
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

            // Create DM session (equivalent to api.create_dms)
            match ctx.api.create_dms(&ctx.token, guild_id, user_id).await {
                Ok(dms_payload) => {
                    info!("Successfully created DM session");
                    info!("DMS Payload: {:?}", dms_payload);

                    // Extract guild_id from the DMS payload
                    let dm_guild_id = dms_payload
                        .get("guild_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or(guild_id);

                    // Send a DM using the created session
                    let params = botrs::models::message::DirectMessageParams::new_text("hello");

                    match ctx
                        .api
                        .post_dms_with_params(&ctx.token, dm_guild_id, params)
                        .await
                    {
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

    info!("Starting DMS reply demo...");

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

    // Set up intents - we want both direct messages and public guild messages
    // This is equivalent to: intents = botpy.Intents(direct_message=True, public_guild_messages=True)
    let intents = Intents::default()
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

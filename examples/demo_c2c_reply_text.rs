//! Demo: C2C Reply Text
//!
//! This example demonstrates how to create a bot that responds to C2C (client-to-client) messages.
//! It's equivalent to the Python demo_c2c_reply_text.py example.

mod common;

use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to C2C messages.
struct C2CReplyHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received C2C message: {}", content);

        // Get user OpenID from the author
        let user_openid = match &message.author {
            Some(author) => match &author.user_openid {
                Some(openid) => openid,
                None => {
                    warn!("C2C message author has no user_openid");
                    return;
                }
            },
            None => {
                warn!("C2C message has no author");
                return;
            }
        };

        // Get message ID for reply
        let msg_id = message.id.as_deref();

        // Create reply content (equivalent to Python version)
        let reply_content = format!("我收到了你的消息：{}", content);

        // Send C2C message using new API (equivalent to message._api.post_c2c_message)
        let params = botrs::models::message::C2CMessageParams {
            msg_type: 0,
            content: Some(reply_content),
            msg_id: msg_id.map(|s| s.to_string()),
            ..Default::default()
        };

        match ctx
            .api
            .post_c2c_message_with_params(&ctx.token, user_openid, params)
            .await
        {
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

    info!("Starting C2C reply text demo...");

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

    // Set up intents - we want to receive public messages (C2C messages)
    // This is equivalent to: intents = botpy.Intents(public_messages=True)
    let intents = Intents::default().with_public_messages();

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

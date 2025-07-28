//! Demo: Group Reply Text
//!
//! This example demonstrates how to create a bot that responds to group messages.
//! It's equivalent to the Python demo_group_reply_text.py example.

mod common;

use botrs::{Client, Context, EventHandler, GroupMessage, Intents, Ready, Token};
use common::{init_logging, Config};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to group @ mentions.
struct GroupReplyHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a group @ message is created.
    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received group message: {}", content);

        // Get group OpenID
        let group_openid = match &message.group_openid {
            Some(openid) => openid,
            None => {
                warn!("Group message has no group_openid");
                return;
            }
        };

        // Get message ID for reply
        let msg_id = message.id.as_deref();

        // Create reply content (equivalent to Python version)
        let reply_content = format!("收到了消息：{}", content);

        // Send group message using API (equivalent to message._api.post_group_message)
        match ctx
            .api
            .post_group_message(
                &ctx.token,
                group_openid,
                Some(0),              // msg_type: 0 for text message
                Some(&reply_content), // content
                None,                 // embed
                None,                 // ark
                None,                 // message_reference
                None,                 // media
                msg_id,               // msg_id for reply
                None,                 // msg_seq
                None,                 // event_id
                None,                 // markdown
                None,                 // keyboard
            )
            .await
        {
            Ok(response) => {
                info!("Successfully sent group message reply");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send group message reply: {}", e),
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

    info!("Starting group reply text demo...");

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

    // Set up intents - we want to receive public messages (group messages)
    // This is equivalent to: intents = botpy.Intents(public_messages=True)
    let intents = Intents::default().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = GroupReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

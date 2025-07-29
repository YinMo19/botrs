//! Demo: AT Reply with Embed Messages
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with embed messages. It's equivalent to the Python demo_at_reply_embed.py example.

mod common;

use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token,
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

        // Create an embed message (equivalent to Python version)
        let embed = Embed {
            title: Some("embed消息".to_string()),
            description: None,
            url: None,
            timestamp: None,
            color: Some(0x00ff00), // Green color
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: Some(vec![
                EmbedField {
                    name: Some("<@!1234>hello world".to_string()),
                    value: Some("第一个字段".to_string()),
                    inline: Some(false),
                },
                EmbedField {
                    name: Some("<@!1234>hello world".to_string()),
                    value: Some("第二个字段".to_string()),
                    inline: Some(false),
                },
            ]),
        };

        // Send embed message using api.post_message (equivalent to Python version)
        let channel_id = match &message.channel_id {
            Some(id) => id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        let params = botrs::models::message::MessageParams {
            embed: Some(embed),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Successfully sent embed message"),
            Err(e) => warn!("Failed to send embed message: {}", e),
        }

        // Alternative: Reply using message.reply with embed
        // This is equivalent to the commented Python line: await message.reply(embed=embed)
        // However, the current reply method only supports text content
        // We would need to extend it to support embeds, or use the API directly as above
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

    info!("Starting AT reply embed demo...");

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
    let handler = EmbedReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

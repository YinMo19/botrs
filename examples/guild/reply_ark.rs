//! Guild Reply ARK
//!
//! This example demonstrates how to create a bot that responds to @ mentions with ARK messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Ark, ArkKv};
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with ARK messages.
struct AtReplyArkHandler;

#[async_trait::async_trait]
impl EventHandler for AtReplyArkHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        // Create ARK payload.
        let ark_payload = Ark {
            template_id: Some(37),
            kv: Some(vec![
                ArkKv {
                    key: Some("#METATITLE#".to_string()),
                    value: Some("通知提醒".to_string()),
                    obj: None,
                },
                ArkKv {
                    key: Some("#PROMPT#".to_string()),
                    value: Some("标题".to_string()),
                    obj: None,
                },
                ArkKv {
                    key: Some("#TITLE#".to_string()),
                    value: Some("标题".to_string()),
                    obj: None,
                },
                ArkKv {
                    key: Some("#METACOVER#".to_string()),
                    value: Some(
                        "https://vfiles.gtimg.cn/vupload/20211029/bf0ed01635493790634.jpg"
                            .to_string(),
                    ),
                    obj: None,
                },
            ]),
        };

        // Send message with ARK payload.
        let params = botrs::models::message::MessageParams {
            ark: Some(ark_payload),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(_) => info!("Successfully sent ARK message"),
            Err(e) => warn!("Failed to send ARK message: {}", e),
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

    info!("Starting guild reply ARK example...");

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
    let handler = AtReplyArkHandler;

    // Create client with caching enabled to store bot info
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

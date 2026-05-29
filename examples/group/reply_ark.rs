//! Group Reply ARK
//!
//! This example demonstrates how to create a bot that responds to group messages with ARK messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Ark, ArkKv};
use botrs::{Client, EventHandler, GroupReplySession, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to group messages with ARK messages.
struct GroupReplyArkHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyArkHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a group @ message is created.
    async fn group_message_create(&self, mut session: GroupReplySession) {
        let message = session.message().clone();
        info!("Received group message: {}", message.content);

        // Create ARK payload. Template variables depend on the configured template_id.
        let ark_payload = Ark {
            template_id: Some(37),
            kv: Some(vec![
                ArkKv {
                    key: Some("#METATITLE#".to_string()),
                    value: Some("通知提醒".to_string()),
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

        match session.send_ark_message(ark_payload).await {
            Ok(response) => {
                info!("Successfully sent group ARK message");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send group ARK message: {}", e),
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

    info!("Starting group reply ARK example...");

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

    // Set up intents - we want to receive public messages (group messages)
    let intents = Intents::new().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = GroupReplyArkHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

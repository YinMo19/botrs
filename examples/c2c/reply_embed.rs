//! C2C Reply Embed
//!
//! Replies to C2C messages with an embed payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Embed, EmbedField};
use botrs::{C2CReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyEmbedHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyEmbedHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let embed = Embed {
            title: Some("C2C Embed".to_string()),
            description: Some("This embed was sent to a C2C conversation.".to_string()),
            prompt: "C2C embed message".to_string(),
            fields: Some(vec![EmbedField {
                name: Some("builder".to_string()),
                value: Some("C2CMessageParams".to_string()),
            }]),
            ..Default::default()
        };

        match session.send_embed_message(embed).await {
            Ok(response) => info!("Successfully sent C2C embed message: {:?}", response),
            Err(e) => warn!("Failed to send C2C embed message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting C2C reply embed example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, C2CReplyEmbedHandler, true)?;
    client.start().await?;
    Ok(())
}

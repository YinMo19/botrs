//! C2C Reply Keyboard
//!
//! Replies to C2C messages with Markdown plus a keyboard template.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::KeyboardPayload;
use botrs::{C2CReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyKeyboardHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyKeyboardHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let keyboard = KeyboardPayload {
            content: serde_json::json!({ "id": "62" }),
        };

        match session
            .send_keyboard_message("# C2C Keyboard\n\n点击下方按钮继续。", keyboard)
            .await
        {
            Ok(response) => info!("Successfully sent C2C keyboard message: {:?}", response),
            Err(e) => warn!("Failed to send C2C keyboard message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting C2C reply keyboard example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, C2CReplyKeyboardHandler, true)?;
    client.start().await?;
    Ok(())
}

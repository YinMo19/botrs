//! Group Reply Keyboard
//!
//! Replies to QQ group messages with Markdown plus a keyboard template.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{GroupMessageParams, KeyboardPayload, MarkdownPayload};
use botrs::{Client, EventHandler, GroupReplySession, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GroupReplyKeyboardHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyKeyboardHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn group_message_create(&self, mut session: GroupReplySession) {
        let markdown = MarkdownPayload {
            content: Some("# Group Keyboard\n\n点击下方按钮继续。".to_string()),
            ..Default::default()
        };
        let keyboard = KeyboardPayload {
            content: serde_json::json!({ "id": "62" }),
        };

        let params = GroupMessageParams {
            msg_type: 2,
            markdown: Some(markdown),
            keyboard: Some(keyboard),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(response) => info!("Successfully sent group keyboard message: {:?}", response),
            Err(e) => warn!("Failed to send group keyboard message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting group reply keyboard example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, GroupReplyKeyboardHandler, true)?;
    client.start().await?;
    Ok(())
}

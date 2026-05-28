//! C2C Reply Markdown
//!
//! Replies to C2C messages with a Markdown payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{C2CMessageParams, MarkdownPayload};
use botrs::{C2CReplySession, Client, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyMarkdownHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyMarkdownHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let message = session.message().clone();
        let content = message.content.as_deref().unwrap_or_default();
        let markdown = MarkdownPayload {
            content: Some(format!(
                "# C2C Markdown\n\n收到单聊消息：{}\n\n- 使用 `C2CMessageParams`\n- `msg_type = 2`",
                content
            )),
            ..Default::default()
        };

        let params = C2CMessageParams {
            msg_type: 2,
            markdown: Some(markdown),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(response) => info!("Successfully sent C2C markdown message: {:?}", response),
            Err(e) => warn!("Failed to send C2C markdown message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting C2C reply markdown example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, C2CReplyMarkdownHandler, true)?;
    client.start().await?;
    Ok(())
}

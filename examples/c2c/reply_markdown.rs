//! C2C Reply Markdown
//!
//! Replies to C2C messages with a Markdown payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{C2CMessageParams, MarkdownPayload};
use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyMarkdownHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyMarkdownHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        let Some(user_openid) = message
            .author
            .as_ref()
            .and_then(|author| author.user_openid.as_deref())
        else {
            warn!("C2C message has no user_openid");
            return;
        };

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
            msg_id: message.id.clone(),
            event_id: message.event_id.clone(),
            ..Default::default()
        };

        match ctx.send_c2c_message(user_openid, params).await {
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

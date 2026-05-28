//! C2C Reply Keyboard
//!
//! Replies to C2C messages with Markdown plus a keyboard template.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{C2CMessageParams, KeyboardPayload, MarkdownPayload};
use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct C2CReplyKeyboardHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyKeyboardHandler {
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

        let markdown = MarkdownPayload {
            content: Some("# C2C Keyboard\n\n点击下方按钮继续。".to_string()),
            ..Default::default()
        };
        let keyboard = KeyboardPayload {
            content: serde_json::json!({ "id": "62" }),
        };

        let params = C2CMessageParams {
            msg_type: 2,
            markdown: Some(markdown),
            keyboard: Some(keyboard),
            msg_id: message.id.clone(),
            event_id: message.event_id.clone(),
            ..Default::default()
        };

        match ctx.send_c2c_message(user_openid, params).await {
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

//! Group Reply Markdown
//!
//! Replies to QQ group messages with a Markdown payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{GroupMessageParams, MarkdownPayload};
use botrs::{Client, Context, EventHandler, GroupMessage, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GroupReplyMarkdownHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyMarkdownHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        let Some(group_openid) = message.group_openid.as_deref() else {
            warn!("Group message has no group_openid");
            return;
        };

        let content = message.content.as_deref().unwrap_or_default();
        info!("Received group message: {}", content);

        let markdown = MarkdownPayload {
            content: Some(format!(
                "# Group Markdown\n\n收到群消息：{}\n\n- 使用 `GroupMessageParams`\n- `msg_type = 2`",
                content
            )),
            ..Default::default()
        };

        let params = GroupMessageParams {
            msg_type: 2,
            markdown: Some(markdown),
            msg_id: message.id.clone(),
            event_id: message.event_id.clone(),
            ..Default::default()
        };

        match ctx.send_group_message(group_openid, params).await {
            Ok(response) => info!("Successfully sent group markdown message: {:?}", response),
            Err(e) => warn!("Failed to send group markdown message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting group reply markdown example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, GroupReplyMarkdownHandler, true)?;
    client.start().await?;
    Ok(())
}

//! Group Reply Markdown
//!
//! Replies to QQ group messages with a Markdown payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::{Client, EventHandler, GroupReplySession, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GroupReplyMarkdownHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyMarkdownHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn group_message_create(&self, mut session: GroupReplySession) {
        let message = session.message().clone();
        let content = message.content.as_str();
        info!("Received group message: {}", content);

        let markdown = format!(
            "# Group Markdown\n\n收到群消息：{}\n\n- 使用 `session.send_markdown_message`\n- 自动填充 `msg_type = 2`\n\n *MATH*: $\\displaystyle\\int_0^\\infty \\frac{{\\sin x}}{{x}} {{\\rm d}} x = \\frac{{\\pi}}{{2}}$",
            content
        );

        match session.send_markdown_message(markdown).await {
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

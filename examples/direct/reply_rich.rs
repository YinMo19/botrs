//! Direct Reply Rich
//!
//! Demonstrates rich direct-message replies. Send a DM containing `ark`,
//! `embed`, or `keyboard` to select that payload; any other text gets Markdown.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Ark, ArkKv, Embed, EmbedField, Keyboard};
use botrs::{Client, DirectReplySession, EventHandler, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct DirectReplyRichHandler;

#[async_trait::async_trait]
impl EventHandler for DirectReplyRichHandler {
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    async fn direct_message_create(&self, mut session: DirectReplySession) {
        let message = session.message().clone();
        let content = message.content.as_str();
        let result = if content.contains("ark") {
            session
                .send_ark_message(Ark {
                    template_id: Some(37),
                    kv: Some(vec![ArkKv {
                        key: Some("#METATITLE#".to_string()),
                        value: Some("Direct ARK".to_string()),
                        obj: None,
                    }]),
                })
                .await
        } else if content.contains("embed") {
            session
                .send_embed_message(Embed {
                    title: Some("Direct Embed".to_string()),
                    description: Some("This embed was sent as a direct message.".to_string()),
                    prompt: "Direct embed".to_string(),
                    fields: Some(vec![EmbedField {
                        name: Some("builder".to_string()),
                        value: Some("DirectMessageParams".to_string()),
                    }]),
                    ..Default::default()
                })
                .await
        } else if content.contains("keyboard") {
            session
                .send_keyboard_message(
                    "# Direct Keyboard\n\n点击下方按钮继续。",
                    Keyboard {
                        id: Some("62".to_string()),
                        ..Default::default()
                    },
                )
                .await
        } else {
            session
                .send_markdown_message(format!("# Direct Markdown\n\n收到私信：{content}"))
                .await
        };

        match result {
            Ok(response) => info!("Successfully sent rich direct message: {:?}", response),
            Err(e) => warn!("Failed to send rich direct message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting direct reply rich example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_direct_message();
    let mut client = Client::new(token, intents, DirectReplyRichHandler, true)?;
    client.start().await?;
    Ok(())
}

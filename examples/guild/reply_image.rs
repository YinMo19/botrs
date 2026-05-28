//! Guild Reply Image
//!
//! Replies to guild channel @ messages with text plus a remote image URL.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::MessageParams;
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GuildReplyImageHandler;

#[async_trait::async_trait]
impl EventHandler for GuildReplyImageHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        let Some(channel_id) = message.channel_id.as_deref() else {
            warn!("Message has no channel_id");
            return;
        };

        let params = MessageParams {
            content: Some("这是一条带远程图片 URL 的频道回复。".to_string()),
            image: Some("https://example.com/image.png".to_string()),
            msg_id: message.id.clone(),
            event_id: message.event_id.clone(),
            ..Default::default()
        };

        match ctx.send_message(channel_id, params).await {
            Ok(response) => info!("Successfully sent guild image message: {:?}", response),
            Err(e) => warn!("Failed to send guild image message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting guild reply image example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_guild_messages();
    let mut client = Client::new(token, intents, GuildReplyImageHandler, true)?;
    client.start().await?;
    Ok(())
}

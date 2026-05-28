//! Group Reply Embed
//!
//! Replies to QQ group messages with an embed payload.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::message::{Embed, EmbedField, GroupMessageParams};
use botrs::{Client, Context, EventHandler, GroupMessage, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct GroupReplyEmbedHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyEmbedHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        let Some(group_openid) = message.group_openid.as_deref() else {
            warn!("Group message has no group_openid");
            return;
        };

        let embed = Embed {
            title: Some("Group Embed".to_string()),
            description: Some("This embed was sent to a QQ group.".to_string()),
            prompt: "Group embed message".to_string(),
            fields: Some(vec![
                EmbedField {
                    name: Some("surface".to_string()),
                    value: Some("group".to_string()),
                },
                EmbedField {
                    name: Some("builder".to_string()),
                    value: Some("GroupMessageParams".to_string()),
                },
            ]),
            ..Default::default()
        };

        let params = GroupMessageParams {
            msg_type: 4,
            embed: Some(embed),
            msg_id: message.id.clone(),
            event_id: message.event_id.clone(),
            ..Default::default()
        };

        match ctx.send_group_message(group_openid, params).await {
            Ok(response) => info!("Successfully sent group embed message: {:?}", response),
            Err(e) => warn!("Failed to send group embed message: {}", e),
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    info!("Starting group reply embed example...");

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    token.validate()?;

    let intents = Intents::new().with_public_messages();
    let mut client = Client::new(token, intents, GroupReplyEmbedHandler, true)?;
    client.start().await?;
    Ok(())
}

//! Interaction Search
//!
//! Demonstrates replying to inline search interactions from a bot service.

#[path = "../common/mod.rs"]
mod common;

use botrs::models::prelude::*;
use botrs::{Client, Context, EventHandler, Intents, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

struct InteractionSearchHandler;

#[async_trait::async_trait]
impl EventHandler for InteractionSearchHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.data.data_type != Some(InteractionDataType::ChatInputSearch) {
            return;
        }

        let Some(interaction_id) = interaction.id.as_deref() else {
            warn!("interaction has no id");
            return;
        };

        let keyword = interaction.data.resolved.keyword.as_str();
        let response = SearchResponse {
            layouts: vec![SearchLayout {
                layout_type: SearchLayoutType::ImageText,
                action_type: SearchActionType::SendArk,
                title: format!("Search result for {keyword}"),
                records: vec![SearchRecord {
                    cover: "https://qzonestyle.gtimg.cn/qzone/qzact/act/external/qq-logo.png"
                        .to_string(),
                    title: "BotRS inline result".to_string(),
                    tips: "Generated from interaction_create".to_string(),
                    url: "https://www.qq.com".to_string(),
                }],
            }],
        };

        if let Err(err) = ctx.put_interaction(interaction_id, &response).await {
            warn!("put interaction failed: {}", err);
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1),
        env::args().nth(2),
    )?;

    let token = Token::new(config.bot.app_id, config.bot.secret);
    let intents = Intents::new().with_interaction();
    let mut client = Client::new(token, intents, InteractionSearchHandler, config.bot.sandbox)?;

    client.start().await?;
    Ok(())
}

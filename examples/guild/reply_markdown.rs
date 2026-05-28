//! Guild Reply Markdown
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with markdown messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::{
    ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token,
    models::message::{MarkdownParam, MarkdownPayload, MessageParams},
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with markdown messages.
struct MarkdownReplyHandler;

fn markdown_template_params() -> MessageParams {
    let params = vec![
        MarkdownParam {
            key: Some("title".to_string()),
            values: Some(vec!["标题".to_string()]),
        },
        MarkdownParam {
            key: Some("content".to_string()),
            values: Some(vec![
                "为了成为一名合格的巫师，请务必阅读频道公告".to_string(),
                "藏馆黑色魔法书".to_string(),
            ]),
        },
    ];

    let markdown = MarkdownPayload {
        custom_template_id: Some("65".to_string()),
        params: Some(params),
        ..Default::default()
    };

    MessageParams {
        markdown: Some(markdown),
        ..Default::default()
    }
}

#[async_trait::async_trait]
impl EventHandler for MarkdownReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received message: {}", content);

        // Get bot name from the bot info if available
        let bot_name = session
            .bot_info()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // First send a regular reply.
        match session.reply(reply_content).await {
            Ok(_) => info!("Successfully sent regular reply"),
            Err(e) => warn!("Failed to send regular reply: {}", e),
        }

        // Send markdown by template.
        match session.send_message(markdown_template_params()).await {
            Ok(_) => info!("Successfully sent markdown message by template"),
            Err(e) => warn!("Failed to send markdown message by template: {}", e),
        }

        // Send raw markdown content through the session helper.
        match session
            .send_markdown_message("# 标题 \n## 简介很开心 \n内容")
            .await
        {
            Ok(_) => info!("Successfully sent markdown message by content"),
            Err(e) => warn!("Failed to send markdown message by content: {}", e),
        }
    }

    /// Called when an error occurs during event processing.
    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting guild reply markdown example...");

    // Load configuration with multiple fallback options
    let config = Config::load_with_fallback(
        Some("examples/config.toml"),
        env::args().nth(1), // app_id from command line
        env::args().nth(2), // secret from command line
    )?;

    info!("Configuration loaded successfully");

    // Create token
    let token = Token::new(config.bot.app_id, config.bot.secret);

    // Validate token
    if let Err(e) = token.validate() {
        panic!("Invalid token: {e}");
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public guild messages (@ mentions)
    let intents = Intents::new().with_public_guild_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = MarkdownReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

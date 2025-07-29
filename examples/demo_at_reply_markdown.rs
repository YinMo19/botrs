//! Demo: AT Reply with Markdown Messages
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with markdown messages. It's equivalent to the Python demo_at_reply_markdown.py example.

mod common;

use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token,
    models::message::{MarkdownParam, MarkdownPayload},
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with markdown messages.
struct MarkdownReplyHandler;

impl MarkdownReplyHandler {
    /// Send markdown message by template (equivalent to handle_send_markdown_by_template)
    async fn send_markdown_by_template(
        &self,
        ctx: &Context,
        channel_id: &str,
        _msg_id: Option<&str>,
    ) {
        // Create markdown parameters (equivalent to Python MessageMarkdownParams)
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
            template_id: None,
            custom_template_id: Some("65".to_string()),
            params: Some(params),
            content: None,
        };

        // Send markdown message using API
        // Send markdown message using new API (equivalent to api.post_markdown_message)
        let params = botrs::models::message::MessageParams {
            markdown: Some(markdown),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Successfully sent markdown message by template"),
            Err(e) => warn!("Failed to send markdown message by template: {}", e),
        }
    }

    /// Send markdown message by content (equivalent to handle_send_markdown_by_content)
    async fn send_markdown_by_content(
        &self,
        ctx: &Context,
        channel_id: &str,
        _msg_id: Option<&str>,
    ) {
        let markdown = MarkdownPayload {
            template_id: None,
            custom_template_id: None,
            params: None,
            content: Some("# 标题 \n## 简介很开心 \n内容".to_string()),
        };

        // Send markdown message using new API
        let params = botrs::models::message::MessageParams {
            markdown: Some(markdown),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Successfully sent markdown message by content"),
            Err(e) => warn!("Failed to send markdown message by content: {}", e),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for MarkdownReplyHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a message is created that mentions the bot.
    async fn message_create(&self, ctx: Context, message: Message) {
        // Get message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("Received message: {}", content);

        // Get bot name from the bot info if available
        let bot_name = ctx
            .bot_info
            .as_ref()
            .map(|info| info.username.as_str())
            .unwrap_or("Bot");

        let reply_content = format!("机器人{}收到你的@消息了: {}", bot_name, content);

        // First send a regular reply (equivalent to Python message.reply)
        match message.reply(&ctx.api, &ctx.token, &reply_content).await {
            Ok(_) => info!("Successfully sent regular reply"),
            Err(e) => warn!("Failed to send regular reply: {}", e),
        }

        // Get required IDs
        let channel_id = match &message.channel_id {
            Some(id) => id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        let msg_id = message.id.as_deref();

        // Send markdown by template
        self.send_markdown_by_template(&ctx, channel_id, msg_id)
            .await;

        // Send markdown by content
        self.send_markdown_by_content(&ctx, channel_id, msg_id)
            .await;
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

    info!("Starting AT reply markdown demo...");

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
        panic!("Invalid token: {}", e);
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public guild messages (@ mentions)
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

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

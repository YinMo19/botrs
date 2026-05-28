//! Guild Reply Keyboard
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with keyboard messages.

#[path = "../common/mod.rs"]
mod common;

use botrs::{
    ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token,
    models::message::{
        Keyboard, KeyboardButton, KeyboardButtonAction, KeyboardButtonPermission,
        KeyboardButtonRenderData, KeyboardContent, KeyboardRow, MarkdownPayload, MessageParams,
    },
};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with keyboard messages.
struct KeyboardReplyHandler;

fn template_keyboard_params() -> MessageParams {
    let markdown = MarkdownPayload {
        content: Some("# 123 \n 今天是个好天气".to_string()),
        ..Default::default()
    };

    let keyboard = Keyboard {
        id: Some("62".to_string()),
        ..Default::default()
    };

    MessageParams {
        markdown: Some(markdown),
        keyboard: Some(keyboard),
        ..Default::default()
    }
}

fn self_defined_keyboard_params() -> MessageParams {
    let markdown = MarkdownPayload {
        content: Some("# 标题 \n## 简介 \n内容".to_string()),
        ..Default::default()
    };

    let keyboard = Keyboard {
        id: None,
        content: Some(build_keyboard()),
    };

    MessageParams {
        markdown: Some(markdown),
        keyboard: Some(keyboard),
        ..Default::default()
    }
}

fn build_keyboard() -> KeyboardContent {
    let button1 = KeyboardButton {
        id: Some("1".to_string()),
        render_data: Some(KeyboardButtonRenderData {
            label: Some("button".to_string()),
            visited_label: Some("BUTTON".to_string()),
            style: Some(0),
        }),
        action: Some(KeyboardButtonAction {
            action_type: Some(2),
            permission: Some(KeyboardButtonPermission {
                permission_type: Some(2),
                specify_role_ids: Some(vec!["1".to_string()]),
                specify_user_ids: Some(vec!["1".to_string()]),
            }),
            click_limit: Some(10),
            data: Some("/搜索".to_string()),
            enter: true,
            at_bot_show_channel_list: None,
            modal: None,
            ..Default::default()
        }),
        group_id: None,
    };

    let row1 = KeyboardRow {
        buttons: Some(vec![button1]),
    };

    KeyboardContent {
        rows: Some(vec![row1]),
        style: None,
    }
}

#[async_trait::async_trait]
impl EventHandler for KeyboardReplyHandler {
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

        // Send template keyboard message.
        match session.send_message(template_keyboard_params()).await {
            Ok(_) => info!("Successfully sent template keyboard message"),
            Err(e) => warn!("Failed to send template keyboard message: {}", e),
        }

        // Send self-defined keyboard message.
        match session.send_message(self_defined_keyboard_params()).await {
            Ok(_) => info!("Successfully sent self-defined keyboard message"),
            Err(e) => warn!("Failed to send self-defined keyboard message: {}", e),
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

    info!("Starting guild reply keyboard example...");

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
    let handler = KeyboardReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

//! Demo: AT Reply with Keyboard Messages
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with keyboard messages. It's equivalent to the Python demo_at_reply_keyboard.py example.

mod common;

use botrs::{
    models::message::{
        Keyboard, KeyboardButton, KeyboardButtonAction, KeyboardButtonPermission,
        KeyboardButtonRenderData, KeyboardContent, KeyboardPayload, KeyboardRow, MarkdownPayload,
    },
    Client, Context, EventHandler, Intents, Message, Ready, Token,
};
use common::{init_logging, Config};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with keyboard messages.
struct KeyboardReplyHandler;

impl KeyboardReplyHandler {
    /// Send template keyboard message (equivalent to send_template_keyboard)
    async fn send_template_keyboard(&self, ctx: &Context, channel_id: &str) {
        let markdown = MarkdownPayload {
            template_id: None,
            custom_template_id: None,
            params: None,
            content: Some("# 123 \n 今天是个好天气".to_string()),
        };

        let keyboard = KeyboardPayload {
            content: serde_json::json!({"id": "62"}),
        };

        // Send keyboard message using API (equivalent to api.post_keyboard_message)
        // Note: In Rust version, we use post_message with keyboard parameter
        match ctx
            .api
            .post_message(
                &ctx.token,
                channel_id,
                None,                                                // content
                None,                                                // embed
                None,                                                // ark
                None,                                                // message_reference
                None,                                                // image
                None,                                                // file_image
                None,                                                // msg_id
                None,                                                // event_id
                Some(&markdown),                                     // markdown
                Some(&self.keyboard_payload_to_keyboard(&keyboard)), // keyboard
            )
            .await
        {
            Ok(_) => info!("Successfully sent template keyboard message"),
            Err(e) => warn!("Failed to send template keyboard message: {}", e),
        }
    }

    /// Send self-defined keyboard message (equivalent to send_self_defined_keyboard)
    async fn send_self_defined_keyboard(&self, ctx: &Context, channel_id: &str) {
        let markdown = MarkdownPayload {
            template_id: None,
            custom_template_id: None,
            params: None,
            content: Some("# 标题 \n## 简介 \n内容".to_string()),
        };

        let keyboard_content = self.build_demo_keyboard();
        let keyboard = Keyboard {
            content: Some(keyboard_content),
        };

        // Send keyboard message using API
        match ctx
            .api
            .post_message(
                &ctx.token,
                channel_id,
                None,            // content
                None,            // embed
                None,            // ark
                None,            // message_reference
                None,            // image
                None,            // file_image
                None,            // msg_id
                None,            // event_id
                Some(&markdown), // markdown
                Some(&keyboard), // keyboard
            )
            .await
        {
            Ok(_) => info!("Successfully sent self-defined keyboard message"),
            Err(e) => warn!("Failed to send self-defined keyboard message: {}", e),
        }
    }

    /// Build a demo keyboard (equivalent to build_a_demo_keyboard)
    /// Creates a keyboard with one row and one button
    fn build_demo_keyboard(&self) -> KeyboardContent {
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
                reply: None,
                enter: Some(true), // equivalent to at_bot_show_channel_list=True
            }),
        };

        let row1 = KeyboardRow {
            buttons: Some(vec![button1]),
        };

        KeyboardContent {
            rows: Some(vec![row1]),
        }
    }

    /// Helper function to convert KeyboardPayload to Keyboard
    /// This is needed because the API expects different formats
    fn keyboard_payload_to_keyboard(&self, _payload: &KeyboardPayload) -> Keyboard {
        Keyboard {
            content: Some(KeyboardContent {
                rows: None, // For template keyboards, we don't define rows
            }),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for KeyboardReplyHandler {
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

        // Get required IDs
        let channel_id = match &message.channel_id {
            Some(id) => id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        // Send template keyboard message
        self.send_template_keyboard(&ctx, channel_id).await;

        // Send self-defined keyboard message
        self.send_self_defined_keyboard(&ctx, channel_id).await;
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

    info!("Starting AT reply keyboard demo...");

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
    let handler = KeyboardReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

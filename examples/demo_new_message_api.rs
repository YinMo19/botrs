//! This example demonstrates the new message parameter API that provides a cleaner
//! interface for sending messages with fewer None parameters.

use botrs::{
    Client, EventHandler, Intents, Token,
    models::message::{
        C2CMessageParams, DirectMessageParams, Embed, EmbedField, GroupMessageParams,
        MarkdownPayload, MessageParams,
    },
};
use tracing::{info, warn};

mod common;

use common::{Config, init_logging};
use std::env;

/// Event handler that demonstrates the new message parameter API.
struct NewApiDemoHandler;

#[async_trait::async_trait]
impl EventHandler for NewApiDemoHandler {
    async fn message_create(&self, ctx: botrs::Context, message: botrs::Message) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if !content.starts_with("/demo") {
            return;
        }

        let channel_id = match &message.channel_id {
            Some(id) => id,
            None => {
                warn!("Received message without channel_id");
                return;
            }
        };

        // Parse command
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 2 {
            self.send_help_message(&ctx, channel_id).await;
            return;
        }

        match parts[1] {
            "text" => self.demo_text_message(&ctx, channel_id).await,
            "embed" => self.demo_embed_message(&ctx, channel_id).await,
            "reply" => self.demo_reply_message(&ctx, channel_id, &message.id).await,
            "markdown" => self.demo_markdown_message(&ctx, channel_id).await,
            "file" => self.demo_file_message(&ctx, channel_id).await,
            _ => self.send_help_message(&ctx, channel_id).await,
        }
    }

    async fn group_message_create(&self, ctx: botrs::Context, message: botrs::GroupMessage) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/demo group" {
            if let Some(group_openid) = &message.group_openid {
                self.demo_group_message(&ctx, group_openid).await;
            }
        }
    }

    async fn c2c_message_create(&self, ctx: botrs::Context, message: botrs::C2CMessage) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/demo c2c" {
            self.demo_c2c_message(&ctx, &message).await;
        }
    }

    async fn direct_message_create(&self, ctx: botrs::Context, message: botrs::DirectMessage) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/demo dm" {
            self.demo_direct_message(&ctx, &message.guild_id).await;
        }
    }
}

impl NewApiDemoHandler {
    async fn send_help_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let help_text = r#"**New Message API Demo Commands:**

• `/demo text` - Send a simple text message
• `/demo embed` - Send a message with embed
• `/demo reply` - Reply to your message
• `/demo markdown` - Send a markdown message
• `/demo file` - Send a message with file attachment

**For other message types:**
• `/demo group` - In group chats
• `/demo c2c` - In C2C chats
• `/demo dm` - In direct messages"#;

        let params = MessageParams::new_text(help_text);

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Sent help message"),
            Err(e) => warn!("Failed to send help message: {}", e),
        }
    }

    async fn demo_text_message(&self, ctx: &botrs::Context, channel_id: &str) {
        // Old way (still works but deprecated):
        // ctx.api.post_message(&ctx.token, channel_id, Some("Hello!"), None, None, ...).await

        // New way - much cleaner!
        let params = MessageParams::new_text("🚀 This is a simple text message using the new API!");

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Sent text message using new API"),
            Err(e) => warn!("Failed to send text message: {}", e),
        }
    }

    async fn demo_embed_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let embed = Embed {
            title: Some("New Message API Demo".to_string()),
            description: Some("This embed was sent using the new MessageParams API!".to_string()),
            color: Some(0x00ff00), // Green
            fields: Some(vec![
                EmbedField {
                    name: Some("Feature".to_string()),
                    value: Some("Cleaner API".to_string()),
                    inline: Some(true),
                },
                EmbedField {
                    name: Some("Benefit".to_string()),
                    value: Some("Less None parameters".to_string()),
                    inline: Some(true),
                },
            ]),
            ..Default::default()
        };

        // Using the new API with Default::default() for unused fields
        let params = MessageParams {
            content: Some("Check out this embed! 📊".to_string()),
            embed: Some(embed),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Sent embed message using new API"),
            Err(e) => warn!("Failed to send embed message: {}", e),
        }
    }

    async fn demo_reply_message(
        &self,
        ctx: &botrs::Context,
        channel_id: &str,
        message_id: &Option<String>,
    ) {
        if let Some(msg_id) = message_id {
            // Using the convenience method
            let params =
                MessageParams::new_text("This is a reply using the new API! 💬").with_reply(msg_id);

            match ctx
                .api
                .post_message_with_params(&ctx.token, channel_id, params)
                .await
            {
                Ok(_) => info!("Sent reply message using new API"),
                Err(e) => warn!("Failed to send reply message: {}", e),
            }
        }
    }

    async fn demo_markdown_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let markdown = MarkdownPayload {
            content: Some(
                "# Markdown Message\n\nThis message uses **markdown** formatting with the new API!"
                    .to_string(),
            ),
            ..Default::default()
        };

        let params = MessageParams {
            markdown: Some(markdown),
            ..Default::default()
        };

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Sent markdown message using new API"),
            Err(e) => warn!("Failed to send markdown message: {}", e),
        }
    }

    async fn demo_file_message(&self, ctx: &botrs::Context, channel_id: &str) {
        // Simulate a small image file (1x1 PNG)
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0x0F, 0x00, 0x00, 0x01, 0x00, 0x01, 0x5C, 0xCD, 0x90, 0x0C, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        // Using the convenience method for file images
        let params = MessageParams::new_text("Here's a file sent with the new API! 📎")
            .with_file_image(&png_data);

        match ctx
            .api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await
        {
            Ok(_) => info!("Sent file message using new API"),
            Err(e) => warn!("Failed to send file message: {}", e),
        }
    }

    async fn demo_group_message(&self, ctx: &botrs::Context, group_openid: &str) {
        // Group messages use GroupMessageParams
        let params = GroupMessageParams::new_text("Hello from the new Group Message API! 👥");

        match ctx
            .api
            .post_group_message_with_params(&ctx.token, group_openid, params)
            .await
        {
            Ok(_) => info!("Sent group message using new API"),
            Err(e) => warn!("Failed to send group message: {}", e),
        }
    }

    async fn demo_c2c_message(&self, ctx: &botrs::Context, message: &botrs::C2CMessage) {
        if let Some(user_openid) = message.author.as_ref().and_then(|a| a.user_openid.as_ref()) {
            // C2C messages use C2CMessageParams
            let params = C2CMessageParams::new_text("Hello from the new C2C Message API! 💬");

            match ctx
                .api
                .post_c2c_message_with_params(&ctx.token, user_openid, params)
                .await
            {
                Ok(_) => info!("Sent C2C message using new API"),
                Err(e) => warn!("Failed to send C2C message: {}", e),
            }
        }
    }

    async fn demo_direct_message(&self, ctx: &botrs::Context, guild_id: &Option<String>) {
        if let Some(guild_id) = guild_id {
            // Direct messages use DirectMessageParams
            let params = DirectMessageParams::new_text("Hello from the new Direct Message API! 📧");

            match ctx
                .api
                .post_dms_with_params(&ctx.token, guild_id, params)
                .await
            {
                Ok(_) => info!("Sent direct message using new API"),
                Err(e) => warn!("Failed to send direct message: {}", e),
            }
        }
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
        panic!("Invalid token: {e}");
    }

    info!("Token validated successfully");

    // Set up intents - we want to receive public guild messages (@ mentions)
    // This is equivalent to: intents = botpy.Intents(public_guild_messages=True)
    let intents = Intents::default().with_public_guild_messages();

    let mut client = Client::new(token, intents, NewApiDemoHandler, true)?;

    println!("🤖 New Message API Demo Bot is starting...");
    println!("💡 Try sending '/demo text' in a channel to see the new API in action!");
    println!("📚 Use '/demo' to see all available commands");

    // Start the bot
    client.start().await?;

    Ok(())
}

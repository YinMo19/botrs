//! Message Params API
//!
//! Demonstrates the shared parameter structs used by the message sending APIs.

use botrs::{
    Client, EventHandler, Intents, Token,
    models::message::{
        C2CMessageParams, DirectMessageParams, Embed, EmbedField, GroupMessageParams,
        MarkdownPayload, MessageParams,
    },
};
use tracing::{info, warn};

#[path = "../common/mod.rs"]
mod common;

use common::{Config, init_logging};
use std::env;

/// Event handler that demonstrates the message parameter API.
struct MessageParamsHandler;

#[async_trait::async_trait]
impl EventHandler for MessageParamsHandler {
    async fn message_create(&self, ctx: botrs::Context, message: botrs::Message) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if !content.starts_with("/params") {
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
            "text" => self.send_text_message(&ctx, channel_id).await,
            "embed" => self.send_embed_message(&ctx, channel_id).await,
            "reply" => self.send_reply_message(&ctx, channel_id, &message.id).await,
            "markdown" => self.send_markdown_message(&ctx, channel_id).await,
            "image" => self.send_image_message(&ctx, channel_id).await,
            _ => self.send_help_message(&ctx, channel_id).await,
        }
    }

    async fn group_message_create(&self, ctx: botrs::Context, message: botrs::GroupMessage) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params group"
            && let Some(group_openid) = &message.group_openid
        {
            self.send_group_message(&ctx, group_openid, &message).await;
        }
    }

    async fn c2c_message_create(&self, ctx: botrs::Context, message: botrs::C2CMessage) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params c2c" {
            self.send_c2c_message(&ctx, &message).await;
        }
    }

    async fn direct_message_create(&self, ctx: botrs::Context, message: botrs::Message) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params dm" {
            self.send_direct_message(&ctx, &message).await;
        }
    }
}

impl MessageParamsHandler {
    async fn send_help_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let help_text = r#"**Message Params Commands:**

• `/params text` - Send a simple text message
• `/params embed` - Send a message with embed
• `/params reply` - Reply to your message
• `/params markdown` - Send a markdown message
• `/params image` - Send a message with an image URL

**For other message types:**
• `/params group` - In group chats
• `/params c2c` - In C2C chats
• `/params dm` - In direct messages"#;

        let params = MessageParams::new_text(help_text);

        match ctx.send_message(channel_id, params).await {
            Ok(_) => info!("Sent help message"),
            Err(e) => warn!("Failed to send help message: {}", e),
        }
    }

    async fn send_text_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let params = MessageParams::new_text("This is a simple text message using MessageParams.");

        match ctx.send_message(channel_id, params).await {
            Ok(_) => info!("Sent text message using MessageParams"),
            Err(e) => warn!("Failed to send text message: {}", e),
        }
    }

    async fn send_embed_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let embed = Embed {
            title: Some("Message Params".to_string()),
            description: Some("This embed was sent using MessageParams.".to_string()),
            prompt: "Message Params".to_string(),
            fields: Some(vec![
                EmbedField {
                    name: Some("Destination".to_string()),
                    value: Some("Guild channel".to_string()),
                },
                EmbedField {
                    name: Some("Params".to_string()),
                    value: Some("MessageParams".to_string()),
                },
            ]),
            ..Default::default()
        };

        let params = MessageParams {
            content: Some("Check out this embed.".to_string()),
            embed: Some(embed),
            ..Default::default()
        };

        match ctx.send_message(channel_id, params).await {
            Ok(_) => info!("Sent embed message using MessageParams"),
            Err(e) => warn!("Failed to send embed message: {}", e),
        }
    }

    async fn send_reply_message(
        &self,
        ctx: &botrs::Context,
        channel_id: &str,
        message_id: &Option<String>,
    ) {
        if let Some(msg_id) = message_id {
            let params =
                MessageParams::new_text("This is a reply using MessageParams.").with_reply(msg_id);

            match ctx.send_message(channel_id, params).await {
                Ok(_) => info!("Sent reply message using MessageParams"),
                Err(e) => warn!("Failed to send reply message: {}", e),
            }
        }
    }

    async fn send_markdown_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let markdown = MarkdownPayload {
            content: Some("# Markdown Message\n\nThis message uses **MessageParams**.".to_string()),
            ..Default::default()
        };

        let params = MessageParams {
            markdown: Some(markdown),
            ..Default::default()
        };

        match ctx.send_message(channel_id, params).await {
            Ok(_) => info!("Sent markdown message using MessageParams"),
            Err(e) => warn!("Failed to send markdown message: {}", e),
        }
    }

    async fn send_image_message(&self, ctx: &botrs::Context, channel_id: &str) {
        let params = MessageParams {
            content: Some("Here is an image URL sent with MessageParams.".to_string()),
            image: Some("https://example.com/image.png".to_string()),
            ..Default::default()
        };

        match ctx.send_message(channel_id, params).await {
            Ok(_) => info!("Sent image URL message using MessageParams"),
            Err(e) => warn!("Failed to send image URL message: {}", e),
        }
    }

    async fn send_group_message(
        &self,
        ctx: &botrs::Context,
        group_openid: &str,
        message: &botrs::GroupMessage,
    ) {
        let mut params = GroupMessageParams {
            event_id: message.event_id.clone(),
            ..GroupMessageParams::new_text("Hello from GroupMessageParams.")
        };
        if let Some(message_id) = &message.id {
            params = params.with_reply(message_id);
        }

        match ctx.send_group_message(group_openid, params).await {
            Ok(_) => info!("Sent group message using GroupMessageParams"),
            Err(e) => warn!("Failed to send group message: {}", e),
        }
    }

    async fn send_c2c_message(&self, ctx: &botrs::Context, message: &botrs::C2CMessage) {
        if let Some(user_openid) = message.author.as_ref().and_then(|a| a.user_openid.as_ref()) {
            let mut params = C2CMessageParams {
                event_id: message.event_id.clone(),
                ..C2CMessageParams::new_text("Hello from C2CMessageParams.")
            };
            if let Some(message_id) = &message.id {
                params = params.with_reply(message_id);
            }

            match ctx.send_c2c_message(user_openid, params).await {
                Ok(_) => info!("Sent C2C message using C2CMessageParams"),
                Err(e) => warn!("Failed to send C2C message: {}", e),
            }
        }
    }

    async fn send_direct_message(&self, ctx: &botrs::Context, message: &botrs::Message) {
        if let Some(guild_id) = &message.guild_id {
            let mut params = DirectMessageParams {
                event_id: message.event_id.clone(),
                ..DirectMessageParams::new_text("Hello from DirectMessageParams.")
            };
            if let Some(message_id) = &message.id {
                params = params.with_reply(message_id);
            }

            match ctx.send_direct_message(guild_id, params).await {
                Ok(_) => info!("Sent direct message using DirectMessageParams"),
                Err(e) => warn!("Failed to send direct message: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting message params API example...");

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

    let intents = Intents::new()
        .with_public_guild_messages()
        .with_public_messages()
        .with_direct_message();

    let mut client = Client::new(token, intents, MessageParamsHandler, true)?;

    info!("Message params API example is starting...");
    info!("Try sending '/params text' in a channel, or '/params' for commands");

    // Start the bot
    client.start().await?;

    Ok(())
}

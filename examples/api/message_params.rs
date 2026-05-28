//! Message Params API
//!
//! Demonstrates the shared parameter structs used by the message sending APIs.

use botrs::{
    C2CReplySession, ChannelReplySession, Client, DirectReplySession, EventHandler,
    GroupReplySession, Intents, Token,
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

const HELP_TEXT: &str = r#"**Message Params Commands:**

• `/params text` - Send a simple text message
• `/params embed` - Send a message with embed
• `/params reply` - Reply to your message
• `/params markdown` - Send a markdown message
• `/params image` - Send a message with an image URL

**For other message types:**
• `/params group` - In group chats
• `/params c2c` - In C2C chats
• `/params dm` - In direct messages"#;

#[async_trait::async_trait]
impl EventHandler for MessageParamsHandler {
    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if !content.starts_with("/params") {
            return;
        }

        // Parse command
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 2 {
            let params = MessageParams::new_text(HELP_TEXT);

            match session.send_message(params).await {
                Ok(_) => info!("Sent help message"),
                Err(e) => warn!("Failed to send help message: {}", e),
            }
            return;
        }

        match parts[1] {
            "text" => {
                let params =
                    MessageParams::new_text("This is a simple text message using MessageParams.");

                match session.send_message(params).await {
                    Ok(_) => info!("Sent text message using MessageParams"),
                    Err(e) => warn!("Failed to send text message: {}", e),
                }
            }
            "embed" => {
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

                match session.send_message(params).await {
                    Ok(_) => info!("Sent embed message using MessageParams"),
                    Err(e) => warn!("Failed to send embed message: {}", e),
                }
            }
            "reply" => match session.reply("This is a reply using MessageParams.").await {
                Ok(_) => info!("Sent reply using MessageParams"),
                Err(e) => warn!("Failed to send reply: {}", e),
            },
            "markdown" => {
                let markdown = MarkdownPayload {
                    content: Some(
                        "# Markdown Message\n\nThis message uses **MessageParams**.".to_string(),
                    ),
                    ..Default::default()
                };

                let params = MessageParams {
                    markdown: Some(markdown),
                    ..Default::default()
                };

                match session.send_message(params).await {
                    Ok(_) => info!("Sent markdown message using MessageParams"),
                    Err(e) => warn!("Failed to send markdown message: {}", e),
                }
            }
            "image" => {
                let params = MessageParams {
                    content: Some("Here is an image URL sent with MessageParams.".to_string()),
                    image: Some("https://example.com/image.png".to_string()),
                    ..Default::default()
                };

                match session.send_message(params).await {
                    Ok(_) => info!("Sent image URL message using MessageParams"),
                    Err(e) => warn!("Failed to send image URL message: {}", e),
                }
            }
            _ => {
                let params = MessageParams::new_text(HELP_TEXT);

                match session.send_message(params).await {
                    Ok(_) => info!("Sent help message"),
                    Err(e) => warn!("Failed to send help message: {}", e),
                }
            }
        }
    }

    async fn group_message_create(&self, mut session: GroupReplySession) {
        let message = session.message().clone();
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params group" {
            let params = GroupMessageParams::new_text("Hello from GroupMessageParams.");

            match session.send_message(params).await {
                Ok(_) => info!("Sent group message using GroupMessageParams"),
                Err(e) => warn!("Failed to send group message: {}", e),
            }
        }
    }

    async fn c2c_message_create(&self, mut session: C2CReplySession) {
        let message = session.message().clone();
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params c2c" {
            let params = C2CMessageParams::new_text("Hello from C2CMessageParams.");

            match session.send_message(params).await {
                Ok(_) => info!("Sent C2C message using C2CMessageParams"),
                Err(e) => warn!("Failed to send C2C message: {}", e),
            }
        }
    }

    async fn direct_message_create(&self, mut session: DirectReplySession) {
        let message = session.message().clone();
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };

        if content == "/params dm" {
            let params = DirectMessageParams::new_text("Hello from DirectMessageParams.");

            match session.send_message(params).await {
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

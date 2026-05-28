//! Group Reply File
//!
//! This example demonstrates how to create a bot that responds to group messages with file uploads.

#[path = "../common/mod.rs"]
mod common;

use botrs::{Client, EventHandler, GroupReplySession, Intents, ReadySession, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to group messages with file uploads.
struct GroupReplyFileHandler;

#[async_trait::async_trait]
impl EventHandler for GroupReplyFileHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, session: ReadySession) {
        info!("robot 「{}」 on_ready!", session.event().user.username);
    }

    /// Called when a group @ message is created.
    async fn group_message_create(&self, mut session: GroupReplySession) {
        let _ = session.reply("uploading Arcaea pictures.").await;

        // File URL - this needs to be filled with an actual uploaded resource URL
        let file_url = "https://arcaea.lowiro.com/assets/character-card_en_Hikari@2x-UqTl1zuc.png"; // 这里需要填写上传的资源Url，夹带私货

        // Upload media file.
        let upload_media_result = session
            .post_file(
                1, // file_type: 1 for image, file type should match the actual file
                file_url, None, // srv_send_msg: Optional flag for server-side message sending
            )
            .await;

        let upload_media = match upload_media_result {
            Ok(media) => media,
            Err(e) => {
                warn!("Failed to upload group file: {}", e);
                return;
            }
        };

        info!("Successfully uploaded group file: {:?}", upload_media);

        // Send group message with media.
        let params = botrs::models::message::GroupMessageParams {
            msg_type: 7, // 7表示富媒体类型 (rich media type)
            media: Some(upload_media),
            ..Default::default()
        };

        match session.send_message(params).await {
            Ok(response) => {
                info!("Successfully sent group file message");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send group file message: {}", e),
        }

        // Can also construct the params and send the message using session.send...
        let _ = session.reply("Upload Successfully.").await;
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

    info!("Starting group reply file example...");

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

    // Set up intents - we want to receive public messages (group messages)
    let intents = Intents::new().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = GroupReplyFileHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

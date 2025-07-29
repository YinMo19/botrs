//! Demo: C2C Reply File
//!
//! This example demonstrates how to create a bot that responds to C2C messages with file uploads.
//! It's equivalent to the Python demo_c2c_reply_file.py example.

mod common;

use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};
use common::{Config, init_logging};
use std::env;
use tracing::{info, warn};

/// Event handler that responds to C2C messages with file uploads.
struct C2CReplyFileHandler;

#[async_trait::async_trait]
impl EventHandler for C2CReplyFileHandler {
    /// Called when the bot is ready and connected.
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("robot 「{}」 on_ready!", ready.user.username);
    }

    /// Called when a C2C message is created.
    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        // Get user OpenID from the author
        let user_openid = match &message.author {
            Some(author) => match &author.user_openid {
                Some(openid) => openid,
                None => {
                    warn!("C2C message author has no user_openid");
                    return;
                }
            },
            None => {
                warn!("C2C message has no author");
                return;
            }
        };

        // File URL - this needs to be filled with an actual uploaded resource URL
        let file_url = ""; // 这里需要填写上传的资源Url

        if file_url.is_empty() {
            warn!("file_url is empty, please provide a valid file URL");
            return;
        }

        // Upload media file (equivalent to message._api.post_c2c_file)
        let upload_media_result = ctx
            .api
            .post_c2c_file(
                &ctx.token,
                user_openid,
                1, // file_type: 1 for image, file type should match the actual file
                file_url,
                None, // srv_send_msg: Optional flag for server-side message sending
            )
            .await;

        let upload_media = match upload_media_result {
            Ok(media) => media,
            Err(e) => {
                warn!("Failed to upload C2C file: {}", e);
                return;
            }
        };

        info!("Successfully uploaded C2C file: {:?}", upload_media);

        // Get message ID for reply
        let msg_id = message.id.as_deref();

        // Convert Value to Media struct
        let media = match serde_json::from_value::<botrs::models::message::Media>(upload_media) {
            Ok(media) => media,
            Err(e) => {
                warn!("Failed to parse media response: {}", e);
                return;
            }
        };

        // Send C2C message with media (equivalent to message._api.post_c2c_message with media)
        let params = botrs::models::message::C2CMessageParams {
            msg_type: 7, // 7表示富媒体类型 (rich media type)
            msg_id: msg_id.map(|s| s.to_string()),
            media: Some(media),
            ..Default::default()
        };

        match ctx
            .api
            .post_c2c_message_with_params(&ctx.token, user_openid, params)
            .await
        {
            Ok(response) => {
                info!("Successfully sent C2C file message");
                info!("Response: {:?}", response);
            }
            Err(e) => warn!("Failed to send C2C file message: {}", e),
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

    info!("Starting C2C reply file demo...");

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

    // Set up intents - we want to receive public messages (C2C messages)
    // This is equivalent to: intents = botpy.Intents(public_messages=True)
    let intents = Intents::default().with_public_messages();

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = C2CReplyFileHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}

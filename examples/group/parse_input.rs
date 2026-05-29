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

    /// Assuming user send a complicated message like
    /// - from YinMo:
    /// ```json
    /// {
    ///   "op": 0,
    ///   "s": 3,
    ///   "t": "GROUP_AT_MESSAGE_CREATE",
    ///   "id": "GROUP_AT_MESSAGE_CREATE:zpxv1sjrb7nihjbymcu82wn4mq29wsvjhtapbwpeftuv957wseidnfe0ixfc6s",
    ///   "d": {
    ///     "attachments": [
    ///       {
    ///         "content": "",
    ///         "content_type": "image/png",
    ///         "filename": "91FE1A7D6BEE23893635173599CE58DF.png",
    ///         "height": 512,
    ///         "size": 67372,
    ///         "url": "https://multimedia.nt.qq.com.cn/download?appid=1407\u0026fileid=EhQJ36lNWhVxr1myK2JLocQF0VRjHBisjgQg_wooy82X3sTdlAMyBHByb2RQgL2jAVoQUwEe_3tvKjcT7Wdc5YgbznoCThCCAQJneg\u0026rkey=CAESOE4_cASDm1t1RMe8wlL7yYwUu7wemt2ZlO5NhdUXll-N4UB6e_P5bkCVbdM4RZueznvK_ZP1EKks\u0026spec=0",
    ///         "width": 754
    ///       },
    ///       {
    ///         "content": "",
    ///         "content_type": "image/png",
    ///         "filename": "17E7D36E3D1520C55653F974E833B3CC.png",
    ///         "height": 2334,
    ///         "size": 1579058,
    ///         "url": "https://multimedia.nt.qq.com.cn/download?appid=1407\u0026fileid=EhQVpMvzsRxBZ8f6E0uVo7fgLkMtBBiysGAg_woorJiX3sTdlAMyBHByb2RQgL2jAVoQdbq0gnT8GyTcFY7XpiP9HHoCJIOCAQJneg\u0026rkey=CAESOE4_cASDm1t1RMe8wlL7yYwUu7wemt2ZlO5NhdUXll-Ny2GFiz8nBsxA_hfkl0qOKaZo3rLI_Ht4\u0026spec=0",
    ///         "width": 2604
    ///       }
    ///     ],
    ///     "author": {
    ///       "bot": false,
    ///       "id": "D31EE39A971DFF6185288CAD11087B9A",
    ///       "member_openid": "D31EE39A971DFF6185288CAD11087B9A",
    ///       "union_openid": "D31EE39A971DFF6185288CAD11087B9A",
    ///       "username": "寅默"
    ///     },
    ///     "content": " 图 A  然后图 B ",
    ///     "group_id": "BC3A2807BAE0A09B8D1CDFD28F84D556",
    ///     "group_openid": "BC3A2807BAE0A09B8D1CDFD28F84D556",
    ///     "id": "ROBOT1.0_ZPXv1SJRB7NiHjBYMCu82cHWYiuZ5-6I4Vlrlxgpjoip6mChseoYggbG1WQV5kBHUNyG7Mk3CPNhwWB0Ff1Pry45cytstbdVa3MPitSCyiQ!",
    ///     "message_scene": {
    ///       "ext": ["msg_idx=REFIDX_f1SkRvYaDCdozS/DPeLVPA=="],
    ///       "source": "default"
    ///     },
    ///     "message_type": 0,
    ///     "timestamp": "2026-05-29T11:18:20+08:00"
    ///   }
    /// }
    /// ```
    /// This example aims to display how to handle the input.
    async fn group_message_create(&self, mut session: GroupReplySession) {
        let message = session.message();
        let image_urls: Vec<&str> = message
            .attachments
            .iter()
            .filter(|att| att.content_type.starts_with("image/"))
            .map(|att| att.url.as_str())
            .collect();

        if !image_urls.is_empty() {
            info!(
                "Received {} image attachment(s): {:?}",
                image_urls.len(),
                image_urls
            );
        } else {
            return;
        }

        // Upload media file.
        let upload_media_result = session
            .post_file(
                1, // file_type: 1 for image, file type should match the actual file
                image_urls[0],
                None, // srv_send_msg: Optional flag for server-side message sending
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

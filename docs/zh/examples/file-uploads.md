# 文件上传示例

此示例演示如何使用 BotRS 处理文件上传、下载和管理，包括图像处理、文件验证和安全检查。

## 概述

文件上传功能允许机器人接收、处理和管理用户上传的文件。这包括图像、文档和其他媒体文件的处理。

## 基本文件上传

### 上传图像
上传图片在 qq bot 中反而有点抽象。我们常用的上传图片的 api 其实是反向上传——在自己的图床/起一个静态资源服务器中挂载自己的文件，然后把 url 发送给 qq，qq会从你的 url 中下载这个图片然后再发送给对应的群聊/频道/私信。此方法已经过测试。

直接上传二进制流还未经过测试，案例如下。
```rust
//! Demo: AT Reply with File Data
//!
//! This example demonstrates how to create a bot that responds to @ mentions
//! with file uploads (images). It's equivalent to the Python demo_at_reply_file_data.py example.

mod common;

use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use common::{Config, init_logging};
use std::env;
use std::fs;
use tracing::{info, warn};

/// Event handler that responds to @ mentions with file uploads.
struct FileReplyHandler;

#[async_trait::async_trait]
impl EventHandler for FileReplyHandler {
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

        let reply_content = format!("机器人{bot_name}收到你的@消息了: {content}");

        // Get required IDs
        let channel_id = match &message.channel_id {
            Some(id) => id,
            None => {
                warn!("Message has no channel_id");
                return;
            }
        };

        // Method 1: Read file as bytes and send (equivalent to Python method 1)
        match self
            .send_file_as_bytes(&ctx, channel_id, &reply_content)
            .await
        {
            Ok(_) => info!("Successfully sent file as bytes"),
            Err(e) => warn!("Failed to send file as bytes: {}", e),
        }

        // Method 2: Send file by reading it again (equivalent to Python method 2)
        // Note: In Rust, this is similar to method 1 since we need to read the file
        match self
            .send_file_direct(&ctx, channel_id, &reply_content)
            .await
        {
            Ok(_) => info!("Successfully sent file directly"),
            Err(e) => warn!("Failed to send file directly: {}", e),
        }

        // Method 3: Send file by path (equivalent to Python method 3)
        // Note: In the current API, we still need to read the file, but this demonstrates
        // the concept of path-based file sending
        match self
            .send_file_by_path(&ctx, channel_id, &reply_content)
            .await
        {
            Ok(_) => info!("Successfully sent file by path"),
            Err(e) => warn!("Failed to send file by path: {}", e),
        }
    }

    /// Called when an error occurs during event processing.
    async fn error(&self, error: botrs::BotError) {
        warn!("Event handler error: {}", error);
    }
}

impl FileReplyHandler {
    /// Method 1: Read file as bytes and send (equivalent to Python method 1)
    async fn send_file_as_bytes(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "examples/resource/test.png";

        // Read file as bytes (equivalent to Python: with open("resource/test.png", "rb") as img: img_bytes = img.read())
        let img_bytes = match fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!(
                    "Could not read file {}: {}. Make sure the file exists.",
                    file_path, e
                );
                info!("Creating a simple placeholder file for demonstration...");
                // Create a simple placeholder if file doesn't exist
                b"This is a placeholder file for demo purposes. Replace with an actual image file."
                    .to_vec()
            }
        };

        // Send message with file attachment
        // Send file image using bytes
        let params =
            botrs::models::message::MessageParams::new_text(content).with_file_image(&img_bytes);

        ctx.api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await?;

        Ok(())
    }

    /// Method 2: Send file by reading it directly (equivalent to Python method 2)
    async fn send_file_direct(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "examples/resource/test.png";

        // Read file directly (equivalent to Python: with open("resource/test.png", "rb") as img:)
        let img_bytes = match fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!(
                    "Could not read file {}: {}. Using placeholder.",
                    file_path, e
                );
                // Create a simple placeholder if file doesn't exist
                b"This is a placeholder file for demo purposes (method 2). Replace with an actual image file.".to_vec()
            }
        };

        // Send message with file attachment
        // Send file image using bytes directly
        let params =
            botrs::models::message::MessageParams::new_text(content).with_file_image(&img_bytes);

        ctx.api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await?;

        Ok(())
    }

    /// Method 3: Send file by path (equivalent to Python method 3)
    /// Note: The API still requires bytes, but this demonstrates path-based approach
    async fn send_file_by_path(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "examples/resource/test.png";

        info!("Sending file from path: {}", file_path);

        // Read file from path (equivalent to Python: file_image="resource/test.png")
        let img_bytes = match fs::read(file_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!(
                    "Could not read file {}: {}. Using placeholder.",
                    file_path, e
                );
                // Create a simple placeholder if file doesn't exist
                b"This is a placeholder file for demo purposes (method 3). Replace with an actual image file.".to_vec()
            }
        };

        // Send message with file attachment
        // Send file image using bytes from path
        let params =
            botrs::models::message::MessageParams::new_text(content).with_file_image(&img_bytes);

        ctx.api
            .post_message_with_params(&ctx.token, channel_id, params)
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init_logging();

    info!("Starting AT reply file data demo...");

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

    info!("Configured intents: {}", intents);

    // Create event handler
    let handler = FileReplyHandler;

    // Create client with caching enabled
    let mut client = Client::new(token, intents, handler, true)?;

    info!("Client created, starting bot...");

    // Start the bot - this will block until the bot stops
    client.start().await?;

    info!("Bot stopped");
    Ok(())
}
```

## 相关链接

- [富消息](./rich-messages.md) - 了解如何发送丰富的消息内容
- [命令处理器](./command-handler.md) - 创建文件管理命令
- [错误恢复](./error-recovery.md) - 处理文件操作错误
- [API 集成](./api-integration.md) - 集成外部文件服务

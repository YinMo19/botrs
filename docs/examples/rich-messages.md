# Rich Messages Example

This example demonstrates how to create rich, interactive messages using embeds, attachments, and various formatting options in BotRS.

## Overview

Rich messages enhance user experience by providing visually appealing content with embeds, images, files, and interactive elements. This guide shows various techniques for creating engaging bot responses.

## Basic Embed Messages

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageEmbed};

struct RichMessageBot;

#[async_trait::async_trait]
impl EventHandler for RichMessageBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Rich message bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            match content.as_str() {
                "!info" => {
                    self.send_info_embed(&ctx, &msg.channel_id).await;
                }
                "!status" => {
                    self.send_status_embed(&ctx, &msg.channel_id).await;
                }
                "!help" => {
                    self.send_help_embed(&ctx, &msg.channel_id).await;
                }
                _ => {}
            }
        }
    }
}

impl RichMessageBot {
    async fn send_info_embed(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("Bot Information")
            .description("This is a demonstration of rich message capabilities")
            .color(0x3498db) // Blue color
            .field("Version", "1.0.0", true)
            .field("Language", "Rust", true)
            .field("Framework", "BotRS", true)
            .field("Uptime", "24 hours", false)
            .thumbnail("https://example.com/bot-icon.png")
            .footer("Powered by BotRS", Some("https://example.com/logo.png"))
            .timestamp(chrono::Utc::now().to_rfc3339());

        if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            eprintln!("Failed to send info embed: {}", e);
        }
    }

    async fn send_status_embed(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("System Status")
            .color(0x2ecc71) // Green color for healthy status
            .field("CPU Usage", "15%", true)
            .field("Memory Usage", "512 MB", true)
            .field("Network", "Stable", true)
            .field("Database", "✅ Connected", true)
            .field("Cache", "✅ Active", true)
            .field("API", "✅ Responding", true)
            .footer("Last updated", None)
            .timestamp(chrono::Utc::now().to_rfc3339());

        if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            eprintln!("Failed to send status embed: {}", e);
        }
    }

    async fn send_help_embed(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("Available Commands")
            .description("Here are all the commands you can use:")
            .color(0xe74c3c) // Red color
            .field("!info", "Show bot information", false)
            .field("!status", "Display system status", false)
            .field("!help", "Show this help message", false)
            .field("!image", "Send an image example", false)
            .field("!file", "Send a file example", false)
            .footer("Use commands without the quotes", None);

        if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            eprintln!("Failed to send help embed: {}", e);
        }
    }
}
```

## Advanced Rich Messages

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageEmbed, MessageParams};
use std::path::Path;

struct AdvancedRichBot;

#[async_trait::async_trait]
impl EventHandler for AdvancedRichBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Advanced rich bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            match content.as_str() {
                "!weather" => {
                    self.send_weather_info(&ctx, &msg.channel_id).await;
                }
                "!profile" => {
                    self.send_user_profile(&ctx, &msg).await;
                }
                "!gallery" => {
                    self.send_image_gallery(&ctx, &msg.channel_id).await;
                }
                "!markdown" => {
                    self.send_markdown_message(&ctx, &msg.channel_id).await;
                }
                _ => {}
            }
        }
    }
}

impl AdvancedRichBot {
    async fn send_weather_info(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("Weather Report")
            .description("Current weather conditions")
            .color(0x87ceeb) // Sky blue
            .field("Location", "Beijing, China", true)
            .field("Temperature", "22°C", true)
            .field("Condition", "Partly Cloudy", true)
            .field("Humidity", "65%", true)
            .field("Wind Speed", "15 km/h", true)
            .field("Visibility", "10 km", true)
            .thumbnail("https://example.com/weather-icon.png")
            .image("https://example.com/weather-map.png")
            .footer("Data updated 5 minutes ago", None)
            .timestamp(chrono::Utc::now().to_rfc3339());

        if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            eprintln!("Failed to send weather embed: {}", e);
        }
    }

    async fn send_user_profile(&self, ctx: &Context, msg: &Message) {
        if let Some(author) = &msg.author {
            let embed = MessageEmbed::new()
                .title("User Profile")
                .description(format!("Profile information for {}",
                    author.username.as_deref().unwrap_or("Unknown")))
                .color(0x9b59b6) // Purple
                .field("Username",
                    author.username.as_deref().unwrap_or("Unknown"), true)
                .field("User ID",
                    author.id.as_deref().unwrap_or("Unknown"), true)
                .field("Bot Account",
                    if author.bot.unwrap_or(false) { "Yes" } else { "No" }, true)
                .thumbnail(author.avatar.as_deref().unwrap_or(""))
                .footer("Profile requested", None)
                .timestamp(chrono::Utc::now().to_rfc3339());

            if let Err(e) = ctx.send_message_with_embed(&msg.channel_id, None, &embed).await {
                eprintln!("Failed to send profile embed: {}", e);
            }
        }
    }

    async fn send_image_gallery(&self, ctx: &Context, channel_id: &str) {
        // Send multiple embeds for a gallery effect
        let images = vec![
            ("Sunset", "https://example.com/sunset.jpg", 0xff6b35),
            ("Ocean", "https://example.com/ocean.jpg", 0x006ba6),
            ("Mountains", "https://example.com/mountains.jpg", 0x0f3460),
        ];

        for (title, url, color) in images {
            let embed = MessageEmbed::new()
                .title(title)
                .color(color)
                .image(url)
                .footer("Image Gallery", None);

            if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
                eprintln!("Failed to send gallery image: {}", e);
            }

            // Small delay between images
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    async fn send_markdown_message(&self, ctx: &Context, channel_id: &str) {
        let markdown_content = r#"
# Markdown Support

BotRS supports **rich text formatting** with markdown!

## Text Formatting
- **Bold text**
- *Italic text*
- ~~Strikethrough text~~
- `Inline code`

## Lists
1. First ordered item
2. Second ordered item
3. Third ordered item

- Unordered item
- Another item
- Last item

## Code Blocks
```rust
fn main() {
    println!("Hello, BotRS!");
}
```

## File and Media Messages

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageParams};
use std::path::Path;

struct MediaBot;

#[async_trait::async_trait]
impl EventHandler for MediaBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Media bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            match content.as_str() {
                "!image" => {
                    self.send_image(&ctx, &msg.channel_id).await;
                }
                "!audio" => {
                    self.send_audio(&ctx, &msg.channel_id).await;
                }
                "!video" => {
                    self.send_video(&ctx, &msg.channel_id).await;
                }
                "!document" => {
                    self.send_document(&ctx, &msg.channel_id).await;
                }
                _ => {}
            }
        }
    }
}

impl MediaBot {
    async fn send_image(&self, ctx: &Context, channel_id: &str) {
        // Method 1: Send from local file
        let image_path = Path::new("./assets/example.png");
        if image_path.exists() {
            let params = MessageParams::new_text("Here's an image from file!")
                .with_image_file(image_path);

            if let Ok(params) = params {
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                    eprintln!("Failed to send image from file: {}", e);
                }
            }
        } else {
            // Method 2: Send from URL
            let params = MessageParams::new_text("Here's an image from URL!")
                .with_image("https://example.com/sample-image.png");

            if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                eprintln!("Failed to send image from URL: {}", e);
            }
        }
    }

    async fn send_audio(&self, ctx: &Context, channel_id: &str) {
        let audio_path = Path::new("./assets/sample.mp3");
        if audio_path.exists() {
            let params = MessageParams::new_text("Here's an audio file!")
                .with_audio_file(audio_path);

            if let Ok(params) = params {
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                    eprintln!("Failed to send audio: {}", e);
                }
            }
        } else {
            let _ = ctx.send_message(channel_id, "Audio file not found!").await;
        }
    }

    async fn send_video(&self, ctx: &Context, channel_id: &str) {
        let video_path = Path::new("./assets/sample.mp4");
        if video_path.exists() {
            let params = MessageParams::new_text("Here's a video!")
                .with_video_file(video_path);

            if let Ok(params) = params {
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                    eprintln!("Failed to send video: {}", e);
                }
            }
        } else {
            let _ = ctx.send_message(channel_id, "Video file not found!").await;
        }
    }

    async fn send_document(&self, ctx: &Context, channel_id: &str) {
        let doc_path = Path::new("./assets/document.pdf");
        if doc_path.exists() {
            let params = MessageParams::new_text("Here's a document!")
                .with_file(doc_path);

            if let Ok(params) = params {
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                    eprintln!("Failed to send document: {}", e);
                }
            }
        } else {
            let _ = ctx.send_message(channel_id, "Document not found!").await;
        }
    }
}
```

## Interactive Rich Messages

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageEmbed};

struct InteractiveBot;

#[async_trait::async_trait]
impl EventHandler for InteractiveBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Interactive bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            match content.as_str() {
                "!poll" => {
                    self.create_poll(&ctx, &msg.channel_id).await;
                }
                "!menu" => {
                    self.create_menu(&ctx, &msg.channel_id).await;
                }
                "!progress" => {
                    self.show_progress(&ctx, &msg.channel_id).await;
                }
                _ => {}
            }
        }
    }
}

impl InteractiveBot {
    async fn create_poll(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("📊 Community Poll")
            .description("What's your favorite programming language?")
            .color(0xf39c12) // Orange
            .field("🦀 Rust", "React with 🦀", false)
            .field("🐍 Python", "React with 🐍", false)
            .field("☕ Java", "React with ☕", false)
            .field("⚡ JavaScript", "React with ⚡", false)
            .footer("Poll expires in 24 hours", None)
            .timestamp(chrono::Utc::now().to_rfc3339());

        if let Ok(sent_msg) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            // Add reactions for voting
            let reactions = ["🦀", "🐍", "☕", "⚡"];
            for emoji in &reactions {
                // Note: Actual reaction adding would require emoji handling
                // This is a simplified example
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }

    async fn create_menu(&self, ctx: &Context, channel_id: &str) {
        let embed = MessageEmbed::new()
            .title("🍽️ Restaurant Menu")
            .description("Welcome to our virtual restaurant!")
            .color(0x27ae60) // Green
            .field("🍕 Pizza", "$12.99 - Margherita with fresh basil", false)
            .field("🍔 Burger", "$9.99 - Classic beef burger with fries", false)
            .field("🍜 Ramen", "$11.50 - Authentic Japanese ramen", false)
            .field("🥗 Salad", "$8.99 - Fresh garden salad", false)
            .field("🍰 Dessert", "$5.99 - Chocolate cake slice", false)
            .thumbnail("https://example.com/restaurant-logo.png")
            .footer("Use reactions to place your order!", None);

        if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
            eprintln!("Failed to send menu: {}", e);
        }
    }

    async fn show_progress(&self, ctx: &Context, channel_id: &str) {
        let progress_bars = vec![
            ("Download", 85, 0x3498db),
            ("Installation", 60, 0xf39c12),
            ("Configuration", 30, 0xe74c3c),
        ];

        for (task, progress, color) in progress_bars {
            let bar_length = 20;
            let filled = (progress * bar_length) / 100;
            let empty = bar_length - filled;

            let progress_bar = format!(
                "[{}{}] {}%",
                "█".repeat(filled),
                "░".repeat(empty),
                progress
            );

            let embed = MessageEmbed::new()
                .title(format!("⚙️ {} Progress", task))
                .description(format!("```\n{}\n```", progress_bar))
                .color(color)
                .field("Status",
                    if progress == 100 { "✅ Complete" }
                    else if progress > 0 { "🔄 In Progress" }
                    else { "⏳ Pending" },
                    true)
                .field("ETA",
                    if progress == 100 { "Complete" }
                    else { "2 minutes" },
                    true)
                .timestamp(chrono::Utc::now().to_rfc3339());

            if let Err(e) = ctx.send_message_with_embed(channel_id, None, &embed).await {
                eprintln!("Failed to send progress: {}", e);
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }
}
```

## Configuration

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
botrs = "0.2"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"
chrono = { version = "0.4", features = ["serde"] }
```

Set up your environment:

```bash
export QQ_BOT_APP_ID=your_app_id
export QQ_BOT_SECRET=your_secret
```

## Asset Files

Create an `assets` directory with sample files for testing:

```
assets/
├── example.png
├── sample.mp3
├── sample.mp4
└── document.pdf
```

## Best Practices

1. **Image Optimization**: Use appropriate image sizes to avoid large uploads
2. **Color Consistency**: Use consistent color schemes across embeds
3. **Content Limits**: Be aware of message length and embed field limits
4. **Accessibility**: Provide alt text and clear descriptions
5. **Performance**: Don't send too many rich messages in quick succession
6. **User Experience**: Use rich messages to enhance, not overwhelm

## Key Features Demonstrated

- **Embed Creation**: Rich formatted messages with multiple fields
- **Media Handling**: Images, audio, video, and document uploads
- **Markdown Support**: Text formatting and structure
- **Interactive Elements**: Polls, menus, and progress indicators
- **Visual Design**: Colors, thumbnails, and layout
- **Timestamps**: Dynamic time information
- **Error Handling**: Graceful fallbacks for missing assets

## Next Steps

- [Command Handler](./command-handler.md) - Structure your commands
- [Interactive Messages](./interactive-messages.md) - Add buttons and forms
- [File Uploads](./file-uploads.md) - Advanced file handling
- [Event Handling](./event-handling.md) - React to user interactions

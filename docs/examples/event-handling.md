# Event Handling Example

This example demonstrates comprehensive event handling patterns for QQ Guild bots using BotRS.

## Overview

Event handling is the core of any bot application. BotRS provides a rich set of events that allow your bot to respond to various activities in guilds, channels, and direct messages. This example shows how to implement robust event handlers for different scenarios.

## Basic Event Handler

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token, BotError,
    Guild, Channel, Member, GroupMessage, C2CMessage,
    MessageAudit, PublicAudio, OpenThread
};
use botrs::models::message::DirectMessageParams;
use async_trait::async_trait;
use tracing::{info, warn, error, debug};

struct ComprehensiveBot {
    startup_time: std::time::Instant,
}

impl ComprehensiveBot {
    pub fn new() -> Self {
        Self {
            startup_time: std::time::Instant::now(),
        }
    }
}

#[async_trait]
impl EventHandler for ComprehensiveBot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("🤖 Bot is ready!");
        info!("👤 Logged in as: {}", ready.user.username);
        info!("🆔 Session ID: {}", ready.session_id);
        info!("🔧 Gateway version: {}", ready.version);
        
        if let Some(shard) = ready.shard {
            info!("🔀 Shard: {}/{}", shard[0], shard[1]);
        }

        // Set bot status or perform initialization tasks
        let startup_duration = self.startup_time.elapsed();
        info!("⚡ Startup completed in {:?}", startup_duration);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        // Ignore bot messages to prevent loops
        if message.is_from_bot() {
            return;
        }

        debug!(
            "📨 Message received in channel {} from user {:?}",
            message.channel_id,
            message.author.as_ref().map(|a| &a.id)
        );

        if let Some(content) = &message.content {
            // Log message for monitoring
            info!("💬 [{}] {}", message.channel_id, content);

            // Handle different message types
            match content.trim() {
                "!ping" => {
                    let _ = message.reply(&ctx.api, &ctx.token, "🏓 Pong!").await;
                }
                "!server" => {
                    self.handle_server_info(&ctx, &message).await;
                }
                "!channels" => {
                    self.handle_channel_list(&ctx, &message).await;
                }
                _ if content.starts_with("!echo ") => {
                    let echo_text = &content[6..];
                    let _ = message.reply(&ctx.api, &ctx.token, echo_text).await;
                }
                _ => {
                    // Handle other message patterns
                    self.handle_general_message(&ctx, &message, content).await;
                }
            }
        }

        // Handle messages with attachments
        if message.has_attachments() {
            self.handle_message_attachments(&ctx, &message).await;
        }

        // Handle mentions
        if message.has_mentions() {
            self.handle_message_mentions(&ctx, &message).await;
        }
    }

    async fn direct_message_create(&self, ctx: Context, message: Message) {
        info!(
            "📩 Direct message received from user {:?}",
            message.author.as_ref().map(|a| &a.id)
        );

        if let (Some(guild_id), Some(content)) = (&message.guild_id, &message.content) {
            // Handle DM-specific commands
            let response = match content.trim() {
                "!help" => {
                    "🆘 **Bot Help (Direct Message)**\n\n\
                        Available commands:\n\
                        • `!help` - Show this help message\n\
                        • `!status` - Show bot status\n\
                        • `!support` - Get support information".to_string()
                }
                "!status" => {
                    let uptime = self.startup_time.elapsed();
                    format!(
                        "🤖 **Bot Status**\n\n\
                         Status: ✅ Online\n\
                         Uptime: {:?}\n\
                         Ready for commands!",
                        uptime
                    )
                }
                _ => "👋 Hello! Send `!help` for available commands.".to_string(),
            };

            let params = DirectMessageParams::new_text(response);
            let _ = ctx.api.post_dms_with_params(&ctx.token, guild_id, params).await;
        }
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        info!(
            "👥 Group message received in group {:?}",
            message.group_openid
        );

        if let Some(content) = &message.content {
            // Handle group-specific logic
            if content.contains("机器人") || content.contains("bot") {
                let response = "🤖 有人在叫我吗？我是 QQ 群机器人！";
                let _ = message.reply(&ctx.api, &ctx.token, response).await;
            }
        }
    }

    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        info!("💬 C2C message received");
        
        if let Some(content) = &message.content {
            // Handle C2C message logic
            let response = format!("收到你的 C2C 消息: {}", content);
            let _ = message.reply(&ctx.api, &ctx.token, &response).await;
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        info!("🏰 Joined guild: {} (ID: {})", guild.name, guild.id);
        
        // Get guild statistics
        if let Ok(channels) = ctx.get_channels(&guild.id).await {
            info!("📊 Guild {} has {} channels", guild.name, channels.len());
        }

        // Welcome message or setup procedures could go here
    }

    async fn guild_update(&self, _ctx: Context, guild: Guild) {
        info!("🔄 Guild updated: {} (ID: {})", guild.name, guild.id);
    }

    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        warn!("👋 Left guild: {} (ID: {})", guild.name, guild.id);
    }

    async fn channel_create(&self, _ctx: Context, channel: Channel) {
        info!(
            "📢 Channel created: {} in guild {} (Type: {:?})",
            channel.name, channel.guild_id, channel.channel_type
        );
    }

    async fn channel_update(&self, _ctx: Context, channel: Channel) {
        info!(
            "🔄 Channel updated: {} in guild {}",
            channel.name, channel.guild_id
        );
    }

    async fn channel_delete(&self, _ctx: Context, channel: Channel) {
        warn!(
            "🗑️ Channel deleted: {} in guild {}",
            channel.name, channel.guild_id
        );
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!("👋 New member joined: {} (ID: {})", 
                  user.username.as_deref().unwrap_or("Unknown"), user.id);
            
            // Send welcome message (you'd need to determine appropriate channel)
            // self.send_welcome_message(&ctx, &member).await;
        }
    }

    async fn guild_member_update(&self, _ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            debug!("🔄 Member updated: {} (ID: {})", 
                   user.username.as_deref().unwrap_or("Unknown"), user.id);
        }
    }

    async fn guild_member_remove(&self, _ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!("👋 Member left: {} (ID: {})", 
                  user.username.as_deref().unwrap_or("Unknown"), user.id);
        }
    }

    async fn message_audit_pass(&self, _ctx: Context, audit: MessageAudit) {
        debug!("✅ Message audit passed: {}", audit.message_id);
    }

    async fn message_audit_reject(&self, _ctx: Context, audit: MessageAudit) {
        warn!("❌ Message audit rejected: {}", audit.message_id);
    }

    async fn audio_or_live_channel_member_enter(&self, _ctx: Context, audio: PublicAudio) {
        if let (Some(channel_id), Some(user_id)) = (&audio.channel_id, &audio.user_id) {
            info!("🎤 User {} entered audio channel {}", user_id, channel_id);
        }
    }

    async fn audio_or_live_channel_member_exit(&self, _ctx: Context, audio: PublicAudio) {
        if let (Some(channel_id), Some(user_id)) = (&audio.channel_id, &audio.user_id) {
            info!("🔇 User {} left audio channel {}", user_id, channel_id);
        }
    }

    async fn open_forum_thread_create(&self, _ctx: Context, thread: OpenThread) {
        info!("🧵 New forum thread created: {}", thread.thread_info.title);
    }

    async fn open_forum_thread_update(&self, _ctx: Context, thread: OpenThread) {
        info!("🔄 Forum thread updated: {}", thread.thread_info.title);
    }

    async fn open_forum_thread_delete(&self, _ctx: Context, thread: OpenThread) {
        info!("🗑️ Forum thread deleted: {}", thread.thread_info.title);
    }

    async fn error(&self, error: BotError) {
        error!("💥 Bot error occurred: {}", error);
        
        // Implement error recovery logic based on error type
        match error {
            BotError::Network(_) => {
                warn!("🌐 Network error - connection may be unstable");
            }
            BotError::RateLimited(_) => {
                warn!("⏱️ Rate limited - backing off");
            }
            BotError::Authentication(_) => {
                error!("🔒 Authentication error - check credentials");
            }
            _ => {
                warn!("❓ Unhandled error type: {}", error);
            }
        }
    }
}
```

## Advanced Event Handling Patterns

### Event Filtering and Routing

```rust
impl ComprehensiveBot {
    async fn handle_general_message(&self, ctx: &Context, message: &Message, content: &str) {
        // URL detection
        if content.contains("http://") || content.contains("https://") {
            self.handle_url_message(ctx, message, content).await;
        }

        // Question detection
        if content.ends_with('?') || content.contains("how") || content.contains("what") {
            self.handle_question_message(ctx, message, content).await;
        }

        // Keyword monitoring
        let monitored_keywords = ["bug", "issue", "problem", "help"];
        if monitored_keywords.iter().any(|&keyword| content.to_lowercase().contains(keyword)) {
            self.handle_support_request(ctx, message, content).await;
        }

        // Spam detection
        if self.is_potential_spam(content) {
            self.handle_potential_spam(ctx, message).await;
        }
    }

    async fn handle_url_message(&self, ctx: &Context, message: &Message, content: &str) {
        debug!("🔗 URL detected in message");
        
        // Extract URLs and validate them
        let urls = self.extract_urls(content);
        for url in urls {
            if self.is_safe_url(&url).await {
                debug!("✅ Safe URL detected: {}", url);
            } else {
                warn!("⚠️ Potentially unsafe URL: {}", url);
                // Could add reaction or warning
            }
        }
    }

    async fn handle_question_message(&self, ctx: &Context, message: &Message, _content: &str) {
        // Add thinking reaction to show the bot is processing
        // Note: Reaction handling would need appropriate API calls
        debug!("❓ Question detected, processing...");
    }

    async fn handle_support_request(&self, ctx: &Context, message: &Message, content: &str) {
        info!("🆘 Support request detected: {}", content);
        
        let support_response = "🤝 I see you might need help! \
                               Our support team has been notified. \
                               You can also use `!help` for quick assistance.";
        
        let _ = message.reply(&ctx.api, &ctx.token, support_response).await;
    }

    async fn handle_potential_spam(&self, ctx: &Context, message: &Message) {
        warn!("🚫 Potential spam detected from user {:?}", 
              message.author.as_ref().map(|a| &a.id));
        
        // Log for moderation review
        // Could implement automatic actions based on confidence level
    }

    fn is_potential_spam(&self, content: &str) -> bool {
        // Simple spam detection logic
        let spam_indicators = [
            content.len() > 500 && content.chars().filter(|&c| c.is_uppercase()).count() > content.len() / 3,
            content.contains("FREE") && content.contains("CLICK"),
            content.chars().filter(|&c| c == '!').count() > 5,
        ];
        
        spam_indicators.iter().any(|&indicator| indicator)
    }

    fn extract_urls(&self, content: &str) -> Vec<String> {
        // Simple URL extraction - in practice, use a proper URL parsing library
        content.split_whitespace()
            .filter(|word| word.starts_with("http://") || word.starts_with("https://"))
            .map(|url| url.to_string())
            .collect()
    }

    async fn is_safe_url(&self, _url: &str) -> bool {
        // Implement URL safety checking (domain whitelist, reputation services, etc.)
        true // Simplified for example
    }
}
```

### Message Attachment Handling

```rust
impl ComprehensiveBot {
    async fn handle_message_attachments(&self, ctx: &Context, message: &Message) {
        info!("📎 Processing {} attachment(s)", message.attachments.len());

        for (index, attachment) in message.attachments.iter().enumerate() {
            let filename = attachment.filename.as_deref().unwrap_or("unknown");
            let size = attachment.size.unwrap_or(0);
            
            info!("📄 Attachment {}: {} ({} bytes)", index + 1, filename, size);

            // Handle different file types
            if attachment.is_image() {
                self.handle_image_attachment(ctx, message, attachment).await;
            } else if attachment.is_video() {
                self.handle_video_attachment(ctx, message, attachment).await;
            } else if attachment.is_audio() {
                self.handle_audio_attachment(ctx, message, attachment).await;
            } else {
                self.handle_document_attachment(ctx, message, attachment).await;
            }
        }
    }

    async fn handle_image_attachment(&self, ctx: &Context, message: &Message, attachment: &botrs::models::message::MessageAttachment) {
        info!("🖼️ Processing image: {:?}", attachment.filename);
        
        if let (Some(width), Some(height)) = (attachment.width, attachment.height) {
            let info = format!(
                "📸 **Image Info**\n\
                 Size: {}x{}\n\
                 File size: {} bytes",
                width, height, attachment.size.unwrap_or(0)
            );
            
            let _ = message.reply(&ctx.api, &ctx.token, &info).await;
        }
    }

    async fn handle_video_attachment(&self, ctx: &Context, message: &Message, attachment: &botrs::models::message::MessageAttachment) {
        info!("🎥 Processing video: {:?}", attachment.filename);
        
        let info = format!(
            "🎬 **Video received**\n\
             File: {}\n\
             Size: {} bytes",
            attachment.filename.as_deref().unwrap_or("unknown"),
            attachment.size.unwrap_or(0)
        );
        
        let _ = message.reply(&ctx.api, &ctx.token, &info).await;
    }

    async fn handle_audio_attachment(&self, ctx: &Context, message: &Message, attachment: &botrs::models::message::MessageAttachment) {
        info!("🎵 Processing audio: {:?}", attachment.filename);
        
        let info = "🎧 **Audio file received** - Thanks for sharing!";
        let _ = message.reply(&ctx.api, &ctx.token, info).await;
    }

    async fn handle_document_attachment(&self, ctx: &Context, message: &Message, attachment: &botrs::models::message::MessageAttachment) {
        info!("📋 Processing document: {:?}", attachment.filename);
        
        let filename = attachment.filename.as_deref().unwrap_or("unknown");
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown");

        let doc_type = match extension.to_lowercase().as_str() {
            "pdf" => "PDF Document",
            "doc" | "docx" => "Word Document",
            "txt" => "Text File",
            "md" => "Markdown File",
            _ => "Document",
        };

        let info = format!("📄 **{}** received: {}", doc_type, filename);
        let _ = message.reply(&ctx.api, &ctx.token, &info).await;
    }
}
```

### Server Information Handlers

```rust
impl ComprehensiveBot {
    async fn handle_server_info(&self, ctx: &Context, message: &Message) {
        match ctx.get_guild(&message.guild_id).await {
            Ok(guild) => {
                let channels_result = ctx.get_channels(&guild.id).await;
                let channel_count = channels_result.map(|c| c.len()).unwrap_or(0);

                let info = format!(
                    "🏰 **Server Information**\n\n\
                     **Name:** {}\n\
                     **ID:** {}\n\
                     **Channels:** {}\n\
                     **Members:** {}\n\
                     **Owner:** {}",
                    guild.name,
                    guild.id,
                    channel_count,
                    guild.member_count.unwrap_or(0),
                    if guild.owner { "This bot" } else { &guild.owner_id }
                );

                let _ = message.reply(&ctx.api, &ctx.token, &info).await;
            }
            Err(e) => {
                error!("Failed to get guild info: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to get server information").await;
            }
        }
    }

    async fn handle_channel_list(&self, ctx: &Context, message: &Message) {
        match ctx.get_channels(&message.guild_id).await {
            Ok(channels) => {
                let mut channel_list = String::from("📋 **Channel List**\n\n");
                
                for channel in channels.iter().take(10) { // Limit to first 10
                    let channel_type = match channel.channel_type {
                        botrs::models::channel::ChannelType::Text => "💬",
                        botrs::models::channel::ChannelType::Voice => "🔊",
                        botrs::models::channel::ChannelType::Category => "📁",
                        botrs::models::channel::ChannelType::Announcement => "📢",
                        botrs::models::channel::ChannelType::Forum => "🧵",
                        botrs::models::channel::ChannelType::Live => "🎥",
                        botrs::models::channel::ChannelType::Application => "🔧",
                    };
                    
                    channel_list.push_str(&format!("{} {}\n", channel_type, channel.name));
                }

                if channels.len() > 10 {
                    channel_list.push_str(&format!("\n... and {} more channels", channels.len() - 10));
                }

                let _ = message.reply(&ctx.api, &ctx.token, &channel_list).await;
            }
            Err(e) => {
                error!("Failed to get channels: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "❌ Failed to get channel list").await;
            }
        }
    }
}
```

### Mention Handling

```rust
impl ComprehensiveBot {
    async fn handle_message_mentions(&self, ctx: &Context, message: &Message) {
        info!("👥 Message contains {} mention(s)", message.mentions.len());

        // Check if bot is mentioned
        if let Some(bot_info) = &ctx.bot_info {
            let bot_mentioned = message.mentions.iter()
                .any(|mention| mention.id == bot_info.id);

            if bot_mentioned {
                self.handle_bot_mention(ctx, message).await;
            }
        }

        // Handle other mentions
        for mention in &message.mentions {
            if let Some(username) = &mention.username {
                debug!("👤 User mentioned: {} ({})", username, mention.id);
            }
        }
    }

    async fn handle_bot_mention(&self, ctx: &Context, message: &Message) {
        info!("🤖 Bot was mentioned in message");

        let responses = [
            "👋 Hello! You mentioned me!",
            "🤖 How can I help you?",
            "👀 I'm here! What do you need?",
            "✨ You called?",
        ];

        let response = responses[fastrand::usize(..responses.len())];
        let _ = message.reply(&ctx.api, &ctx.token, response).await;
    }
}
```

## Event Statistics and Monitoring

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct EventStatistics {
    pub messages_received: AtomicU64,
    pub commands_processed: AtomicU64,
    pub guilds_joined: AtomicU64,
    pub guilds_left: AtomicU64,
    pub members_joined: AtomicU64,
    pub members_left: AtomicU64,
    pub errors_encountered: AtomicU64,
}

struct MonitoredBot {
    stats: Arc<EventStatistics>,
    startup_time: std::time::Instant,
}

impl MonitoredBot {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(EventStatistics::default()),
            startup_time: std::time::Instant::now(),
        }
    }
}

#[async_trait]
impl EventHandler for MonitoredBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("📊 Monitored bot ready: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        self.stats.messages_received.fetch_add(1, Ordering::Relaxed);

        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content.starts_with('!') {
                self.stats.commands_processed.fetch_add(1, Ordering::Relaxed);
            }

            if content.trim() == "!stats" {
                self.send_statistics(&ctx, &message).await;
            }
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        self.stats.guilds_joined.fetch_add(1, Ordering::Relaxed);
        info!("📈 Guild joined: {} (Total: {})", 
              guild.name, 
              self.stats.guilds_joined.load(Ordering::Relaxed));
    }

    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        self.stats.guilds_left.fetch_add(1, Ordering::Relaxed);
        info!("📉 Guild left: {} (Total left: {})", 
              guild.name, 
              self.stats.guilds_left.load(Ordering::Relaxed));
    }

    async fn guild_member_add(&self, _ctx: Context, _member: Member) {
        self.stats.members_joined.fetch_add(1, Ordering::Relaxed);
    }

    async fn guild_member_remove(&self, _ctx: Context, _member: Member) {
        self.stats.members_left.fetch_add(1, Ordering::Relaxed);
    }

    async fn error(&self, error: BotError) {
        self.stats.errors_encountered.fetch_add(1, Ordering::Relaxed);
        error!("📊 Error recorded: {} (Total errors: {})", 
               error, 
               self.stats.errors_encountered.load(Ordering::Relaxed));
    }
}

impl MonitoredBot {
    async fn send_statistics(&self, ctx: &Context, message: &Message) {
        let uptime = self.startup_time.elapsed();
        
        let stats_message = format!(
            "📊 **Bot Statistics**\n\n\
             **Uptime:** {:?}\n\
             **Messages Received:** {}\n\
             **Commands Processed:** {}\n\
             **Guilds Joined:** {}\n\
             **Guilds Left:** {}\n\
             **Members Joined:** {}\n\
             **Members Left:** {}\n\
             **Errors Encountered:** {}",
            uptime,
            self.stats.messages_received.load(Ordering::Relaxed),
            self.stats.commands_processed.load(Ordering::Relaxed),
            self.stats.guilds_joined.load(Ordering::Relaxed),
            self.stats.guilds_left.load(Ordering::Relaxed),
            self.stats.members_joined.load(Ordering::Relaxed),
            self.stats.members_left.load(Ordering::Relaxed),
            self.stats.errors_encountered.load(Ordering::Relaxed),
        );

        let _ = message.reply(&ctx.api, &ctx.token, &stats_message).await;
    }
}
```

## Main Application

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,event_handling=info")
        .init();

    info!("🚀 Starting comprehensive event handling bot...");

    // Get credentials
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("QQ_BOT_APP_ID environment variable required");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("QQ_BOT_SECRET environment variable required");

    // Create and validate token
    let token = Token::new(app_id, secret);
    token.validate()?;

    // Configure intents for comprehensive event handling
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds()
        .with_guild_members()
        .with_audio_action()
        .with_forum_event()
        .with_message_audit()
        .with_c2c_group_at_messages();

    // Choose your bot implementation
    let handler = ComprehensiveBot::new();
    // let handler = MonitoredBot::new(); // Alternative with statistics

    let mut client = Client::new(token, intents, handler, false)?;

    info!("🎯 Event handling bot starting with comprehensive intents...");
    client.start().await?;

    Ok(())
}
```

## Usage Examples

### Basic Commands

```
# Test basic functionality
!ping

# Get server information
!server

# List channels
!channels

# Get bot statistics (with MonitoredBot)
!stats

# Echo messages
!echo Hello, world!
```

### Event Triggers

- **Message Events**: Send any message to trigger message_create
- **Guild Events**: Add/remove bot from servers
- **Member Events**: Users joining/leaving servers
- **Channel Events**: Creating/updating/deleting channels
- **Audio Events**: Users joining/leaving voice channels
- **Forum Events**: Creating/updating forum threads

## Best Practices

1. **Event Filtering**: Don't process unnecessary events to improve performance
2. **Error Handling**: Always handle errors gracefully in event handlers
3. **Logging**: Use structured logging to track event processing
4. **Rate Limiting**: Be mindful of API rate limits when responding to events
5. **Async Safety**: Use proper async patterns and avoid blocking operations
6. **Resource Management**: Clean up resources and avoid memory leaks
7. **Monitoring**: Track event statistics for performance insights

## Performance Considerations

- **Selective Processing**: Only process events you actually need
- **Batch Operations**: Group similar operations when possible
- **Caching**: Cache frequently accessed data
- **Background Tasks**: Use background tasks for heavy processing
- **Connection Pooling**: Reuse connections when making API calls

## Common Patterns

- **Command Routing**: Route different commands to specific handlers
- **Event Filtering**: Filter events based on content, user, or channel
- **State Management**: Track bot state across different events
- **Error Recovery**: Implement retry logic for failed operations
- **Audit Logging**: Log important events for compliance and debugging

## See Also

- [Command Handler](./command-handler.md) - Structured command processing
- [Interactive Messages](./interactive-messages.md) - User interaction patterns
- [Error Recovery](./error-recovery.md) - Advanced error handling
- [Getting Started](./getting-started.md) - Basic bot setup

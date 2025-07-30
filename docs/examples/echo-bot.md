# Echo Bot Example

This example demonstrates how to create a simple echo bot that responds to messages by repeating them back to the user.

## Overview

An echo bot is the simplest type of bot that demonstrates basic message handling. When a user sends a message, the bot responds with the same message content.

## Basic Echo Bot

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents};

struct EchoBot;

#[async_trait::async_trait]
impl EventHandler for EchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        // Skip bot messages to avoid loops
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        // Echo the message content
        if let Some(content) = &msg.content {
            let echo_msg = format!("Echo: {}", content);
            if let Err(e) = ctx.send_message(&msg.channel_id, &echo_msg).await {
                eprintln!("Failed to send echo message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();

    let bot = EchoBot;
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE | Intents::GUILDS)
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Enhanced Echo Bot with Commands

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents};

struct SmartEchoBot;

#[async_trait::async_trait]
impl EventHandler for SmartEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Smart echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        // Skip bot messages
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            match content.as_str() {
                "!ping" => {
                    let _ = ctx.send_message(&msg.channel_id, "Pong!").await;
                }
                "!help" => {
                    let help_text = "Available commands:\n• `!ping` - Test bot responsiveness\n• `!echo <message>` - Echo a custom message\n• Any other message will be echoed back";
                    let _ = ctx.send_message(&msg.channel_id, help_text).await;
                }
                _ if content.starts_with("!echo ") => {
                    let echo_content = &content[6..]; // Remove "!echo " prefix
                    let echo_msg = format!("You said: {}", echo_content);
                    let _ = ctx.send_message(&msg.channel_id, &echo_msg).await;
                }
                _ => {
                    // Echo regular messages
                    let echo_msg = format!("Echo: {}", content);
                    let _ = ctx.send_message(&msg.channel_id, &echo_msg).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let bot = SmartEchoBot;
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE | Intents::GUILDS)
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Echo Bot with Reply Support

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageParams};

struct ReplyEchoBot;

#[async_trait::async_trait]
impl EventHandler for ReplyEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Reply echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            // Create a reply to the original message
            let echo_content = format!("You said: {}", content);
            
            // Use reply functionality
            if let Err(e) = ctx.reply_message(&msg.channel_id, &msg.id, &echo_content).await {
                eprintln!("Failed to reply to message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let bot = ReplyEchoBot;
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE | Intents::GUILDS)
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Echo Bot with Rich Embeds

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, MessageEmbed};

struct RichEchoBot;

#[async_trait::async_trait]
impl EventHandler for RichEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Rich echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            // Create an embed for the echo response
            let embed = MessageEmbed::new()
                .title("Echo Response")
                .description(format!("You said: {}", content))
                .color(0x00ff00) // Green color
                .field("Original Message", content, false)
                .field("Channel", &msg.channel_id, true)
                .timestamp(chrono::Utc::now().to_rfc3339());

            if let Err(e) = ctx.send_message_with_embed(&msg.channel_id, None, &embed).await {
                eprintln!("Failed to send embed message: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let bot = RichEchoBot;
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE | Intents::GUILDS)
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Multi-Channel Echo Bot

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, DirectMessage, GroupMessage};

struct MultiChannelEchoBot;

#[async_trait::async_trait]
impl EventHandler for MultiChannelEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Multi-channel echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(content) = &msg.content {
            let echo_msg = format!("Guild Echo: {}", content);
            let _ = ctx.send_message(&msg.channel_id, &echo_msg).await;
        }
    }

    async fn direct_message_create(&self, ctx: Context, msg: DirectMessage) {
        if let Some(content) = &msg.content {
            let echo_msg = format!("DM Echo: {}", content);
            // For direct messages, we reply to the same channel
            if let Some(channel_id) = &msg.channel_id {
                let _ = ctx.send_message(channel_id, &echo_msg).await;
            }
        }
    }

    async fn group_message_create(&self, ctx: Context, msg: GroupMessage) {
        if let Some(content) = &msg.content {
            let echo_msg = format!("Group Echo: {}", content);
            if let Some(group_id) = &msg.group_openid {
                let _ = ctx.send_group_message(group_id, &echo_msg).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let bot = MultiChannelEchoBot;
    
    let mut client = Client::new("your_app_id", bot)
        .intents(
            Intents::PUBLIC_GUILD_MESSAGES 
            | Intents::DIRECT_MESSAGE 
            | Intents::PUBLIC_MESSAGES 
            | Intents::GUILDS
        )
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Echo Bot with Rate Limiting

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

struct RateLimitedEchoBot {
    last_message: Arc<Mutex<HashMap<String, Instant>>>,
}

impl RateLimitedEchoBot {
    fn new() -> Self {
        Self {
            last_message: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn can_respond(&self, user_id: &str) -> bool {
        let mut last_messages = self.last_message.lock().await;
        let now = Instant::now();
        
        if let Some(last_time) = last_messages.get(user_id) {
            if now.duration_since(*last_time) < Duration::from_secs(5) {
                return false; // Rate limited
            }
        }
        
        last_messages.insert(user_id.to_string(), now);
        true
    }
}

#[async_trait::async_trait]
impl EventHandler for RateLimitedEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Rate-limited echo bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return;
        }

        if let Some(author) = &msg.author {
            if let Some(user_id) = &author.id {
                // Check rate limit
                if !self.can_respond(user_id).await {
                    return; // Skip if rate limited
                }

                if let Some(content) = &msg.content {
                    let echo_msg = format!("Echo: {}", content);
                    if let Err(e) = ctx.send_message(&msg.channel_id, &echo_msg).await {
                        eprintln!("Failed to send echo message: {}", e);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let bot = RateLimitedEchoBot::new();
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE | Intents::GUILDS)
        .build()
        .await?;

    client.start().await?;
    Ok(())
}
```

## Configuration

Before running any of these examples, make sure to:

1. **Set up environment variables:**
   ```bash
   export QQ_BOT_APP_ID=your_app_id
   export QQ_BOT_SECRET=your_secret
   ```

2. **Add dependencies to Cargo.toml:**
   ```toml
   [dependencies]
   botrs = "0.2"
   tokio = { version = "1.0", features = ["full"] }
   async-trait = "0.1"
   tracing = "0.1"
   tracing-subscriber = "0.3"
   chrono = { version = "0.4", features = ["serde"] }
   ```

3. **Enable required intents in QQ Developer Portal:**
   - Public Guild Messages
   - Direct Messages (if using DM features)
   - Guild information

## Key Concepts Demonstrated

1. **Basic Message Handling**: Responding to incoming messages
2. **Bot Message Filtering**: Avoiding infinite loops with bot messages
3. **Command Processing**: Handling specific command patterns
4. **Reply Functionality**: Using message replies for better UX
5. **Rich Content**: Creating embed messages for enhanced presentation
6. **Multi-Channel Support**: Handling different message types
7. **Rate Limiting**: Preventing spam and abuse

## Next Steps

- [Command Handler](./command-handler.md) - Learn about structured command handling
- [Rich Messages](./rich-messages.md) - Explore advanced message formatting
- [Event Handling](./event-handling.md) - Handle more event types beyond messages
- [Error Recovery](./error-recovery.md) - Implement robust error handling
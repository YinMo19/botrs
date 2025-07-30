# Quick Start

This guide will help you create your first QQ Guild bot with BotRS in just a few minutes. By the end of this tutorial, you'll have a working bot that can respond to messages.

## Step 1: Set Up Your Project

First, create a new Rust project and add the necessary dependencies:

```bash
cargo new my-first-bot
cd my-first-bot
```

Edit your `Cargo.toml` to include BotRS and its dependencies:

```toml
[package]
name = "my-first-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
botrs = "0.2.5"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"
```

## Step 2: Get Your Bot Credentials

Before writing code, you'll need credentials from the QQ Guild Developer Portal:

1. Visit [QQ Guild Developer Portal](https://bot.q.qq.com/)
2. Create a new application or select an existing one
3. Copy your **App ID** and **Secret**

For this tutorial, you can set them as environment variables:

```bash
export QQ_BOT_APP_ID="your_app_id_here"
export QQ_BOT_SECRET="your_secret_here"
```

## Step 3: Write Your First Bot

Replace the contents of `src/main.rs` with the following code:

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

// Define your bot's event handler
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // Called when the bot successfully connects
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("🤖 Bot is ready! Logged in as: {}", ready.user.username);
    }

    // Called when someone mentions your bot in a message
    async fn message_create(&self, ctx: Context, message: Message) {
        // Ignore messages from other bots
        if message.is_from_bot() {
            return;
        }

        // Get the message content
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("📨 Received message: {}", content);

        // Respond to different commands
        let response = match content.trim() {
            "!ping" => "🏓 Pong!",
            "!hello" => "👋 Hello there!",
            "!help" => "🤖 Available commands: !ping, !hello, !help, !about",
            "!about" => "🦀 I'm a QQ bot built with BotRS - a Rust framework for QQ Guild bots!",
            _ => return, // Don't respond to other messages
        };

        // Send the response
        match message.reply(&ctx.api, &ctx.token, response).await {
            Ok(_) => info!("✅ Reply sent successfully"),
            Err(e) => warn!("❌ Failed to send reply: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,my_first_bot=info")
        .init();

    info!("🚀 Starting bot...");

    // Get credentials from environment variables
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("QQ_BOT_APP_ID environment variable not set");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("QQ_BOT_SECRET environment variable not set");

    // Create authentication token
    let token = Token::new(app_id, secret);

    // Configure which events your bot wants to receive
    let intents = Intents::default()
        .with_public_guild_messages()  // Receive @ mentions
        .with_guilds();                // Receive guild events

    // Create the bot client
    let mut client = Client::new(token, intents, MyBot, true)?;

    info!("🔌 Connecting to QQ Guild...");

    // Start the bot (this will run until the program is stopped)
    client.start().await?;

    Ok(())
}
```

## Step 4: Run Your Bot

Now run your bot:

```bash
cargo run
```

You should see output similar to:

```
2024-01-01T12:00:00.000Z  INFO my_first_bot: 🚀 Starting bot...
2024-01-01T12:00:00.100Z  INFO my_first_bot: 🔌 Connecting to QQ Guild...
2024-01-01T12:00:01.200Z  INFO my_first_bot: 🤖 Bot is ready! Logged in as: MyBot
```

## Step 5: Test Your Bot

1. Add your bot to a QQ Guild (server)
2. In a channel where your bot has permissions, try these commands:
   - `@YourBot !ping` - Bot should respond with "🏓 Pong!"
   - `@YourBot !hello` - Bot should respond with "👋 Hello there!"
   - `@YourBot !help` - Bot should show available commands

## Understanding the Code

Let's break down what's happening in your bot:

### Event Handler
```rust
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // Your event handling methods go here
}
```

The `EventHandler` trait defines how your bot responds to different events. You only need to implement the events you care about.

### Ready Event
```rust
async fn ready(&self, _ctx: Context, ready: Ready) {
    info!("🤖 Bot is ready! Logged in as: {}", ready.user.username);
}
```

This is called once when your bot successfully connects and is ready to receive events.

### Message Event
```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // Handle incoming messages
}
```

This is called whenever someone mentions your bot in a message. The `ctx` parameter provides access to the API client and authentication token.

### Intents
```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_guilds();
```

Intents control which events your bot receives. This helps optimize performance by only subscribing to events you need.

## Next Steps

Congratulations! You've created your first QQ Guild bot with BotRS. Here are some ideas for expanding your bot:

### Add More Commands
```rust
let response = match content.trim() {
    "!ping" => "🏓 Pong!",
    "!time" => &format!("⏰ Current time: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")),
    "!random" => &format!("🎲 Random number: {}", rand::random::<u32>() % 100),
    // ... more commands
};
```

### Handle Different Message Types
```rust
// Handle group messages
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    // Handle group chat messages
}

// Handle direct messages
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    // Handle private messages
}
```

### Send Rich Messages
```rust
use botrs::models::message::{MessageParams, MessageEmbed};

let embed = MessageEmbed {
    title: Some("Hello from BotRS!".to_string()),
    description: Some("This is a rich embed message".to_string()),
    color: Some(0x00ff00),
    ..Default::default()
};

let params = MessageParams::new_embed(embed);
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
```

## Troubleshooting

### Common Issues

**Bot doesn't respond to messages:**
- Make sure your bot has the proper permissions in the guild
- Verify that you're mentioning the bot (@BotName command)
- Check that `public_guild_messages` intent is enabled

**Authentication errors:**
- Double-check your App ID and Secret
- Ensure environment variables are set correctly
- Verify your bot is properly configured in the QQ Guild Developer Portal

**Connection issues:**
- Check your internet connection
- Verify that QQ Guild services are operational
- Look for firewall or proxy issues

### Getting Help

If you run into issues:

1. Check the [Examples](/examples/getting-started) for more code samples
2. Read the [API Reference](/api/client) for detailed documentation
3. Visit the [GitHub repository](https://github.com/YinMo19/botrs) for issues and discussions

## What's Next?

Now that you have a basic bot running, explore these guides to learn more:

- **[Configuration](/guide/configuration)** - Learn about advanced configuration options
- **[Messages & Responses](/guide/messages)** - Discover all the ways to send messages
- **[Error Handling](/guide/error-handling)** - Build robust, production-ready bots
- **[Examples](/examples/getting-started)** - See more complex bot implementations

Happy bot building! 🤖✨
# Getting Started Examples

This page provides practical examples to help you get started with BotRS. Each example builds upon the previous one, demonstrating core concepts and common patterns.

## Basic Echo Bot

A simple bot that echoes back messages when mentioned.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

struct EchoBot;

#[async_trait::async_trait]
impl EventHandler for EchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Echo bot is ready! Logged in as: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // Echo back the message with a prefix
            let echo_response = format!("Echo: {}", content);
            
            match message.reply(&ctx.api, &ctx.token, &echo_response).await {
                Ok(_) => info!("Echoed message: {}", content),
                Err(e) => warn!("Failed to echo message: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,echo_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("QQ_BOT_APP_ID not set"),
        std::env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET not set"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, EchoBot, false)?;

    client.start().await?;
    Ok(())
}
```

## Command Handler Bot

A more sophisticated bot that handles multiple commands with different responses.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

struct CommandBot;

impl CommandBot {
    fn handle_command(&self, command: &str, args: &[&str]) -> Option<String> {
        match command {
            "ping" => Some("Pong! 🏓".to_string()),
            "hello" => Some("Hello there! 👋".to_string()),
            "time" => {
                let now = chrono::Utc::now();
                Some(format!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S UTC")))
            }
            "echo" => {
                if args.is_empty() {
                    Some("Usage: !echo <message>".to_string())
                } else {
                    Some(args.join(" "))
                }
            }
            "help" => Some(
                "Available commands:\n\
                • !ping - Test bot responsiveness\n\
                • !hello - Get a greeting\n\
                • !time - Get current time\n\
                • !echo <message> - Echo a message\n\
                • !help - Show this help message"
                    .to_string(),
            ),
            _ => Some(format!("Unknown command: {}. Type !help for available commands.", command)),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Command bot is ready! Logged in as: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            let content = content.trim();
            
            // Check if message starts with command prefix
            if let Some(command_text) = content.strip_prefix('!') {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if parts.is_empty() {
                    return;
                }

                let command = parts[0];
                let args = &parts[1..];

                info!("Processing command: {} with args: {:?}", command, args);

                if let Some(response) = self.handle_command(command, args) {
                    match message.reply(&ctx.api, &ctx.token, &response).await {
                        Ok(_) => info!("Command {} executed successfully", command),
                        Err(e) => warn!("Failed to respond to command {}: {}", command, e),
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,command_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("QQ_BOT_APP_ID not set"),
        std::env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET not set"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, CommandBot, false)?;

    client.start().await?;
    Ok(())
}
```

## Multi-Event Bot

A bot that handles multiple types of events including guild and member events.

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token,
    Guild, Channel, Member, GroupMessage, DirectMessage
};
use tracing::{info, warn};

struct MultiEventBot;

#[async_trait::async_trait]
impl EventHandler for MultiEventBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Multi-event bot is ready!");
        info!("Bot user: {}", ready.user.username);
        info!("Connected to {} guilds", ready.guilds.len());
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content == "!serverinfo" {
                let response = format!(
                    "Guild message received in channel: {}",
                    message.channel_id.as_deref().unwrap_or("Unknown")
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
        }
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            if content == "!groupinfo" {
                let response = format!(
                    "Group message in: {}",
                    message.group_openid.as_deref().unwrap_or("Unknown group")
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
        }
    }

    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            let response = format!("You said: {}", content);
            let _ = message.reply(&ctx.api, &ctx.token, &response).await;
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        info!(
            "Joined guild: {} (ID: {})",
            guild.name.as_deref().unwrap_or("Unknown"),
            guild.id.as_deref().unwrap_or("Unknown")
        );
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!(
                "New member joined: {}",
                user.username.as_deref().unwrap_or("Unknown")
            );

            // You could send a welcome message here
            // Note: You'd need to know the welcome channel ID
            // let welcome_msg = format!("Welcome to the server, {}!", 
            //                          user.username.as_deref().unwrap_or("friend"));
        }
    }

    async fn channel_create(&self, _ctx: Context, channel: Channel) {
        info!(
            "New channel created: {} (Type: {:?})",
            channel.name.as_deref().unwrap_or("Unnamed"),
            channel.type_
        );
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("Bot error occurred: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,multi_event_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("QQ_BOT_APP_ID not set"),
        std::env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET not set"),
    );

    // Subscribe to multiple event types
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds()
        .with_guild_members();

    let mut client = Client::new(token, intents, MultiEventBot, false)?;

    info!("Starting multi-event bot...");
    client.start().await?;
    Ok(())
}
```

## Stateful Bot with Data Storage

A bot that maintains state and tracks user interactions.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct UserStats {
    message_count: u64,
    last_message: chrono::DateTime<chrono::Utc>,
    first_seen: chrono::DateTime<chrono::Utc>,
}

struct StatefulBot {
    user_stats: Arc<RwLock<HashMap<String, UserStats>>>,
}

impl StatefulBot {
    fn new() -> Self {
        Self {
            user_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn update_user_stats(&self, user_id: &str) {
        let mut stats = self.user_stats.write().await;
        let now = chrono::Utc::now();

        match stats.get_mut(user_id) {
            Some(user_stat) => {
                user_stat.message_count += 1;
                user_stat.last_message = now;
            }
            None => {
                stats.insert(
                    user_id.to_string(),
                    UserStats {
                        message_count: 1,
                        last_message: now,
                        first_seen: now,
                    },
                );
            }
        }
    }

    async fn get_user_stats(&self, user_id: &str) -> Option<UserStats> {
        let stats = self.user_stats.read().await;
        stats.get(user_id).cloned()
    }
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Stateful bot is ready! Logged in as: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // Update user statistics
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                self.update_user_stats(user_id).await;

                if let Some(content) = &message.content {
                    match content.trim() {
                        "!stats" => {
                            if let Some(stats) = self.get_user_stats(user_id).await {
                                let response = format!(
                                    "Your statistics:\n\
                                    • Messages sent: {}\n\
                                    • First seen: {}\n\
                                    • Last message: {}",
                                    stats.message_count,
                                    stats.first_seen.format("%Y-%m-%d %H:%M:%S UTC"),
                                    stats.last_message.format("%Y-%m-%d %H:%M:%S UTC")
                                );
                                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                            }
                        }
                        "!leaderboard" => {
                            let stats = self.user_stats.read().await;
                            let mut sorted_users: Vec<_> = stats.iter().collect();
                            sorted_users.sort_by(|a, b| b.1.message_count.cmp(&a.1.message_count));

                            let mut response = "Message Leaderboard:\n".to_string();
                            for (i, (user_id, user_stats)) in sorted_users.iter().take(5).enumerate() {
                                response.push_str(&format!(
                                    "{}. User {}: {} messages\n",
                                    i + 1,
                                    &user_id[..8], // Show first 8 chars of user ID
                                    user_stats.message_count
                                ));
                            }

                            let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                        }
                        "!reset" => {
                            self.user_stats.write().await.remove(user_id);
                            let _ = message.reply(&ctx.api, &ctx.token, "Your statistics have been reset!").await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,stateful_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("QQ_BOT_APP_ID not set"),
        std::env::var("QQ_BOT_SECRET").expect("QQ_BOT_SECRET not set"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, StatefulBot::new(), false)?;

    info!("Starting stateful bot...");
    client.start().await?;
    Ok(())
}
```

## Configuration-Based Bot

A bot that loads configuration from files and environment variables.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct BotConfig {
    bot: BotSettings,
    commands: CommandSettings,
    logging: LoggingSettings,
}

#[derive(Debug, Deserialize, Serialize)]
struct BotSettings {
    app_id: String,
    secret: String,
    sandbox: bool,
    command_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CommandSettings {
    enabled: Vec<String>,
    admin_only: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingSettings {
    level: String,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            bot: BotSettings {
                app_id: "your_app_id".to_string(),
                secret: "your_secret".to_string(),
                sandbox: false,
                command_prefix: "!".to_string(),
            },
            commands: CommandSettings {
                enabled: vec!["ping".to_string(), "help".to_string()],
                admin_only: vec!["reload".to_string()],
            },
            logging: LoggingSettings {
                level: "info".to_string(),
            },
        }
    }
}

struct ConfigurableBot {
    config: BotConfig,
}

impl ConfigurableBot {
    fn new(config: BotConfig) -> Self {
        Self { config }
    }

    fn is_command_enabled(&self, command: &str) -> bool {
        self.config.commands.enabled.contains(&command.to_string())
    }

    fn handle_command(&self, command: &str, _args: &[&str]) -> Option<String> {
        if !self.is_command_enabled(command) {
            return Some("This command is disabled.".to_string());
        }

        match command {
            "ping" => Some("Pong!".to_string()),
            "help" => {
                let enabled_commands = self.config.commands.enabled.join(", ");
                Some(format!("Available commands: {}", enabled_commands))
            }
            "config" => Some(format!(
                "Bot configuration:\n\
                • Command prefix: {}\n\
                • Sandbox mode: {}\n\
                • Enabled commands: {}",
                self.config.bot.command_prefix,
                self.config.bot.sandbox,
                self.config.commands.enabled.join(", ")
            )),
            _ => Some("Unknown command.".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ConfigurableBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Configurable bot is ready! Logged in as: {}", ready.user.username);
        info!("Using command prefix: {}", self.config.bot.command_prefix);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            let content = content.trim();

            if let Some(command_text) = content.strip_prefix(&self.config.bot.command_prefix) {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if parts.is_empty() {
                    return;
                }

                let command = parts[0];
                let args = &parts[1..];

                if let Some(response) = self.handle_command(command, args) {
                    match message.reply(&ctx.api, &ctx.token, &response).await {
                        Ok(_) => info!("Responded to command: {}", command),
                        Err(e) => warn!("Failed to respond to command {}: {}", command, e),
                    }
                }
            }
        }
    }
}

fn load_config() -> Result<BotConfig, Box<dyn std::error::Error>> {
    // Try to load from file first
    if let Ok(config_content) = fs::read_to_string("config.toml") {
        let mut config: BotConfig = toml::from_str(&config_content)?;
        
        // Override with environment variables if present
        if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
            config.bot.app_id = app_id;
        }
        if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
            config.bot.secret = secret;
        }
        
        Ok(config)
    } else {
        // Create default config and save it
        let default_config = BotConfig::default();
        let config_content = toml::to_string_pretty(&default_config)?;
        fs::write("config.toml", config_content)?;
        
        info!("Created default config.toml - please update it with your bot credentials");
        
        // Still try to use environment variables
        let mut config = default_config;
        if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
            config.bot.app_id = app_id;
        }
        if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
            config.bot.secret = secret;
        }
        
        Ok(config)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    tracing_subscriber::fmt()
        .with_env_filter(format!("botrs={},configurable_bot={}", config.logging.level, config.logging.level))
        .init();

    let token = Token::new(config.bot.app_id.clone(), config.bot.secret.clone());
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, ConfigurableBot::new(config), false)?;

    info!("Starting configurable bot...");
    client.start().await?;
    Ok(())
}
```

## Environment Setup

For all examples above, you'll need to set up your environment:

### Environment Variables

```bash
export QQ_BOT_APP_ID="your_app_id_here"
export QQ_BOT_SECRET="your_secret_here"
export RUST_LOG="botrs=info"
```

### Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
botrs = "0.2.5"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"

# For configuration example
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# For stateful example
chrono = { version = "0.4", features = ["serde"] }
```

### Running the Examples

1. Set your environment variables
2. Copy the example code to `src/main.rs`
3. Run with `cargo run`

## Next Steps

These examples demonstrate the core patterns for building QQ Guild bots with BotRS. To learn more:

- [Rich Messages](./rich-messages.md) - Send embeds, files, and interactive content
- [Error Handling](./error-recovery.md) - Build robust, production-ready bots
- [API Integration](./api-integration.md) - Use the full QQ Guild API
- [Event Handling](./event-handling.md) - Handle all types of guild events
# Client & Event Handler

This guide covers the core concepts of BotRS: the `Client` and `EventHandler`. These two components form the foundation of every bot application, handling connections, authentication, and event processing.

## Understanding the Client

The `Client` is the main orchestrator of your bot. It manages the WebSocket connection to QQ's servers, handles authentication, and dispatches events to your event handler.

### Client Lifecycle

```rust
use botrs::{Client, EventHandler, Intents, Token};

// 1. Create token with credentials
let token = Token::new("your_app_id", "your_secret");

// 2. Configure intents (what events to receive)
let intents = Intents::default().with_public_guild_messages();

// 3. Create your event handler
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // Define how to handle events
}

// 4. Create and start the client
let mut client = Client::new(token, intents, MyBot, false)?;
client.start().await?; // This blocks until the bot stops
```

### Client Configuration

#### Environment Selection

```rust
// Production environment
let client = Client::new(token, intents, handler, false)?;

// Sandbox environment (for testing)
let client = Client::new(token, intents, handler, true)?;
```

#### Connection Management

The client automatically handles:
- WebSocket connection establishment
- Authentication with QQ servers
- Heartbeat maintenance
- Automatic reconnection on network issues
- Rate limiting compliance

## Understanding Event Handlers

The `EventHandler` trait defines how your bot responds to events from QQ Guild. You implement this trait to define your bot's behavior.

### Basic Event Handler

```rust
use botrs::{Context, EventHandler, Message, Ready};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // Called once when bot connects
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready!", ready.user.username);
    }

    // Called when someone mentions your bot
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}
```

### Event Handler with State

For more complex bots, you can maintain state within your event handler:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

struct StatefulBot {
    // Shared state between events
    user_data: Arc<RwLock<HashMap<String, UserInfo>>>,
    config: BotConfig,
}

impl StatefulBot {
    fn new(config: BotConfig) -> Self {
        Self {
            user_data: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    async fn get_user_info(&self, user_id: &str) -> Option<UserInfo> {
        let data = self.user_data.read().await;
        data.get(user_id).cloned()
    }
    
    async fn update_user_info(&self, user_id: String, info: UserInfo) {
        let mut data = self.user_data.write().await;
        data.insert(user_id, info);
    }
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        // Access shared state
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                // Update user information
                let info = UserInfo {
                    last_message: chrono::Utc::now(),
                    message_count: self.get_user_info(user_id)
                        .await
                        .map(|u| u.message_count + 1)
                        .unwrap_or(1),
                };
                self.update_user_info(user_id.clone(), info).await;
            }
        }
    }
}
```

## The Context Parameter

Every event handler method receives a `Context` parameter that provides access to essential bot functionality:

```rust
pub struct Context {
    pub api: BotApi,     // API client for making requests
    pub token: Token,    // Authentication token
    // Additional context data...
}
```

### Using Context

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // Send a message
    let params = MessageParams::new_text("Hello!");
    ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
    
    // Get guild information
    let guild = ctx.api.get_guild(&ctx.token, &guild_id).await?;
    
    // Manage channel permissions
    ctx.api.modify_channel_permissions(&ctx.token, &channel_id, &permissions).await?;
}
```

## Event Types

### Core Events

#### Ready Event
```rust
async fn ready(&self, ctx: Context, ready: Ready) {
    // Bot is connected and ready
    // Access bot user info: ready.user
    // Access initial guild list: ready.guilds
}
```

#### Message Events
```rust
// Guild message with @mention
async fn message_create(&self, ctx: Context, message: Message) {
    // Handle @ mentions in guild channels
}

// Direct messages
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    // Handle private messages
}

// Group messages
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    // Handle group chat messages
}
```

### Guild Events

```rust
// Guild lifecycle
async fn guild_create(&self, ctx: Context, guild: Guild) {
    // Bot joined a guild or guild became available
}

async fn guild_update(&self, ctx: Context, guild: Guild) {
    // Guild information changed
}

async fn guild_delete(&self, ctx: Context, guild: Guild) {
    // Bot left guild or guild became unavailable
}
```

### Channel Events

```rust
async fn channel_create(&self, ctx: Context, channel: Channel) {
    // New channel created
}

async fn channel_update(&self, ctx: Context, channel: Channel) {
    // Channel updated
}

async fn channel_delete(&self, ctx: Context, channel: Channel) {
    // Channel deleted
}
```

### Member Events

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {
    // New member joined
}

async fn guild_member_update(&self, ctx: Context, member: Member) {
    // Member information updated
}

async fn guild_member_remove(&self, ctx: Context, member: Member) {
    // Member left or was removed
}
```

## Error Handling in Event Handlers

### Basic Error Handling

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        match self.process_command(content).await {
            Ok(response) => {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("Failed to send reply: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error processing command: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "Sorry, something went wrong!").await;
            }
        }
    }
}
```

### Centralized Error Handling

```rust
async fn error(&self, error: BotError) {
    match error {
        BotError::Network(e) => {
            eprintln!("Network error: {}", e);
            // Maybe implement reconnection logic
        }
        BotError::RateLimited(info) => {
            println!("Rate limited for {} seconds", info.retry_after);
            // Wait and retry logic
        }
        BotError::Authentication(e) => {
            eprintln!("Auth error: {}", e);
            // Handle authentication issues
        }
        _ => {
            eprintln!("Unexpected error: {}", error);
        }
    }
}
```

## Best Practices

### Performance

1. **Keep event handlers lightweight**
   ```rust
   async fn message_create(&self, ctx: Context, message: Message) {
       // Spawn heavy work in background
       let api = ctx.api.clone();
       let token = ctx.token.clone();
       
       tokio::spawn(async move {
           // Heavy computation here
           let result = heavy_computation().await;
           // Send result back to channel
       });
   }
   ```

2. **Use appropriate data structures for state**
   ```rust
   // For read-heavy workloads
   use std::sync::Arc;
   use tokio::sync::RwLock;
   
   // For simple atomic operations
   use std::sync::atomic::{AtomicU64, Ordering};
   
   // For concurrent collections
   use dashmap::DashMap;
   ```

### Error Recovery

1. **Graceful degradation**
   ```rust
   async fn message_create(&self, ctx: Context, message: Message) {
       match self.get_user_permissions(&ctx, &message).await {
           Ok(perms) if perms.can_execute_commands() => {
               // Execute command
           }
           Ok(_) => {
               // User doesn't have permission
               let _ = message.reply(&ctx.api, &ctx.token, "Permission denied").await;
           }
           Err(_) => {
               // Fallback: allow command but log the error
               eprintln!("Failed to check permissions, allowing command");
           }
       }
   }
   ```

2. **Retry logic for transient failures**
   ```rust
   async fn send_with_retry(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), BotError> {
       for attempt in 1..=3 {
           match ctx.api.post_message_with_params(
               &ctx.token, 
               channel_id, 
               MessageParams::new_text(content)
           ).await {
               Ok(response) => return Ok(()),
               Err(BotError::Network(_)) if attempt < 3 => {
                   tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                   continue;
               }
               Err(e) => return Err(e),
           }
       }
       unreachable!()
   }
   ```

### Resource Management

1. **Limit concurrent operations**
   ```rust
   use tokio::sync::Semaphore;
   
   struct MyBot {
       semaphore: Arc<Semaphore>,
   }
   
   impl MyBot {
       fn new() -> Self {
           Self {
               semaphore: Arc::new(Semaphore::new(10)), // Max 10 concurrent operations
           }
       }
   }
   
   #[async_trait::async_trait]
   impl EventHandler for MyBot {
       async fn message_create(&self, ctx: Context, message: Message) {
           let _permit = self.semaphore.acquire().await.unwrap();
           // Process message with limited concurrency
       }
   }
   ```

## Complete Example

Here's a comprehensive example that demonstrates these concepts:

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token, BotError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Clone)]
struct UserStats {
    message_count: u64,
    last_active: chrono::DateTime<chrono::Utc>,
}

struct ComprehensiveBot {
    stats: Arc<RwLock<HashMap<String, UserStats>>>,
    start_time: chrono::DateTime<chrono::Utc>,
}

impl ComprehensiveBot {
    fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(HashMap::new())),
            start_time: chrono::Utc::now(),
        }
    }
    
    async fn update_user_stats(&self, user_id: &str) {
        let mut stats = self.stats.write().await;
        let entry = stats.entry(user_id.to_string()).or_insert(UserStats {
            message_count: 0,
            last_active: chrono::Utc::now(),
        });
        entry.message_count += 1;
        entry.last_active = chrono::Utc::now();
    }
    
    async fn handle_command(&self, ctx: &Context, message: &Message, command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            "ping" => Ok("Pong! 🏓".to_string()),
            "uptime" => {
                let uptime = chrono::Utc::now() - self.start_time;
                Ok(format!("Bot uptime: {} seconds", uptime.num_seconds()))
            }
            "stats" => {
                if let Some(author) = &message.author {
                    if let Some(user_id) = &author.id {
                        let stats = self.stats.read().await;
                        if let Some(user_stats) = stats.get(user_id) {
                            Ok(format!("Messages sent: {}, Last active: {}", 
                                     user_stats.message_count, 
                                     user_stats.last_active.format("%Y-%m-%d %H:%M:%S")))
                        } else {
                            Ok("No stats available".to_string())
                        }
                    } else {
                        Ok("Could not identify user".to_string())
                    }
                } else {
                    Ok("No author information".to_string())
                }
            }
            "help" => Ok("Available commands: !ping, !uptime, !stats, !help".to_string()),
            _ => Ok(format!("Unknown command: {}. Type !help for available commands.", command)),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ComprehensiveBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("🤖 Bot is ready! Logged in as: {}", ready.user.username);
        info!("📊 Connected to {} guilds", ready.guilds.len());
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        // Skip bot messages
        if message.is_from_bot() {
            return;
        }
        
        // Update user statistics
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                self.update_user_stats(user_id).await;
            }
        }
        
        // Process commands
        if let Some(content) = &message.content {
            let content = content.trim();
            if let Some(command_text) = content.strip_prefix('!') {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if !parts.is_empty() {
                    let command = parts[0];
                    let args = &parts[1..];
                    
                    match self.handle_command(&ctx, &message, command, args).await {
                        Ok(response) => {
                            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                                warn!("Failed to send reply: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Error handling command '{}': {}", command, e);
                            let _ = message.reply(&ctx.api, &ctx.token, "Sorry, something went wrong!").await;
                        }
                    }
                }
            }
        }
    }
    
    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        info!("📥 Joined guild: {}", guild.name.as_deref().unwrap_or("Unknown"));
    }
    
    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        info!("📤 Left guild: {}", guild.name.as_deref().unwrap_or("Unknown"));
    }
    
    async fn error(&self, error: BotError) {
        match error {
            BotError::Network(ref e) => {
                warn!("🌐 Network error: {}", e);
            }
            BotError::RateLimited(ref info) => {
                warn!("⏰ Rate limited for {} seconds", info.retry_after);
            }
            BotError::Authentication(ref e) => {
                error!("🔐 Authentication error: {}", e);
            }
            _ => {
                error!("❌ Unexpected error: {}", error);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,comprehensive_bot=info")
        .init();
    
    // Load configuration
    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID")?,
        std::env::var("QQ_BOT_SECRET")?,
    );
    
    // Configure intents
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds();
    
    // Create and start bot
    let mut client = Client::new(token, intents, ComprehensiveBot::new(), false)?;
    
    info!("🚀 Starting comprehensive bot...");
    client.start().await?;
    
    Ok(())
}
```

This example demonstrates:
- Stateful event handling with user statistics
- Command processing with error handling
- Proper logging and monitoring
- Resource management with async operations
- Comprehensive event coverage

## Next Steps

- [Messages & Responses](./messages.md) - Learn about sending different types of messages
- [Intents System](./intents.md) - Understand event filtering and permissions
- [Configuration](./configuration.md) - Advanced configuration options
- [Error Handling](./error-handling.md) - Robust error handling patterns
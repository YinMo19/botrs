# EventHandler API Reference

The `EventHandler` trait defines how your bot responds to events from the QQ Guild gateway. You implement this trait to handle messages, guild changes, member updates, and other events.

## Overview

```rust
use botrs::{Context, EventHandler};

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    // Event handler methods...
}
```

All event handler methods are optional - you only need to implement the events your bot cares about. Each method receives a `Context` parameter providing access to the API client and authentication token.

## Core Events

### `ready`

Called when the bot successfully connects and is ready to receive events.

```rust
async fn ready(&self, ctx: Context, ready: Ready) {}
```

#### Parameters

- `ctx`: Context containing API client and token
- `ready`: Information about the bot user and session

#### Example

```rust
async fn ready(&self, _ctx: Context, ready: Ready) {
    println!("Bot is ready! Logged in as: {}", ready.user.username);
    println!("Session ID: {}", ready.session_id);
    println!("Connected to {} guilds", ready.guilds.len());
}
```

### `error`

Called when an error occurs during event processing.

```rust
async fn error(&self, error: BotError) {}
```

#### Parameters

- `error`: The error that occurred

#### Example

```rust
async fn error(&self, error: BotError) {
    eprintln!("Event handler error: {}", error);
    
    match error {
        BotError::Network(_) => {
            // Handle network errors
        }
        BotError::RateLimited(info) => {
            println!("Rate limited for {} seconds", info.retry_after);
        }
        _ => {}
    }
}
```

## Message Events

### `message_create`

Called when a message is created that mentions your bot (@mentions in guild channels).

```rust
async fn message_create(&self, ctx: Context, message: Message) {}
```

#### Parameters

- `ctx`: Context for API access
- `message`: The message that was created

#### Example

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // Ignore bot messages
    if message.is_from_bot() {
        return;
    }
    
    let content = match &message.content {
        Some(content) => content,
        None => return,
    };
    
    if content.starts_with("!echo ") {
        let echo_text = &content[6..];
        let _ = message.reply(&ctx.api, &ctx.token, echo_text).await;
    }
}
```

### `direct_message_create`

Called when a direct message is created.

```rust
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {}
```

#### Parameters

- `ctx`: Context for API access
- `message`: The direct message

#### Example

```rust
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    if let Some(content) = &message.content {
        println!("Direct message from {}: {}", 
                 message.author.as_ref()
                     .and_then(|a| a.username.as_deref())
                     .unwrap_or("Unknown"), 
                 content);
        
        // Echo back the message
        let _ = message.reply(&ctx.api, &ctx.token, &format!("You said: {}", content)).await;
    }
}
```

### `group_message_create`

Called when a group message is created.

```rust
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {}
```

#### Parameters

- `ctx`: Context for API access
- `message`: The group message

#### Example

```rust
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    if let Some(content) = &message.content {
        if content == "!groupinfo" {
            let info = format!("Group ID: {}", message.group_openid.as_deref().unwrap_or("Unknown"));
            let _ = message.reply(&ctx.api, &ctx.token, &info).await;
        }
    }
}
```

### `c2c_message_create`

Called when a C2C (client-to-client) message is created.

```rust
async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {}
```

#### Parameters

- `ctx`: Context for API access
- `message`: The C2C message

#### Example

```rust
async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
    if let Some(content) = &message.content {
        println!("C2C message: {}", content);
        // Handle private conversation messages
    }
}
```

### `message_delete`

Called when a message is deleted.

```rust
async fn message_delete(&self, ctx: Context, message: Message) {}
```

#### Parameters

- `ctx`: Context for API access
- `message`: Information about the deleted message

#### Example

```rust
async fn message_delete(&self, _ctx: Context, message: Message) {
    println!("Message deleted in channel {}", 
             message.channel_id.as_deref().unwrap_or("Unknown"));
}
```

## Guild Events

### `guild_create`

Called when the bot joins a guild or when a guild becomes available.

```rust
async fn guild_create(&self, ctx: Context, guild: Guild) {}
```

#### Parameters

- `ctx`: Context for API access
- `guild`: The guild information

#### Example

```rust
async fn guild_create(&self, _ctx: Context, guild: Guild) {
    println!("Joined guild: {} (ID: {})", 
             guild.name.as_deref().unwrap_or("Unknown"),
             guild.id.as_deref().unwrap_or("Unknown"));
}
```

### `guild_update`

Called when a guild is updated.

```rust
async fn guild_update(&self, ctx: Context, guild: Guild) {}
```

### `guild_delete`

Called when the bot leaves a guild or when a guild becomes unavailable.

```rust
async fn guild_delete(&self, ctx: Context, guild: Guild) {}
```

## Channel Events

### `channel_create`

Called when a channel is created.

```rust
async fn channel_create(&self, ctx: Context, channel: Channel) {}
```

#### Example

```rust
async fn channel_create(&self, _ctx: Context, channel: Channel) {
    println!("New channel created: {} (Type: {:?})", 
             channel.name.as_deref().unwrap_or("Unnamed"),
             channel.type_);
}
```

### `channel_update`

Called when a channel is updated.

```rust
async fn channel_update(&self, ctx: Context, channel: Channel) {}
```

### `channel_delete`

Called when a channel is deleted.

```rust
async fn channel_delete(&self, ctx: Context, channel: Channel) {}
```

## Member Events

### `guild_member_add`

Called when a member joins a guild.

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {}
```

#### Example

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {
    if let Some(user) = &member.user {
        println!("New member joined: {}", 
                 user.username.as_deref().unwrap_or("Unknown"));
        
        // Send welcome message to a specific channel
        if let Some(welcome_channel) = get_welcome_channel() {
            let welcome_msg = format!("Welcome to the server, {}!", 
                                    user.username.as_deref().unwrap_or("friend"));
            let params = MessageParams::new_text(&welcome_msg);
            let _ = ctx.api.post_message_with_params(&ctx.token, &welcome_channel, params).await;
        }
    }
}
```

### `guild_member_update`

Called when a guild member is updated.

```rust
async fn guild_member_update(&self, ctx: Context, member: Member) {}
```

### `guild_member_remove`

Called when a member leaves a guild.

```rust
async fn guild_member_remove(&self, ctx: Context, member: Member) {}
```

## Audit Events

### `message_audit_pass`

Called when a message passes audit review.

```rust
async fn message_audit_pass(&self, ctx: Context, audit: MessageAudit) {}
```

### `message_audit_reject`

Called when a message is rejected by audit review.

```rust
async fn message_audit_reject(&self, ctx: Context, audit: MessageAudit) {}
```

## Management Events

### Friend Management

```rust
async fn friend_add(&self, ctx: Context, event: C2CManageEvent) {}
async fn friend_del(&self, ctx: Context, event: C2CManageEvent) {}
async fn c2c_msg_reject(&self, ctx: Context, event: C2CManageEvent) {}
async fn c2c_msg_receive(&self, ctx: Context, event: C2CManageEvent) {}
```

### Group Management

```rust
async fn group_add_robot(&self, ctx: Context, event: GroupManageEvent) {}
async fn group_del_robot(&self, ctx: Context, event: GroupManageEvent) {}
async fn group_msg_reject(&self, ctx: Context, event: GroupManageEvent) {}
async fn group_msg_receive(&self, ctx: Context, event: GroupManageEvent) {}
```

## Implementation Examples

### Basic Event Handler

```rust
use botrs::{Context, EventHandler, Message, Ready};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready!", ready.user.username);
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }
        
        if let Some(content) = &message.content {
            match content.as_str() {
                "!ping" => {
                    let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
                }
                "!help" => {
                    let help_text = "Available commands: !ping, !help";
                    let _ = message.reply(&ctx.api, &ctx.token, help_text).await;
                }
                _ => {}
            }
        }
    }
}
```

### Advanced Event Handler with State

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

struct StatefulBot {
    user_data: Arc<RwLock<HashMap<String, UserData>>>,
}

struct UserData {
    message_count: u64,
    last_seen: chrono::DateTime<chrono::Utc>,
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                let mut user_data = self.user_data.write().await;
                let entry = user_data.entry(user_id.clone()).or_insert(UserData {
                    message_count: 0,
                    last_seen: chrono::Utc::now(),
                });
                
                entry.message_count += 1;
                entry.last_seen = chrono::Utc::now();
                
                if let Some(content) = &message.content {
                    if content == "!stats" {
                        let stats = format!("You have sent {} messages", entry.message_count);
                        let _ = message.reply(&ctx.api, &ctx.token, &stats).await;
                    }
                }
            }
        }
    }
}
```

### Error Handling

```rust
#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!error_test" {
                match message.reply(&ctx.api, &ctx.token, "Test reply").await {
                    Ok(_) => println!("Reply sent successfully"),
                    Err(e) => eprintln!("Failed to send reply: {}", e),
                }
            }
        }
    }
    
    async fn error(&self, error: BotError) {
        match error {
            BotError::RateLimited(info) => {
                println!("Rate limited for {} seconds", info.retry_after);
            }
            BotError::Network(e) => {
                eprintln!("Network error: {}", e);
            }
            _ => {
                eprintln!("Unexpected error: {}", error);
            }
        }
    }
}
```

## Best Practices

### Performance

- Keep event handlers lightweight - offload heavy work to background tasks
- Use `tokio::spawn` for CPU-intensive operations
- Avoid blocking operations in event handlers

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        if content.starts_with("!heavy_task") {
            // Spawn background task for heavy processing
            let api = ctx.api.clone();
            let token = ctx.token.clone();
            let channel_id = message.channel_id.clone();
            
            tokio::spawn(async move {
                // Heavy processing here
                let result = perform_heavy_computation().await;
                
                let params = MessageParams::new_text(&result);
                if let Some(channel) = channel_id {
                    let _ = api.post_message_with_params(&token, &channel, params).await;
                }
            });
        }
    }
}
```

### Error Recovery

- Always handle errors gracefully
- Log errors for debugging
- Provide fallback responses when possible

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    match self.process_message(&ctx, &message).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error processing message: {}", e);
            
            // Send error message to user
            let error_msg = "Sorry, something went wrong processing your request.";
            let _ = message.reply(&ctx.api, &ctx.token, error_msg).await;
        }
    }
}
```

## See Also

- [`Context`](./context.md) - API access in event handlers
- [`Client`](./client.md) - Main bot client
- [`Message Types`](./models/messages.md) - Message data structures
- [`Error Types`](./error-types.md) - Error handling
# API Client Usage

The BotRS API client provides direct access to QQ Guild's REST API endpoints, allowing you to perform operations beyond event handling. This guide covers how to use the API client effectively for message management, guild administration, and other bot operations.

## Overview

The `BotApi` client is the core interface for making HTTP requests to QQ Guild's API. It handles authentication, request formatting, and response parsing automatically.

```rust
use botrs::{BotApi, Token, MessageParams};

// API client is available through the context in event handlers
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // ctx.api is the BotApi instance
        // ctx.token is the authentication token
    }
}

// Or create a standalone client
let api = BotApi::new();
let token = Token::bot("your_bot_token");
```

## Basic API Operations

### Message Operations

#### Sending Messages

```rust
use botrs::{MessageParams, MessageEmbed};

// Simple text message
let params = MessageParams::new_text("Hello, world!");
let message = ctx.api.post_message_with_params(
    &ctx.token,
    &channel_id,
    params
).await?;

// Message with embed
let embed = MessageEmbed::new()
    .title("Bot Status")
    .description("All systems operational")
    .color(0x00ff00);

let params = MessageParams::new_text("Status Update")
    .with_embed(embed);

let message = ctx.api.post_message_with_params(
    &ctx.token,
    &channel_id,
    params
).await?;

// Reply to a message
let params = MessageParams::new_text("Thanks for your message!")
    .with_reply(&original_message.id);

let reply = ctx.api.post_message_with_params(
    &ctx.token,
    &channel_id,
    params
).await?;
```

#### Message Management

```rust
// Get a specific message
let message = ctx.api.get_message(
    &ctx.token,
    &channel_id,
    &message_id
).await?;

// Delete a message
ctx.api.delete_message(
    &ctx.token,
    &channel_id,
    &message_id,
    false // hidetip parameter
).await?;

// Get message history
let messages = ctx.api.get_messages(
    &ctx.token,
    &channel_id,
    Some(&MessageQuery {
        type_: Some(MessageType::Text),
        limit: Some(50),
        id: None,
    })
).await?;
```

### File and Media Operations

#### Uploading Files

```rust
use std::path::Path;

// Upload an image file
let file_path = Path::new("./image.png");
let file_info = ctx.api.post_file(
    &ctx.token,
    file_path,
    FileType::Image
).await?;

// Send message with uploaded file
let params = MessageParams::new_text("Check out this image!")
    .with_image(&file_info.url);

let message = ctx.api.post_message_with_params(
    &ctx.token,
    &channel_id,
    params
).await?;

// Upload and send file in one operation
let params = MessageParams::new_file(file_path)
    .with_text("Here's the document you requested");

let message = ctx.api.post_message_with_params(
    &ctx.token,
    &channel_id,
    params
).await?;
```

#### Rich Media Messages

```rust
// Audio message
let audio_file = Path::new("./audio.mp3");
let params = MessageParams::new_audio(audio_file)
    .with_text("Voice message");

// Video message
let video_file = Path::new("./video.mp4");
let params = MessageParams::new_video(video_file)
    .with_text("Video content");

// Markdown formatted message
let markdown_content = r#"
# Bot Report
- **Status**: Online
- **Uptime**: 24 hours
- **Messages Processed**: 1,234
"#;

let params = MessageParams::new_markdown(markdown_content);
```

## Guild and Channel Management

### Guild Information

```rust
// Get current user's guilds
let guilds = ctx.api.me_guilds(&ctx.token).await?;

// Get specific guild information
let guild = ctx.api.guild(&ctx.token, &guild_id).await?;
println!("Guild: {} (Members: {})", guild.name, guild.member_count);

// Get guild channels
let channels = ctx.api.guild_channels(&ctx.token, &guild_id).await?;
for channel in channels {
    println!("Channel: {} (Type: {:?})", channel.name, channel.type_);
}
```

### Channel Operations

```rust
// Get channel information
let channel = ctx.api.channel(&ctx.token, &channel_id).await?;

// Create a new channel
let new_channel = CreateChannel {
    name: "new-channel".to_string(),
    type_: ChannelType::Text,
    sub_type: ChannelSubType::Chat,
    position: None,
    parent_id: None,
    private_type: None,
    private_user_ids: None,
    speak_permission: None,
    application_id: None,
};

let created_channel = ctx.api.post_channel(
    &ctx.token,
    &guild_id,
    new_channel
).await?;

// Modify channel
let modify_channel = ModifyChannel {
    name: Some("updated-channel-name".to_string()),
    position: Some(1),
    ..Default::default()
};

let updated_channel = ctx.api.patch_channel(
    &ctx.token,
    &channel_id,
    modify_channel
).await?;

// Delete channel
ctx.api.delete_channel(&ctx.token, &channel_id).await?;
```

## Member and Permission Management

### Member Operations

```rust
// Get guild members
let members = ctx.api.guild_members(
    &ctx.token,
    &guild_id,
    Some(&MemberQuery {
        after: None,
        limit: Some(100),
    })
).await?;

// Get specific member
let member = ctx.api.guild_member(
    &ctx.token,
    &guild_id,
    &user_id
).await?;

// Remove member from guild
ctx.api.delete_guild_member(
    &ctx.token,
    &guild_id,
    &user_id,
    Some("Violation of community guidelines")
).await?;
```

### Role Management

```rust
// Get guild roles
let roles = ctx.api.guild_roles(&ctx.token, &guild_id).await?;

// Create a new role
let new_role = CreateRole {
    name: "Moderator".to_string(),
    color: Some(0x9932cc),
    hoist: Some(true),
    mentionable: Some(true),
    ..Default::default()
};

let created_role = ctx.api.post_guild_role(
    &ctx.token,
    &guild_id,
    new_role
).await?;

// Assign role to member
ctx.api.put_guild_member_role(
    &ctx.token,
    &guild_id,
    &user_id,
    &role_id,
    Some("Promoted to moderator")
).await?;

// Remove role from member
ctx.api.delete_guild_member_role(
    &ctx.token,
    &guild_id,
    &user_id,
    &role_id,
    Some("Role rotation")
).await?;
```

## Direct Messages and Private Channels

### Private Message Operations

```rust
// Create private message session
let dm_guild = ctx.api.create_direct_message_guild(
    &ctx.token,
    &CreateDirectMessageGuild {
        recipient_id: user_id.clone(),
        source_guild_id: guild_id.clone(),
    }
).await?;

// Send private message
let params = MessageParams::new_text("Hello! This is a private message.");
let dm_message = ctx.api.post_direct_message(
    &ctx.token,
    &dm_guild.guild_id,
    params
).await?;

// Send group message
let group_params = GroupMessageParams::new_text("Group announcement");
let group_message = ctx.api.post_group_message(
    &ctx.token,
    &group_id,
    group_params
).await?;

// Send C2C (user-to-user) message
let c2c_params = C2CMessageParams::new_text("Direct user message");
let c2c_message = ctx.api.post_c2c_message(
    &ctx.token,
    &openid,
    c2c_params
).await?;
```

## Advanced API Usage

### Batch Operations

```rust
use futures::future::try_join_all;

// Send messages to multiple channels concurrently
async fn broadcast_message(
    api: &BotApi,
    token: &Token,
    channel_ids: &[String],
    content: &str,
) -> Result<Vec<Message>> {
    let futures = channel_ids.iter().map(|channel_id| {
        let params = MessageParams::new_text(content);
        api.post_message_with_params(token, channel_id, params)
    });
    
    try_join_all(futures).await
}

// Usage
let channels = vec!["channel1".to_string(), "channel2".to_string()];
let messages = broadcast_message(
    &ctx.api,
    &ctx.token,
    &channels,
    "Important announcement!"
).await?;
```

### Pagination Handling

```rust
// Get all members with pagination
async fn get_all_members(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
) -> Result<Vec<Member>> {
    let mut all_members = Vec::new();
    let mut after = None;
    
    loop {
        let query = MemberQuery {
            after: after.clone(),
            limit: Some(100),
        };
        
        let members = api.guild_members(token, guild_id, Some(&query)).await?;
        
        if members.is_empty() {
            break;
        }
        
        // Update pagination cursor
        after = members.last().map(|m| m.user.id.clone());
        all_members.extend(members);
    }
    
    Ok(all_members)
}
```

### API Rate Limiting

```rust
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct RateLimitedApi {
    api: BotApi,
    semaphore: Arc<Semaphore>,
    delay: Duration,
}

impl RateLimitedApi {
    pub fn new(requests_per_second: usize) -> Self {
        Self {
            api: BotApi::new(),
            semaphore: Arc::new(Semaphore::new(requests_per_second)),
            delay: Duration::from_millis(1000 / requests_per_second as u64),
        }
    }
    
    pub async fn send_message_rate_limited(
        &self,
        token: &Token,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<Message> {
        let _permit = self.semaphore.acquire().await
            .map_err(|_| BotError::RateLimited)?;
        
        let result = self.api.post_message_with_params(token, channel_id, params).await;
        
        // Add delay between requests
        sleep(self.delay).await;
        
        result
    }
}
```

## Error Handling in API Calls

### Specific API Error Handling

```rust
use botrs::BotError;

async fn handle_api_errors(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    content: &str,
) -> Result<Message> {
    let params = MessageParams::new_text(content);
    
    match api.post_message_with_params(token, channel_id, params).await {
        Ok(message) => Ok(message),
        Err(BotError::Http(status)) => {
            match status.as_u16() {
                401 => {
                    tracing::error!("Invalid token");
                    Err(BotError::InvalidToken)
                }
                403 => {
                    tracing::warn!("Missing permissions for channel {}", channel_id);
                    Err(BotError::MissingPermissions)
                }
                404 => {
                    tracing::warn!("Channel {} not found", channel_id);
                    Err(BotError::ChannelNotFound)
                }
                429 => {
                    tracing::warn!("Rate limited, retrying after delay");
                    sleep(Duration::from_secs(1)).await;
                    let params = MessageParams::new_text(content);
                    api.post_message_with_params(token, channel_id, params).await
                }
                _ => Err(BotError::Http(status))
            }
        }
        Err(e) => Err(e)
    }
}
```

## Configuration and Customization

### Custom HTTP Client Configuration

```rust
use reqwest::ClientBuilder;
use std::time::Duration;

// Create API client with custom HTTP configuration
let http_client = ClientBuilder::new()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    .tcp_keepalive(Duration::from_secs(60))
    .pool_max_idle_per_host(10)
    .build()?;

let api = BotApi::with_client(http_client);
```

### Request Middleware

```rust
// Custom request interceptor
pub struct ApiInterceptor<T> {
    inner: T,
    request_id_generator: Arc<AtomicU64>,
}

impl<T> ApiInterceptor<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            request_id_generator: Arc::new(AtomicU64::new(0)),
        }
    }
    
    async fn make_request_with_logging<F, R>(&self, operation: F) -> Result<R>
    where
        F: Future<Output = Result<R>>,
    {
        let request_id = self.request_id_generator.fetch_add(1, Ordering::Relaxed);
        
        tracing::info!("API request started: {}", request_id);
        let start = Instant::now();
        
        let result = operation.await;
        
        let duration = start.elapsed();
        match &result {
            Ok(_) => tracing::info!(
                "API request completed: {} (took {:?})",
                request_id,
                duration
            ),
            Err(e) => tracing::error!(
                "API request failed: {} (took {:?}): {}",
                request_id,
                duration,
                e
            ),
        }
        
        result
    }
}
```

## Testing API Interactions

### Mock API for Testing

```rust
use mockall::predicate::*;
use mockall::mock;

mock! {
    ApiClient {}
    
    impl BotApiTrait for ApiClient {
        async fn post_message_with_params(
            &self,
            token: &Token,
            channel_id: &str,
            params: MessageParams,
        ) -> Result<Message>;
        
        async fn get_message(
            &self,
            token: &Token,
            channel_id: &str,
            message_id: &str,
        ) -> Result<Message>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_message_sending() {
        let mut mock_api = MockApiClient::new();
        
        mock_api
            .expect_post_message_with_params()
            .with(
                eq(Token::bot("test_token")),
                eq("test_channel"),
                predicate::always()
            )
            .times(1)
            .returning(|_, _, _| {
                Ok(Message {
                    id: "test_message_id".to_string(),
                    content: Some("Test message".to_string()),
                    channel_id: "test_channel".to_string(),
                    ..Default::default()
                })
            });
        
        let result = mock_api.post_message_with_params(
            &Token::bot("test_token"),
            "test_channel",
            MessageParams::new_text("Test message")
        ).await;
        
        assert!(result.is_ok());
    }
}
```

## Best Practices

### API Usage Guidelines

1. **Use Structured Parameters**: Prefer `MessageParams` over individual parameters
2. **Handle Rate Limits**: Implement proper backoff strategies
3. **Cache Where Appropriate**: Store frequently accessed data like guild information
4. **Validate Input**: Check parameters before making API calls
5. **Log Requests**: Track API usage for debugging and monitoring

### Performance Optimization

```rust
// ✅ Good: Reuse API client instance
struct MyBot {
    api: BotApi,
}

// ❌ Avoid: Creating new clients for each request
async fn bad_example() {
    let api = BotApi::new(); // Creates new HTTP client
    // Use api once...
}

// ✅ Good: Batch related operations
async fn update_multiple_channels(
    api: &BotApi,
    token: &Token,
    updates: Vec<(String, ModifyChannel)>,
) -> Result<Vec<Channel>> {
    let futures = updates.into_iter().map(|(channel_id, update)| {
        api.patch_channel(token, &channel_id, update)
    });
    
    try_join_all(futures).await
}

// ✅ Good: Use connection pooling for high-throughput applications
let http_client = ClientBuilder::new()
    .pool_max_idle_per_host(20)
    .pool_idle_timeout(Duration::from_secs(30))
    .build()?;
```

The API client is a powerful tool for building sophisticated bot functionality. By understanding its capabilities and following best practices, you can create efficient, reliable bots that make full use of QQ Guild's feature set.
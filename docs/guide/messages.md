# Messages & Responses

This guide covers how to send and handle messages using BotRS. From simple text messages to rich interactive content, you'll learn all the ways your bot can communicate with users.

## Message Types Overview

BotRS supports all QQ Guild message types:

- **Text Messages**: Plain text content
- **Rich Messages**: Text with formatting, mentions, and embeds
- **File Messages**: Images, documents, and other attachments
- **Interactive Messages**: Keyboards, buttons, and components
- **Special Messages**: ARK templates, markdown, and system messages

## Structured Message API (v0.2.0+)

BotRS v0.2.0 introduced a new structured parameter system that replaces the confusing multiple `None` parameters with clean, type-safe objects.

### Basic Text Messages

```rust
use botrs::models::message::MessageParams;

// Simple text message
let params = MessageParams::new_text("Hello, world!");
api.post_message_with_params(&token, &channel_id, params).await?;

// Text with markdown formatting
let params = MessageParams::new_text("**Bold** and *italic* text")
    .with_markdown(true);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### Reply Messages

```rust
// Reply to a specific message
let params = MessageParams::new_text("Thanks for your message!")
    .with_reply(&original_message_id);
api.post_message_with_params(&token, &channel_id, params).await?;

// Using the convenience method
message.reply(&api, &token, "Quick reply!").await?;
```

## Rich Content Messages

### Embed Messages

Embeds allow you to send structured, visually appealing messages:

```rust
use botrs::models::message::{MessageParams, MessageEmbed};

let embed = MessageEmbed {
    title: Some("Bot Status".to_string()),
    description: Some("All systems operational".to_string()),
    color: Some(0x00ff00), // Green color
    fields: Some(vec![
        EmbedField {
            name: "Uptime".to_string(),
            value: "2 hours 30 minutes".to_string(),
            inline: Some(true),
        },
        EmbedField {
            name: "Guilds".to_string(),
            value: "42".to_string(),
            inline: Some(true),
        },
    ]),
    footer: Some(EmbedFooter {
        text: "Last updated".to_string(),
        icon_url: None,
    }),
    timestamp: Some(chrono::Utc::now()),
    ..Default::default()
};

let params = MessageParams::new_embed(embed);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### Combined Text and Embed

```rust
let embed = MessageEmbed {
    title: Some("Error Details".to_string()),
    description: Some("Something went wrong".to_string()),
    color: Some(0xff0000), // Red
    ..Default::default()
};

let params = MessageParams::new_text("An error occurred:")
    .with_embed(embed);
api.post_message_with_params(&token, &channel_id, params).await?;
```

## File Attachments

### Image Messages

```rust
use botrs::models::message::{MessageParams, FileInfo};

// Send an image file
let file_info = FileInfo {
    url: "https://example.com/image.png".to_string(),
    ..Default::default()
};

let params = MessageParams::new_text("Check out this image!")
    .with_file_image(&file_info.url);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### Document Attachments

```rust
// Upload a document
let params = MessageParams::new_text("Here's the report:")
    .with_file_info(FileInfo {
        url: "https://example.com/report.pdf".to_string(),
        ..Default::default()
    });
api.post_message_with_params(&token, &channel_id, params).await?;
```

## Interactive Messages

### Keyboard Messages

Create interactive buttons for users to click:

```rust
use botrs::models::message::{MessageParams, MessageKeyboard, InlineKeyboard, KeyboardRow, Button};

let keyboard = MessageKeyboard {
    inline_keyboard: Some(InlineKeyboard {
        rows: vec![
            KeyboardRow {
                buttons: vec![
                    Button {
                        id: "yes".to_string(),
                        render_data: ButtonRenderData {
                            label: "Yes".to_string(),
                            visited_label: "Yes ✓".to_string(),
                            style: ButtonStyle::Primary,
                        },
                        action: ButtonAction {
                            type_: ActionType::Callback,
                            permission: ActionPermission {
                                type_: PermissionType::SpecifyUserIds,
                                specify_user_ids: vec![user_id.clone()],
                            },
                            data: "user_clicked_yes".to_string(),
                            ..Default::default()
                        },
                    },
                    Button {
                        id: "no".to_string(),
                        render_data: ButtonRenderData {
                            label: "No".to_string(),
                            visited_label: "No ✗".to_string(),
                            style: ButtonStyle::Secondary,
                        },
                        action: ButtonAction {
                            type_: ActionType::Callback,
                            permission: ActionPermission {
                                type_: PermissionType::SpecifyUserIds,
                                specify_user_ids: vec![user_id.clone()],
                            },
                            data: "user_clicked_no".to_string(),
                            ..Default::default()
                        },
                    },
                ],
            },
        ],
    }),
    ..Default::default()
};

let params = MessageParams::new_text("Do you agree?")
    .with_keyboard(keyboard);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### URL Buttons

```rust
let keyboard = MessageKeyboard {
    inline_keyboard: Some(InlineKeyboard {
        rows: vec![
            KeyboardRow {
                buttons: vec![
                    Button {
                        id: "docs".to_string(),
                        render_data: ButtonRenderData {
                            label: "View Documentation".to_string(),
                            visited_label: "Documentation".to_string(),
                            style: ButtonStyle::Link,
                        },
                        action: ButtonAction {
                            type_: ActionType::JumpUrl,
                            url: Some("https://github.com/YinMo19/botrs".to_string()),
                            ..Default::default()
                        },
                    },
                ],
            },
        ],
    }),
    ..Default::default()
};

let params = MessageParams::new_text("Learn more about BotRS:")
    .with_keyboard(keyboard);
api.post_message_with_params(&token, &channel_id, params).await?;
```

## Different Message Contexts

### Guild Messages

Regular guild channel messages (requires @mention):

```rust
let params = MessageParams::new_text("Hello guild!");
api.post_message_with_params(&token, &channel_id, params).await?;
```

### Group Messages

```rust
use botrs::models::message::GroupMessageParams;

let params = GroupMessageParams::new_text("Hello group!")
    .with_reply(&message_id);
api.post_group_message_with_params(&token, &group_openid, params).await?;
```

### Private Messages (C2C)

```rust
use botrs::models::message::C2CMessageParams;

let params = C2CMessageParams::new_text("Private message");
api.post_c2c_message_with_params(&token, &user_openid, params).await?;
```

### Direct Messages

```rust
use botrs::models::message::DirectMessageParams;

let params = DirectMessageParams::new_text("Direct message")
    .with_file_image("https://example.com/image.png");
api.post_dms_with_params(&token, &guild_id, params).await?;
```

## Builder Pattern Usage

The new message API supports convenient builder patterns:

```rust
let params = MessageParams::new_text("Complex message")
    .with_reply(&message_id)
    .with_markdown(true)
    .with_file_image("https://example.com/image.png")
    .with_embed(embed)
    .with_keyboard(keyboard);

api.post_message_with_params(&token, &channel_id, params).await?;
```

## Message Events and Responses

### Handling Message Events

```rust
use botrs::{Context, EventHandler, Message};

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            match content.as_str() {
                "!help" => {
                    let help_text = "Available commands:\n• !help - Show this help\n• !ping - Test bot";
                    let _ = message.reply(&ctx.api, &ctx.token, help_text).await;
                }
                "!ping" => {
                    let params = MessageParams::new_text("Pong! 🏓")
                        .with_reply(message.id.as_ref().unwrap());
                    let _ = ctx.api.post_message_with_params(
                        &ctx.token,
                        message.channel_id.as_ref().unwrap(),
                        params
                    ).await;
                }
                _ => {
                    // Handle other messages
                }
            }
        }
    }
}
```

### Dynamic Message Generation

```rust
async fn generate_status_message(&self, ctx: &Context) -> MessageParams {
    let uptime = self.get_uptime();
    let guild_count = self.get_guild_count().await;
    
    let embed = MessageEmbed {
        title: Some("Bot Status".to_string()),
        color: Some(0x00ff00),
        fields: Some(vec![
            EmbedField {
                name: "Uptime".to_string(),
                value: format!("{} minutes", uptime.num_minutes()),
                inline: Some(true),
            },
            EmbedField {
                name: "Guilds".to_string(),
                value: guild_count.to_string(),
                inline: Some(true),
            },
        ]),
        timestamp: Some(chrono::Utc::now()),
        ..Default::default()
    };
    
    MessageParams::new_embed(embed)
}

// Usage in event handler
async fn message_create(&self, ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        if content == "!status" {
            let params = self.generate_status_message(&ctx).await;
            let _ = ctx.api.post_message_with_params(
                &ctx.token,
                message.channel_id.as_ref().unwrap(),
                params
            ).await;
        }
    }
}
```

## Error Handling

### Basic Error Handling

```rust
async fn send_message(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), BotError> {
    let params = MessageParams::new_text(content);
    
    match ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
        Ok(_) => {
            println!("Message sent successfully");
            Ok(())
        }
        Err(BotError::RateLimited(info)) => {
            println!("Rate limited, retry after {} seconds", info.retry_after);
            Err(BotError::RateLimited(info))
        }
        Err(BotError::Network(e)) => {
            eprintln!("Network error: {}", e);
            Err(BotError::Network(e))
        }
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            Err(e)
        }
    }
}
```

### Retry Logic

```rust
async fn send_with_retry(
    &self,
    ctx: &Context,
    channel_id: &str,
    params: MessageParams,
    max_retries: u32,
) -> Result<Message, BotError> {
    let mut last_error = None;
    
    for attempt in 1..=max_retries {
        match ctx.api.post_message_with_params(&ctx.token, channel_id, params.clone()).await {
            Ok(message) => return Ok(message),
            Err(BotError::RateLimited(info)) => {
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(info.retry_after)).await;
                    continue;
                }
                last_error = Some(BotError::RateLimited(info));
            }
            Err(BotError::Network(_)) if attempt < max_retries => {
                tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
                continue;
            }
            Err(e) => {
                last_error = Some(e);
                break;
            }
        }
    }
    
    Err(last_error.unwrap())
}
```

## Performance Considerations

### Message Batching

```rust
use tokio::time::{interval, Duration};
use std::collections::VecDeque;

struct MessageQueue {
    queue: Arc<Mutex<VecDeque<(String, MessageParams)>>>,
}

impl MessageQueue {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    async fn enqueue(&self, channel_id: String, params: MessageParams) {
        let mut queue = self.queue.lock().await;
        queue.push_back((channel_id, params));
    }
    
    async fn process_queue(&self, ctx: &Context) {
        let mut interval = interval(Duration::from_millis(500));
        
        loop {
            interval.tick().await;
            
            let item = {
                let mut queue = self.queue.lock().await;
                queue.pop_front()
            };
            
            if let Some((channel_id, params)) = item {
                let _ = ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await;
            }
        }
    }
}
```

### Rate Limiting

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

struct RateLimitedSender {
    semaphore: Arc<Semaphore>,
}

impl RateLimitedSender {
    fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
    
    async fn send_message(
        &self,
        ctx: &Context,
        channel_id: &str,
        params: MessageParams,
    ) -> Result<Message, BotError> {
        let _permit = self.semaphore.acquire().await.unwrap();
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
    }
}
```

## Migration from v0.1.x

### Old API (Deprecated)

```rust
// DON'T USE - This is the old confusing API
api.post_message(
    token, 
    "channel_id", 
    Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

### New API (Recommended)

```rust
// USE THIS - Clean and readable
let params = MessageParams::new_text("Hello!");
api.post_message_with_params(token, "channel_id", params).await?;
```

### Migration Helper

```rust
// Helper function to ease migration
fn create_simple_message(content: &str) -> MessageParams {
    MessageParams::new_text(content)
}

fn create_reply_message(content: &str, reply_to: &str) -> MessageParams {
    MessageParams::new_text(content).with_reply(reply_to)
}

// Usage
let params = create_reply_message("Thanks!", &message_id);
api.post_message_with_params(&token, &channel_id, params).await?;
```

## Best Practices

1. **Use structured parameters**: Always prefer the new `MessageParams` API over deprecated methods

2. **Handle errors gracefully**: Implement proper error handling and retry logic

3. **Respect rate limits**: Don't spam messages; implement queuing if necessary

4. **Validate input**: Check message content and parameters before sending

5. **Use appropriate message types**: Choose the right message type for your content

6. **Keep messages focused**: Don't try to pack too much information into a single message

7. **Test thoroughly**: Test your messages in different contexts (guild, group, DM)

## Examples

### Complete Message Handler

```rust
use botrs::{Context, EventHandler, Message, models::message::*};

struct MessageBot;

#[async_trait::async_trait]
impl EventHandler for MessageBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }
        
        if let Some(content) = &message.content {
            let response = match content.trim() {
                "!embed" => {
                    let embed = MessageEmbed {
                        title: Some("Example Embed".to_string()),
                        description: Some("This is an example embed message".to_string()),
                        color: Some(0x7289da),
                        ..Default::default()
                    };
                    Some(MessageParams::new_embed(embed))
                }
                "!file" => {
                    Some(MessageParams::new_text("Here's an image:")
                        .with_file_image("https://example.com/image.png"))
                }
                "!keyboard" => {
                    let keyboard = create_yes_no_keyboard(&message.author.as_ref()?.id.as_ref()?);
                    Some(MessageParams::new_text("Choose an option:")
                        .with_keyboard(keyboard))
                }
                _ => None,
            };
            
            if let Some(params) = response {
                let channel_id = message.channel_id.as_ref()?;
                if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        }
    }
}

fn create_yes_no_keyboard(user_id: &str) -> MessageKeyboard {
    // Implementation of yes/no keyboard
    // ... (see Interactive Messages section above)
}
```

This comprehensive guide covers all aspects of message handling in BotRS. The new structured parameter system makes code more readable, maintainable, and less error-prone compared to the old API.

## Next Steps

- [Error Handling](./error-handling.md) - Learn robust error handling patterns
- [API Client](./api-client.md) - Explore the full API capabilities
- [Examples](../examples/getting-started.md) - See working code examples
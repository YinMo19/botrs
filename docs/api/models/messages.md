# Message Models API Reference

This module provides data structures for different types of messages in the QQ Guild Bot API, including guild messages, direct messages, group messages, and C2C (client-to-client) messages.

## Core Message Types

### `Message`

Represents a message in a guild channel.

```rust
pub struct Message {
    pub id: String,
    pub content: Option<String>,
    pub channel_id: String,
    pub guild_id: String,
    pub author: Option<MessageUser>,
    pub member: Option<MessageMember>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<MessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<String>,
    pub timestamp: Option<String>,
    pub event_id: Option<String>,
}
```

#### Methods

##### `reply`

Replies to the message with text content.

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<Message>
```

**Parameters:**
- `api`: Bot API client instance
- `token`: Authentication token
- `content`: Reply text content

**Example:**
```rust
async fn handle_message(ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        if content == "!ping" {
            message.reply(&ctx.api, &ctx.token, "Pong!").await?;
        }
    }
}
```

##### `is_from_bot`

Checks if the message was sent by a bot.

```rust
pub fn is_from_bot(&self) -> bool
```

**Returns:** `true` if the message author is a bot, `false` otherwise.

**Example:**
```rust
if message.is_from_bot() {
    // Ignore bot messages
    return;
}
```

##### `has_content`

Checks if the message has text content.

```rust
pub fn has_content(&self) -> bool
```

##### `has_attachments`

Checks if the message has file attachments.

```rust
pub fn has_attachments(&self) -> bool
```

##### `has_mentions`

Checks if the message mentions other users.

```rust
pub fn has_mentions(&self) -> bool
```

### `DirectMessage`

Represents a direct message between the bot and a user.

```rust
pub struct DirectMessage {
    pub id: String,
    pub content: Option<String>,
    pub channel_id: String,
    pub guild_id: String,
    pub direct_message: bool,
    pub author: Option<DirectMessageUser>,
    pub member: Option<DirectMessageMember>,
    pub message_reference: Option<MessageReference>,
    pub attachments: Vec<MessageAttachment>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<String>,
    pub src_guild_id: Option<String>,
    pub timestamp: Option<String>,
    pub event_id: Option<String>,
}
```

#### Methods

##### `reply`

Replies to the direct message.

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<DirectMessage>
```

### `GroupMessage`

Represents a message in a QQ group.

```rust
pub struct GroupMessage {
    pub id: String,
    pub content: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<GroupMessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub msg_seq: Option<u64>,
    pub timestamp: Option<String>,
    pub author: Option<GroupMessageUser>,
    pub group_openid: Option<String>,
    pub event_id: Option<String>,
}
```

#### Methods

##### `reply`

Replies to the group message.

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<GroupMessage>
```

### `C2CMessage`

Represents a client-to-client message.

```rust
pub struct C2CMessage {
    pub id: String,
    pub content: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<C2CMessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub msg_seq: Option<u64>,
    pub timestamp: Option<String>,
    pub author: Option<C2CMessageUser>,
    pub event_id: Option<String>,
}
```

#### Methods

##### `reply`

Replies to the C2C message.

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<C2CMessage>
```

## User Types

### `MessageUser`

User information in guild messages.

```rust
pub struct MessageUser {
    pub id: String,
    pub username: Option<String>,
    pub bot: Option<bool>,
    pub avatar: Option<String>,
}
```

### `DirectMessageUser`

User information in direct messages.

```rust
pub struct DirectMessageUser {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}
```

### `GroupMessageUser`

User information in group messages.

```rust
pub struct GroupMessageUser {
    pub id: Option<String>,
    pub member_openid: Option<String>,
    pub union_openid: Option<String>,
}
```

### `C2CMessageUser`

User information in C2C messages.

```rust
pub struct C2CMessageUser {
    pub user_openid: Option<String>,
}
```

## Rich Content Types

### `Embed`

Rich embed content for messages.

```rust
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>,
}
```

### `EmbedField`

A field in an embed.

```rust
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
```

### `Ark`

ARK template message for rich interactive content.

```rust
pub struct Ark {
    pub template_id: Option<u32>,
    pub kv: Vec<ArkKv>,
}
```

### `Keyboard`

Interactive keyboard with buttons.

```rust
pub struct Keyboard {
    pub content: Option<KeyboardContent>,
}
```

### `KeyboardButton`

A button in an interactive keyboard.

```rust
pub struct KeyboardButton {
    pub id: Option<String>,
    pub render_data: Option<KeyboardButtonRenderData>,
    pub action: Option<KeyboardButtonAction>,
}
```

## Message Parameters

### `MessageParams`

Parameters for sending guild messages.

```rust
pub struct MessageParams {
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub ark: Option<Ark>,
    pub message_reference: Option<Reference>,
    pub image: Option<String>,
    pub file_image: Option<String>,
    pub msg_id: Option<String>,
    pub event_id: Option<String>,
    pub markdown: Option<MarkdownPayload>,
    pub keyboard: Option<Keyboard>,
}
```

#### Methods

##### `new_text`

Creates message parameters with text content.

```rust
pub fn new_text(content: &str) -> Self
```

##### `with_file_image`

Adds a file image to the message.

```rust
pub fn with_file_image(mut self, file_info: &str) -> Self
```

##### `with_reply`

Sets the message as a reply to another message.

```rust
pub fn with_reply(mut self, message_id: &str) -> Self
```

**Example:**
```rust
let params = MessageParams::new_text("Hello!")
    .with_file_image("file_info_string")
    .with_reply("original_message_id");
```

### `GroupMessageParams`

Parameters for sending group messages.

```rust
pub struct GroupMessageParams {
    pub msg_type: Option<u32>,
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub ark: Option<Ark>,
    pub message_reference: Option<Reference>,
    pub media: Option<Media>,
    pub msg_id: Option<String>,
    pub msg_seq: Option<u64>,
    pub event_id: Option<String>,
    pub markdown: Option<MarkdownPayload>,
    pub keyboard: Option<Keyboard>,
}
```

### `C2CMessageParams`

Parameters for sending C2C messages.

```rust
pub struct C2CMessageParams {
    pub msg_type: Option<u32>,
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub ark: Option<Ark>,
    pub message_reference: Option<Reference>,
    pub media: Option<Media>,
    pub msg_id: Option<String>,
    pub msg_seq: Option<u64>,
    pub event_id: Option<String>,
    pub markdown: Option<MarkdownPayload>,
    pub keyboard: Option<Keyboard>,
}
```

### `DirectMessageParams`

Parameters for sending direct messages.

```rust
pub struct DirectMessageParams {
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub ark: Option<Ark>,
    pub message_reference: Option<Reference>,
    pub image: Option<String>,
    pub file_image: Option<String>,
    pub msg_id: Option<String>,
    pub event_id: Option<String>,
    pub markdown: Option<MarkdownPayload>,
    pub keyboard: Option<Keyboard>,
}
```

## Attachments and Media

### `MessageAttachment`

File attachment in a message.

```rust
pub struct MessageAttachment {
    pub id: Option<String>,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<u64>,
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
```

#### Methods

##### `is_image`

Checks if the attachment is an image.

```rust
pub fn is_image(&self) -> bool
```

##### `is_video`

Checks if the attachment is a video.

```rust
pub fn is_video(&self) -> bool
```

##### `is_audio`

Checks if the attachment is an audio file.

```rust
pub fn is_audio(&self) -> bool
```

### `Media`

Media content for messages.

```rust
pub struct Media {
    pub file_info: Option<String>,
    pub ttl: Option<u32>,
}
```

## Common Usage Patterns

### Basic Text Reply

```rust
async fn handle_message(ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        if content.starts_with("!echo ") {
            let echo_text = &content[6..];
            message.reply(&ctx.api, &ctx.token, echo_text).await?;
        }
    }
}
```

### Rich Embed Response

```rust
use botrs::models::message::{Embed, EmbedField};

let embed = Embed {
    title: Some("Bot Information".to_string()),
    description: Some("A QQ Guild bot built with BotRS".to_string()),
    color: Some(0x00ff00),
    fields: vec![
        EmbedField {
            name: "Version".to_string(),
            value: "0.2.5".to_string(),
            inline: Some(true),
        },
        EmbedField {
            name: "Language".to_string(),
            value: "Rust".to_string(),
            inline: Some(true),
        },
    ],
    ..Default::default()
};

let params = MessageParams {
    embed: Some(embed),
    ..Default::default()
};
```

### File Upload

```rust
let params = MessageParams::new_text("Here's an image!")
    .with_file_image("base64_encoded_file_info");
```

### Message Reference

```rust
let params = MessageParams::new_text("This is a reply")
    .with_reply(&original_message.id);
```

## See Also

- [Client API](../client.md) - Main client for bot operations
- [Context API](../context.md) - Context object passed to event handlers
- [Event Handler](../event-handler.md) - Handling different message events
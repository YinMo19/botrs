# 消息模型 API 参考

该模块为 QQ 频道机器人 API 中不同类型的消息提供数据结构，包括频道消息、私信、群聊消息和 C2C（客户端到客户端）消息。

## 核心消息类型

### `Message`

表示频道中的消息。

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

#### 方法

##### `reply`

回复消息，发送文本内容。

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<Message>
```

**参数：**
- `api`: 机器人 API 客户端实例
- `token`: 认证令牌
- `content`: 回复文本内容

**示例：**
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

检查消息是否由机器人发送。

```rust
pub fn is_from_bot(&self) -> bool
```

**返回值：** 如果消息作者是机器人则返回 `true`，否则返回 `false`。

**示例：**
```rust
if message.is_from_bot() {
    // 忽略机器人消息
    return;
}
```

##### `has_content`

检查消息是否包含文本内容。

```rust
pub fn has_content(&self) -> bool
```

##### `has_attachments`

检查消息是否包含文件附件。

```rust
pub fn has_attachments(&self) -> bool
```

##### `has_mentions`

检查消息是否提及其他用户。

```rust
pub fn has_mentions(&self) -> bool
```

### `DirectMessage`

表示机器人与用户之间的私信。

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

#### 方法

##### `reply`

回复私信。

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<DirectMessage>
```

### `GroupMessage`

表示 QQ 群中的消息。

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

#### 方法

##### `reply`

回复群消息。

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<GroupMessage>
```

### `C2CMessage`

表示客户端到客户端的消息。

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

#### 方法

##### `reply`

回复 C2C 消息。

```rust
pub async fn reply(
    &self,
    api: &BotApi,
    token: &Token,
    content: &str,
) -> Result<C2CMessage>
```

## 用户类型

### `MessageUser`

频道消息中的用户信息。

```rust
pub struct MessageUser {
    pub id: String,
    pub username: Option<String>,
    pub bot: Option<bool>,
    pub avatar: Option<String>,
}
```

### `DirectMessageUser`

私信中的用户信息。

```rust
pub struct DirectMessageUser {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}
```

### `GroupMessageUser`

群消息中的用户信息。

```rust
pub struct GroupMessageUser {
    pub id: Option<String>,
    pub member_openid: Option<String>,
    pub union_openid: Option<String>,
}
```

### `C2CMessageUser`

C2C 消息中的用户信息。

```rust
pub struct C2CMessageUser {
    pub user_openid: Option<String>,
}
```

## 富内容类型

### `Embed`

消息的富嵌入内容。

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

嵌入内容中的字段。

```rust
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
```

### `Ark`

用于富交互内容的 ARK 模板消息。

```rust
pub struct Ark {
    pub template_id: Option<u32>,
    pub kv: Vec<ArkKv>,
}
```

### `Keyboard`

交互式键盘按钮。

```rust
pub struct Keyboard {
    pub content: Option<KeyboardContent>,
}
```

### `KeyboardButton`

交互式键盘中的按钮。

```rust
pub struct KeyboardButton {
    pub id: Option<String>,
    pub render_data: Option<KeyboardButtonRenderData>,
    pub action: Option<KeyboardButtonAction>,
}
```

## 消息参数

### `MessageParams`

发送频道消息的参数。

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

#### 方法

##### `new_text`

创建包含文本内容的消息参数。

```rust
pub fn new_text(content: &str) -> Self
```

##### `with_file_image`

为消息添加文件图片。

```rust
pub fn with_file_image(mut self, file_info: &str) -> Self
```

##### `with_reply`

将消息设置为对另一条消息的回复。

```rust
pub fn with_reply(mut self, message_id: &str) -> Self
```

**示例：**
```rust
let params = MessageParams::new_text("你好！")
    .with_file_image("file_info_string")
    .with_reply("original_message_id");
```

### `GroupMessageParams`

发送群消息的参数。

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

发送 C2C 消息的参数。

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

发送私信的参数。

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

## 附件和媒体

### `MessageAttachment`

消息中的文件附件。

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

#### 方法

##### `is_image`

检查附件是否为图片。

```rust
pub fn is_image(&self) -> bool
```

##### `is_video`

检查附件是否为视频。

```rust
pub fn is_video(&self) -> bool
```

##### `is_audio`

检查附件是否为音频文件。

```rust
pub fn is_audio(&self) -> bool
```

### `Media`

消息的媒体内容。

```rust
pub struct Media {
    pub file_info: Option<String>,
    pub ttl: Option<u32>,
}
```

## 常见使用模式

### 基础文本回复

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

### 富嵌入响应

```rust
use botrs::models::message::{Embed, EmbedField};

let embed = Embed {
    title: Some("机器人信息".to_string()),
    description: Some("使用 BotRS 构建的 QQ 频道机器人".to_string()),
    color: Some(0x00ff00),
    fields: vec![
        EmbedField {
            name: "版本".to_string(),
            value: "0.2.5".to_string(),
            inline: Some(true),
        },
        EmbedField {
            name: "语言".to_string(),
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

### 文件上传

```rust
let params = MessageParams::new_text("这是一张图片！")
    .with_file_image("base64_encoded_file_info");
```

### 消息引用

```rust
let params = MessageParams::new_text("这是一条回复")
    .with_reply(&original_message.id);
```

## 相关文档

- [客户端 API](../client.md) - 机器人操作的主要客户端
- [上下文 API](../context.md) - 传递给事件处理器的上下文对象
- [事件处理器](../event-handler.md) - 处理不同消息事件
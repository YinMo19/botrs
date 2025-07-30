# 机器人 API 参考

`BotApi` 是用于与 QQ 频道 REST API 交互的核心客户端。它提供了完整的 API 端点访问，支持消息发送、频道管理、成员操作等所有功能。

## 概述

```rust
pub struct BotApi {
    http_client: HttpClient,
}
```

`BotApi` 封装了所有 QQ 频道机器人 API 端点，提供类型安全的接口和自动的错误处理。所有方法都是异步的，支持高并发操作。

## 构造函数

### `new`

创建新的 BotApi 实例。

```rust
pub fn new() -> Self
```

#### 返回值

返回新的 `BotApi` 实例，使用默认的 HTTP 客户端配置。

#### 示例

```rust
use botrs::BotApi;

let api = BotApi::new();
```

### `with_http_client`

使用自定义 HTTP 客户端创建 BotApi 实例。

```rust
pub fn with_http_client(http_client: HttpClient) -> Self
```

#### 参数

- `http_client`: 自定义的 HTTP 客户端

#### 示例

```rust
use botrs::{BotApi, HttpClient};
use std::time::Duration;

let http_client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .build()?;

let api = BotApi::with_http_client(http_client);
```

## 消息 API

### `post_message`

发送消息到指定频道（旧版 API）。

```rust
pub async fn post_message(
    &self,
    token: &Token,
    channel_id: &str,
    content: Option<&str>,
    embed: Option<&Embed>,
) -> Result<Message, BotError>
```

#### 参数

- `token`: 身份验证令牌
- `channel_id`: 目标频道 ID
- `content`: 消息文本内容（可选）
- `embed`: 嵌入内容（可选）

#### 返回值

返回发送成功的消息对象或错误。

#### 示例

```rust
let message = api.post_message(
    &token,
    "channel_123",
    Some("你好，世界！"),
    None
).await?;
```

### `post_message_with_params`

使用结构化参数发送消息（推荐）。

```rust
pub async fn post_message_with_params(
    &self,
    token: &Token,
    channel_id: &str,
    params: MessageParams,
) -> Result<Message, BotError>
```

#### 参数

- `token`: 身份验证令牌
- `channel_id`: 目标频道 ID
- `params`: 消息参数结构体

#### 示例

```rust
use botrs::{MessageParams, Embed};

// 发送文本消息
let params = MessageParams::new_text("Hello, World!");
let message = api.post_message_with_params(&token, "channel_123", params).await?;

// 发送嵌入消息
let embed = Embed::new()
    .title("标题")
    .description("描述")
    .color(0x3498db);
let params = MessageParams::new_embed(embed);
let message = api.post_message_with_params(&token, "channel_123", params).await?;

// 发送回复消息
let params = MessageParams::new_text("这是回复")
    .with_reply("original_message_id");
let message = api.post_message_with_params(&token, "channel_123", params).await?;
```

### `post_message_with_file`

发送带文件的消息。

```rust
pub async fn post_message_with_file(
    &self,
    token: &Token,
    channel_id: &str,
    filename: &str,
    file_data: &[u8],
    file_type: &str,
) -> Result<Message, BotError>
```

#### 参数

- `token`: 身份验证令牌
- `channel_id`: 目标频道 ID
- `filename`: 文件名
- `file_data`: 文件数据
- `file_type`: 文件类型（如 "image", "video", "audio"）

#### 示例

```rust
let file_data = std::fs::read("image.png")?;
let message = api.post_message_with_file(
    &token,
    "channel_123",
    "image.png",
    &file_data,
    "image"
).await?;
```

### `get_message`

获取指定消息。

```rust
pub async fn get_message(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
) -> Result<Message, BotError>
```

### `delete_message`

删除指定消息。

```rust
pub async fn delete_message(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    hidetip: Option<bool>,
) -> Result<(), BotError>
```

## 频道管理 API

### `get_guild`

获取频道信息。

```rust
pub async fn get_guild(
    &self,
    token: &Token,
    guild_id: &str,
) -> Result<Guild, BotError>
```

#### 示例

```rust
let guild = api.get_guild(&token, "guild_123").await?;
println!("频道名称: {}", guild.name.unwrap_or_default());
```

### `get_guild_channels`

获取频道的子频道列表。

```rust
pub async fn get_guild_channels(
    &self,
    token: &Token,
    guild_id: &str,
) -> Result<Vec<Channel>, BotError>
```

#### 示例

```rust
let channels = api.get_guild_channels(&token, "guild_123").await?;
for channel in channels {
    println!("子频道: {} ({})", 
             channel.name.unwrap_or_default(), 
             channel.id);
}
```

### `get_channel`

获取指定子频道信息。

```rust
pub async fn get_channel(
    &self,
    token: &Token,
    channel_id: &str,
) -> Result<Channel, BotError>
```

### `create_guild_channel`

创建新的子频道。

```rust
pub async fn create_guild_channel(
    &self,
    token: &Token,
    guild_id: &str,
    channel_data: &serde_json::Value,
) -> Result<Channel, BotError>
```

#### 示例

```rust
use serde_json::json;

let channel_data = json!({
    "name": "新子频道",
    "type": 0,  // 文本频道
    "sub_type": 0,  // 聊天频道
    "position": 1
});

let channel = api.create_guild_channel(&token, "guild_123", &channel_data).await?;
```

### `modify_guild_channel`

修改子频道信息。

```rust
pub async fn modify_guild_channel(
    &self,
    token: &Token,
    channel_id: &str,
    channel_data: &serde_json::Value,
) -> Result<Channel, BotError>
```

### `delete_guild_channel`

删除子频道。

```rust
pub async fn delete_guild_channel(
    &self,
    token: &Token,
    channel_id: &str,
) -> Result<(), BotError>
```

## 成员管理 API

### `get_guild_members`

获取频道成员列表。

```rust
pub async fn get_guild_members(
    &self,
    token: &Token,
    guild_id: &str,
    after: Option<&str>,
    limit: Option<u32>,
) -> Result<Vec<Member>, BotError>
```

#### 参数

- `after`: 分页参数，获取此 ID 之后的成员
- `limit`: 返回成员数量限制（最大 400）

#### 示例

```rust
// 获取前100个成员
let members = api.get_guild_members(&token, "guild_123", None, Some(100)).await?;

// 分页获取
let first_batch = api.get_guild_members(&token, "guild_123", None, Some(400)).await?;
if let Some(last_member) = first_batch.last() {
    let second_batch = api.get_guild_members(
        &token, 
        "guild_123", 
        Some(&last_member.user.id), 
        Some(400)
    ).await?;
}
```

### `get_guild_member`

获取指定成员信息。

```rust
pub async fn get_guild_member(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
) -> Result<Member, BotError>
```

### `add_guild_member_role`

为成员添加身份组。

```rust
pub async fn add_guild_member_role(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<(), BotError>
```

### `remove_guild_member_role`

移除成员的身份组。

```rust
pub async fn remove_guild_member_role(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<(), BotError>
```

### `create_guild_member_mute`

禁言频道成员。

```rust
pub async fn create_guild_member_mute(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    mute_data: &serde_json::Value,
) -> Result<(), BotError>
```

#### 示例

```rust
use serde_json::json;

// 禁言10分钟
let mute_data = json!({
    "mute_end_timestamp": (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp().to_string(),
    "mute_seconds": 600
});

api.create_guild_member_mute(&token, "guild_123", "user_456", &mute_data).await?;
```

## 私信 API

### `create_direct_message_session`

创建私信会话。

```rust
pub async fn create_direct_message_session(
    &self,
    token: &Token,
    guild_id: &str,
    user_id: &str,
) -> Result<DirectMessageSession, BotError>
```

### `post_direct_message_with_params`

发送私信消息。

```rust
pub async fn post_direct_message_with_params(
    &self,
    token: &Token,
    guild_id: &str,
    channel_id: &str,
    params: MessageParams,
) -> Result<DirectMessage, BotError>
```

#### 示例

```rust
// 创建私信会话
let session = api.create_direct_message_session(&token, "guild_123", "user_456").await?;

// 发送私信
let params = MessageParams::new_text("这是一条私信");
let dm = api.post_direct_message_with_params(
    &token,
    "guild_123",
    &session.channel_id,
    params
).await?;
```

### `get_direct_messages`

获取私信历史。

```rust
pub async fn get_direct_messages(
    &self,
    token: &Token,
    guild_id: &str,
    channel_id: &str,
    limit: Option<u32>,
) -> Result<Vec<DirectMessage>, BotError>
```

## 群组消息 API

### `post_group_message_with_params`

发送群组消息。

```rust
pub async fn post_group_message_with_params(
    &self,
    token: &Token,
    group_id: &str,
    params: MessageParams,
) -> Result<GroupMessage, BotError>
```

### `post_c2c_message_with_params`

发送 C2C（用户对用户）消息。

```rust
pub async fn post_c2c_message_with_params(
    &self,
    token: &Token,
    user_id: &str,
    params: MessageParams,
) -> Result<C2CMessage, BotError>
```

## 公告 API

### `create_guild_announce`

创建频道公告。

```rust
pub async fn create_guild_announce(
    &self,
    token: &Token,
    guild_id: &str,
    announce_data: &serde_json::Value,
) -> Result<Announce, BotError>
```

#### 示例

```rust
use serde_json::json;

let announce_data = json!({
    "message": "重要通知：系统维护将在今晚进行",
    "channel_id": "channel_123"
});

let announce = api.create_guild_announce(&token, "guild_123", &announce_data).await?;
```

### `delete_guild_announce`

删除频道公告。

```rust
pub async fn delete_guild_announce(
    &self,
    token: &Token,
    guild_id: &str,
    announce_id: &str,
) -> Result<(), BotError>
```

## 表情回应 API

### `put_message_reaction`

为消息添加表情回应。

```rust
pub async fn put_message_reaction(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<(), BotError>
```

#### 示例

```rust
// 添加点赞表情
api.put_message_reaction(&token, "channel_123", "message_456", "👍").await?;
```

### `delete_message_reaction`

删除消息的表情回应。

```rust
pub async fn delete_message_reaction(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
) -> Result<(), BotError>
```

### `get_message_reaction_users`

获取对消息添加特定表情的用户列表。

```rust
pub async fn get_message_reaction_users(
    &self,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str,
    cookie: Option<&str>,
    limit: Option<u32>,
) -> Result<ReactionUsers, BotError>
```

## 身份组 API

### `get_guild_roles`

获取频道身份组列表。

```rust
pub async fn get_guild_roles(
    &self,
    token: &Token,
    guild_id: &str,
) -> Result<GuildRoles, BotError>
```

### `create_guild_role`

创建频道身份组。

```rust
pub async fn create_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role_data: &serde_json::Value,
) -> Result<GuildRole, BotError>
```

### `modify_guild_role`

修改频道身份组。

```rust
pub async fn modify_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
    role_data: &serde_json::Value,
) -> Result<GuildRole, BotError>
```

### `delete_guild_role`

删除频道身份组。

```rust
pub async fn delete_guild_role(
    &self,
    token: &Token,
    guild_id: &str,
    role_id: &str,
) -> Result<(), BotError>
```

## 音频 API

### `get_channel_audio_members`

获取音频频道成员列表。

```rust
pub async fn get_channel_audio_members(
    &self,
    token: &Token,
    channel_id: &str,
) -> Result<Vec<Member>, BotError>
```

### `audio_control`

控制音频播放。

```rust
pub async fn audio_control(
    &self,
    token: &Token,
    channel_id: &str,
    control_data: &serde_json::Value,
) -> Result<AudioControl, BotError>
```

## 日程 API

### `get_guild_schedules`

获取频道日程列表。

```rust
pub async fn get_guild_schedules(
    &self,
    token: &Token,
    guild_id: &str,
    since: Option<u64>,
) -> Result<Vec<Schedule>, BotError>
```

### `get_guild_schedule`

获取指定日程信息。

```rust
pub async fn get_guild_schedule(
    &self,
    token: &Token,
    guild_id: &str,
    schedule_id: &str,
) -> Result<Schedule, BotError>
```

### `create_guild_schedule`

创建频道日程。

```rust
pub async fn create_guild_schedule(
    &self,
    token: &Token,
    guild_id: &str,
    schedule_data: &serde_json::Value,
) -> Result<Schedule, BotError>
```

### `modify_guild_schedule`

修改频道日程。

```rust
pub async fn modify_guild_schedule(
    &self,
    token: &Token,
    guild_id: &str,
    schedule_id: &str,
    schedule_data: &serde_json::Value,
) -> Result<Schedule, BotError>
```

### `delete_guild_schedule`

删除频道日程。

```rust
pub async fn delete_guild_schedule(
    &self,
    token: &Token,
    guild_id: &str,
    schedule_id: &str,
) -> Result<(), BotError>
```

## 论坛 API

### `get_threads`

获取论坛帖子列表。

```rust
pub async fn get_threads(
    &self,
    token: &Token,
    channel_id: &str,
) -> Result<Vec<Thread>, BotError>
```

### `get_thread`

获取指定论坛帖子信息。

```rust
pub async fn get_thread(
    &self,
    token: &Token,
    channel_id: &str,
    thread_id: &str,
) -> Result<ThreadInfo, BotError>
```

### `create_thread`

创建论坛帖子。

```rust
pub async fn create_thread(
    &self,
    token: &Token,
    channel_id: &str,
    thread_data: &serde_json::Value,
) -> Result<OpenThread, BotError>
```

### `delete_thread`

删除论坛帖子。

```rust
pub async fn delete_thread(
    &self,
    token: &Token,
    channel_id: &str,
    thread_id: &str,
) -> Result<(), BotError>
```

## 权限 API

### `get_guild_api_permission`

获取频道 API 权限。

```rust
pub async fn get_guild_api_permission(
    &self,
    token: &Token,
    guild_id: &str,
) -> Result<ApiPermission, BotError>
```

### `post_guild_api_permission_demand`

申请频道 API 权限。

```rust
pub async fn post_guild_api_permission_demand(
    &self,
    token: &Token,
    guild_id: &str,
    permission_data: &serde_json::Value,
) -> Result<ApiPermissionDemand, BotError>
```

## 错误处理

所有 API 方法都返回 `Result<T, BotError>`，其中 `BotError` 包含详细的错误信息：

```rust
use botrs::BotError;

match api.get_guild(&token, "invalid_guild_id").await {
    Ok(guild) => println!("获取频道成功: {}", guild.id),
    Err(BotError::NotFound) => eprintln!("频道不存在"),
    Err(BotError::Forbidden) => eprintln!("权限不足"),
    Err(BotError::RateLimited(retry_after)) => {
        eprintln!("速率限制，{}秒后重试", retry_after);
    }
    Err(BotError::Authentication(_)) => eprintln!("身份验证失败"),
    Err(BotError::Network(_)) => eprintln!("网络连接错误"),
    Err(e) => eprintln!("其他错误: {}", e),
}
```

## 批量操作示例

### 批量获取频道信息

```rust
use futures::future::try_join_all;

async fn get_multiple_guilds(
    api: &BotApi,
    token: &Token,
    guild_ids: &[String]
) -> Result<Vec<Guild>, BotError> {
    let futures: Vec<_> = guild_ids.iter()
        .map(|id| api.get_guild(token, id))
        .collect();
    
    try_join_all(futures).await
}
```

### 批量发送消息

```rust
async fn broadcast_message(
    api: &BotApi,
    token: &Token,
    channel_ids: &[String],
    content: &str
) -> Result<Vec<Message>, BotError> {
    let mut results = Vec::new();
    
    for channel_id in channel_ids {
        let params = MessageParams::new_text(content);
        match api.post_message_with_params(token, channel_id, params).await {
            Ok(message) => results.push(message),
            Err(e) => eprintln!("发送到频道 {} 失败: {}", channel_id, e),
        }
        
        // 避免速率限制
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(results)
}
```

## 最佳实践

1. **错误处理**: 始终处理可能的 API 错误
2. **速率限制**: 在批量操作中添加适当的延迟
3. **重试机制**: 对临时错误实现自动重试
4. **参数验证**: 在调用 API 前验证输入参数
5. **日志记录**: 记录重要的 API 调用和错误

## 另请参阅

- [`Token` API 参考](./token.md) - 身份验证令牌管理
- [`Context` API 参考](./context.md) - 事件处理器中的 API 使用
- [API 客户端使用指南](/zh/guide/api-client.md) - API 使用最佳实践
- [错误处理指南](/zh/guide/error-handling.md) - API 错误处理策略
# API 客户端使用指南

BotRS 提供了完整的 QQ 频道 REST API 客户端，允许您直接与 QQ 频道的各种端点进行交互。本指南将详细介绍如何使用 `BotApi` 客户端来执行各种操作。

## 概述

`BotApi` 是一个完整的 HTTP 客户端，封装了所有 QQ 频道机器人 API 端点。它提供了类型安全的接口，自动处理身份验证、序列化和错误处理。

```rust
use botrs::{BotApi, Token};

let api = BotApi::new();
let token = Token::new("应用ID", "密钥");
```

## 基础设置

### 创建 API 客户端

```rust
use botrs::{BotApi, Token};

// 创建 API 客户端
let api = BotApi::new();

// 创建身份验证令牌
let token = Token::new("你的应用ID", "你的密钥");

// 验证令牌
token.validate()?;
```

### 自定义配置

```rust
use botrs::{BotApi, HttpClient};
use std::time::Duration;

// 创建自定义 HTTP 客户端
let http_client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .user_agent("MyBot/1.0")
    .build()?;

let api = BotApi::with_http_client(http_client);
```

## 消息 API

### 发送文本消息

```rust
use botrs::{MessageParams, BotApi, Token};

async fn send_text_message(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    content: &str
) -> Result<Message, BotError> {
    let params = MessageParams::new_text(content);
    api.post_message_with_params(token, channel_id, params).await
}

// 使用示例
let message = send_text_message(
    &api,
    &token,
    "channel_id_123",
    "你好，这是一条测试消息！"
).await?;

println!("消息发送成功，ID: {}", message.id);
```

### 发送富文本消息

```rust
use botrs::{Embed, MessageParams};

async fn send_embed_message(
    api: &BotApi,
    token: &Token,
    channel_id: &str
) -> Result<Message, BotError> {
    let embed = Embed::new()
        .title("机器人状态")
        .description("当前机器人运行正常")
        .color(0x00ff00)
        .field("服务器", "在线", true)
        .field("延迟", "25ms", true)
        .timestamp(chrono::Utc::now());

    let params = MessageParams::new_embed(embed);
    api.post_message_with_params(token, channel_id, params).await
}
```

### 发送回复消息

```rust
async fn send_reply(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    original_message_id: &str,
    reply_content: &str
) -> Result<Message, BotError> {
    let params = MessageParams::new_text(reply_content)
        .with_reply(original_message_id);
    
    api.post_message_with_params(token, channel_id, params).await
}
```

### 发送 Markdown 消息

```rust
async fn send_markdown_message(
    api: &BotApi,
    token: &Token,
    channel_id: &str
) -> Result<Message, BotError> {
    let markdown_content = r#"
# 欢迎使用机器人

这是一条 **Markdown** 格式的消息。

## 功能列表

- 发送消息
- 管理频道
- 处理事件

[点击访问官网](https://example.com)
"#;

    let params = MessageParams::new_markdown(markdown_content);
    api.post_message_with_params(token, channel_id, params).await
}
```

## 文件上传

### 上传图片

```rust
async fn upload_image(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    image_path: &str
) -> Result<Message, BotError> {
    // 读取图片文件
    let image_data = tokio::fs::read(image_path).await?;
    
    // 提取文件名
    let filename = std::path::Path::new(image_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("image.png");
    
    // 发送带文件的消息
    api.post_message_with_file(
        token,
        channel_id,
        filename,
        &image_data,
        "png"
    ).await
}

// 使用示例
let message = upload_image(&api, &token, "channel_123", "assets/logo.png").await?;
```

### 上传多种文件类型

```rust
async fn upload_file_by_type(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    file_path: &str
) -> Result<Message, BotError> {
    let file_data = tokio::fs::read(file_path).await?;
    let filename = std::path::Path::new(file_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    
    // 根据文件扩展名确定类型
    let file_type = match std::path::Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str()) {
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "image",
        Some("mp4") | Some("avi") | Some("mov") => "video",
        Some("mp3") | Some("wav") | Some("ogg") => "audio",
        _ => "file",
    };
    
    api.post_message_with_file(token, channel_id, filename, &file_data, file_type).await
}
```

## 频道管理

### 获取频道信息

```rust
async fn get_guild_info(
    api: &BotApi,
    token: &Token,
    guild_id: &str
) -> Result<Guild, BotError> {
    api.get_guild(token, guild_id).await
}

// 使用示例
let guild = get_guild_info(&api, &token, "guild_123").await?;
println!("频道名称: {}", guild.name.unwrap_or_default());
println!("成员数量: {}", guild.member_count.unwrap_or_default());
```

### 获取子频道列表

```rust
async fn list_channels(
    api: &BotApi,
    token: &Token,
    guild_id: &str
) -> Result<Vec<Channel>, BotError> {
    api.get_guild_channels(token, guild_id).await
}

// 使用示例
let channels = list_channels(&api, &token, "guild_123").await?;
for channel in channels {
    println!("频道: {} ({})", 
             channel.name.unwrap_or_default(), 
             channel.id);
}
```

### 创建子频道

```rust
use botrs::{ChannelType, ChannelSubType};

async fn create_text_channel(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    channel_name: &str
) -> Result<Channel, BotError> {
    let channel_data = serde_json::json!({
        "name": channel_name,
        "type": ChannelType::Text as u8,
        "sub_type": ChannelSubType::Chat as u8,
        "position": 1
    });
    
    api.create_guild_channel(token, guild_id, &channel_data).await
}
```

### 修改子频道

```rust
async fn modify_channel(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    new_name: &str
) -> Result<Channel, BotError> {
    let update_data = serde_json::json!({
        "name": new_name
    });
    
    api.modify_guild_channel(token, channel_id, &update_data).await
}
```

## 成员管理

### 获取成员信息

```rust
async fn get_member_info(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    user_id: &str
) -> Result<Member, BotError> {
    api.get_guild_member(token, guild_id, user_id).await
}

// 使用示例
let member = get_member_info(&api, &token, "guild_123", "user_456").await?;
println!("成员昵称: {}", member.nick.unwrap_or_default());
```

### 获取成员列表

```rust
async fn list_guild_members(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    limit: Option<u32>
) -> Result<Vec<Member>, BotError> {
    api.get_guild_members(token, guild_id, None, limit).await
}

// 分页获取所有成员
async fn get_all_members(
    api: &BotApi,
    token: &Token,
    guild_id: &str
) -> Result<Vec<Member>, BotError> {
    let mut all_members = Vec::new();
    let mut after = None;
    let limit = 400; // QQ API 的最大限制
    
    loop {
        let members = api.get_guild_members(token, guild_id, after.as_deref(), Some(limit)).await?;
        
        if members.is_empty() {
            break;
        }
        
        after = members.last().map(|m| m.user.id.clone());
        all_members.extend(members);
        
        // 避免速率限制
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    Ok(all_members)
}
```

### 管理成员权限

```rust
async fn add_member_role(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    role_id: &str
) -> Result<(), BotError> {
    api.add_guild_member_role(token, guild_id, user_id, role_id).await
}

async fn remove_member_role(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    role_id: &str
) -> Result<(), BotError> {
    api.remove_guild_member_role(token, guild_id, user_id, role_id).await
}
```

## 私信 API

### 发送私信

```rust
async fn send_direct_message(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    user_id: &str,
    content: &str
) -> Result<DirectMessage, BotError> {
    // 首先创建私信会话
    let dm_session = api.create_direct_message_session(token, guild_id, user_id).await?;
    
    // 然后发送消息
    let params = MessageParams::new_text(content);
    api.post_direct_message_with_params(token, guild_id, &dm_session.channel_id, params).await
}
```

### 获取私信历史

```rust
async fn get_dm_history(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    channel_id: &str,
    limit: u32
) -> Result<Vec<DirectMessage>, BotError> {
    api.get_direct_messages(token, guild_id, channel_id, Some(limit)).await
}
```

## 群组 API

### 发送群组消息

```rust
async fn send_group_message(
    api: &BotApi,
    token: &Token,
    group_id: &str,
    content: &str
) -> Result<GroupMessage, BotError> {
    let params = MessageParams::new_text(content);
    api.post_group_message_with_params(token, group_id, params).await
}
```

### 发送 C2C 消息

```rust
async fn send_c2c_message(
    api: &BotApi,
    token: &Token,
    user_id: &str,
    content: &str
) -> Result<C2CMessage, BotError> {
    let params = MessageParams::new_text(content);
    api.post_c2c_message_with_params(token, user_id, params).await
}
```

## 公告 API

### 创建公告

```rust
use botrs::Announce;

async fn create_announcement(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    channel_id: &str,
    content: &str
) -> Result<Announce, BotError> {
    let announce_data = serde_json::json!({
        "message": content,
        "channel_id": channel_id
    });
    
    api.create_guild_announce(token, guild_id, &announce_data).await
}
```

### 删除公告

```rust
async fn delete_announcement(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
    announce_id: &str
) -> Result<(), BotError> {
    api.delete_guild_announce(token, guild_id, announce_id).await
}
```

## 表情回应 API

### 添加表情回应

```rust
async fn add_reaction(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str
) -> Result<(), BotError> {
    api.put_message_reaction(token, channel_id, message_id, emoji).await
}
```

### 删除表情回应

```rust
async fn remove_reaction(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str
) -> Result<(), BotError> {
    api.delete_message_reaction(token, channel_id, message_id, emoji).await
}
```

### 获取表情回应用户列表

```rust
async fn get_reaction_users(
    api: &BotApi,
    token: &Token,
    channel_id: &str,
    message_id: &str,
    emoji: &str
) -> Result<Vec<User>, BotError> {
    let reaction_users = api.get_message_reaction_users(
        token, 
        channel_id, 
        message_id, 
        emoji,
        None, // cookie
        None  // limit
    ).await?;
    
    Ok(reaction_users.users)
}
```

## 错误处理

### 基础错误处理

```rust
use botrs::BotError;

async fn handle_api_errors(api: &BotApi, token: &Token) {
    let result = api.get_guild(token, "invalid_guild_id").await;
    
    match result {
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
}
```

### 自动重试机制

```rust
use tokio::time::{sleep, Duration};

async fn api_call_with_retry<T, F, Fut>(
    operation: F,
    max_retries: usize,
) -> Result<T, BotError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, BotError>>,
{
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(BotError::RateLimited(retry_after)) => {
                if attempt < max_retries {
                    sleep(Duration::from_secs(retry_after)).await;
                    continue;
                }
                last_error = Some(BotError::RateLimited(retry_after));
            }
            Err(BotError::Network(_)) => {
                if attempt < max_retries {
                    sleep(Duration::from_secs(2_u64.pow(attempt as u32))).await;
                    continue;
                }
                last_error = Some(BotError::Network("重试失败".to_string()));
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(last_error.unwrap_or_else(|| BotError::Custom("重试次数用尽".to_string())))
}

// 使用示例
let result = api_call_with_retry(
    || api.get_guild(&token, "guild_123"),
    3
).await?;
```

## 批量操作

### 批量发送消息

```rust
async fn send_messages_to_multiple_channels(
    api: &BotApi,
    token: &Token,
    channel_ids: &[String],
    content: &str
) -> Result<Vec<Message>, BotError> {
    let mut results = Vec::new();
    
    for channel_id in channel_ids {
        let params = MessageParams::new_text(content);
        
        match api.post_message_with_params(token, channel_id, params).await {
            Ok(message) => {
                results.push(message);
                println!("消息发送到频道 {} 成功", channel_id);
            }
            Err(e) => {
                eprintln!("向频道 {} 发送消息失败: {}", channel_id, e);
            }
        }
        
        // 避免速率限制
        sleep(Duration::from_millis(500)).await;
    }
    
    Ok(results)
}
```

### 并发 API 调用

```rust
use futures::future::try_join_all;

async fn concurrent_guild_info(
    api: &BotApi,
    token: &Token,
    guild_ids: &[String]
) -> Result<Vec<Guild>, BotError> {
    let futures: Vec<_> = guild_ids.iter()
        .map(|guild_id| api.get_guild(token, guild_id))
        .collect();
    
    try_join_all(futures).await
}
```

## 数据缓存

### 简单缓存实现

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

struct CachedApiClient {
    api: BotApi,
    guild_cache: Arc<RwLock<HashMap<String, (Guild, Instant)>>>,
    cache_duration: Duration,
}

impl CachedApiClient {
    pub fn new(api: BotApi, cache_duration: Duration) -> Self {
        Self {
            api,
            guild_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration,
        }
    }
    
    pub async fn get_guild_cached(
        &self,
        token: &Token,
        guild_id: &str
    ) -> Result<Guild, BotError> {
        // 检查缓存
        {
            let cache = self.guild_cache.read().await;
            if let Some((guild, cached_at)) = cache.get(guild_id) {
                if cached_at.elapsed() < self.cache_duration {
                    return Ok(guild.clone());
                }
            }
        }
        
        // 缓存未命中，从 API 获取
        let guild = self.api.get_guild(token, guild_id).await?;
        
        // 更新缓存
        {
            let mut cache = self.guild_cache.write().await;
            cache.insert(guild_id.to_string(), (guild.clone(), Instant::now()));
        }
        
        Ok(guild)
    }
}
```

## 最佳实践

### API 客户端封装

```rust
pub struct BotApiWrapper {
    api: BotApi,
    token: Token,
}

impl BotApiWrapper {
    pub fn new(token: Token) -> Self {
        Self {
            api: BotApi::new(),
            token,
        }
    }
    
    pub async fn send_message(&self, channel_id: &str, content: &str) -> Result<Message, BotError> {
        let params = MessageParams::new_text(content);
        self.api.post_message_with_params(&self.token, channel_id, params).await
    }
    
    pub async fn send_embed(&self, channel_id: &str, embed: Embed) -> Result<Message, BotError> {
        let params = MessageParams::new_embed(embed);
        self.api.post_message_with_params(&self.token, channel_id, params).await
    }
    
    pub async fn get_guild(&self, guild_id: &str) -> Result<Guild, BotError> {
        self.api.get_guild(&self.token, guild_id).await
    }
}
```

### 配置管理

```rust
#[derive(Clone)]
pub struct ApiConfig {
    pub timeout: Duration,
    pub retry_attempts: usize,
    pub rate_limit_delay: Duration,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            rate_limit_delay: Duration::from_millis(500),
        }
    }
}
```

### 日志记录

```rust
use tracing::{info, warn, error};

async fn logged_api_call<T>(
    operation_name: &str,
    operation: impl std::future::Future<Output = Result<T, BotError>>
) -> Result<T, BotError> {
    info!("开始执行 API 操作: {}", operation_name);
    
    match operation.await {
        Ok(result) => {
            info!("API 操作成功: {}", operation_name);
            Ok(result)
        }
        Err(e) => {
            error!("API 操作失败: {} - {}", operation_name, e);
            Err(e)
        }
    }
}

// 使用示例
let guild = logged_api_call(
    "获取频道信息",
    api.get_guild(&token, "guild_123")
).await?;
```

API 客户端是 BotRS 的核心组件之一，提供了与 QQ 频道平台交互的所有必要功能。通过合理使用错误处理、重试机制和缓存策略，您可以构建出健壮且高性能的机器人应用程序。

## 另请参阅

- [消息处理指南](/zh/guide/messages.md) - 详细的消息发送和处理
- [错误处理指南](/zh/guide/error-handling.md) - API 错误处理策略
- [`BotApi` API 参考](/zh/api/bot-api.md) - 完整的 API 参考文档
- [`Token` API 参考](/zh/api/token.md) - 身份验证令牌管理
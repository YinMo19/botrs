# 上下文 API 参考

`Context` 结构体在事件处理器中提供对 API 客户端和身份验证令牌的访问。它是事件处理器方法的标准参数，允许您与 QQ 频道 API 进行交互。

## 概述

```rust
pub struct Context {
    pub api: BotApi,
    pub token: Token,
}
```

`Context` 包含：
- `api`: 用于发送请求到 QQ 频道 API 的客户端
- `token`: 用于身份验证的令牌

## 字段

### `api`

提供对所有 QQ 频道 REST API 端点的访问。

```rust
pub api: BotApi
```

通过此字段，您可以：
- 发送消息到频道和私信
- 管理频道和子频道
- 处理成员权限
- 上传文件和媒体
- 管理机器人设置

#### 示例

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // 使用 API 客户端发送消息
    let params = MessageParams::new_text("你好！");
    let result = ctx.api.post_message_with_params(
        &ctx.token,
        &message.channel_id,
        params
    ).await;
}
```

### `token`

包含机器人身份验证凭据的令牌。

```rust
pub token: Token
```

令牌用于：
- API 请求的身份验证
- 授权头的生成
- 访问令牌管理

#### 示例

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // 令牌会自动用于身份验证
    let channels = ctx.api.get_guild_channels(&ctx.token, &guild_id).await?;
}
```

## 使用方法

### 发送消息

#### 文本消息

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let params = MessageParams::new_text("简单文本消息");
    let _ = ctx.api.post_message_with_params(
        &ctx.token,
        &message.channel_id,
        params
    ).await;
}
```

#### 富文本消息

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let embed = Embed::new()
        .title("标题")
        .description("描述")
        .color(0x00ff00);
    
    let params = MessageParams::new_embed(embed);
    let _ = ctx.api.post_message_with_params(
        &ctx.token,
        &message.channel_id,
        params
    ).await;
}
```

#### 回复消息

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let params = MessageParams::new_text("这是一个回复")
        .with_reply(&message.id);
    
    let _ = ctx.api.post_message_with_params(
        &ctx.token,
        &message.channel_id,
        params
    ).await;
}
```

### 频道管理

#### 获取频道信息

```rust
async fn guild_create(&self, ctx: Context, guild: Guild) {
    match ctx.api.get_guild(&ctx.token, &guild.id).await {
        Ok(guild_info) => {
            println!("频道名称: {}", guild_info.name.unwrap_or_default());
        }
        Err(e) => {
            eprintln!("获取频道信息失败: {}", e);
        }
    }
}
```

#### 获取子频道列表

```rust
async fn ready(&self, ctx: Context, ready: Ready) {
    for guild in &ready.guilds {
        match ctx.api.get_guild_channels(&ctx.token, &guild.id).await {
            Ok(channels) => {
                println!("频道 {} 有 {} 个子频道", guild.id, channels.len());
            }
            Err(e) => {
                eprintln!("获取子频道失败: {}", e);
            }
        }
    }
}
```

### 成员管理

#### 获取成员信息

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {
    if let Some(guild_id) = &member.guild_id {
        match ctx.api.get_guild_member(&ctx.token, guild_id, &member.user.id).await {
            Ok(member_info) => {
                println!("新成员: {:?}", member_info.nick);
            }
            Err(e) => {
                eprintln!("获取成员信息失败: {}", e);
            }
        }
    }
}
```

#### 获取成员列表

```rust
async fn ready(&self, ctx: Context, ready: Ready) {
    for guild in &ready.guilds {
        match ctx.api.get_guild_members(&ctx.token, &guild.id, None, None).await {
            Ok(members) => {
                println!("频道 {} 有 {} 个成员", guild.id, members.len());
            }
            Err(e) => {
                eprintln!("获取成员列表失败: {}", e);
            }
        }
    }
}
```

### 文件上传

#### 上传图片

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        if content.trim() == "!upload_image" {
            // 读取本地图片文件
            let image_data = std::fs::read("path/to/image.png").unwrap();
            
            match ctx.api.post_message_with_file(
                &ctx.token,
                &message.channel_id,
                "image.png",
                &image_data,
                "png"
            ).await {
                Ok(_) => println!("图片上传成功"),
                Err(e) => eprintln!("图片上传失败: {}", e),
            }
        }
    }
}
```

### 私信处理

```rust
async fn direct_message_create(&self, ctx: Context, dm: DirectMessage) {
    // 回复私信
    let params = MessageParams::new_text("感谢您的私信！");
    
    if let Some(guild_id) = &dm.guild_id {
        let _ = ctx.api.post_direct_message_with_params(
            &ctx.token,
            guild_id,
            &dm.channel_id,
            params
        ).await;
    }
}
```

### 群组消息处理

```rust
async fn group_message_create(&self, ctx: Context, group_msg: GroupMessage) {
    if let Some(content) = &group_msg.content {
        if content.contains("帮助") {
            let help_text = "这里是帮助信息...";
            let _ = group_msg.reply(&ctx.api, &ctx.token, help_text).await;
        }
    }
}
```

## 错误处理

### 基础错误处理

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let params = MessageParams::new_text("测试消息");
    
    match ctx.api.post_message_with_params(&ctx.token, &message.channel_id, params).await {
        Ok(sent_message) => {
            println!("消息发送成功，ID: {}", sent_message.id);
        }
        Err(BotError::RateLimited(retry_after)) => {
            println!("触发速率限制，{}秒后重试", retry_after);
            tokio::time::sleep(Duration::from_secs(retry_after)).await;
            // 重试逻辑
        }
        Err(BotError::Authentication(_)) => {
            eprintln!("身份验证失败，检查令牌");
        }
        Err(BotError::Network(_)) => {
            eprintln!("网络错误，稍后重试");
        }
        Err(e) => {
            eprintln!("其他错误: {}", e);
        }
    }
}
```

### 高级错误处理

```rust
use botrs::{BotError, Result};
use tracing::{warn, error};

impl MyEventHandler {
    async fn safe_send_message(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<Message> {
        let params = MessageParams::new_text(content);
        
        for attempt in 1..=3 {
            match ctx.api.post_message_with_params(&ctx.token, channel_id, params.clone()).await {
                Ok(message) => return Ok(message),
                Err(BotError::RateLimited(retry_after)) => {
                    warn!("速率限制，尝试 {}/3，{}秒后重试", attempt, retry_after);
                    tokio::time::sleep(Duration::from_secs(retry_after)).await;
                }
                Err(BotError::Network(_)) if attempt < 3 => {
                    warn!("网络错误，尝试 {}/3，1秒后重试", attempt);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Err(e) => {
                    error!("发送消息失败: {}", e);
                    return Err(e);
                }
            }
        }
        
        Err(BotError::Custom("重试次数已用尽".to_string()))
    }
}
```

## 实用工具

### 消息构建器

```rust
impl MyEventHandler {
    fn build_help_message(&self) -> MessageParams {
        let embed = Embed::new()
            .title("机器人帮助")
            .description("可用命令列表")
            .field("!ping", "测试机器人响应", false)
            .field("!help", "显示此帮助信息", false)
            .field("!stats", "显示统计信息", false)
            .color(0x3498db)
            .timestamp(chrono::Utc::now());
        
        MessageParams::new_embed(embed)
    }
    
    async fn send_help(&self, ctx: &Context, channel_id: &str) -> Result<()> {
        let params = self.build_help_message();
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        Ok(())
    }
}
```

### 权限检查

```rust
impl MyEventHandler {
    async fn check_admin_permission(&self, ctx: &Context, guild_id: &str, user_id: &str) -> bool {
        match ctx.api.get_guild_member(&ctx.token, guild_id, user_id).await {
            Ok(member) => {
                // 检查成员是否有管理员权限
                member.roles.iter().any(|role| role.permissions & 0x8 != 0) // ADMINISTRATOR
            }
            Err(_) => false,
        }
    }
}
```

### 批量操作

```rust
impl MyEventHandler {
    async fn send_to_multiple_channels(&self, ctx: &Context, guild_id: &str, content: &str) {
        match ctx.api.get_guild_channels(&ctx.token, guild_id).await {
            Ok(channels) => {
                let text_channels: Vec<_> = channels.iter()
                    .filter(|ch| ch.channel_type == ChannelType::Text)
                    .collect();
                
                for channel in text_channels {
                    let params = MessageParams::new_text(content);
                    if let Err(e) = ctx.api.post_message_with_params(
                        &ctx.token, 
                        &channel.id, 
                        params
                    ).await {
                        warn!("向频道 {} 发送消息失败: {}", channel.id, e);
                    }
                    
                    // 避免速率限制
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
            Err(e) => {
                error!("获取频道列表失败: {}", e);
            }
        }
    }
}
```

## 最佳实践

### 并发安全

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

struct RateLimitedHandler {
    semaphore: Arc<Semaphore>,
}

impl RateLimitedHandler {
    pub fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(5)), // 最多5个并发请求
        }
    }
    
    async fn safe_api_call<F, T>(&self, f: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let _permit = self.semaphore.acquire().await.unwrap();
        f.await
    }
}
```

### 资源管理

```rust
impl MyEventHandler {
    async fn cleanup_resources(&self, ctx: &Context) {
        // 清理临时文件
        // 关闭数据库连接
        // 保存状态等
    }
}
```

### 配置管理

```rust
#[derive(Clone)]
struct BotConfig {
    admin_users: Vec<String>,
    allowed_channels: Vec<String>,
    command_prefix: String,
}

struct ConfigurableBot {
    config: BotConfig,
}

impl ConfigurableBot {
    fn is_admin(&self, user_id: &str) -> bool {
        self.config.admin_users.contains(&user_id.to_string())
    }
    
    fn is_allowed_channel(&self, channel_id: &str) -> bool {
        self.config.allowed_channels.is_empty() || 
        self.config.allowed_channels.contains(&channel_id.to_string())
    }
}
```

## 另请参阅

- [`BotApi`](./bot-api.md) - 详细的 API 客户端参考
- [`Token`](./token.md) - 身份验证令牌管理
- [`EventHandler`](./event-handler.md) - 事件处理器实现
- [消息处理指南](/zh/guide/messages.md) - 消息发送和处理
- [错误处理指南](/zh/guide/error-handling.md) - 错误处理策略
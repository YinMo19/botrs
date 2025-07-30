# 事件处理器 API 参考

`EventHandler` trait 定义了机器人如何响应来自 QQ 频道网关的各种事件。您需要实现此 trait 来定义机器人的行为。

## 概述

```rust
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    // 事件处理方法...
}
```

`EventHandler` 提供了处理所有 QQ 频道事件的钩子方法。所有方法都有默认的空实现，因此您只需要实现您关心的事件。

## 核心事件

### `ready`

当机器人成功连接到网关并准备接收事件时调用。

```rust
async fn ready(&self, ctx: Context, ready: Ready) {}
```

#### 参数

- `ctx`: 包含 API 客户端和令牌的上下文
- `ready`: 包含机器人信息和会话详情的就绪事件

#### 示例

```rust
async fn ready(&self, ctx: Context, ready: Ready) {
    println!("机器人已就绪: {}", ready.user.username);
    println!("会话 ID: {}", ready.session_id);
    println!("分片: {}/{}", ready.shard.unwrap_or_default().0, ready.shard.unwrap_or_default().1);
}
```

## 消息事件

### `message_create`

当收到 @ 提及机器人的消息时调用。

```rust
async fn message_create(&self, ctx: Context, message: Message) {}
```

#### 参数

- `ctx`: API 上下文
- `message`: 收到的消息对象

#### 示例

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // 忽略来自机器人的消息
    if message.is_from_bot() {
        return;
    }

    if let Some(content) = &message.content {
        if content.trim() == "!ping" {
            let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
        }
    }
}
```

### `direct_message_create`

当收到私信消息时调用。

```rust
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {}
```

#### 参数

- `ctx`: API 上下文
- `message`: 收到的私信对象

#### 示例

```rust
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    if let Some(content) = &message.content {
        println!("收到私信: {}", content);
        let _ = message.reply(&ctx.api, &ctx.token, "感谢您的私信！").await;
    }
}
```

### `group_message_create`

当在群组中收到消息时调用。

```rust
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {}
```

#### 参数

- `ctx`: API 上下文
- `message`: 收到的群组消息对象

#### 示例

```rust
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    if let Some(content) = &message.content {
        if content.contains("机器人") {
            let _ = message.reply(&ctx.api, &ctx.token, "您提到了我！").await;
        }
    }
}
```

### `c2c_message_create`

当收到用户对用户（C2C）消息时调用。

```rust
async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {}
```

### `message_delete`

当消息被删除时调用。

```rust
async fn message_delete(&self, ctx: Context, message: Message) {}
```

## 频道事件

### `guild_create`

当机器人加入新频道时调用。

```rust
async fn guild_create(&self, ctx: Context, guild: Guild) {}
```

#### 示例

```rust
async fn guild_create(&self, ctx: Context, guild: Guild) {
    println!("加入了新频道: {}", guild.name.unwrap_or_default());
}
```

### `guild_update`

当频道信息更新时调用。

```rust
async fn guild_update(&self, ctx: Context, guild: Guild) {}
```

### `guild_delete`

当机器人离开频道时调用。

```rust
async fn guild_delete(&self, ctx: Context, guild: Guild) {}
```

## 子频道事件

### `channel_create`

当创建新子频道时调用。

```rust
async fn channel_create(&self, ctx: Context, channel: Channel) {}
```

### `channel_update`

当子频道信息更新时调用。

```rust
async fn channel_update(&self, ctx: Context, channel: Channel) {}
```

### `channel_delete`

当子频道被删除时调用。

```rust
async fn channel_delete(&self, ctx: Context, channel: Channel) {}
```

## 成员事件

### `guild_member_add`

当新成员加入频道时调用。

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {}
```

### `guild_member_update`

当成员信息更新时调用。

```rust
async fn guild_member_update(&self, ctx: Context, member: Member) {}
```

### `guild_member_remove`

当成员离开频道时调用。

```rust
async fn guild_member_remove(&self, ctx: Context, member: Member) {}
```

## 审核事件

### `message_audit_pass`

当消息审核通过时调用。

```rust
async fn message_audit_pass(&self, ctx: Context, audit: MessageAudit) {}
```

### `message_audit_reject`

当消息审核被拒绝时调用。

```rust
async fn message_audit_reject(&self, ctx: Context, audit: MessageAudit) {}
```

## 好友管理事件

### `friend_add`

当添加好友时调用。

```rust
async fn friend_add(&self, ctx: Context, event: C2CManageEvent) {}
```

### `friend_del`

当删除好友时调用。

```rust
async fn friend_del(&self, ctx: Context, event: C2CManageEvent) {}
```

### `c2c_msg_reject`

当 C2C 消息被拒绝时调用。

```rust
async fn c2c_msg_reject(&self, ctx: Context, event: C2CManageEvent) {}
```

### `c2c_msg_receive`

当收到 C2C 消息时调用。

```rust
async fn c2c_msg_receive(&self, ctx: Context, event: C2CManageEvent) {}
```

## 群组管理事件

### `group_add_robot`

当机器人被添加到群组时调用。

```rust
async fn group_add_robot(&self, ctx: Context, event: GroupManageEvent) {}
```

### `group_del_robot`

当机器人从群组中移除时调用。

```rust
async fn group_del_robot(&self, ctx: Context, event: GroupManageEvent) {}
```

### `group_msg_reject`

当群组消息被拒绝时调用。

```rust
async fn group_msg_reject(&self, ctx: Context, event: GroupManageEvent) {}
```

### `group_msg_receive`

当接收到群组消息时调用。

```rust
async fn group_msg_receive(&self, ctx: Context, event: GroupManageEvent) {}
```

## 实现示例

### 基础事件处理器

```rust
use botrs::{Context, EventHandler, Message, Ready};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人 {} 已就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            match content.trim() {
                "!ping" => {
                    let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
                }
                "!help" => {
                    let help_text = "可用命令:\n• !ping - 测试响应\n• !help - 显示此帮助";
                    let _ = message.reply(&ctx.api, &ctx.token, help_text).await;
                }
                _ => {}
            }
        }
    }
}
```

### 带状态的事件处理器

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

struct StatefulBot {
    message_count: Arc<Mutex<HashMap<String, u64>>>,
}

impl StatefulBot {
    pub fn new() -> Self {
        Self {
            message_count: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // 更新消息计数
        if let Some(author) = &message.author {
            if let Some(username) = &author.username {
                let mut counts = self.message_count.lock().await;
                let count = counts.entry(username.clone()).or_insert(0);
                *count += 1;

                if let Some(content) = &message.content {
                    if content.trim() == "!stats" {
                        let response = format!("您已发送 {} 条消息", count);
                        let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                    }
                }
            }
        }
    }
}
```

### 错误处理

```rust
use tracing::{error, warn};

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content.trim() == "!error_demo" {
                match message.reply(&ctx.api, &ctx.token, "测试回复").await {
                    Ok(_) => println!("回复发送成功"),
                    Err(e) => {
                        error!("发送回复失败: {}", e);
                        // 可以尝试其他恢复策略
                    }
                }
            }
        }
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("事件处理器错误: {}", error);
    }
}
```

## 最佳实践

### 性能考虑

1. **避免阻塞操作**: 所有方法都是异步的，避免在其中执行阻塞操作
2. **并发处理**: 可以并发处理多个事件，但要注意共享状态的同步
3. **错误处理**: 始终妥善处理可能的错误，避免崩溃

### 状态管理

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

struct BotState {
    commands_processed: Arc<RwLock<u64>>,
    start_time: std::time::Instant,
}

impl BotState {
    async fn increment_commands(&self) {
        let mut count = self.commands_processed.write().await;
        *count += 1;
    }

    async fn get_stats(&self) -> (u64, std::time::Duration) {
        let count = *self.commands_processed.read().await;
        let uptime = self.start_time.elapsed();
        (count, uptime)
    }
}
```

### 模块化设计

```rust
mod commands;
mod utils;

use commands::CommandHandler;
use utils::MessageUtils;

struct ModularBot {
    command_handler: CommandHandler,
    utils: MessageUtils,
}

#[async_trait::async_trait]
impl EventHandler for ModularBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if self.utils.is_command(content) {
                self.command_handler.handle(&ctx, &message, content).await;
            }
        }
    }
}
```

## 另请参阅

- [`Context`](./context.md) - 事件处理器中可用的上下文对象
- [`Client`](./client.md) - 配置和启动机器人
- [消息处理指南](/zh/guide/messages.md) - 处理不同类型的消息
- [错误处理指南](/zh/guide/error-handling.md) - 错误处理最佳实践
# 回声机器人示例

此示例演示如何创建一个简单的回声机器人，通过重复用户发送的消息内容来响应用户。

## 概述

回声机器人是最简单的机器人类型，用于演示基本的消息处理。当用户发送消息时，机器人会用相同的消息内容进行响应。

## 基础回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token};
use tracing::{info, warn};

struct EchoBot;

#[async_trait::async_trait]
impl EventHandler for EchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        // 跳过机器人消息以避免循环
        if msg.is_from_bot() {
            return;
        }

        // 回声消息内容
        if let Some(content) = &msg.content {
            let echo_msg = format!("回声：{}", content);
            if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_msg).await {
                warn!("发送回声消息失败：{}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    let mut client = Client::new(token, intents, EchoBot, false)?;
    client.start().await?;
    Ok(())
}
```

## 带命令的增强回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token};
use tracing::{info, warn};

struct SmartEchoBot;

#[async_trait::async_trait]
impl EventHandler for SmartEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("智能回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        // 跳过机器人消息
        if msg.is_from_bot() {
            return;
        }

        if let Some(content) = &msg.content {
            let response = match content.trim() {
                "!ping" => Some("Pong! 🏓".to_string()),
                "!帮助" | "!help" => Some(
                    "可用命令：\n• `!ping` - 测试机器人响应\n• `!回声 <消息>` - 回声自定义消息\n• 其他任何消息都会被回声"
                        .to_string(),
                ),
                _ if content.starts_with("!回声 ") => {
                    let echo_content = &content[7..]; // 移除 "!回声 " 前缀
                    Some(format!("你说：{}", echo_content))
                }
                _ if content.starts_with("!echo ") => {
                    let echo_content = &content[6..]; // 移除 "!echo " 前缀
                    Some(format!("你说：{}", echo_content))
                }
                _ => {
                    // 回声普通消息
                    Some(format!("回声：{}", content))
                }
            };

            if let Some(response_text) = response {
                if let Err(e) = msg.reply(&ctx.api, &ctx.token, &response_text).await {
                    warn!("发送回复失败：{}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,smart_echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    let mut client = Client::new(token, intents, SmartEchoBot, false)?;
    client.start().await?;
    Ok(())
}
```

## 支持回复的回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token, MessageParams};
use tracing::{info, warn};

struct ReplyEchoBot;

#[async_trait::async_trait]
impl EventHandler for ReplyEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("回复回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.is_from_bot() {
            return;
        }

        if let Some(content) = &msg.content {
            // 创建对原消息的回复
            let echo_content = format!("你说：{}", content);

            // 使用回复功能
            if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_content).await {
                warn!("回复消息失败：{}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,reply_echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    let mut client = Client::new(token, intents, ReplyEchoBot, false)?;
    client.start().await?;
    Ok(())
}
```

## 富嵌入消息回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token, MessageEmbed, MessageParams};
use tracing::{info, warn};

struct RichEchoBot;

#[async_trait::async_trait]
impl EventHandler for RichEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("富消息回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.is_from_bot() {
            return;
        }

        if let Some(content) = &msg.content {
            // 为回声响应创建嵌入消息
            let embed = MessageEmbed {
                title: Some("回声响应".to_string()),
                description: Some(format!("你说：{}", content)),
                color: Some(0x00ff00), // 绿色
                fields: Some(vec![
                    botrs::MessageEmbedField {
                        name: "原始消息".to_string(),
                        value: content.clone(),
                        inline: Some(false),
                    },
                    botrs::MessageEmbedField {
                        name: "频道".to_string(),
                        value: msg.channel_id.clone(),
                        inline: Some(true),
                    },
                ]),
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
                ..Default::default()
            };

            let params = MessageParams::new_embed(embed);
            if let Err(e) = ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await {
                warn!("发送嵌入消息失败：{}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,rich_echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    let mut client = Client::new(token, intents, RichEchoBot, false)?;
    client.start().await?;
    Ok(())
}
```

## 多频道回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token, DirectMessage, GroupMessage};
use tracing::{info, warn};

struct MultiChannelEchoBot;

#[async_trait::async_trait]
impl EventHandler for MultiChannelEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("多频道回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.is_from_bot() {
            return;
        }

        if let Some(content) = &msg.content {
            let echo_msg = format!("频道回声：{}", content);
            if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_msg).await {
                warn!("发送频道回声消息失败：{}", e);
            }
        }
    }

    async fn direct_message_create(&self, ctx: Context, msg: DirectMessage) {
        if let Some(content) = &msg.content {
            let echo_msg = format!("私信回声：{}", content);
            if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_msg).await {
                warn!("发送私信回声消息失败：{}", e);
            }
        }
    }

    async fn group_message_create(&self, ctx: Context, msg: GroupMessage) {
        if let Some(content) = &msg.content {
            let echo_msg = format!("群组回声：{}", content);
            if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_msg).await {
                warn!("发送群组回声消息失败：{}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,multi_channel_echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_group_at_messages()
        .with_guilds();

    let mut client = Client::new(token, intents, MultiChannelEchoBot, false)?;
    client.start().await?;
    Ok(())
}
```

## 带速率限制的回声机器人

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};
use tracing::{info, warn};

struct RateLimitedEchoBot {
    last_message: Arc<Mutex<HashMap<String, Instant>>>,
}

impl RateLimitedEchoBot {
    fn new() -> Self {
        Self {
            last_message: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn can_respond(&self, user_id: &str) -> bool {
        let mut last_messages = self.last_message.lock().await;
        let now = Instant::now();

        if let Some(last_time) = last_messages.get(user_id) {
            if now.duration_since(*last_time) < Duration::from_secs(5) {
                return false; // 速率限制
            }
        }

        last_messages.insert(user_id.to_string(), now);
        true
    }
}

#[async_trait::async_trait]
impl EventHandler for RateLimitedEchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("限速回声机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.is_from_bot() {
            return;
        }

        if let Some(author) = &msg.author {
            if let Some(user_id) = &author.id {
                // 检查速率限制
                if !self.can_respond(user_id).await {
                    return; // 如果被限速则跳过
                }

                if let Some(content) = &msg.content {
                    let echo_msg = format!("回声：{}", content);
                    if let Err(e) = msg.reply(&ctx.api, &ctx.token, &echo_msg).await {
                        warn!("发送回声消息失败：{}", e);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,rate_limited_echo_bot=info")
        .init();

    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds();

    let mut client = Client::new(token, intents, RateLimitedEchoBot::new(), false)?;
    client.start().await?;
    Ok(())
}
```

## 配置

在运行任何这些示例之前，请确保：

1. **设置环境变量：**
```bash
export QQ_BOT_APP_ID=your_app_id
export QQ_BOT_SECRET=your_secret
```

2. **在 Cargo.toml 中添加依赖项：**
```toml
[dependencies]
botrs = "0.2"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"
chrono = { version = "0.4", features = ["serde"] }
```

3. **在 QQ 开发者门户中启用所需的 Intent：**
   - 公域消息事件
   - 私信消息事件（如果使用私信功能）
   - 频道信息

## 演示的关键概念

1. **基本消息处理**：响应传入消息
2. **机器人消息过滤**：避免机器人消息的无限循环
3. **命令处理**：处理特定的命令模式
4. **回复功能**：使用消息回复提升用户体验
5. **富内容**：创建嵌入消息以增强展示效果
6. **多频道支持**：处理不同类型的消息
7. **速率限制**：防止垃圾信息和滥用

## 下一步

- [命令处理器](./command-handler.md) - 学习结构化命令处理
- [富消息](./rich-messages.md) - 探索高级消息格式
- [事件处理](./event-handling.md) - 处理消息以外的更多事件类型
- [错误恢复](./error-recovery.md) - 实现健壮的错误处理

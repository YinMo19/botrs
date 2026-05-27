# BotRS - Rust QQ Guild Bot Framework
## Author: YinMo19

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![GitHub](https://img.shields.io/badge/github-YinMo19-blue.svg)](https://github.com/YinMo19)
[![Crates.io](https://img.shields.io/crates/v/botrs.svg)](https://crates.io/crates/botrs)

> 本实现不是 qqbot 的官方 rust 版本实现。

BotRS 是一个用 Rust 实现的 QQ 频道机器人框架，基于 [QQ 频道机器人 API](https://bot.q.qq.com/wiki/develop/api/)。它提供了类型安全、高性能、易于使用的接口来开发 QQ 频道机器人。

## 快速开始

### 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
botrs = "0.11.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"
```

### 基础示例

```rust,no_run
use botrs::{Client, Context, EventHandler, Intents, Token, Message};
use botrs::models::gateway::Ready;
use botrs::models::message::MessageParams;
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                info!("Received ping command from message ID: {:?}", message.id);

                // 使用新的参数结构 API
                let params = MessageParams::new_text("Pong! 🏓");
                if let Some(channel_id) = &message.channel_id {
                    if let Err(e) = ctx.send_message(channel_id, params).await {
                        info!("Failed to reply: {}", e);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建令牌
    let token = Token::new("your_app_id", "your_secret");

    // 设置意图
    let intents = Intents::default();

    // 创建客户端
    let mut client = Client::new(token, intents, MyBot, false)?;

    // 启动机器人
    client.start().await?;

    Ok(())
}
```

我们强烈推荐你查看 [完整文档](https://botrs.yinmo.site/) 来进行开发。这个仓库基于官方网络接口行为和 Rust 的语言特性进行了深度开发。

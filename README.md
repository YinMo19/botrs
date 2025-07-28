# BotRS - Rust QQ Guild Bot Framework
## Author: YinMo19

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

BotRS 是一个用 Rust 实现的 QQ 频道机器人框架，基于 [QQ 频道机器人 API](https://bot.q.qq.com/wiki/develop/api/)。它提供了类型安全、高性能、易于使用的接口来开发 QQ 频道机器人。

## 特性

- ✨ **类型安全** - 完全类型化的 API，编译时捕获错误
- 🚀 **高性能** - 基于 Tokio 的异步运行时，支持高并发
- 🔧 **易于使用** - 简单直观的 API 设计，快速上手
- 🛡️ **内存安全** - Rust 的所有权系统保证内存安全
- 🔄 **事件驱动** - 基于事件的架构，响应各种 QQ 频道事件
- 📝 **丰富的文档** - 完整的 API 文档和示例代码
- ⚡ **WebSocket 支持** - 实时接收和处理事件
- 🎯 **Intent 系统** - 精确控制接收的事件类型

## 快速开始

### 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
botrs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"
```

### 基础示例

```rust
use botrs::{Client, Context, EventHandler, Intents, Token, Message};
use botrs::models::gateway::Ready;
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot {} is ready!", ready.user.username);
    }

    async fn message_create(&self, _ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                info!("Received ping command from message ID: {:?}", message.id);
                // 可以在这里回复消息
                if let Err(e) = message.reply("Pong!").await {
                    eprintln!("Failed to reply: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建令牌
    let token = Token::new("your_app_id", "your_secret");

    // 设置意图
    let intents = Intents::default();

    // 创建客户端
    let mut client = Client::new(token, intents, MyBot)?;

    // 启动机器人
    client.start().await?;

    Ok(())
}
```

## 环境变量配置

你可以使用环境变量来配置机器人凭据：

```bash
export QQ_BOT_APP_ID="your_app_id"
export QQ_BOT_SECRET="your_secret"
```

然后在代码中使用：

```rust
let token = Token::from_env()?;
```

## 事件处理

BotRS 支持多种事件类型：

### 消息事件

```rust
use botrs::{Message, DirectMessage, GroupMessage, C2CMessage};

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // @ 消息事件
    async fn message_create(&self, _ctx: Context, message: Message) {
        // 处理频道消息
        if let Some(content) = &message.content {
            println!("Received message: {}", content);
        }
    }

    // 私信事件
    async fn direct_message_create(&self, _ctx: Context, message: DirectMessage) {
        // 处理私信
        if let Some(content) = &message.content {
            println!("Received DM: {}", content);
        }
    }

    // 群消息事件
    async fn group_message_create(&self, _ctx: Context, message: GroupMessage) {
        // 处理群消息
        if let Some(content) = &message.content {
            println!("Received group message: {}", content);
        }
    }

    // C2C 消息事件
    async fn c2c_message_create(&self, _ctx: Context, message: C2CMessage) {
        // 处理单聊消息
        if let Some(content) = &message.content {
            println!("Received C2C message: {}", content);
        }
    }
}
```

### 频道事件

```rust
use botrs::Guild;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 加入频道
    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        // 机器人加入新频道时触发
        if let Some(name) = &guild.name {
            println!("Joined guild: {}", name);
        }
    }

    // 频道更新
    async fn guild_update(&self, _ctx: Context, guild: Guild) {
        // 频道信息更新时触发
        if let Some(name) = &guild.name {
            println!("Guild updated: {}", name);
        }
    }

    // 离开频道
    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        // 机器人离开频道时触发
        if let Some(name) = &guild.name {
            println!("Left guild: {}", name);
        }
    }
}
```

### 成员事件

```rust
use botrs::Member;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 成员加入
    async fn guild_member_add(&self, _ctx: Context, member: Member) {
        // 新成员加入频道时触发
        println!("Member joined: {}", member.user.username);
    }

    // 成员更新
    async fn guild_member_update(&self, _ctx: Context, member: Member) {
        // 成员信息更新时触发
        println!("Member updated: {}", member.user.username);
    }

    // 成员离开
    async fn guild_member_remove(&self, _ctx: Context, member: Member) {
        // 成员离开频道时触发
        println!("Member left: {}", member.user.username);
    }
}
```

## Intent 系统

Intent 系统允许你精确控制机器人接收的事件类型：

```rust
use botrs::Intents;

// 默认 intents（基础事件）
let intents = Intents::default();

// 自定义 intents
let intents = Intents::none()
    .with_guilds()                // 频道事件
    .with_guild_members()         // 成员事件
    .with_guild_messages()        // 频道消息
    .with_direct_message()        // 私信
    .with_public_messages();      // 群消息和单聊消息

// 所有可用的 intents
let intents = Intents::all();

// 检查特权 intent
if intents.is_privileged() {
    println!("Contains privileged intents");
}
```

### 特权 Intent

某些 Intent 需要特殊权限，可通过 `is_privileged()` 方法检查：

```rust
let intents = Intents::none()
    .with_guild_members()   // 特权 intent
    .with_guild_messages(); // 特权 intent

if intents.is_privileged() {
    println!("需要申请特殊权限");
}
```

## API 客户端

BotRS 提供了完整的 API 客户端来与 QQ 频道 API 交互：

```rust
use botrs::{BotApi, HttpClient, Token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("app_id", "secret");
    let http = HttpClient::new(30, false)?; // 30秒超时，非沙盒环境
    let api = BotApi::new(http);

    // 获取机器人信息
    let bot_info = api.get_bot_info(&token).await?;
    println!("Bot: {}", bot_info.username);

    // 获取网关信息
    let gateway = api.get_gateway(&token).await?;
    println!("Gateway URL: {}", gateway.url);

    // 注意：消息发送功能正在开发中
    // 目前可以通过事件处理中的 reply 方法回复消息

    Ok(())
}
```

## 错误处理

BotRS 提供了统一的错误处理：

```rust
use botrs::{BotError, Result};

async fn handle_api_call() -> Result<()> {
    match api.get_bot_info(&token).await {
        Ok(info) => {
            println!("Bot: {}", info.username);
        }
        Err(BotError::Api { code, message }) => {
            eprintln!("API error {}: {}", code, message);
        }
        Err(BotError::RateLimit { retry_after }) => {
            eprintln!("Rate limited, retry after {} seconds", retry_after);
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
    Ok(())
}
```

## 配置选项

### HTTP 客户端配置

```rust
use botrs::HttpClient;

// 自定义超时和环境
let http = HttpClient::new(60, true)?; // 60秒超时，沙盒环境
```

### 客户端配置

```rust
use botrs::Client;

// 标准创建方式
let client = Client::new(token, intents, handler)?;

// HTTP 客户端可以通过 HttpClient 进行配置
let http = HttpClient::new(60, true)?; // 60秒超时，沙盒环境
let api = BotApi::new(http);
```

## 运行示例

项目包含一个完整的示例机器人：

```bash
# 设置环境变量
export QQ_BOT_APP_ID="your_app_id"
export QQ_BOT_SECRET="your_secret"

# 运行示例
cargo run --example simple_bot --features examples
```

或者传递参数：

```bash
cargo run --example simple_bot --features examples -- your_app_id your_secret
```

## 开发状态

### 已完成功能

- ✅ 基础 HTTP 客户端和 API 封装
- ✅ WebSocket 网关连接和事件处理
- ✅ 完整的 Intent 系统实现
- ✅ 类型安全的错误处理
- ✅ 完整的消息模型 (Message, DirectMessage, GroupMessage, C2CMessage, MessageAudit)
- ✅ 频道、成员、用户、机器人数据模型
- ✅ Token 认证和验证系统
- ✅ 基于 Tokio 的异步支持
- ✅ 与 Python botpy 完全兼容的接口设计
- ✅ 完整的单元测试和文档测试覆盖
- ✅ 详细的 API 文档和使用示例

### 计划功能

- 🔄 完整的消息发送 API 实现
- 🔄 WebSocket 分片支持
- 🔄 中间件和插件系统
- 🔄 内置命令框架
- 🔄 更多实用示例和教程
- 🔄 性能优化和内存使用优化
- 🔄 更多 QQ 频道 API 功能支持

## 与 Python botpy 的对比

BotRS 的设计灵感来自 Python 的 [botpy](https://github.com/tencent-connect/botpy) 库，但提供了以下优势：

| 特性 | Python botpy | BotRS |
|------|--------------|-------|
| 类型安全 | ❌ | ✅ |
| 性能 | 中等 | 高 |
| 内存安全 | ❌ | ✅ |
| 并发模型 | asyncio | Tokio |
| 包大小 | 较大 | 较小 |
| 部署 | 需要Python环境 | 单一可执行文件 |

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献代码！我的个人 git commit 提交风格是
```
[type] simple message

- detail message 1: detailed description.
- detail message 2: detailed description.
- detail message 3: detailed description.
- detail message 4: detailed description.
- etc.
```
例如
```
[bug fix] remove error interface

- `models/api.rs` send interface: no `just/a/example/` interface exists.
```

## 支持

- 📖 [API 文档](https://docs.rs/botrs)
- 🐛 [问题反馈](https://github.com/YinMo19/botrs/issues)
- 💬 [讨论区](https://github.com/YinMo19/botrs/discussions)
- 📧 联系我们：me@yinmo19.top

## 架构特点

### 与 Python botpy 的完全兼容
BotRS 在设计时严格参照 Python botpy 的接口设计，确保：
- 相同的消息模型结构
- 一致的事件处理接口
- 兼容的数据类型定义
- 相同的 Intent 系统

### 类型安全保证
- 编译时类型检查
- Rust 所有权系统保证内存安全
- 详细的错误类型定义
- 可靠的异步处理

## 相关链接

- [QQ 频道机器人官方文档](https://bot.q.qq.com/wiki/)
- [QQ 频道机器人开发者平台](https://q.qq.com/qqbot/)
- [Python botpy 项目](https://github.com/tencent-connect/botpy)
- [Rust 官方网站](https://www.rust-lang.org/)
- [Tokio 异步运行时](https://tokio.rs/)

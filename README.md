# BotRS - Rust QQ Guild Bot Framework
## Author: YinMo19

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![GitHub](https://img.shields.io/badge/github-YinMo19-blue.svg)](https://github.com/YinMo19)
[![Crates.io](https://img.shields.io/crates/v/botrs.svg)](https://crates.io/crates/botrs)

BotRS 是一个用 Rust 实现的 QQ 频道机器人框架，基于 [QQ 频道机器人 API](https://bot.q.qq.com/wiki/develop/api/)。它提供了类型安全、高性能、易于使用的接口来开发 QQ 频道机器人。

## 📚 文档

- **[完整文档](https://botrs.yinmo.site/)** - 包含英文和中文的全面指南
- **[快速开始](https://botrs.yinmo.site/guide/quick-start)** - 5分钟内运行你的第一个机器人
- **[API 参考](https://botrs.yinmo.site/api/client)** - 详细的 API 文档
- **[示例代码](https://botrs.yinmo.site/examples/getting-started)** - 实用的代码示例
- **[更新日志](https://botrs.yinmo.site/changelog)** - 版本历史和迁移指南

### 本地运行文档

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm docs:dev

# 构建文档
pnpm build
```

## ✨ v0.2.0 重大更新：全新消息参数 API

我们完全重构了消息发送 API，告别了多个 `None` 参数的混乱，引入了结构化参数系统，带来更清洁的开发体验！

### 🚀 **问题解决**

**旧版 API（已弃用）：**
```rust,ignore
// 😱 太多令人困惑的 None 参数！
api.post_message(
    token, "channel_id", Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

**新版 API（推荐）：**
```rust,ignore
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("Hello! 🌍");
api.post_message_with_params(token, "channel_id", params).await?;
```

### 🎯 **新 API 方法（推荐）**

- `post_message_with_params` - 发送频道消息（使用 [`MessageParams`]）
- `post_group_message_with_params` - 发送群消息（使用 [`GroupMessageParams`]）
- `post_c2c_message_with_params` - 发送私聊消息（使用 [`C2CMessageParams`]）
- `post_dms_with_params` - 发送私信（使用 [`DirectMessageParams`]）

### ⚠️ **旧版 API 方法（已弃用）**

- `post_message` → 请使用 `post_message_with_params`
- `post_group_message` → 请使用 `post_group_message_with_params`
- `post_c2c_message` → 请使用 `post_c2c_message_with_params`
- `post_dms` → 请使用 `post_dms_with_params`

### 🌟 **主要优势**

- **✨ 更清洁的代码**：使用 `..Default::default()` 替代多个 `None` 参数
- **📖 更好的可读性**：命名字段而非位置参数
- **🛡️ 类型安全**：结构化参数防止参数顺序错误
- **🔧 构建器模式**：便捷的 `.with_reply()` 和 `.with_file_image()` 方法
- **🚀 易于扩展**：添加新字段而不破坏现有代码
- **🔄 向后兼容**：基于官方 Python botpy API 结构

## 特性

- ✨ **类型安全** - 完全类型化的 API，编译时捕获错误
- 🚀 **高性能** - 基于 Tokio 的异步运行时，支持高并发
- 🔧 **易于使用** - 简单直观的 API 设计，快速上手
- 🛡️ **内存安全** - Rust 的所有权系统保证内存安全
- 🔄 **事件驱动** - 基于事件的架构，响应各种 QQ 频道事件
- 📝 **丰富的文档** - 完整的 API 文档和示例代码
- ⚡ **WebSocket 支持** - 实时接收和处理事件
- 🎯 **Intent 系统** - 精确控制接收的事件类型
- 🏗️ **结构化 API** - 新的参数结构系统，告别多 `None` 参数

## 快速开始

### 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
botrs = "0.2.4"
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

                // 🚀 使用新的参数结构 API
                let params = MessageParams::new_text("Pong! 🏓");
                if let Some(channel_id) = &message.channel_id {
                    if let Err(e) = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
                        info!("Failed to reply: {}", e);
                    }
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
    let mut client = Client::new(token, intents, MyBot, false)?;

    // 启动机器人
    client.start().await?;

    Ok(())
}
```

## 📋 新消息 API 迁移指南

> **📖 文档说明**：本 README 被包含在 `lib.rs` 中作为项目文档，所有代码示例都会参与文档测试。
> - 完整的可运行示例使用 `no_run` 标记（会进行编译检查，但不执行）
> - 代码片段使用 `ignore` 标记（跳过编译检查，便于阅读）
> - 在实际使用时，请参考 [examples/](examples/) 目录中的完整示例

### 简单文本消息
```rust,ignore
use botrs::models::message::MessageParams;

// ✨ 新 API - 简洁明了
let params = MessageParams::new_text("Hello World! 🌍");
api.post_message_with_params(token, "channel_id", params).await?;
```

### 带嵌入内容的消息
```rust,ignore
use botrs::models::message::{MessageParams, Embed};

let embed = Embed {
    title: Some("标题".to_string()),
    description: Some("这是一个嵌入消息示例".to_string()),
    color: Some(0x00ff00),
    ..Default::default()
};

let params = MessageParams {
    content: Some("查看这个嵌入内容！".to_string()),
    embed: Some(embed),
    ..Default::default()
};
api.post_message_with_params(token, "channel_id", params).await?;
```

### 回复消息并附带文件
```rust,ignore
use botrs::models::message::MessageParams;

let file_data = std::fs::read("image.png")?;
let params = MessageParams::new_text("这是你要的文件！")
    .with_file_image(&file_data)
    .with_reply("reply_to_message_id");
api.post_message_with_params(token, "channel_id", params).await?;
```

### 群消息发送
```rust,ignore
use botrs::models::message::GroupMessageParams;

let params = GroupMessageParams::new_text("群里好！")
    .with_reply("reply_to_message_id");
api.post_group_message_with_params(token, "group_openid", params).await?;
```

### 私聊消息发送
```rust,ignore
use botrs::models::message::C2CMessageParams;

let params = C2CMessageParams::new_text("私聊消息");
api.post_c2c_message_with_params(token, "user_openid", params).await?;
```

### 私信发送
```rust,ignore
use botrs::models::message::DirectMessageParams;

let params = DirectMessageParams::new_text("私信内容")
    .with_reply("reply_to_message_id");
api.post_dms_with_params(token, "guild_id", params).await?;
```

更详细和更具体的内容可以在 <https://docs.rs/botrs> 阅读，另有 <https://deepwiki.com/YinMo19/botrs> 作为 AI 文档可以参照阅读代码结构。

## 环境变量配置

你可以使用环境变量来配置机器人凭据：

```bash
export QQ_BOT_APP_ID="your_app_id"
export QQ_BOT_SECRET="your_secret"
```

然后在代码中使用：

```rust,ignore
use botrs::Token;

let token = Token::from_env()?;
```

## 事件处理

BotRS 支持多种事件类型：

### 消息事件

```rust,no_run
use botrs::{Message, DirectMessage, GroupMessage, C2CMessage, Context, EventHandler};
use botrs::models::message::{MessageParams, GroupMessageParams, C2CMessageParams, DirectMessageParams};
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // @ 消息事件
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            info!("Received message: {}", content);

            // 使用新 API 回复
            let params = MessageParams::new_text("收到您的消息了！");
            if let Some(channel_id) = &message.channel_id {
                let _ = ctx.api.post_message_with_params(&ctx.token, channel_id, params).await;
            }
        }
    }
}
```

```rust,no_run
use botrs::{DirectMessage, Context, EventHandler};
use botrs::models::message::DirectMessageParams;
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 私信事件
    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            info!("Received DM: {}", content);

            // 使用新 API 回复私信
            let params = DirectMessageParams::new_text("私信回复！");
            if let Some(guild_id) = &message.guild_id {
                let _ = ctx.api.post_dms_with_params(&ctx.token, guild_id, params).await;
            }
        }
    }
}
```

```rust,no_run
use botrs::{GroupMessage, Context, EventHandler};
use botrs::models::message::GroupMessageParams;
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 群消息事件
    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            info!("Received group message: {}", content);

            // 使用新 API 回复群消息
            let params = GroupMessageParams::new_text("收到您的群消息了！");
            if let Some(group_openid) = &message.group_openid {
                let _ = ctx.api.post_group_message_with_params(&ctx.token, group_openid, params).await;
            }
        }
    }
}
```

```rust,no_run
use botrs::{C2CMessage, Context, EventHandler};
use botrs::models::message::C2CMessageParams;
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // C2C 私聊事件
    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        if let Some(content) = &message.content {
            info!("Received C2C message: {}", content);

            // 使用新 API 回复 C2C 消息
            let params = C2CMessageParams::new_text("C2C 回复！");
            if let Some(author) = &message.author {
                if let Some(user_openid) = &author.user_openid {
                    let _ = ctx.api.post_c2c_message_with_params(&ctx.token, user_openid, params).await;
                }
            }
        }
    }
}
```

### 频道事件

```rust,no_run
use botrs::{Guild, Context, EventHandler};
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 加入频道
    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        if let Some(name) = &guild.name {
            info!("Joined guild: {}", name);
        }
    }

    // 频道更新
    async fn guild_update(&self, _ctx: Context, guild: Guild) {
        if let Some(name) = &guild.name {
            info!("Guild updated: {}", name);
        }
    }

    // 离开频道
    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        if let Some(name) = &guild.name {
            info!("Left guild: {}", name);
        }
    }
}
```

### 成员事件

```rust,no_run
use botrs::{Member, Context, EventHandler};
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 成员加入
    async fn guild_member_add(&self, _ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!("Member joined: {}", user.username);
        }
    }

    // 成员更新
    async fn guild_member_update(&self, _ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!("Member updated: {}", user.username);
        }
    }

    // 成员离开
    async fn guild_member_remove(&self, _ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!("Member left: {}", user.username);
        }
    }
}
```

## Intent 系统

Intent 系统允许你精确控制机器人接收的事件类型：

```rust,ignore
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
if intents.has_privileged() {
    println!("Contains privileged intents");
}
```

### 特权 Intent

某些 Intent 需要特殊权限，可通过 `has_privileged()` 方法检查：

```rust,ignore
use botrs::Intents;

let intents = Intents::none()
    .with_guild_members()   // 特权 intent
    .with_guild_messages(); // 特权 intent

if intents.has_privileged() {
    println!("需要申请特殊权限");
}
```

## API 客户端

BotRS 提供了完整的 API 客户端来与 QQ 频道 API 交互：

```rust,no_run
use botrs::{BotApi, Token};
use botrs::http::HttpClient;
use botrs::models::message::MessageParams;

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

    // 🚀 使用新的消息发送 API
    let params = MessageParams::new_text("Hello from BotRS! 🤖");
    api.post_message_with_params(&token, "channel_id", params).await?;

    Ok(())
}
```

## 错误处理

BotRS 提供了统一的错误处理：

```rust,no_run
use botrs::{BotError, Result, BotApi, Token};
use tracing::{info, error};

async fn handle_api_call(api: &BotApi, token: &Token) -> Result<()> {
    match api.get_bot_info(token).await {
        Ok(info) => {
            info!("Bot: {}", info.username);
        }
        Err(BotError::Api { code, message }) => {
            error!("API error {}: {}", code, message);
        }
        Err(BotError::RateLimit { retry_after }) => {
            error!("Rate limited, retry after {} seconds", retry_after);
        }
        Err(e) => {
            error!("Other error: {}", e);
        }
    }
    Ok(())
}
```

## 配置选项

### HTTP 客户端配置

```rust,ignore
use botrs::http::HttpClient;

// 自定义超时和环境
let http = HttpClient::new(60, true)?; // 60秒超时，沙盒环境
```

### 客户端配置

```rust,ignore
use botrs::{Client, BotApi, Token, Intents, EventHandler};
use botrs::http::HttpClient;

// 标准创建方式
let client = Client::new(token, intents, handler, false)?;

// HTTP 客户端可以通过 HttpClient 进行配置
let http = HttpClient::new(60, true)?; // 60秒超时，沙盒环境
let api = BotApi::new(http);
```

## 运行示例

项目包含多个完整的示例机器人，展示新 API 的使用：

```bash
# 设置环境变量
export QQ_BOT_APP_ID="your_app_id"
export QQ_BOT_SECRET="your_secret"

# 运行基础示例
cargo run --example simple_bot --features examples

# 运行新 API 演示
cargo run --example demo_new_message_api --features examples

# 运行嵌入消息演示
cargo run --example demo_at_reply_embed --features examples

# 运行文件上传演示
cargo run --example demo_at_reply_file_data --features examples

# 运行键盘消息演示
cargo run --example demo_at_reply_keyboard --features examples

# 运行 Markdown 消息演示
cargo run --example demo_at_reply_markdown --features examples

# 运行群消息演示
cargo run --example demo_group_reply_text --features examples

# 运行 C2C 消息演示
cargo run --example demo_c2c_reply_text --features examples

# 运行私信演示
cargo run --example demo_dms_reply --features examples
```

或者传递参数：

```bash
cargo run --example demo_new_message_api --features examples -- your_app_id your_secret
```

## 开发状态

### ✅ 已完成功能

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
- ✅ **新的结构化消息参数 API（v0.2.0）**
- ✅ **完整的消息发送 API 实现**
- ✅ **多种消息类型支持（文本、嵌入、Markdown、键盘、文件）**

### 🔄 计划功能

- 🔄 WebSocket 分片支持
- 🔄 中间件和插件系统
- 🔄 内置命令框架
- 🔄 更多实用示例和教程
- 🔄 性能优化和内存使用优化
- 🔄 更多 QQ 频道 API 功能支持

### ⚠️ 已知问题

目前代码是从 Python 版本重写来的，测试相对较少。作者自己的 bot 并没有申请很多权限，因此关于 Ark、企业级功能等都没有进行充分测试。不过基础的回复、群聊消息等 API 已经经过测试。

从目前的情况来看，Python 版本中也用到了一些不太准确的地方，可能改动了 API。但是 Python 本身的校验机制是很松的，不像这里使用的 serde 库，在 parse 的时候一个字段对不上直接失败。因此如果有一些消息返回失败，或者哪里很明显的 parse 失败了，请在 issue 中告诉我们。如果你能解决，非常欢迎 PR。

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
| API 设计 | 多 None 参数 | 结构化参数 |
| 代码可读性 | 一般 | 优秀 |

## 版本历史

### v0.2.0 (最新)
- 🚀 **重大更新**：引入结构化消息参数 API
- ✨ 新增 `MessageParams`、`GroupMessageParams`、`C2CMessageParams`、`DirectMessageParams`
- 🔧 新增 `post_*_with_params` 系列方法
- 📚 完善示例和文档
- ⚠️ 弃用旧的多参数 API（仍可使用，但推荐迁移）

### v0.1.3
- 🛠️ 基础功能实现
- 🔄 多参数消息发送 API
- 📖 基础文档和示例

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](./LICENSE) 文件。

## 贡献

欢迎贡献代码！我的个人 git commit 提交风格是：

```text
[type] simple message

- detail message 1: detailed description.
- detail message 2: detailed description.
- detail message 3: detailed description.
- detail message 4: detailed description.
- etc.
```

例如：
```text
[feature] add structured message parameters API

- models/message.rs: add MessageParams, GroupMessageParams, C2CMessageParams, DirectMessageParams structs.
- api.rs: add post_*_with_params methods for structured parameter sending.
- examples/: add demo_new_message_api.rs showing the new API usage.
- deprecate old multi-parameter API methods but keep backward compatibility.
```

## 支持

- 📖 [API 文档](https://docs.rs/botrs)
- 🤖 [AI 文档](https://deepwiki.com/YinMo19/botrs)
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
- **更优雅的参数传递方式**

### 类型安全保证
- 编译时类型检查
- Rust 所有权系统保证内存安全
- 详细的错误类型定义
- 可靠的异步处理
- **结构化参数防止运行时错误**

## 相关链接

- [QQ 频道机器人官方文档](https://bot.q.qq.com/wiki/)
- [QQ 频道机器人开发者平台](https://q.qq.com/qqbot/)
- [Python botpy 项目](https://github.com/tencent-connect/botpy)
- [Rust 官方网站](https://www.rust-lang.org/)
- [Tokio 异步运行时](https://tokio.rs/)

---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "BotRS"
  text: "Rust QQ 机器人框架"
  tagline: "聚焦 QQ 频道网关事件与核心 REST 动作的异步 Rust 框架"
  actions:
    - theme: brand
      text: 开始使用
      link: /zh/guide/introduction
    - theme: alt
      text: 在 GitHub 查看
      link: https://github.com/YinMo19/botrs

features:
  - icon: 🛡️
    title: 类型化事件载荷
    details: 网关 dispatch 会解析成消息、频道、子频道、成员、表情回应、互动、音频、管理和论坛等 Rust 模型。

  - icon: ⚡
    title: 异步运行时
    details: 基于 Tokio，包含 WebSocket session、心跳、重连处理和类型化事件分发。

  - icon: 🔧
    title: 小核心 API
    details: Client、EventHandler、Context、BotApi、Token 和 Intents 构成应用使用的主表面。

  - icon: 🎯
    title: 事件驱动设计
    details: 只实现机器人关心的回调即可；每个 EventHandler 方法都有默认空实现。

  - icon: 📝
    title: 结构化消息
    details: 使用 MessageParams、GroupMessageParams、C2CMessageParams 和 DirectMessageParams 发送频道、群、C2C 和私信消息。

  - icon: 🔄
    title: Intent 系统
    details: 通过链式 Intents helper 选择机器人需要接收的网关事件类别。

  - icon: 🌐
    title: Client 管理网关
    details: Client 发现网关、启动 shard session、发送心跳，并把 dispatch 转发给处理器。

  - icon: 📚
    title: 核心 REST 动作
    details: BotApi 覆盖 bot 信息、网关发现、消息、撤回、媒体上传、公告、日程、权限、表情回应和精华消息。
---

## 什么是 BotRS？

BotRS 是围绕 QQ 网关和核心 REST 能力构建的异步 Rust QQ 频道机器人框架。它提供多数机器人在实时事件路径中需要的组件：网关连接管理、类型化事件载荷、共享 REST 客户端、token 处理和 intent 选择。

核心类型包括：

- `Client` 负责启动、网关 session 和事件分发。
- `EventHandler` 是你为网关事件实现的 trait。
- `Context` 为每个回调提供共享 `BotApi` 和 bot 信息。
- `BotApi` 负责发送消息，以及 examples 中使用的核心 REST 动作。
- `Token` 保存凭据，并支持从环境变量加载。
- `Intents` 控制 QQ 会发送哪些网关类别。

## 当前能力

BotRS 处理核心的“事件到动作”链路：

- 接收 guild/channel/member 变化、频道消息、私信、群与 C2C 消息、表情回应、互动、审核、管理、音频和论坛等类型化网关事件。
- 纯文本回复可用 `message.reply(&ctx, "text")`，更复杂的回复可构造对应 params。
- 通过 `BotApi` 发送频道、群、C2C 和私信消息。
- 上传群/C2C 媒体，再作为 media 消息发送。
- 使用公告、日程、API 权限申请、表情回应和精华消息相关 API。

## 快速示例

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人已就绪：{}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.content.as_deref() == Some("!ping") {
            let _ = message.reply(&ctx, "pong").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::from_env()?;
    let intents = Intents::new().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyBot, true)?;

    client.start().await?;
    Ok(())
}
```

## 开始使用

1. **[安装](/zh/guide/installation)** - 将 BotRS 加入 Rust 项目。
2. **[快速开始](/zh/guide/quick-start)** - 运行最小机器人。
3. **[客户端与事件处理器](/zh/guide/client-handler)** - 了解事件循环。
4. **[消息](/zh/guide/messages)** - 发送文本、媒体、Markdown、Ark、embed 和 keyboard payload。
5. **[API 客户端](/zh/guide/api-client)** - 使用 `BotApi` 与 `Context`。

## 链接

- **[GitHub 仓库](https://github.com/YinMo19/botrs)** - 源码和问题。
- **[文档](https://docs.rs/botrs)** - docs.rs API 参考。
- **[示例](/zh/examples/getting-started)** - 端到端使用示例。
- **[更新日志](/zh/changelog)** - 当前版本摘要。

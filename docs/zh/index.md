---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "BotRS"
  text: "Rust QQ 机器人框架"
  tagline: "基于 QQ 频道机器人 API 的类型安全、高性能、易于使用的 Rust 框架"
  # image:
  #   src: /logo.svg
  #   alt: BotRS
  actions:
    - theme: brand
      text: 开始使用
      link: /zh/guide/introduction
    - theme: alt
      text: 在 GitHub 查看
      link: https://github.com/YinMo19/botrs

features:
  - icon: 🛡️
    title: 类型安全
    details: 完全类型化的 API，编译时捕获错误。Rust 的所有权系统确保内存安全，防止常见的编程错误。

  - icon: ⚡
    title: 高性能
    details: 基于 Tokio 异步运行时，支持高并发。高效的 WebSocket 处理和带连接池的 HTTP 客户端。

  - icon: 🔧
    title: 易于使用
    details: 直观的 API 设计配有清晰的文档。用最少的样板代码即可在几分钟内启动机器人。

  - icon: 🎯
    title: 事件驱动架构
    details: 通过清洁的事件处理器系统响应各种 QQ 频道事件。支持消息、频道、成员等多种事件。

  - icon: 📝
    title: 丰富的消息支持
    details: 发送文本、嵌入内容、文件、Markdown、键盘和交互式消息。完全支持所有 QQ 频道消息类型。

  - icon: 🔄
    title: Intent 系统
    details: 对事件订阅进行细粒度控制。通过只接收机器人需要的事件来优化性能。

  - icon: 🌐
    title: WebSocket 网关
    details: 实时事件处理，自动重连和心跳处理。可靠的连接管理。

  - icon: 📚
    title: 全面的 API
    details: 完整覆盖 QQ 频道机器人 API，使用结构化参数系统。告别令人困惑的多个 None 参数。

  - icon: 🏗️
    title: 结构化参数
    details: 清洁、可读的消息 API，采用构建器模式。类型安全的参数构造和默认值。
---

## 什么是 BotRS？

BotRS 是为 Rust 编程语言设计的异步框架，专门用于构建 QQ 频道机器人。它提供了创建交互式机器人应用程序所需的基本构建块，这些应用程序可以处理消息、管理频道，并实时响应各种事件。

从高层次来看，BotRS 提供了几个主要组件：

- **异步运行时集成**：基于 Tokio 构建，可处理数千个并发连接
- **类型安全的 API 绑定**：为所有 QQ 频道机器人 API 端点提供完整的 Rust 类型定义
- **事件驱动架构**：用于响应频道事件的清洁事件处理器系统
- **丰富的消息支持**：发送文本、嵌入内容、文件和交互式内容
- **WebSocket 网关**：实时事件处理，自动连接管理

## BotRS 在您项目中的作用

在构建 QQ 频道机器人时，您需要一个能够处理实时消息传递、API 交互和事件处理复杂性的框架。BotRS 作为基础，让您专注于机器人的逻辑，而不是底层基础设施。

该框架处理：

- **连接管理**：自动 WebSocket 重连和心跳处理
- **速率限制**：内置请求节流以遵守 API 限制
- **类型安全**：编译时保证防止运行时错误
- **异步处理**：非阻塞事件处理以获得最大性能
- **错误处理**：带有上下文和恢复选项的全面错误类型

## 快速示例

这是一个响应消息的简单机器人：

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人已就绪！登录为：{}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("你的应用ID", "你的密钥");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyBot, true)?;

    client.start().await?;
    Ok(())
}
```

## 开始使用

准备用 BotRS 构建您的第一个 QQ 频道机器人了吗？请遵循我们的全面指南：

1. **[安装](/zh/guide/installation)** - 将 BotRS 添加到您的 Rust 项目
2. **[快速开始](/zh/guide/quick-start)** - 在几分钟内创建您的第一个机器人
3. **[配置](/zh/guide/configuration)** - 设置您的机器人凭据和选项
4. **[示例](/zh/examples/getting-started)** - 探索工作代码示例

## 架构亮点

### 与 Python botpy 的兼容性

BotRS 保持与官方 Python botpy 库的 API 兼容性，使熟悉 Python 生态系统的开发者能够直接迁移。结构化参数系统镜像了 botpy 的方法，同时添加了 Rust 的类型安全优势。

### 性能特性

- **内存高效**：尽可能零拷贝反序列化
- **并发处理**：同时处理多个事件
- **连接池**：重用 HTTP 连接进行 API 调用
- **最小分配**：为高吞吐量场景进行细心的内存管理

### 类型安全保证

Rust 的类型系统防止动态语言中常见的整类错误：

- **编译时验证**：在部署前捕获 API 误用
- **无空指针异常**：Option 类型使空值处理显式化
- **内存安全**：无释放后使用或缓冲区溢出漏洞
- **线程安全**：编译时验证并发访问模式

## 社区和支持

- **[GitHub 仓库](https://github.com/YinMo19/botrs)** - 源代码、问题和讨论
- **[文档](https://docs.rs/botrs)** - docs.rs 上的完整 API 参考
- **[示例](/zh/examples/getting-started)** - 常见用例的工作代码示例
- **[更新日志](/zh/changelog)** - 版本历史和破坏性更改

---

*BotRS 是在 MIT 许可证下发布的开源软件。欢迎贡献！*

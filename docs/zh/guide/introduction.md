# 介绍

BotRS 是一个用于在 Rust 中构建 QQ 频道机器人的异步运行时框架。它提供了创建健壮、高性能机器人应用程序所需的基本构建块，这些应用程序可以处理实时消息、频道管理和事件处理。

## 什么是 BotRS？

BotRS 围绕类型安全、性能和易用性的原则设计。它作为 QQ 频道机器人 API 的全面包装器，提供：

- **类型安全的 API 绑定**：为所有 API 端点提供完整的 Rust 类型定义
- **异步运行时集成**：基于 Tokio 构建，处理并发操作
- **事件驱动架构**：用于响应频道事件的清洁抽象
- **丰富的消息支持**：发送文本、嵌入内容、文件和交互式内容
- **WebSocket 网关**：实时事件处理，自动连接管理

## 核心架构

BotRS 的核心由几个关键组件组成：

### 客户端
`Client` 是机器人应用程序的主要入口点。它管理到 QQ 服务器的 WebSocket 连接，处理身份验证，并将事件分派给您的事件处理器。

### 事件处理器
`EventHandler` trait 定义了您的机器人如何响应各种事件，如消息、成员加入、频道更新等。您实现此 trait 来定义机器人的行为。

### API 客户端
`BotApi` 提供对 QQ 频道 REST API 端点的直接访问，允许您发送消息、管理频道、处理权限和执行其他管理任务。

### 网关
WebSocket 网关管理到 QQ 服务器的实时连接，自动处理心跳、重连逻辑和事件分派。

## 主要特性

### 类型安全
Rust 的类型系统防止动态类型语言中常见的整类运行时错误：

```rust
// 消息参数的编译时验证
let params = MessageParams::new_text("你好，世界！")
    .with_reply(message_id)
    .with_markdown(true);

// 类型安全的事件处理
async fn message_create(&self, ctx: Context, message: Message) {
    // message.content 是 Option<String> - 显式的空值处理
    if let Some(content) = &message.content {
        // 安全地处理消息内容
    }
}
```

### 高性能
基于 Tokio 的异步运行时构建，BotRS 可以处理数千个并发操作：

- **非阻塞 I/O**：所有网络操作都是异步的
- **连接池**：HTTP 客户端高效地重用连接
- **内存效率**：尽可能零拷贝反序列化
- **并发事件处理**：同时处理多个事件

### 结构化参数
BotRS v0.2.0 引入了新的结构化参数系统，消除了多个 `None` 参数的困惑：

```rust
// 旧 API（已弃用）
api.post_message(
    token, "channel_id", Some("你好！"),
    None, None, None, None, None, None, None, None, None
).await?;

// 新 API（推荐）
let params = MessageParams::new_text("你好！")
    .with_reply("message_id")
    .with_embed(embed);
api.post_message_with_params(token, "channel_id", params).await?;
```

## 与其他解决方案的比较

### vs Python botpy
BotRS 保持与官方 Python botpy 库的 API 兼容性，同时添加：

- **编译时安全**：在部署前捕获错误
- **更好的性能**：原生代码执行和高效的内存使用
- **结构化并发**：内置的 async/await 支持
- **零成本抽象**：高级 API 具有最小的运行时开销

### vs 其他 Rust Discord 库
虽然有优秀的 Discord Rust 库，但 BotRS 专门为 QQ 频道的独特 API 和功能设计：

- **QQ 频道专用**：原生支持 QQ 的消息类型和功能
- **官方 API 覆盖**：完整实现 QQ 频道机器人 API
- **中文生态系统**：考虑中文开发者和用例构建
- **积极维护**：定期更新跟随 QQ 的 API 变化

## 开始使用

准备构建您的第一个机器人？以下是您需要的：

1. **Rust 安装**：BotRS 需要 Rust 1.70 或更高版本
2. **QQ 频道机器人凭据**：从 QQ 频道开发者门户获取应用 ID 和密钥
3. **基础异步知识**：熟悉 Rust 的 async/await 语法

最快的入门方式是我们的[快速开始指南](/zh/guide/quick-start)，它将让您在 5 分钟内运行一个基本机器人。

## 社区和生态系统

BotRS 是构建聊天机器人和自动化工具的 Rust 工具不断增长的生态系统的一部分：

- **积极开发**：定期更新和新功能
- **社区驱动**：开源，欢迎贡献
- **生产就绪**：被多个组织在生产中使用
- **全面文档**：详细的指南和 API 参考

## 设计理念

BotRS 遵循几个关键设计原则：

### 人体工程学优先
API 应该直观易用，即使对于 Rust 或机器人开发新手也是如此。

### 安全不牺牲
类型安全和内存安全不应以性能或表达能力为代价。

### 默认异步
所有 I/O 操作都是异步的，以最大化吞吐量和响应性。

### 向后兼容性
API 更改遵循语义版本控制，为破坏性更改提供清晰的迁移路径。

## 下一步

- **[安装](/zh/guide/installation)** - 将 BotRS 添加到您的项目
- **[快速开始](/zh/guide/quick-start)** - 构建您的第一个机器人
- **[配置](/zh/guide/configuration)** - 设置凭据和选项
- **[示例](/zh/examples/getting-started)** - 探索工作代码示例

构建强大 QQ 频道机器人的旅程从这里开始。让我们开始构建吧！
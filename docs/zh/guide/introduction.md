# 介绍

BotRS 是一个用 Rust 构建 QQ 频道机器人的异步框架。它将 QQ 频道 Bot API 与网关 WebSocket 协议封装为少量类型：`Client`、`EventHandler`、`Context`、`BotApi`、`Token` 和 `Intents`。使用本框架时，几乎所有工作都从这些类型出发。

## 架构概览

`Client` 拥有网关连接和 HTTP 客户端。构造它需要 `Token`、`Intents` 位集、`EventHandler` 实现，以及一个布尔值用来选择沙箱或正式环境基础地址。调用 `client.start().await` 之后，网关会把事件分派到你的处理器。

`EventHandler` 是一个 trait，每个事件对应一个带默认实现的 `async fn`（例如 `message_create`、`at_message_create`、`guild_create`、`forum_thread_create` 等）。只实现关心的事件即可。

每次回调都会传入一个 `Context`，其中持有 `Arc<BotApi>` 和启动客户端时的 `Token`。`BotApi` 是带类型签名的 HTTP 层，如果需要在事件处理之外调用 REST API，也可以自行构造一个。

`Intents` 是一个位标志集合，告诉网关需要投递哪些事件类别，按需订阅可以减少无用流量。

## 消息发送

所有发送方法都接受类型化的 `*Params` 构建器，而非一长串 `Option` 参数：

```rust
let params = MessageParams::new_text("你好")
    .with_reply(message_id)
    .with_markdown(true);
ctx.send_message("channel_id", params).await?;
```

`GroupMessageParams`、`C2CMessageParams`、`DirectMessageParams` 等的形态与之相同，完整集合见消息指南。

## 后续阅读

- [安装](/zh/guide/installation) —— 将 `botrs` 加入 `Cargo.toml`。
- [快速开始](/zh/guide/quick-start) —— 一个最小可运行机器人。
- [客户端与事件处理器](/zh/guide/client-handler) —— 事件循环 API。
- [消息](/zh/guide/messages) —— `*MessageParams` 构建器。
- [API 客户端](/zh/guide/api-client) —— 使用 `BotApi` 与 `Context`。

# Context

`Context` 是事件回调里的运行上下文。它的作用很简单：把当前事件处理逻辑和共享的 `BotApi` 连接起来，并携带启动时拿到的 bot 信息。

你不需要手动构造 `Context`。`Client` 在网关事件循环里创建它，然后传给每个 `EventHandler` 方法。

## 直接调用 API

`Context` 会解引用到 `BotApi`，所以 handler 里可以直接写：

```rust
let params = MessageParams::new_text("pong");
ctx.send_message(&channel_id, params).await?;
```

这和 `ctx.api().send_message(...)` 是同一个 API 客户端。需要把 API 客户端传给辅助函数时，可以使用 `ctx.api()` 拿到 `&BotApi`。

## bot_info

`ctx.bot_info` 来自启动阶段的 `get_bot_info`。正常由 `Client::start` 驱动的事件里它会有值，但类型上仍是 `Option<BotInfo>`，因为测试或内部构造场景可能没有填充。

常见用途是日志、判断当前 bot 名称、或生成响应文本：

```rust
let bot_name = ctx
    .bot_info
    .as_ref()
    .map(|bot| bot.username.as_str())
    .unwrap_or("bot");
```

## 并发使用

`Context` 克隆成本很低，内部共享同一个 API 客户端和 token 缓存。需要把回复动作放到后台任务时，可以 clone 一份：

```rust
let background = ctx.clone();
tokio::spawn(async move {
    let params = MessageParams::new_text("background work done");
    let _ = background.send_message(&channel_id, params).await;
});
```

## 参见

- [BotApi](./bot-api.md)
- [EventHandler](./event-handler.md)
- [Client](./client.md)

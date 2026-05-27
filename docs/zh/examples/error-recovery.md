# 错误恢复

`examples/` 下没有专门讲错误恢复的 demo——你真正需要的恢复逻辑（网关重连、会话恢复、心跳）由 `Client::start` 内部自动处理。剩下的只是检查每次 `BotApi` 调用的 `Result`，决定记日志、忽略，还是重试。

## 框架已经做了什么

- WebSocket 重连（指数退避）
- 临时断开后的会话恢复
- 心跳保活
- `Token::validate()` 校验 token

如果想观察 handler 级错误，实现 `EventHandler::error(&self, error: BotError)`（默认实现只打日志）。`examples/` 中的每个 demo 都是一行 `warn!("…: {}", e)`。

## 单次调用

`BotApi` 方法返回 `botrs::Result<T>`。失败分支是 `botrs::BotError`，本身已经能区分瞬时错误（网络、限流）和永久错误（鉴权、报文非法）。需要时就 match，否则打个日志继续。

```rust
match message.reply(&ctx, &reply).await {
    Ok(_) => {},
    Err(e) => tracing::warn!("reply failed: {e}"),
}
```

## 参见

- 指南：[`docs/zh/guide/error-handling.md`](../guide/error-handling.md) ——`BotError` 变体与处理模式的权威参考
- `BotError` 定义：`src/error.rs`

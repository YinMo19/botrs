# 回声机器人

最朴素的「@我，我回你」模式实现在 [`examples/demo_at_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply.rs)。它注册 `Intents::new().with_public_guild_messages()`、实现 `EventHandler::message_create`、用 `Message::reply` 把字符串发回。这就是 BotRS 中回声机器人的全部表面。

## 唯一需要记住的调用

`message.reply(&ctx, &reply_content)` 会发送一条自动关联到原始消息的回复。简单场景下不需要 builder。

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let Some(content) = &message.content else { return };
    let reply = format!("echo: {content}");
    if let Err(e) = message.reply(&ctx, &reply).await {
        tracing::warn!("reply failed: {e}");
    }
}
```

如果想把原消息作为引用块而不是普通 @，用 `MessageParams` 加 `message_reference`——参见[文本消息](./text-messages.md) 与 [`demo_at_reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_reference.rs)。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Demo：[`examples/demo_at_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply.rs)
- 运行：`cargo run --example demo_at_reply --features examples`

# Echo Bot

The plain "@-me, I reply" pattern is implemented in [`examples/guild/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_text.rs). It registers `Intents::new().with_public_guild_messages()`, implements `EventHandler::message_create`, and uses `Message::reply` to send a string back. That is the entire surface of a guild-channel echo bot in BotRS.

## The one call you need

`message.reply(&ctx, &reply_content)` posts a reply that is automatically threaded to the inbound message. There is no builder for the simple case.

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let Some(content) = &message.content else { return };
    let reply = format!("echo: {content}");
    if let Err(e) = message.reply(&ctx, &reply).await {
        tracing::warn!("reply failed: {e}");
    }
}
```

If you want the reply quoted as a reference instead of a plain mention, use `MessageParams` with `message_reference`; see [Text Messages](./text-messages.md) and [`examples/guild/reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_reference.rs).

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Example: [`examples/guild/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_text.rs)
- Run with `cargo run --example guild_reply_text --features examples`

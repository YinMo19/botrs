# Echo Bot

The plain "@-me, I reply" pattern is implemented in [`examples/demo_at_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply.rs). It registers `Intents::default().with_public_guild_messages()`, implements `EventHandler::message_create`, and uses `Message::reply` to send a string back. That is the entire surface of an echo bot in BotRS.

## The one call you need

`Message::reply(&ctx.api, &ctx.token, &reply_content)` posts a reply that is automatically threaded to the inbound message. There is no builder for the simple case — pass three arguments and you are done.

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let Some(content) = &message.content else { return };
    let reply = format!("echo: {content}");
    if let Err(e) = message.reply(&ctx.api, &ctx.token, &reply).await {
        tracing::warn!("reply failed: {e}");
    }
}
```

If you want the reply quoted as a reference instead of a plain mention, use `MessageParams` with `message_reference` — see [Text Messages](./text-messages.md) and [`demo_at_reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_reference.rs).

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Demo: [`examples/demo_at_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply.rs)
- Run with `cargo run --example demo_at_reply --features examples`

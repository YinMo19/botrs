# Context

`Context` is the runtime context passed to event callbacks. It connects the current handler logic to the shared `BotApi` and carries the bot information fetched during startup.

You do not construct `Context` yourself. `Client` creates it inside the gateway event loop and passes it to every `EventHandler` method.

## Calling the API

`Context` dereferences to `BotApi`, so handlers can call API methods directly:

```rust
let params = MessageParams::new_text("pong");
ctx.send_message(&channel_id, params).await?;
```

This uses the same client as `ctx.api().send_message(...)`. If a helper function needs an explicit API reference, call `ctx.api()` to get `&BotApi`.

## bot_info

`ctx.bot_info` comes from startup-time `get_bot_info`. It is normally present for events driven by `Client::start`, but its type is `Option<BotInfo>` because tests and internal construction paths may leave it empty.

A common use is logging or generating response text:

```rust
let bot_name = ctx
    .bot_info
    .as_ref()
    .map(|bot| bot.username.as_str())
    .unwrap_or("bot");
```

## Concurrency

`Context` is cheap to clone. Internally it shares the same API client and token cache. Clone it when you need to move reply work into a background task:

```rust
let background = ctx.clone();
tokio::spawn(async move {
    let params = MessageParams::new_text("background work done");
    let _ = background.send_message(&channel_id, params).await;
});
```

## See Also

- [BotApi](./bot-api.md)
- [EventHandler](./event-handler.md)
- [Client](./client.md)

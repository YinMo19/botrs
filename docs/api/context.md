# Context

`Context` is the request-scoped object passed to every `EventHandler` callback. It holds the shared API client and cached `BotInfo`; the authentication token lives on `BotApi`.

```rust
pub struct Context {
    api: Arc<BotApi>,
    pub bot_info: Option<BotInfo>,
}
```

`Context` is `Clone`-cheap and implements `Deref<Target = BotApi>`, so handler code can call `BotApi` methods directly on `ctx`.

## Construction

You do not build `Context` yourself. `Client` constructs it from the same `BotApi` it uses to drive the gateway and fills `bot_info` after the gateway `READY` data is available.

## What it gives you

Because `Context` dereferences to `BotApi`, every REST method listed in [Bot API](./bot-api.md) is available directly:

```rust
ctx.send_message(channel_id, MessageParams::new_text("pong")).await?;
ctx.send_group_message(group_openid, GroupMessageParams::new_text("pong")).await?;
ctx.recall_message(channel_id, message_id, Some(true)).await?;
```

For helper APIs that need a `BotApi` reference, use `&ctx`. For explicit borrowing, `ctx.api()` returns `&BotApi`.

## Example: replying with embed

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.is_from_bot() { return; }

    let params = MessageParams {
        embed: Some(embed),
        ..Default::default()
    };

    if let Err(err) = ctx
        .send_message(message.channel_id.as_deref().unwrap_or(""), params)
        .await
    {
        warn!("send failed: {err}");
    }
}
```

## Example: deleting a member

```rust
ctx.delete_member(&guild_id, &user_id, Some(true), Some(1)).await?;
```

`delete_member` accepts `add_blacklist: Option<bool>` and `delete_history_msg_days: Option<i32>`; use `None` for platform defaults.

## Concurrency

Because `Context` is cheap to clone and the inner `Arc<BotApi>` is shared, fanning work out to spawned tasks is trivial:

```rust
let context = ctx.clone();
tokio::spawn(async move {
    let params = MessageParams::new_text("background work done");
    let _ = context.send_message(&channel, params).await;
});
```

The token inside `BotApi` refreshes through its shared cache; you do not need to pass tokens between tasks.

## See also

- [Client](./client.md) — owner of the `Context` instances passed to handlers.
- [BotApi](./bot-api.md) — full route catalogue.
- [Event handler](./event-handler.md) — the trait whose methods receive `Context`.
- [Token](./token.md) — credential model embedded in `BotApi`.

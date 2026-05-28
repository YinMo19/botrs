# Sessions

Event callbacks are session-first. `Context` is now an internal runtime detail; application handlers receive a session object instead.

Reply-capable message events use specialized sessions:

- `ChannelReplySession`
- `DirectReplySession`
- `GroupReplySession`
- `C2CReplySession`

Other events use `EventSession<T>` aliases such as `ReadySession`, `MessageDeleteSession`, `MemberSession`, `ReactionSession`, and `OpenForumSession`.

## Reading Event Data

Use the session accessor that matches the callback:

```rust
async fn ready(&self, session: ReadySession) {
    tracing::info!("ready as {}", session.event().user.username);
}

async fn message_create(&self, mut session: ChannelReplySession) {
    let message = session.message().clone();
    if message.content.as_deref() == Some("!ping") {
        let _ = session.reply("pong").await;
    }
}
```

For generic event sessions, `session.event()` returns the typed gateway payload.

## Calling The API

Every session exposes the shared REST client:

```rust
let api = session.api();
```

Sessions also dereference to `BotApi`, so existing REST methods remain available:

```rust
let pins = session.get_pins(channel_id).await?;
```

When work must outlive the callback, move an owned API handle into the task:

```rust
let api = session.api_handle();
tokio::spawn(async move {
    let _ = api.get_bot_info().await;
});
```

## bot_info

`session.bot_info()` returns the bot information fetched during startup when it is available.

```rust
let bot_name = session
    .bot_info()
    .map(|bot| bot.username.as_str())
    .unwrap_or("bot");
```

## See Also

- [BotApi](./bot-api.md)
- [EventHandler](./event-handler.md)
- [Client](./client.md)

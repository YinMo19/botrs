# Introduction

BotRS is an asynchronous framework for building QQ bots in Rust. It wraps QQ gateway events and the bot OpenAPI behind a small set of types: `Client`, `EventHandler`, session types, `BotApi`, `Token`, and `Intents`. Almost everything you do with the framework starts from one of those.

## Architecture at a glance

The `Client` owns the gateway connection and the HTTP client. You construct it with a `Token`, an `Intents` bitset, an implementation of `EventHandler`, and a `bool` that picks the sandbox or production base URL. After `client.start().await`, the gateway dispatches events into your handler.

The `EventHandler` trait is a single trait with a default `async fn` per event (`message_create`, `direct_message_create`, `guild_create`, `forum_thread_create`, and so on). You implement only the ones you care about.

Each handler call receives a session object. Reply sessions provide ergonomic `reply`, `send_*_message`, and lower-level `send_message` methods; event sessions expose the typed payload through `session.event()`. `BotApi` is the typed HTTP layer and owns the token used for REST calls; you can also construct one yourself if you need REST access outside of an event handler.

`Intents` is a bitflag set that tells the gateway which event categories to deliver. Subscribing only to what you need keeps payload volume down.

## Message sending

Reply sessions cover the common cases directly:

```rust
session.reply("hello").await?;
session.send_markdown_message("# hello").await?;
```

For standalone `BotApi` calls or custom fields, use typed params constructors such as `MessageParams::new_text`, `GroupMessageParams::new_media`, `C2CMessageParams::new_embed`, and `DirectMessageParams::new_markdown`.

## Where to go next

- [Installation](/guide/installation) — adding `botrs` to your `Cargo.toml`.
- [Quick start](/guide/quick-start) — a minimal working bot.
- [Client and event handler](/guide/client-handler) — the event-loop API.
- [Messages](/guide/messages) — the `*MessageParams` builders.
- [API client](/guide/api-client) — using `BotApi` through sessions or standalone.

# Introduction

BotRS is an asynchronous framework for building QQ Guild bots in Rust. It wraps the QQ Guild Bot API and the gateway WebSocket protocol behind a small set of types: `Client`, `EventHandler`, `Context`, `BotApi`, `Token`, and `Intents`. Almost everything you do with the framework starts from one of those.

## Architecture at a glance

The `Client` owns the gateway connection and the HTTP client. You construct it with a `Token`, an `Intents` bitset, an implementation of `EventHandler`, and a `bool` that picks the sandbox or production base URL. After `client.start().await`, the gateway dispatches events into your handler.

The `EventHandler` trait is a single trait with a default `async fn` per event (`message_create`, `direct_message_create`, `guild_create`, `forum_thread_create`, and so on). You implement only the ones you care about.

Each handler call receives a `Context`, which holds the shared `BotApi`. `BotApi` is the typed HTTP layer and owns the token used for REST calls; you can also construct one yourself if you need REST access outside of an event handler.

`Intents` is a bitflag set that tells the gateway which event categories to deliver. Subscribing only to what you need keeps payload volume down.

## Message sending

Every send method takes a typed `*Params` builder rather than a long list of `Option` arguments:

```rust
let mut params = MessageParams::new_text("hello")
    .with_reply(message_id);
params.markdown = Some(markdown);
ctx.send_message("channel_id", params).await?;
```

The same shape applies to `GroupMessageParams`, `C2CMessageParams`, `DirectMessageParams`, etc. See the messages guide for the full set.

## Where to go next

- [Installation](/guide/installation) — adding `botrs` to your `Cargo.toml`.
- [Quick start](/guide/quick-start) — a minimal working bot.
- [Client and event handler](/guide/client-handler) — the event-loop API.
- [Messages](/guide/messages) — the `*MessageParams` builders.
- [API client](/guide/api-client) — using `BotApi` and `Context`.

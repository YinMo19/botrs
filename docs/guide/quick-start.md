# Quick Start

A minimal bot is one struct that implements `EventHandler`, plus a `Client::new(token, intents, handler, is_sandbox)` call followed by `client.start().await`.

## Cargo.toml

```toml
[dependencies]
botrs = "0.13.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## main.rs

The handler below replies to `!ping` when the bot is mentioned in a guild channel. `Token::new(app_id, secret)` and the `is_sandbox` flag on `Client::new` are the only credentials and environment knobs you need at this stage.

```rust
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, session: ReadySession) {
        tracing::info!("ready as {}", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        if message
            .author
            .as_ref()
            .and_then(|author| author.bot)
            .unwrap_or_default()
        {
            return;
        }
        let Some(content) = message.content.as_deref() else { return };
        if content.trim() == "!ping" {
            let _ = session.reply("pong").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("botrs=info").init();

    let app_id = std::env::var("QQ_BOT_APP_ID")?;
    let secret = std::env::var("QQ_BOT_SECRET")?;
    let token = Token::new(app_id, secret);

    let intents = Intents::new()
        .with_public_guild_messages()
        .with_guilds();

    let mut client = Client::new(token, intents, MyBot, true)?;
    client.start().await?;
    Ok(())
}
```

## What the pieces do

- `Token` carries the App ID and Secret. `Token::from_env()` is also available; it reads `QQ_BOT_APP_ID` and `QQ_BOT_SECRET`.
- `Intents` is a bitflag set. `Intents::new()` starts empty, and each `with_*` method enables one event category. `Intents::default()` is the standard public preset: it enables public, non-privileged event categories while leaving privileged events and `ENTER_AIO` opt-in.
- `Client::new(token, intents, handler, is_sandbox)` — pass `true` for the sandbox base URL while developing, `false` for production.
- `session.reply(text)` is the convenience for replying to the current event; for richer payloads use `session.send_message` with `MessageParams` (see [Messages](/guide/messages)).

## Next

- [Client and event handler](/guide/client-handler) — every event the framework dispatches.
- [Messages](/guide/messages) — embed, file, markdown, ARK, keyboard payloads.
- [Configuration](/guide/configuration) — env vars and sandbox vs production.

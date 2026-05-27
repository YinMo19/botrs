# Quick Start

A minimal bot is one struct that implements `EventHandler`, plus a `Client::new(token, intents, handler, is_sandbox)` call followed by `client.start().await`.

## Cargo.toml

```toml
[dependencies]
botrs = "0.11.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## main.rs

The handler below replies to `!ping` when the bot is mentioned in a guild channel. `Token::new(app_id, secret)` and the `is_sandbox` flag on `Client::new` are the only credentials and environment knobs you need at this stage.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("ready as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }
        let Some(content) = message.content.as_deref() else { return };
        if content.trim() == "!ping" {
            let _ = message.reply(&ctx, "pong").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("botrs=info").init();

    let app_id = std::env::var("QQ_BOT_APP_ID")?;
    let secret = std::env::var("QQ_BOT_SECRET")?;
    let token = Token::new(app_id, secret);

    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds();

    let mut client = Client::new(token, intents, MyBot, true)?;
    client.start().await?;
    Ok(())
}
```

## What the pieces do

- `Token` carries the App ID and Secret. `Token::from_env()` is also available; it reads `QQ_BOT_APP_ID` and `QQ_BOT_SECRET`.
- `Intents` is a bitflag set. `Intents::default()` is empty; chain `with_*` methods for the categories you need. The bot receives only events you opted into.
- `Client::new(token, intents, handler, is_sandbox)` — pass `true` for the sandbox base URL while developing, `false` for production.
- `Message::reply(&api, text)` is the convenience for replying to the same channel; for richer payloads use `BotApi::send_message` (see [Messages](/guide/messages)).

## Next

- [Client and event handler](/guide/client-handler) — every event the framework dispatches.
- [Messages](/guide/messages) — embed, file, markdown, ARK, keyboard payloads.
- [Configuration](/guide/configuration) — env vars and sandbox vs production.

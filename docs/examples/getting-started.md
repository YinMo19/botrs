# Getting Started

The smallest end-to-end bot lives in [`examples/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/simple_bot.rs). It wires up a `Token`, builds an `Intents` set, implements `EventHandler::message_create` / `group_message_create`, and starts the `Client`. Read it once — every other demo follows the same skeleton.

## The shape of a bot

Three things make a BotRS process: a `Token`, an `Intents` flagset describing which gateway events you care about, and an `EventHandler` impl. `Client::new(token, intents, handler, true)` glues them together and `client.start().await` runs the loop.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("ready as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
        let _ = message.reply(&ctx, "pong").await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new(std::env::var("QQ_BOT_APP_ID")?, std::env::var("QQ_BOT_SECRET")?);
    let intents = Intents::default().with_public_guild_messages().with_direct_message();
    Client::new(token, intents, Handler, true)?.start().await?;
    Ok(())
}
```

## See also

- Guide: [`docs/guide/quick-start.md`](../guide/quick-start.md), [`docs/guide/intents.md`](../guide/intents.md)
- Working source: [`examples/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/simple_bot.rs)
- Run with `cargo run --example simple_bot --features examples -- $APP_ID $SECRET`

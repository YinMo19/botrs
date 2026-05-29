# Getting Started

The smallest end-to-end bot lives in [`examples/basic/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/basic/simple_bot.rs). It wires up a `Token`, builds an `Intents` set, implements message callbacks, and starts the `Client`. Read it once; the scene-specific examples in `examples/guild`, `examples/group`, `examples/c2c`, and `examples/direct` follow the same runtime shape.

## The shape of a bot

Three things make a BotRS process: a `Token`, an `Intents` flagset describing which gateway events you care about, and an `EventHandler` impl. `Client::new(token, intents, handler, true)` glues them together and `client.start().await` runs the loop.

```rust
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};

struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, session: ReadySession) {
        println!("ready as {}", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        if message.author.bot {
            return;
        }

        let _ = session.reply("pong").await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new(std::env::var("QQ_BOT_APP_ID")?, std::env::var("QQ_BOT_SECRET")?);
    let intents = Intents::new().with_public_guild_messages();
    Client::new(token, intents, Handler, true)?.start().await?;
    Ok(())
}
```

## See also

- Guide: [`docs/guide/quick-start.md`](../guide/quick-start.md), [`docs/guide/intents.md`](../guide/intents.md)
- Working source: [`examples/basic/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/basic/simple_bot.rs)
- Run with `cargo run --example simple_bot --features examples -- $APP_ID $SECRET`

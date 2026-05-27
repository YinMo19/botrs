# Migration from a Python QQ bot SDK

This page maps the most common Python QQ-bot SDK patterns to their `botrs` equivalents. The framework's surface area is intentionally similar — the same gateway, the same intents, the same message types — so most of the work is mechanical.

## Handler class → `EventHandler` trait

In a Python SDK you typically subclass a `Client` (or register handlers via decorators) and override `async def on_at_message_create(self, msg)` and friends.

In `botrs` you implement the `EventHandler` trait on a struct of your own. Each event is one `async fn` with a default empty body, so you only override what you care about. The handler must be `Send + Sync` and `'static`.

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn at_message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = msg.content.as_deref() {
            if content.contains("ping") {
                let _ = msg.reply(&ctx, "pong").await;
            }
        }
    }
}
```

The Python `Context`-equivalent is passed in explicitly as the second parameter rather than reached for via `self`; REST credentials live inside its shared `BotApi`.

## Token

Python SDKs accept an `app_id` and a `secret` (sometimes called `token`). In `botrs`:

```rust
let token = Token::new(app_id, secret);
// or read QQ_BOT_APP_ID + QQ_BOT_SECRET from the environment
let token = Token::from_env()?;
```

`Token::validate()` exists for sanity-checking before you start; it errors with `BotError::Auth` on missing fields.

## Intents

Where Python uses something like `Intents.default() | Intents.public_guild_messages`, `botrs` uses bitflag constructors:

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message();
```

`Intents::default()` matches the Python "all public, no privileged" preset. The full table is in [intents](/guide/intents).

## Sending a message

The Python pattern of `await api.post_message(channel_id, content="hi", msg_id=...)` translates to the `*Params` builder family:

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("hi").with_reply(&msg_id);
ctx.send_message(&channel_id, params).await?;
```

The same pattern applies to group, C2C, and DM messages: `GroupMessageParams` → `send_group_message`, `C2CMessageParams` → `send_c2c_message`, `DirectMessageParams` → `send_direct_message`.

For richer payloads (embed, ark, markdown, keyboard) build the params struct directly with `..Default::default()`. There is no kwarg explosion — every channel kind has exactly one builder type.

## Starting the bot

Python: `client.run(app_id, secret)`.

`botrs`:

```rust
let mut client = Client::new(token, intents, MyBot, false /* is_sandbox */)?;
client.start().await?;
```

`client.start().await` is the long-lived future. Pair it with `tokio::main` (or your own runtime).

## Logging

`tracing` replaces Python's `logging`. The framework emits at `info` for lifecycle events and `debug` for gateway frames. Initialize a subscriber once:

```rust
tracing_subscriber::fmt().with_env_filter("botrs=info").init();
```

## Error handling

Python's `try/except` becomes Rust's `match` / `?`. Every API call returns `Result<T, BotError>`. See [error handling](/guide/error-handling) for the variant list. The framework does not retry on your behalf, so if your Python code relied on the SDK retrying transient failures, port that loop over manually.

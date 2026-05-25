# 入门

最小的端到端机器人在 [`examples/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/simple_bot.rs)。它创建 `Token`、构造 `Intents`、实现 `EventHandler::message_create` / `group_message_create`，然后启动 `Client`。看一遍即可——其他所有 demo 都遵循同样的骨架。

## 一个机器人的结构

BotRS 进程由三部分组成：`Token`、描述需要哪些网关事件的 `Intents`、以及 `EventHandler` 实现。`Client::new(token, intents, handler, true)` 把它们粘起来，`client.start().await` 跑事件循环。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct Handler;

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("ready as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() { return; }
        let _ = message.reply(&ctx.api, &ctx.token, "pong").await;
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

## 参见

- 指南：[`docs/zh/guide/quick-start.md`](../guide/quick-start.md)、[`docs/zh/guide/intents.md`](../guide/intents.md)
- 实际源码：[`examples/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/simple_bot.rs)
- 运行：`cargo run --example simple_bot --features examples -- $APP_ID $SECRET`

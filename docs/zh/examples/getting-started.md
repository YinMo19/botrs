# 入门

最小的端到端机器人在 [`examples/basic/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/basic/simple_bot.rs)。它创建 `Token`、构造 `Intents`、实现消息回调，然后启动 `Client`。看一遍即可；`examples/guild`、`examples/group`、`examples/c2c`、`examples/direct` 下的场景示例都遵循同样的运行骨架。

## 一个机器人的结构

BotRS 进程由三部分组成：`Token`、描述需要哪些网关事件的 `Intents`、以及 `EventHandler` 实现。`Client::new(token, intents, handler, true)` 把它们粘起来，`client.start().await` 跑事件循环。

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
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
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

## 参见

- 指南：[`docs/zh/guide/quick-start.md`](../guide/quick-start.md)、[`docs/zh/guide/intents.md`](../guide/intents.md)
- 实际源码：[`examples/basic/simple_bot.rs`](https://github.com/YinMo19/botrs/blob/main/examples/basic/simple_bot.rs)
- 运行：`cargo run --example simple_bot --features examples -- $APP_ID $SECRET`

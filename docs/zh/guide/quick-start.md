# 快速开始

一个最小机器人只需要：实现 `EventHandler` 的结构体、`Client::new(token, intents, handler, is_sandbox)` 调用，再加上 `client.start().await`。

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

下面的处理器在频道里被 @ 时回复 `!ping`。在此阶段你只需要 `Token::new(app_id, secret)` 和 `Client::new` 的 `is_sandbox` 这两项配置。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("ready as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
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

    let intents = Intents::new()
        .with_public_guild_messages()
        .with_guilds();

    let mut client = Client::new(token, intents, MyBot, true)?;
    client.start().await?;
    Ok(())
}
```

## 各部分说明

- `Token` 保存 App ID 与 Secret。也可以用 `Token::from_env()`，它会读取 `QQ_BOT_APP_ID` 和 `QQ_BOT_SECRET`。
- `Intents` 是位标志集合。`Intents::new()` 是空集合，链式调用 `with_*` 方法会启用对应事件类别。`Intents::default()` 是标准公域预设：默认开启非特权的公开事件类别，但特权事件和 `ENTER_AIO` 仍需显式开启。
- `Client::new(token, intents, handler, is_sandbox)` —— 开发期传 `true` 走沙箱地址，正式环境传 `false`。
- `message.reply(&ctx, text)` 是回复同频道的便捷方法；要发送更复杂的内容，使用 `ctx.send_message` 与 `MessageParams`（详见 [消息](/zh/guide/messages)）。

## 接下来

- [客户端与事件处理器](/zh/guide/client-handler) —— 所有可分派事件。
- [消息](/zh/guide/messages) —— 嵌入、文件、Markdown、ARK、键盘等消息形式。
- [配置](/zh/guide/configuration) —— 环境变量与沙箱／正式切换。

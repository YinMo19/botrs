# 从 Python QQ 机器人 SDK 迁移

本页把常见的 Python QQ 机器人 SDK 用法对照到 `botrs` 的等价写法。两边的边界一致 —— 同一套网关、同一组 intents、同一种消息类型，因此迁移大都是机械替换。

## 处理器类 → `EventHandler` trait

Python SDK 通常让你继承一个 `Client`（或通过装饰器注册），重写 `async def on_at_message_create(self, msg)` 等回调。

在 `botrs` 中，给自己的 struct 实现 `EventHandler` trait，每个事件对应一个带默认空实现的 `async fn`，按需重写。处理器必须满足 `Send + Sync` 与 `'static`。

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

Python 中通过 `self` 拿到的 `Context`（即 `api` + `token`）在这里以第二个参数显式传入。

## Token

Python SDK 要求传入 `app_id` 与 `secret`（有时称作 `token`）。在 `botrs`：

```rust
let token = Token::new(app_id, secret);
// 或从环境变量 QQ_BOT_APP_ID + QQ_BOT_SECRET 读取
let token = Token::from_env()?;
```

调用 `Token::validate()` 可以提前做基本校验，缺字段时会返回 `BotError::Auth`。

## Intents

Python 中常见的 `Intents.default() | Intents.public_guild_messages`，在 `botrs` 中改用链式构建：

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message();
```

`Intents::default()` 对应 Python 的“全部公开、不含特权”预设。完整表见 [Intents](/zh/guide/intents)。

## 发送消息

Python 的 `await api.post_message(channel_id, content="hi", msg_id=...)` 对应 `*Params` 构建器系列：

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("hi").with_reply(&msg_id);
ctx.send_message(&channel_id, params).await?;
```

群、C2C、私信的形态相同：`GroupMessageParams` → `send_group_message`、`C2CMessageParams` → `send_c2c_message`、`DirectMessageParams` → `send_direct_message`。

复杂载荷（embed、ark、markdown、keyboard）直接 `..Default::default()` 初始化字段即可 —— 不会出现一长串 kwargs，每种场景只有一个构建器类型。

## 启动机器人

Python：`client.run(app_id, secret)`。

`botrs`：

```rust
let mut client = Client::new(token, intents, MyBot, false /* is_sandbox */)?;
client.start().await?;
```

`client.start().await` 是长生命周期的 future，配合 `tokio::main`（或自定义 runtime）使用。

## 日志

`tracing` 取代 Python 的 `logging`。框架在 `info` 级别打印生命周期事件、`debug` 级别打印网关帧。在程序入口初始化一次订阅器：

```rust
tracing_subscriber::fmt().with_env_filter("botrs=info").init();
```

## 错误处理

Python 的 `try/except` 在 Rust 里是 `match` / `?`。所有 API 调用返回 `Result<T, BotError>`，变体列表见 [错误处理](/zh/guide/error-handling)。框架不会替你重试，如果 Python 代码依赖 SDK 内部重试瞬时错误，请把这段循环手动迁移过来。

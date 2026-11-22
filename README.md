# BotRS - Rust QQ Bot Framework
## Author: YinMo19

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88+-blue.svg)](https://www.rust-lang.org)
[![GitHub](https://img.shields.io/badge/github-YinMo19-blue.svg)](https://github.com/YinMo19)
[![Crates.io](https://img.shields.io/crates/v/botrs.svg)](https://crates.io/crates/botrs)

> 本实现不是 qqbot 的官方 rust 版本实现。

BotRS 是一个用 Rust 实现的 QQ 机器人框架，基于 [QQ 频道机器人 API](https://bot.q.qq.com/wiki/develop/api/) 与当前开放平台接口，覆盖频道、私信、群和 C2C 消息场景。它提供类型安全、异步、session-first 的接口来开发 QQ 机器人。

## 快速开始

### 安装

将以下内容添加到你的 `Cargo.toml`:

```toml
[dependencies]
botrs = "0.14.1"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
async-trait = "0.1"
```

### 基础示例

```rust,no_run
use botrs::{Client, Event, Intents, Token};

#[tokio::main]
async fn main() -> botrs::Result<()> {
    let intents = Intents::new().with_public_guild_messages();
    let mut client = Client::new(Token::from_env()?, intents, |event| async move {
        if let Event::MessageCreate(mut session) = event {
            let message = session.message();
            if !message.author.bot && message.content.trim() == "!ping" {
                session.reply("Pong!").await?;
            }
        }
        Ok(())
    }, false)?;
    client.start().await
}
```

设置 `QQ_BOT_APP_ID` 和 `QQ_BOT_SECRET` 后即可运行。也可以实现 `EventHandler`，继续使用 `message_create` 等独立回调。

### Webhook 接入

在仓库中运行 `cargo run --example webhook` 会启动监听于 `127.0.0.1:8080/qq` 的 HTTP 示例。将平台配置的 HTTPS 回调地址转发到这个 POST 路由。框架使用请求头和原始请求体验签，负责地址校验、心跳和事件 ACK。处理器可以通过 `?` 返回错误，由平台按失败 ACK 重试。

我们强烈推荐你查看 [完整文档](https://botrs.yinmo.site/) 来进行开发。这个仓库基于官方网络接口行为和 Rust 的语言特性进行了深度开发。

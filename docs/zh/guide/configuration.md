# 配置

`botrs` 刻意保持极小的配置面：一个 `Token`、一个 `Intents`、一个沙箱标志、一个 HTTP 超时，其他都属于应用层关切，与框架无关。

## Token

`Token::new(app_id, secret)` 是基础构造函数。`Token::from_env()` 从进程环境变量读取 `QQ_BOT_APP_ID` 与 `QQ_BOT_SECRET`，校验后返回 token，缺字段时返回 `BotError::Config`。

```rust
let token = Token::from_env()?;
token.validate()?; // 可选，from_env 已经调用过
```

`Token::validate()` 在 `app_id` 或 `secret` 为空时返回 `BotError::Auth`。框架不做更多格式检查 —— 错误凭据由网关和 REST 接口在运行时拒绝。

打日志时使用 `token.safe_display()`，它会把 secret 打码为 `前4****后4`：

```rust
tracing::info!("loaded {}", token.safe_display());
```

## Intents

详见 [Intents](/zh/guide/intents)。配置形态如下：

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message();
```

按需开启即可，网关只投递启用标志对应的事件。

## 沙箱与正式

`Client::new(token, intents, handler, is_sandbox: bool)` —— 传 `true` 指向 QQ 沙箱地址，传 `false` 指向正式环境。两个端点常量也对外暴露：

```rust
botrs::DEFAULT_API_URL  // "https://api.sgroup.qq.com"
botrs::SANDBOX_API_URL  // "https://sandbox.api.sgroup.qq.com"
botrs::DEFAULT_WS_URL   // "wss://api.sgroup.qq.com/websocket"
```

客户端内部用 `HttpClient::new(timeout_secs, is_sandbox)` 构造 HTTP 客户端。如果你在 `Client` 之外单独构造 `BotApi` 调用 REST，请用同一构造函数选择环境。

## HTTP 超时

`Client::new` 使用 `botrs::DEFAULT_TIMEOUT`（30 秒）。如需修改，使用 `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)`。该超时作用于每个 HTTP 请求；框架未使用长轮询。

```rust
let client = Client::with_config(token, intents, MyBot, 60, false)?;
```

自定义 `HttpClient` 时超时参数完全相同：

```rust
let http = botrs::http::HttpClient::new(60, false)?;
let api = botrs::BotApi::new(http);
```

## 日志

框架使用 `tracing`，未初始化订阅器之前不输出日志：

```rust
tracing_subscriber::fmt()
    .with_env_filter("botrs=info")
    .init();
```

将级别调到 `botrs=debug` 可看到网关帧与 HTTP 调试日志。

## `botrs` 不负责的部分

框架自身不读取配置文件，除 `Token::from_env()` 关心的两个变量外不消费、不设置、不解释任何环境变量。配置文件格式、机密管理、运行时调优、可观测性栈等应用层关切应保留在你自己的代码中。

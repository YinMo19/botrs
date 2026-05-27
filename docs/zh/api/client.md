# Client

`Client<H>` 是高层驱动器：拥有 WebSocket 网关连接、把事件派发给 `EventHandler`，并把 `Arc<BotApi>` 与该处理器共享。`H` 是实现了 `EventHandler` 的具体类型。

```rust
let token = Token::new("app_id", "secret");
let intents = Intents::default().with_public_guild_messages();
let mut client = Client::new(token, intents, MyHandler, /* is_sandbox: */ false)?;

client.start().await?;
```

`Client::new` 返回 `Result<Client<H>>`，因为它会构造底层 `HttpClient`，TLS/DNS 配置异常时会失败。

## 构造器

- `Client::new(token, intents, handler, is_sandbox)` —— 默认 30 秒 HTTP 超时，自动选择网关端点。
- `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` —— 与 `new` 类似，但允许自定义 HTTP 超时。

`is_sandbox = true` 会同时把 REST 主机和 WebSocket 网关切换到 QQ 的沙箱环境，便于联调。

## 生命周期

- `start().await` —— 建立连接、完成 identify，并运行事件循环；事件循环退出（正常或致命错误）后返回。
丢弃正在运行的 `Client` 任务会关闭网关连接。

框架特意保持小表面，没有 `stop` / `is_connected` / `get_session_info` 这些方法 —— 会话状态应当通过事件感知。请用 `EventHandler::ready` 与 `EventHandler::resumed` 监听生命周期变化。

## 访问器

- `api()` —— `&BotApi`，方便在事件循环之外（例如 `start()` 调用前）发请求。
- `http()` —— `&HttpClient`，可微调底层传输参数。
- `intents()` —— 当前配置的 `Intents`。
- `is_sandbox()` —— 是否处于沙箱模式。

## 重连行为

`start()` 会按照 [网关指南](../guide/gateway.md) 描述的节流策略，自动处理瞬时网关故障。检测到不可恢复错误（无效 token、握手失败）时，`start()` 直接返回对应的 `BotError`。

## 优雅停机

```rust
let mut client = Client::new(token, intents, MyHandler, false)?;
let main = tokio::spawn(async move { client.start().await });

tokio::signal::ctrl_c().await?;
main.abort();
let _ = main.await;
```

丢弃 `Client` 任务会关闭网关连接；`reqwest::Client` 不需要显式 close 调用。

## 参见

- [EventHandler](./event-handler.md) —— `H` 必须实现的 trait。
- [Bot API](./bot-api.md) —— 通过 `client.api()` 暴露的完整接口集。
- [Intents](./intents.md) —— 控制网关将派发哪些事件。
- [网关指南](../guide/gateway.md) —— 连接生命周期、心跳、重连节流。

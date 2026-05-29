# Client

`Client<H>` 是高层驱动器：拥有 WebSocket 网关连接、把事件派发给 `EventHandler`，并把 `Arc<BotApi>` 与该处理器共享。`H` 是实现了 `EventHandler` 的具体类型。

```rust
let token = Token::new("app_id", "secret");
let intents = Intents::new().with_public_guild_messages();
let mut client = Client::new(token, intents, MyHandler, /* is_sandbox: */ false)?;

client.start().await?;
```

`Client::new` 返回 `Result<Client<H>>`，因为它会校验 token 并构造底层 `HttpClient`。

## 构造器

- `Client::new(token, intents, handler, is_sandbox)` —— 默认 30 秒 HTTP 超时，自动选择网关端点。
- `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` —— 与 `new` 类似，但允许自定义 HTTP 超时。

`is_sandbox = true` 会同时把 REST 主机和 WebSocket 网关切换到 QQ 的沙箱环境，便于联调。

## 生命周期

- `start().await` —— 执行启动阶段 REST 调用、启动网关 session，并运行事件循环直到事件通道关闭。
丢弃正在运行的 `Client` 任务会关闭网关连接。

框架特意保持小表面，没有 `stop` / `is_connected` / `get_session_info` 这些方法 —— 会话状态应当通过事件感知。请用 `EventHandler::ready` 与 `EventHandler::resumed` 监听生命周期变化。

## 重连行为

`start()` 会把 token 校验、`get_bot_info`、`get_gateway` 的启动阶段错误直接返回。网关 session 启动后，瞬时故障会按照 [网关指南](../guide/gateway.md) 描述的节流策略自动重试；不可恢复的 identify 错误会停止该 shard 的重连循环，并由 session manager 记录日志。

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
- [Bot API](./bot-api.md) —— 事件 session 中可用的完整接口集。
- [Intents](./intents.md) —— 控制网关将派发哪些事件。
- [网关指南](../guide/gateway.md) —— 连接生命周期、心跳、重连节流。

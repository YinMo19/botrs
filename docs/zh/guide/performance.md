# 性能

框架本身只是 `reqwest` 与 `tokio-tungstenite` 之上的薄层；真正影响性能的是网关侧的几个调节项（shard 数与 intents），以及任务之间如何共享 `BotApi` 与 `Token`。通用 Rust 性能建议不在此处展开。

## 收紧 Intents

每开启一个 `Intents` 标志，就多一类事件需要网关推送、处理器忽略或处理。`Intents::default()` 已经过滤掉特权标志；正式机器人最好显式列出真正消费的事件：

```rust
let intents = Intents::none()
    .with_public_guild_messages()
    .with_direct_message();
```

这样既节省网关带宽，也节省了无关事件的反序列化开销。

## Sharding

`Client::start()` 从 `BotApi::get_gateway` 返回的 `gateway_info.shards` 读取推荐 shard 数。每个 shard 是独立的 WebSocket 连接，由会话管理器驱动。一般无需手动配置 —— 数量由 QQ 侧根据机器人加入的频道数量给出。

小机器人单 shard 即可，资源占用最低。规模较大时会话管理器会按 `Gateway::session_start_interval(max_concurrency)` 节流地依次拉起多 shard，避免触及每日会话启动配额。处理器并不重 CPU 但频道很多时，向 QQ 申请更高的 `session_start_limit.max_concurrency` 能缩短各 shard 上线时间差。

重连节流实现为 `round(2 / max_concurrency)`，下限 1 秒，请不要绕过。详见 [网关](/zh/guide/gateway)。

## 用 `Arc` 共享 `BotApi`

`Context::api` 已经是 `Arc<BotApi>`。在处理器里 `tokio::spawn` 时，克隆 `Arc`（以及 `Token`），而不是把 `Context` 整体移动进去：

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let api = ctx.api.clone();
    let token = ctx.token.clone();
    tokio::spawn(async move {
        let _ = api.post_message_with_params(&token, "channel", params).await;
    });
}
```

`Arc<BotApi>` 的克隆只是一次原子加 1，`Token` 的克隆只复制两段短字符串，相对一次 HTTP 请求开销可以忽略。不要在每次调用里新建 `BotApi` —— 内部 `reqwest::Client` 维护连接池，重新构造会丢掉连接复用。

## 保持处理器非阻塞

每个 shard 的回调按串行 `await`。在处理器里做长时间同步工作会拖延同一 shard 的后续事件。要么保持工作量很小，要么按上面的写法 `tokio::spawn`。框架没有其他背压机制 —— 事件通道是无界的，处理器跟不上时它会持续增长。

## HTTP 超时

默认 `HttpClient` 超时为 30 秒（`botrs::DEFAULT_TIMEOUT`）。延迟预算更紧时通过 `Client::with_config(... , timeout_secs, ...)` 调小，但要记住上传或大响应本就需要时间，过激进的超时只会让 `BotError::Timeout` 频繁抖动，并不能提升性能。

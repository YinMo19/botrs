# Performance

The framework is a thin layer on top of `reqwest` and `tokio-tungstenite`; the things that move the needle are gateway-side knobs (shards and intents) and how you share `Context` / `BotApi` between tasks. Generic Rust performance advice doesn't belong here.

## Narrow your intents

Every `Intents` flag you enable adds another category of dispatch the gateway has to push and your handler has to ignore (or handle). `Intents::default()` is a convenient public preset; for non-trivial bots, replace it with the explicit set you actually consume:

```rust
let intents = Intents::new()
    .with_public_guild_messages()
    .with_direct_message();
```

The savings come both from gateway bandwidth and from skipping deserialization for events you don't want.

## Sharding

`Client::start()` reads the recommended shard count from `BotApi::get_gateway` (`gateway_info.shards`). Each shard is a separate WebSocket connection driven by the session manager. You don't normally configure this — the count comes from QQ's side and reflects your bot's guild population.

For very small bots a single shard is fine and uses the least resource. For larger bots, the session manager opens multiple shards in sequence, throttled by `Gateway::session_start_interval(max_concurrency)`, so you don't trip the daily session-start budget. If your handler is CPU-light but you have many guilds, increasing concurrency on QQ's side (which you negotiate via `gateway_info.session_start_limit.max_concurrency`) reduces the spread between shards coming online.

The reconnect throttle is implemented as `round(2 / max_concurrency)` with a 1-second floor; do not bypass it. See [gateway](/guide/gateway) for the details.

## Share `Context` or `BotApi`

`Context` is cheap to clone because it holds an `Arc<BotApi>`. When you spawn work from inside a handler, clone the context and move it into the task:

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let context = ctx.clone();
    tokio::spawn(async move {
        let _ = context.send_message("channel", params).await;
    });
}
```

The clone is cheap relative to a single HTTP request. Don't build a new `BotApi` per call — the `reqwest::Client` it wraps maintains its own connection pool, and rebuilding it throws that pool away.

## Keep handlers non-blocking

Handler calls are awaited serially per shard. Long-running synchronous work in a handler delays the next event for that shard. Either keep the work small or `tokio::spawn` it as shown above. The framework gives you no other backpressure mechanism — the event channel is unbounded and will grow if your handler can't keep up.

## HTTP timeout

The default `HttpClient` timeout is 30 seconds (`botrs::DEFAULT_TIMEOUT`). Lower it with `Client::with_config(... , timeout_secs, ...)` if your latency budget is tighter than that, but remember that uploads and large-payload responses can legitimately take time. A too-aggressive timeout produces `BotError::Timeout` flapping rather than performance.

# Error Recovery

There is no specific example for error recovery in `examples/`; the framework does the recovery you actually need (gateway reconnect, session resume, heartbeat) automatically inside `Client::start`. What is left to your code is checking the `Result` of each `BotApi` call and deciding whether to log, ignore, or retry.

## What the framework already handles

- WebSocket reconnect with session-start throttling
- Session resume after transient disconnects
- Heartbeat keepalive
- Token validation on `Token::validate()`

You implement `EventHandler::error(&self, error: BotError)` if you want to observe dispatch errors surfaced by the framework. The default implementation logs them.

## Per-call

`BotApi` methods return `botrs::Result<T>`. The variant you care about is `botrs::BotError`, which already exposes whether the error is transient (network, rate limit) or permanent (auth, malformed payload). Match on it where it matters; otherwise log and move on.

```rust
match session.reply(&reply).await {
    Ok(_) => {},
    Err(e) => tracing::warn!("reply failed: {e}"),
}
```

## See also

- Guide: [`docs/guide/error-handling.md`](../guide/error-handling.md) — the canonical reference for `BotError` variants and patterns
- `BotError` definition: `src/error/bot_error.rs`

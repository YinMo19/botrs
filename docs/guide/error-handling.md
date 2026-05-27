# Error handling

Every fallible function in `botrs` returns `Result<T, BotError>`. `BotError` is a `thiserror` enum; you usually pattern-match a few variants and propagate the rest with `?`.

## Variants you'll actually see

The variants you'll match on most often correspond to outcomes the QQ API and gateway produce:

- `BotError::Api { code, message }` — QQ-side rejection from any REST call. `code` is the platform error code; `message` is the human-readable reason.
- `BotError::AuthenticationFailed(String)` — HTTP 401. The `Token` is wrong or revoked.
- `BotError::Forbidden(String)` — HTTP 403. The bot lacks permission for the action.
- `BotError::NotFound(String)` — HTTP 404. Targeted resource (channel, message, member) does not exist.
- `BotError::MethodNotAllowed(String)` — HTTP 405. Wrong endpoint or method.
- `BotError::Server(String)` — HTTP 500 / 504. QQ-side outage; transient.
- `BotError::SequenceNumber(String)` — HTTP 429-shaped sequence error. Distinct from `RateLimit` below; usually means message ordering, not throttling.
- `BotError::RateLimit { retry_after }` — explicit rate-limit response. `retry_after` is in seconds.
- `BotError::Auth(String)` / `BotError::Config(String)` / `BotError::InvalidData(String)` — local validation failures (bad app id, malformed env, payload that can't serialize).
- `BotError::Connection(String)` / `BotError::Gateway(String)` / `BotError::Session(String)` — gateway lifecycle failures.
- `BotError::Timeout` — request didn't complete inside the configured `HttpClient` timeout.
- `BotError::Http(reqwest::Error)` / `BotError::WebSocket(...)` / `BotError::Json(...)` / `BotError::Url(...)` / `BotError::Io(...)` — transport-level wrappers. These already render usefully with `Display`, so logging them is usually enough.
- `BotError::Sdk(SdkError)` — an internal SDK error carrying a numeric code and trace ID, used by the gateway state machine. Match it for diagnostics, then propagate.

## How errors reach you

REST calls (`BotApi::*`) return the error directly. Inside a handler, you decide what to do:

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let params = MessageParams::new_text("hi");
    if let Err(e) = ctx.api.post_message_with_params(&ctx.token, "channel", params).await {
        match e {
            BotError::RateLimit { retry_after } => {
                tracing::warn!(retry_after, "rate limited; dropping reply");
            }
            BotError::Forbidden(_) => {
                tracing::warn!("missing permission for channel");
            }
            other => tracing::error!(error = %other, "send failed"),
        }
    }
}
```

If event dispatch itself fails (a malformed payload, a panic in your handler converted to error, etc.), the framework calls `EventHandler::error(&self, error: BotError)` once and then continues consuming events. The default implementation logs with `error!`. Override it to plug in metrics, alerting, or your own panic handling — but note that the framework does not retry the original event for you.

## Retry helpers

`BotError::is_retryable(&self)` returns true for `Http` (timeouts and connect errors only), `WebSocket`, `Connection`, `Gateway`, `Timeout`, and `RateLimit`. `BotError::retry_after(&self)` returns a suggested delay in seconds (`retry_after` for rate limits, `5` for `Connection`, `1` for `Gateway`, `3` for `Timeout`, `1` for everything else retryable, `None` otherwise).

These are advisory. The framework itself does not retry on your behalf — every `BotApi` call fails immediately on the first error, and gateway reconnects are throttled by `Gateway::session_start_interval`, not by `retry_after`. If you want retry semantics around a specific call, write the loop yourself, consulting `is_retryable` / `retry_after`:

```rust
loop {
    match ctx.api.post_message_with_params(&ctx.token, channel, params.clone()).await {
        Ok(resp) => break Ok(resp),
        Err(e) if e.is_retryable() => {
            if let Some(secs) = e.retry_after() {
                tokio::time::sleep(Duration::from_secs(secs)).await;
                continue;
            }
            break Err(e);
        }
        Err(e) => break Err(e),
    }
}
```

Keep these loops bounded — there is no protection against an infinite retry inside a handler tying up the dispatch task.

## Status code mapping

`http_error_from_status(status, message)` is the conversion the HTTP layer applies internally. It's exposed if you build your own clients on top of `HttpClient`:

| Status      | Variant                       |
|-------------|-------------------------------|
| 401         | `AuthenticationFailed`        |
| 403         | `Forbidden`                   |
| 404         | `NotFound`                    |
| 405         | `MethodNotAllowed`            |
| 429         | `SequenceNumber`              |
| 500 / 504   | `Server`                      |
| other       | `Api { code: status, message }` |

# Error Types

Every fallible call in BotRS returns `Result<T, BotError>`. `BotError` is an `enum` covering transport, protocol, and local-validation failures, plus a flexible `Sdk` variant for QQ-side error codes that don't fit the typed variants.

```rust
pub type Result<T> = std::result::Result<T, BotError>;
```

`BotError` derives `thiserror::Error` (so it implements `std::error::Error`) and produces useful `Display` strings.

## Variants

| Variant                                      | Source                                                   |
|----------------------------------------------|----------------------------------------------------------|
| `Sdk(SdkError)`                              | QQ-defined error code with optional response body.       |
| `Http(reqwest::Error)`                       | Transport / TLS / DNS / timeout from the HTTP client.    |
| `WebSocket(Box<tungstenite::Error>)`         | Gateway WebSocket failure.                               |
| `Json(serde_json::Error)`                    | Payload decoding failure.                                |
| `Url(url::ParseError)`                       | Invalid URL during request building.                     |
| `Api { code: u32, message: String }`         | Non-2xx HTTP response with parsed body.                  |
| `AuthenticationFailed(String)`               | Token signing or `Authorization` header rejected.        |
| `NotFound(String)`                           | 404 / "object missing" responses.                        |
| `MethodNotAllowed(String)`                   | 405.                                                     |
| `Forbidden(String)`                          | 403 / permission denied.                                 |
| `RateLimit { retry_after: u64 }`             | 429 with parsed `Retry-After` (in seconds).              |
| `SequenceNumber(String)`                     | Gateway sequence skew that demands resume / reconnect.   |
| `Server(String)`                             | 5xx responses without structured codes.                  |
| `Auth(String)`                               | Higher-level authentication / session issue.             |
| `Connection(String)`                         | Gateway lifecycle / heartbeat failure.                   |
| `Config(String)`                             | Invalid configuration before requests run.               |
| `InvalidData(String)`                        | Local validation (e.g. malformed permission string).     |
| `Timeout`                                    | Network timeout, surfaced from internal stages.          |
| `Gateway(String)`                            | Gateway protocol error not covered by other variants.    |
| `Session(String)`                            | Gateway session bookkeeping failure.                     |
| `Internal(String)`                           | Bug-style internal invariant violation.                  |
| `Io(std::io::Error)`                         | Local I/O (e.g. file uploads).                           |

`Sdk` wraps `SdkError`, the framework's internal error code carrier. It exposes `code()`, `message()`, and `trace_id()`. The framework uses it as the catch-all for QQ-defined codes (e.g. 9999 for "unknown SDK error", `CODE_CONN_CLOSE_CANT_RESUME`, `CODE_CONN_CLOSE_CANT_IDENTIFY`).

## Construction helpers

A few short constructors live on `BotError` for code that builds errors locally:

- `BotError::invalid_data(msg)` — quick `InvalidData` wrap.
- `BotError::auth(msg)` / `BotError::config(msg)` / `BotError::internal(msg)` — same idea for the corresponding variants.

## Pattern matching

```rust
use botrs::BotError;

match api.get_bot_info().await {
    Ok(bot) => println!("{}", bot.username),
    Err(BotError::Http(e)) if e.is_timeout() => warn!("timed out"),
    Err(BotError::RateLimit { retry_after }) => {
        tokio::time::sleep(Duration::from_secs(retry_after)).await;
    }
    Err(BotError::Forbidden(msg)) => warn!("permission denied: {msg}"),
    Err(BotError::Api { code, message }) => warn!("qq api error {code}: {message}"),
    Err(other) => return Err(other),
}
```

For protocol-defined codes carried inside `Sdk`, use:

```rust
if let BotError::Sdk(ref err) = e {
    match err.code() {
        CODE_CONN_CLOSE_CANT_RESUME => /* discard session, identify fresh */,
        CODE_CONN_CLOSE_CANT_IDENTIFY => /* stop identifying until state changes */,
        _ => {}
    }
}
```

Constants for these codes are exposed from `botrs::error`, for example `botrs::error::CODE_CONN_CLOSE_CANT_RESUME`.

## Conversions

`BotError` derives `From` for the wrapped error types (`reqwest::Error`, `serde_json::Error`, `url::ParseError`, `std::io::Error`), so the `?` operator works naturally inside `async fn -> Result<...>`.

## See also

- [Bot API](./bot-api.md) — every route that can produce these errors.
- [Token](./token.md) — credential refresh logic that surfaces `Auth` errors.
- [Gateway guide](../guide/gateway.md) — how the framework reacts to `Connection` / `SequenceNumber` errors.

# Client

`Client<H>` is the high-level driver that owns the WebSocket gateway connection, dispatches events to your `EventHandler`, and shares an `Arc<BotApi>` with that handler. `H` is the concrete type implementing `EventHandler`.

```rust
let token = Token::new("app_id", "secret");
let intents = Intents::default().with_public_guild_messages();
let mut client = Client::new(token, intents, MyHandler, /* is_sandbox: */ false)?;

client.start().await?;
```

`Client::new` returns `Result<Client<H>>` because it constructs the underlying `HttpClient` and may fail if TLS / DNS configuration is invalid.

## Constructors

- `Client::new(token, intents, handler, is_sandbox)` — default 30-second HTTP timeout, gateway endpoints picked automatically.
- `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` — same as `new` but lets you override the HTTP client timeout.

`is_sandbox = true` switches both the REST host and the WebSocket gateway to QQ's sandbox environment for testing.

## Lifecycle

- `start().await` — connects, identifies, and runs the event loop. Returns once the loop terminates (gracefully or via fatal error).
Dropping the running `Client` task closes the gateway connection.

There are no `stop` / `is_connected` / `get_session_info` methods — the framework intentionally exposes a small surface and pushes session details into events instead. Use `EventHandler::ready` and `EventHandler::resumed` to observe lifecycle changes.

## Reconnect behaviour

`start()` reconnects automatically on transient gateway failures, applying the throttling described in the [gateway guide](../guide/gateway.md). When a fatal failure (invalid token, unrecoverable handshake) is detected, `start()` resolves with the propagated `BotError`.

## Graceful shutdown

```rust
let mut client = Client::new(token, intents, MyHandler, false)?;
let main = tokio::spawn(async move { client.start().await });

tokio::signal::ctrl_c().await?;
main.abort();
let _ = main.await;
```

Dropping the `Client` task closes the gateway connection; `reqwest::Client` does not require an explicit close call.

## See also

- [EventHandler](./event-handler.md) — the trait `H` must implement.
- [Bot API](./bot-api.md) — full route catalogue exposed through `Context` during event handling.
- [Intents](./intents.md) — controls which events the gateway will dispatch.
- [Gateway guide](../guide/gateway.md) — connection lifecycle, heartbeats, reconnect throttling.

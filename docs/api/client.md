# Client

`Client<H>` is the high-level driver that owns the WebSocket gateway connection, dispatches events to your `EventHandler`, and shares an `Arc<BotApi>` with that handler. `H` is the concrete type implementing `EventHandler`.

```rust
let token = Token::new("app_id", "secret");
let intents = Intents::new().with_public_guild_messages();
let mut client = Client::new(token, intents, MyHandler, /* is_sandbox: */ false)?;

client.start().await?;
```

`Client::new` returns `Result<Client<H>>` because it validates the token and constructs the underlying `HttpClient`.

## Constructors

- `Client::new(token, intents, handler, is_sandbox)` — default 30-second HTTP timeout, gateway endpoints picked automatically.
- `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` — same as `new` but lets you override the HTTP client timeout.

`is_sandbox = true` switches both the REST host and the WebSocket gateway to QQ's sandbox environment for testing.

## Lifecycle

- `start().await` — performs startup REST calls, starts gateway sessions, and runs the event loop until the event channel closes.
Cancelling the `start()` future closes the gateway connections. Use `abort()` when running it in a spawned Tokio task.

There are no `stop` / `is_connected` / `get_session_info` methods — the framework intentionally exposes a small surface and pushes session details into events instead. Use `EventHandler::ready` and `EventHandler::resumed` to observe lifecycle changes.

## HTTP callbacks

Pass the request headers and original body bytes to `client.handle_webhook(&headers, &body).await`. Serialize the returned `WebhookReply` as JSON with HTTP 200. The client verifies Ed25519 signatures and handles endpoint validation, heartbeats and dispatch acknowledgements. Run `cargo run --example webhook` from the repository for a working server. Webhook delivery works independently of `start()`. Webhook sessions return `None` from `bot_info()`. Use `session.api().get_bot_info().await` when needed.

## Reconnect behaviour

`start()` propagates startup failures from token validation, `get_bot_info`, and `get_gateway`. After gateway sessions are spawned, transient gateway failures are retried with the throttling described in the [gateway guide](../guide/gateway.md); a fatal identify error stops all shards and is returned by `start()`.

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
- [Bot API](./bot-api.md) — full route catalogue exposed through event sessions.
- [Intents](./intents.md) — controls which events the gateway will dispatch.
- [Gateway guide](../guide/gateway.md) — connection lifecycle, heartbeats, reconnect throttling.

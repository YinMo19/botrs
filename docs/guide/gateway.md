# Gateway

The gateway is the WebSocket connection between your bot and QQ. The `Client` builds and runs it for you — most users never touch the gateway types directly. This page documents the lifecycle the framework manages on your behalf and the small number of knobs that affect it.

## Lifecycle managed by the client

When you call `client.start().await`, the framework:

1. Validates your `Token` and fetches the current bot user via `BotApi::get_bot_info`.
2. Calls `BotApi::get_gateway` to obtain the WebSocket URL, the recommended shard count, and the session-start limits.
3. Validates the limits via `CheckSessionLimit` (returns `BotError::Session` if you've already exhausted your daily start budget).
4. Computes a reconnect interval from `session_start_limit.max_concurrency` using `CalcInterval`.
5. Spawns a session manager that opens one `Gateway` per shard and pumps events into the channel the client reads.

The client then loops on the receiver: every dispatched event is decoded into the typed payload and routed to the matching `EventHandler` callback.

## Heartbeat

After a `HELLO` op the gateway records the server-supplied `heartbeat_interval` (in milliseconds) and starts a heartbeat task. Each tick sends an op-1 `HEARTBEAT` carrying the last received sequence number; if a `HEARTBEAT_ACK` does not return before the next tick, the connection is considered dead and the socket is closed so the reconnect path can run.

You don't need to send heartbeats yourself, but you can observe heartbeat health through tracing — the gateway logs ack latency at `debug` level.

## Resume vs identify

After a clean disconnect the gateway tries `RESUME` first using the cached `session_id` and `last_seq`. If the server responds with `INVALID_SESSION` (or a close code in the "cannot resume" set returned by `CanNotResume`), the next attempt falls back to a fresh `IDENTIFY`. Close codes in the `CanNotIdentify` set (for example, `4014` "disallowed intent") cause the gateway to stop reconnecting; the client surface that as `BotError::Gateway`.

## Reconnect throttling

The framework follows the official QQ guidance: never reconnect in a tight loop. The interval between two `connect_once` attempts comes from `Gateway::session_start_interval(max_concurrency)`, which implements `round(2 / max_concurrency)` with a one-second floor. For `max_concurrency = 1` that's two seconds; higher concurrency tiers shorten it accordingly.

If you need a custom interval (e.g. for tests), call `Gateway::with_reconnect_interval(Duration::from_secs(n))` before driving it. Zero is normalized to one second to avoid pathological loops.

## Inspecting state

A live `Gateway` exposes a few `pub fn`s for observability:

- `is_ready() -> bool` — `true` after the first `READY` dispatch.
- `can_reconnect() -> bool` — flips to `false` once a non-resumable close is observed.
- `session_id() -> Option<&str>` — the resume session id or `None` before identify completes.
- `last_sequence() -> u64` — the last received `s` value, also used in heartbeats.

You usually access these from a custom session manager. The default `SessionManager` (created by `NewSessionManager()`) is the one the client uses.

## Sharding

The number of shards comes from `gateway_info.shards` (returned by `BotApi::get_gateway`). Each shard is a separate WebSocket connection that the session manager throttles using the calculated session start interval. There is no manual shard configuration in `Client`; if you need a custom topology, build `Gateway` instances yourself, pass `Some([shard_id, total])` to `Gateway::new`, and run them through a session manager you constructed via `set_session_manager_factory`.

## Related types

- `botrs::ConnectionState` / `botrs::ConnectionSession` (re-exported from `connection`) describe the websocket-level state machine when you implement a custom session manager.
- Constants such as `botrs::DEFAULT_WS_URL` (`wss://api.sgroup.qq.com/websocket`) and `botrs::SANDBOX_API_URL` are the published endpoints.

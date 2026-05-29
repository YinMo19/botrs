# Gateway

The gateway is managed by `Client`. Application code normally interacts with gateway traffic by implementing `EventHandler`.

## Lifecycle

When `client.start().await` runs, the framework:

1. validates the `Token`;
2. builds a shared `BotApi`;
3. fetches bot info with `get_bot_info`;
4. fetches gateway metadata with `get_gateway`;
5. creates shard sessions from the returned URL, shard count, and session-start limit;
6. connects each shard, authenticates, sends heartbeats, and forwards gateway dispatch payloads into the client event loop.

The client then parses each gateway dispatch into the relevant Rust model and calls the matching `EventHandler` method.

## Heartbeat and reconnect

After `HELLO`, the runtime starts a heartbeat task using the server-provided heartbeat interval. Heartbeats carry the last received sequence number. If the socket closes, the runtime tries to resume with the cached session id and sequence number. Non-resumable close codes force a new identify; fatal identify errors stop that shard's reconnect loop and are logged by the session manager.

The session manager spaces shard starts according to `session_start_limit.max_concurrency`. This keeps reconnects from hammering the platform after a network failure.

## Event dispatch

Known event names are parsed into typed payloads. Examples:

- `READY` -> `ready(ReadySession)`
- `AT_MESSAGE_CREATE` -> `message_create(ChannelReplySession)`
- `DIRECT_MESSAGE_CREATE` -> `direct_message_create(DirectReplySession)`
- `GROUP_AT_MESSAGE_CREATE` -> `group_message_create(GroupReplySession)`
- `C2C_MESSAGE_CREATE` -> `c2c_message_create(C2CReplySession)`
- `MESSAGE_REACTION_ADD` / `MESSAGE_REACTION_REMOVE` -> reaction callbacks
- guild, channel, member, manage, audio, forum, and open-forum events -> their corresponding callbacks

Unknown event names are passed to `unknown_event(UnknownEventSession)` so newer platform events can still be observed.

## Configuration knobs

The public knobs are intentionally small:

- `Client::new(token, intents, handler, is_sandbox)`
- `Client::with_config(token, intents, handler, timeout, is_sandbox)`
- `Intents` controls which gateway event categories QQ sends.
- `is_sandbox` switches REST and gateway discovery to the sandbox environment.

## See also

- [Client](../api/client.md)
- [Event handler](../api/event-handler.md)
- [Intents](./intents.md)

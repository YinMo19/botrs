# Changelog

This page summarizes the currently documented BotRS release line. The main API surface is intentionally small and follows the runtime path used by examples and gateway event handling.

## [0.14.1] - Current

- Handle ordinary group messages with `group_full_message_create`. Read the group's receive mode, proactive messaging permission and bot role with `get_group_bot_state`.
- Use `GroupMessage::quote_reference()` to quote the triggering message. Incoming messages also expose structured elements and mention details.
- Remove group members with `remove_group_members` when QQ grants the required permissions.
- The WebSocket gateway buffers up to 1,024 events across shards and waits for capacity when handlers fall behind.
- Incoming message structs gained fields. Code that initializes every field explicitly can use `..Default::default()` for the new fields.

## [0.14.0]

- WebSocket and signed Webhook callbacks share typed events. Handlers can return a result and use the question-mark operator.
- Message sessions use platform reply IDs and handle QQ's minimal payloads. REST errors expose the HTTP status, QQ code, trace ID and response body.
- The minimum Rust version is 1.88. Channel sequence numbers are optional, and group and C2C keyboards use the typed Keyboard model.

## [0.13.0]

### Current Surface

- `Client` owns the gateway lifecycle and dispatches typed events to `EventHandler`.
- Event callbacks receive session objects that expose the shared `BotApi` and add reply helpers for the current event.
- `BotApi` covers bot info, gateway discovery, message sending/recall, group/C2C file upload, guild/channel resources, roles, mute, channel permissions, announcements, schedules, API permissions, reactions, pinned messages, and audio controls.
- Message sending uses session helpers for common replies and `MessageParams`, `GroupMessageParams`, `C2CMessageParams`, and `DirectMessageParams` constructors for lower-level calls.
- Gateway events are decoded into typed payloads for messages, direct messages, group/C2C messages, reactions, interactions, guilds, channels, members, audits, manage events, audio events, and forum events.

### Notes

- The public docs focus on what users can build with the current API.
- Examples under `examples/` are the best entry point for end-to-end usage.

## Links

- [Repository](https://github.com/YinMo19/botrs)
- [Documentation](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [Issues](https://github.com/YinMo19/botrs/issues)

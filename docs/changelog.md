# Changelog

This page summarizes the currently documented BotRS release line. The main API surface is intentionally small and follows the runtime path used by examples and gateway event handling.

## [0.13.0] - Current

### Current Surface

- `Client` owns the gateway lifecycle and dispatches typed events to `EventHandler`.
- `Context` dereferences to the shared `BotApi`, so handler code can call REST methods directly.
- `BotApi` covers bot info, gateway discovery, message sending, message recall, group/C2C file upload, announcements, schedules, API permissions, reactions, and pinned messages.
- Message sending uses `MessageParams`, `GroupMessageParams`, `C2CMessageParams`, and `DirectMessageParams`.
- Gateway events are decoded into typed payloads for messages, direct messages, group/C2C messages, reactions, interactions, guilds, channels, members, audits, manage events, audio events, and forum events.

### Notes

- The crate version in `Cargo.toml` is `0.13.0`.
- The public docs focus on what users can build with the current API.
- Examples under `examples/` are the best entry point for end-to-end usage.

## Links

- [Repository](https://github.com/YinMo19/botrs)
- [Documentation](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [Issues](https://github.com/YinMo19/botrs/issues)

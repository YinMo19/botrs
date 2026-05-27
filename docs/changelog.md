# Changelog

All notable changes to BotRS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Removed
- Removed the `src/api/compat` PascalCase `BotApi` facade (`PostMessage`, `Channels`, `PostAudio`, etc.). Use the native snake_case `BotApi` methods instead.
- Removed the top-level Go-style OpenAPI compatibility modules (`facade`, `openapi`, and `options`) and the HTTP filter registry that only existed for that facade.
- Removed standalone Botgo helper facades (`log`, `remote`, `search`, and `version`) that were not part of the REST or gateway wire behavior.
- Removed Go-style error/session helper names (`Err`, `New`, `Error`, `CanNotResume`, `CheckSessionLimit`, etc.) in favor of native Rust names.
- Removed the unused Botgo-style `websocket` facade; use `Client`, `Gateway`, and `session_manager` for Rust-native gateway handling.
- Removed Go-style event, webhook, and signature helper names (`ParseData`, `RegisterHandlers`, `HTTPHandler`, `Gen*ACK`, `Generate`, `Verify`, etc.) in favor of native Rust names.
- Removed Go-style token/API/constant helper names (`NewQQBotTokenSource`, `GetAppID`, `StartRefreshAccessToken`, `APIv1`, `HeaderTraceID`, etc.) in favor of native Rust names.
- Removed Go-style intent constants (`IntentGuilds`, `IntentGroupMessages`, `IntentNone`, etc.); use `Intents::GUILDS`, `Intents::PUBLIC_MESSAGES`, or `0` instead.
- Removed Go-style enum constant aliases (`AudioStatusStart`, `InteractionTypePing`, `LayoutTypeImageText`, etc.); use enum variants or Rust-style constants instead.
- Removed Go-style pager, member-delete option, and message helper names (`QueryParams`, `WithAddBlackList`, `GetEventID`, `GetSendType`, `APIMessage`, etc.) in favor of native Rust methods.
- Removed Go-style message mention helper names (`MentionUser`, `MentionAllUser`, `MentionChannel`, `Emoji`, `ETLInput`, `ParseCommand`); use `mention_user`, `mention_all_user`, `mention_channel`, `emoji`, `etl_input`, and `parse_command`.
- Removed Go-style message payload and keyboard constant aliases (`TextMsg`, `MarkdownMsg`, `RichMedia`, `ActionTypeURL`, `PermissionTypAll`, etc.); use `MessageCreateType` and Rust-style `ACTION_TYPE_*` / `PERMISSION_TYPE_*` constants.
- Removed Go-style and botpy-style message content type aliases (`ArkKV`, `ArkObjKV`, `MessageKeyboard`, `CustomKeyboard`, `TemplateID`, `MarkdownParams`, etc.); use the concrete Rust types such as `ArkKv`, `Keyboard`, `KeyboardContent`, `KeyboardTemplateId`, and `MarkdownParam`.
- Renamed Go-style error code constants (`CodeNeedReConnect`, `WSCodeBackendAuthenticationFail`, `APICodeTokenExpireOrNotExist`, etc.) to Rust-style `CODE_*`, `WS_CODE_*`, and `API_CODE_*` names.
- Removed Go-style channel enum aliases and duplicate enum-value constants (`ChannelTypeText`, `ChannelSubTypeChat`, `ChannelPrivateTypePublic`, `SpeakPermissionTypePublic`, `CHANNEL_TYPE_TEXT`, etc.); use enum variants such as `ChannelType::Text`.
- Removed Go-style gateway opcode/event aliases (`OPCode`, `WSDispatchEvent`, `WSIdentity`, `HTTPCallbackAck`, `EventMessageCreate`, `OPMeans`, etc.); use `OpCode`, `WS_DISPATCH_EVENT`, `WS_IDENTIFY`, `HTTP_CALLBACK_ACK`, `EVENT_MESSAGE_CREATE`, and `op_meaning`.
- Removed redundant Rust alias methods (`BotApi::me`, `BotApi::me_guilds`, `BotApi::get_ws_url`, `BotApi::get_permissions`, `BotApi::patch_message`, and matching `Context` wrappers); use `get_bot_info`, `get_guilds`, `get_gateway`, `get_api_permissions`, and `edit_message`.
- Removed botpy-style duplicate methods (`BotApi::create_dms`, `Context::create_dms`, `BotApi::get_delete_member`, and `Context::get_delete_member`); use `create_direct_message` and `delete_member`.
- Removed redundant `Context` alias methods (`add_reaction`, `remove_reaction`, `pin_message`, `unpin_message`, `add_guild_role_member`, and `remove_guild_role_member`); use `put_reaction`, `delete_reaction`, `put_pin`, `delete_pin`, `create_guild_role_member`, and `delete_guild_role_member`.
- Removed compatibility type aliases (`WebsocketAP`, `DirectMessageSession`, `ReactionUser`, `MessageReactionUsers`, `Announces`, `EnterAIO`, `HTTPIdentity`, `HTTPReady`, `HTTPSession`, `WHValidationReq`, `WHValidationRsp`, `RoleID`, and `Roles`) and the Go-style `SessionManager::Start` / `DefaultColor` aliases; use the concrete Rust types and `SessionManager::start` / `DEFAULT_ROLE_COLOR`.
- Removed the deprecated multi-`Option` message sending methods (`post_message`, `post_group_message`, `post_c2c_message`, `post_dms`), the `*_with_params` compatibility names, and their `Context` wrappers. Use `send_message`, `send_group_message`, `send_c2c_message`, and `send_direct_message`.
- Removed the redundant `post_message_api` and `patch_message_api` aliases.

## [0.11.0] - 2026-05-25

### Changed
- Aligned user, guild/member/role, mute, manage-event, gateway/webhook, interaction-search, message-audit, and message-setting DTOs with the QQ Bot Open API.
- Matched the documented zero-value and `omitempty` JSON wire shapes for the updated DTOs.

### Fixed
- Kept local gateway/event helper context out of DTO JSON where the protocol uses pure payload shapes.

## [0.10.0] - 2026-05-25

### Changed
- Aligned channel, schedule, audio, announce, and API permission DTOs with the documented zero-value wire shapes.
- `PostAudio`, `post_audio`, and `Context::post_audio` now use `AudioControl`; audio events retain `AudioAction`.
- `AudioStatus` now serializes and deserializes as the protocol's numeric status values.
- Updated gateway documentation to describe fixed reconnect throttling instead of exponential backoff.

### Fixed
- `ListSchedules` now always sends the `since` query value, matching the documented API behavior.

## [0.9.0] - 2026-05-25

### Changed
- Aligned `MessageReaction` and `WSMessageReactionData` with the QQ Bot Open API's pure message reaction DTO.
- Made reaction `Emoji`, `ReactionTarget`, and `MessageReaction` fields required instead of silently defaulting missing protocol data.
- Aligned forum `ThreadInfo.title` and `ThreadInfo.content` with the QQ Bot Open API by keeping raw strings.
- Updated GitHub Actions to current major versions to avoid Node 20 action deprecation.

### Fixed
- Reaction event parsing now propagates malformed payload errors instead of constructing partial empty reaction values.

## [0.8.0] - 2026-05-25

### Changed
- Aligned `DirectMessage` with the QQ Bot Open API's direct-message session DTO.
- `direct_message_create` now receives a regular `Message`, matching the documented `WSDirectMessageData`.
- Updated direct-message examples and API docs to use `DirectMessageParams` with `send_direct_message`.

### Fixed
- Completed DTO parity with the QQ Bot Open API for message, guild, and interaction wire formats.
- Matched the documented message helper parsing behavior for mentions and command splitting.

## [0.7.0] - 2026-05-25

### Added
- OpenAPI v1 concrete constants `HeaderCallbackAppID` and `MaxIdleConns` aligned with the QQ Bot Open API.
- `Session::from_app_id` helper for HTTP callback payload sessions.

### Changed
- OpenAPI instances now preserve the configured app ID state and send `X-Union-Appid` on requests.
- `PutInteraction` now uses the OpenAPI app ID for `X-Callback-AppID`, matching the QQ Bot Open API.
- HTTP webhook dispatch payloads now include a session with the configured app ID.

### Fixed
- Clear stale session ID and sequence before requeueing sessions after non-resumable gateway close errors.
- Accept the legacy wrapped single-message response shape (`{"message": ...}`) returned by some endpoints.

## [0.6.0] - 2026-05-25

### Added
- `WSPayload.session`, `C2CFriendData`, `WSC2CFriendData`, message helper, pager helper, and OpenAPI facade names aligned with the QQ Bot Open API.
- `APIMessage`, `GetEventID`, `GetSendType`, `QueryParams`, OpenAPI group aliases, `openapi::Register`, and `DefaultImpl` compatibility entry points.

### Changed
- Aligned websocket event payload DTOs and C2C friend events with the QQ Bot Open API's pure data-model shape.
- Kept Rust-style lowercase helpers while adding upper-camel-case exported names for smoother migration.

## [0.5.0] - 2026-05-25

### Added
- OpenAPI facade, version registry, request options, HTTP filters, and token refresh helpers aligned with the QQ Bot Open API.

### Changed
- Aligned OpenAPI success status handling with the QQ Bot Open API (`200` and `204` only).
- Shared token caches across cloned token sources without unsafe mutation.
- Refreshed the lockfile to avoid yanked transitive dependencies.

### Fixed
- Reset WebSocket reconnect backoff after successful reconnects.
- Isolated OpenAPI registry tests to avoid parallel-test flakiness.

## [0.2.5] - 2025-07-30

### Added
- Additional message parameter validation
- Enhanced error context in API responses
- Support for more message attachment types

### Fixed
- Memory leak in WebSocket connection handling
- Race condition in event dispatching
- Incorrect handling of empty message content

### Changed
- Improved connection stability with better retry logic
- Updated dependencies to latest versions

## [0.2.0] - 2025-07-29

### Added
- **New Structured Message API**: Complete redesign of message sending with structured parameters
- `MessageParams` for guild messages with builder pattern support
- `GroupMessageParams` for group messages
- `C2CMessageParams` for private messages
- `DirectMessageParams` for direct messages
- New methods: `send_message`, `send_group_message`, `send_c2c_message`, `send_direct_message`
- Comprehensive support for all QQ Guild message types (text, embeds, files, markdown, keyboards, ARK messages)
- Enhanced file upload capabilities with proper MIME type detection
- Message reference and reply functionality
- Interactive keyboard and button support
- Forum and thread management APIs

### Changed
- **Breaking**: Moved from multiple `None` parameters to structured parameter objects
- Improved API ergonomics with builder patterns such as `.with_reply()`
- Better type safety with compile-time parameter validation
- Enhanced error messages with more context
- Optimized memory usage in message handling

### Deprecated
- Old message API methods (`post_message`, `post_group_message`, `post_c2c_message`, `post_dms`)
- Multiple `None` parameter patterns (still functional but deprecated)

### Fixed
- WebSocket reconnection issues in unstable network conditions
- Message encoding problems with special characters
- Memory leaks in long-running bot instances
- Rate limiting edge cases

### Security
- Improved token validation and error handling
- Better input sanitization for user-provided content

## [0.1.3] - 2025-07-29

### Added
- Support for group message events (`GROUP_ADD_ROBOT`, `GROUP_DEL_ROBOT`, `GROUP_MSG_RECEIVE`, `GROUP_MSG_REJECT`)
- C2C (Client-to-Client) message handling (`FRIEND_ADD`, `FRIEND_DEL`, `C2C_MSG_RECEIVE`, `C2C_MSG_REJECT`)
- Audio and live channel member management
- Message reaction APIs (`PUT /channels/{channel_id}/messages/{message_id}/reactions/{type}`)
- Forum thread creation and management
- Scheduled message support
- PIN message functionality
- Advanced permission management APIs

### Changed
- Improved event handler trait with more granular event types
- Better error propagation in API calls
- Enhanced logging with structured output
- Updated to latest QQ Guild API specifications

### Fixed
- Event parsing issues with new message formats
- Connection stability improvements
- Memory usage optimization

## [0.1.2] - 2025-07-29

### Added
- Message audit event handling (`MESSAGE_AUDIT_PASS`, `MESSAGE_AUDIT_REJECT`)
- Enhanced guild member event support
- Better WebSocket error recovery
- Configurable retry mechanisms for API calls

### Changed
- Improved documentation with more examples
- Better error types with more specific error information
- Enhanced performance for high-throughput scenarios

### Fixed
- Issues with special characters in message content
- WebSocket connection drops in certain network conditions
- Memory leaks in event processing

## [0.1.1] - 2025-07-29

### Added
- Basic message recall functionality
- Enhanced file upload support with progress tracking
- Better logging integration with `tracing` crate

### Fixed
- Critical bug in message parsing for embed content
- Issues with bot user identification
- WebSocket heartbeat timing problems

### Changed
- Improved API response parsing
- Better handling of rate limits

## [0.1.0] - 2025-07-29

### Added
- Initial release of BotRS
- Core WebSocket gateway connection handling
- Basic message sending and receiving
- Event-driven architecture with `EventHandler` trait
- Support for guild messages, direct messages, and system events
- Intent system for event filtering
- Built-in rate limiting and retry logic
- Comprehensive error handling with `BotError` types
- Integration with Tokio async runtime
- Support for embeds, files, and rich message content
- Guild and channel management APIs
- Member and role management
- Basic authentication and token management

### Core Features
- `Client` - Main bot client with WebSocket management
- `EventHandler` - Trait for handling various bot events
- `BotApi` - REST API client for QQ Guild endpoints
- `Token` - Authentication and credential management
- `Intents` - Event subscription configuration
- Message types: `Message`, `DirectMessage`, `GroupMessage`
- Guild types: `Guild`, `Channel`, `Member`, `Role`
- Comprehensive error handling and logging

### Supported Events
- `READY` - Bot connection established
- `GUILD_CREATE`, `GUILD_UPDATE`, `GUILD_DELETE` - Guild lifecycle
- `CHANNEL_CREATE`, `CHANNEL_UPDATE`, `CHANNEL_DELETE` - Channel management
- `GUILD_MEMBER_ADD`, `GUILD_MEMBER_UPDATE`, `GUILD_MEMBER_REMOVE` - Member events
- `AT_MESSAGE_CREATE` - Message mentions
- `DIRECT_MESSAGE_CREATE` - Private messages
- `MESSAGE_DELETE` - Message deletions

## Migration Guides

### Migrating from 0.1.x to 0.2.x

The major change in v0.2.0 is the introduction of structured message parameters. Here's how to migrate:

#### Old API (Deprecated)
```rust
// Multiple None parameters - confusing and error-prone
api.post_message(
    token, "channel_id", Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

#### New API (Recommended)
```rust
use botrs::models::message::MessageParams;

// Clean, readable, type-safe
let params = MessageParams::new_text("Hello!")
    .with_reply("message_id")
    .with_markdown(true);
api.send_message("channel_id", params).await?;
```

#### Method Mappings
- `post_message` → `send_message`
- `post_group_message` → `send_group_message`
- `post_c2c_message` → `send_c2c_message`
- `post_dms` → `send_direct_message`

### Breaking Changes in 0.2.0

1. **Message API Structure**: Parameter objects replace positional arguments
2. **Import Paths**: Some message types moved to `botrs::models::message`
3. **Builder Patterns**: New `.with_*()` methods for parameter construction
4. **Default Values**: Use `..Default::default()` instead of multiple `None`

## Security Advisories

### RUSTSEC-2023-0001 (Resolved in 0.1.2)
- **Issue**: Potential memory leak in WebSocket connection handling
- **Impact**: Long-running bots could experience increased memory usage
- **Resolution**: Fixed connection cleanup in event loop
- **Affected Versions**: 0.1.0, 0.1.1
- **Fixed In**: 0.1.2+

## Removed APIs

### v0.1.x Message API
The old message API with multiple `None` parameters has been removed. Use the structured parameter API.

```rust
let params = MessageParams::new_text(content);
api.send_message(channel, params).await?;
```

## Version Support

| Version | Status | End of Life |
|---------|--------|-------------|
| 0.2.x   | ✅ Active | TBD |
| 0.1.x   | ⚠️ Security fixes only | 2024-06-01 |

## Contributing

See [CONTRIBUTING.md](contributing.md) for guidelines on contributing to BotRS.

## Links

- [Repository](https://github.com/YinMo19/botrs)
- [Documentation](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [Issues](https://github.com/YinMo19/botrs/issues)

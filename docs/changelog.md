# Changelog

All notable changes to BotRS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation website with English and Chinese support
- API reference documentation for all core components
- Getting started examples and tutorials
- Configuration guide with best practices

### Changed
- Documentation improvements and restructuring

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
- New methods: `post_message_with_params`, `post_group_message_with_params`, `post_c2c_message_with_params`, `post_dms_with_params`
- Comprehensive support for all QQ Guild message types (text, embeds, files, markdown, keyboards, ARK messages)
- Enhanced file upload capabilities with proper MIME type detection
- Message reference and reply functionality
- Interactive keyboard and button support
- Forum and thread management APIs

### Changed
- **Breaking**: Moved from multiple `None` parameters to structured parameter objects
- Improved API ergonomics with builder patterns (`.with_reply()`, `.with_file_image()`, etc.)
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
api.post_message_with_params(token, "channel_id", params).await?;
```

#### Method Mappings
- `post_message` → `post_message_with_params`
- `post_group_message` → `post_group_message_with_params`
- `post_c2c_message` → `post_c2c_message_with_params`
- `post_dms` → `post_dms_with_params`

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

## Deprecation Notice

### v0.1.x Message API (Deprecated in 0.2.0)
The old message API with multiple `None` parameters is deprecated and will be removed in v0.3.0. Please migrate to the new structured parameter API.

```rust
// ❌ Deprecated - will be removed in v0.3.0
api.post_message(token, channel, Some(content), None, None, None, None, None, None, None, None, None).await?;

// ✅ New API - recommended
let params = MessageParams::new_text(content);
api.post_message_with_params(token, channel, params).await?;
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

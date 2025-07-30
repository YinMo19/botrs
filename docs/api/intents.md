# Intents API Reference

The `Intents` struct controls which gateway events your bot receives from QQ Guild. This system allows you to optimize performance and bandwidth by only subscribing to the events your bot actually needs.

## Overview

```rust
use botrs::Intents;

// Default intents for most bots
let intents = Intents::default();

// Custom intent combinations
let intents = Intents::GUILD_MESSAGES | Intents::GUILDS;

// Builder pattern
let intents = Intents::new()
    .with_guilds()
    .with_public_guild_messages()
    .with_direct_message();
```

Intents act as a subscription system for gateway events. By selecting only the intents you need, you can reduce bandwidth usage and improve bot performance.

## Intent Types

### Public Intents

These intents are available to all bots without special approval:

#### `GUILDS`

Guild creation, update, and deletion events.

```rust
const GUILDS: u32 = 1 << 0;
```

**Events enabled:**
- `guild_create`
- `guild_update` 
- `guild_delete`

#### `GUILD_MEMBERS`

Guild member join, update, and leave events.

```rust
const GUILD_MEMBERS: u32 = 1 << 1;
```

**Events enabled:**
- `guild_member_add`
- `guild_member_update`
- `guild_member_remove`

#### `GUILD_MESSAGE_REACTIONS`

Message reaction add and remove events.

```rust
const GUILD_MESSAGE_REACTIONS: u32 = 1 << 10;
```

**Events enabled:**
- Reaction add/remove events
- Emoji interaction events

#### `DIRECT_MESSAGE`

Private message events between users and the bot.

```rust
const DIRECT_MESSAGE: u32 = 1 << 12;
```

**Events enabled:**
- `direct_message_create`
- Private message events

#### `INTERACTION`

Interactive component events like button clicks and slash commands.

```rust
const INTERACTION: u32 = 1 << 26;
```

**Events enabled:**
- Button interactions
- Select menu interactions
- Slash command interactions

#### `MESSAGE_AUDIT`

Message audit and moderation events.

```rust
const MESSAGE_AUDIT: u32 = 1 << 27;
```

**Events enabled:**
- `message_audit_pass`
- `message_audit_reject`

#### `AUDIO_ACTION`

Voice channel and audio events.

```rust
const AUDIO_ACTION: u32 = 1 << 29;
```

**Events enabled:**
- Voice channel updates
- Audio state changes

#### `PUBLIC_GUILD_MESSAGES`

Public guild messages including @mentions and replies to the bot.

```rust
const PUBLIC_GUILD_MESSAGES: u32 = 1 << 30;
```

**Events enabled:**
- `message_create` (when bot is mentioned)
- Reply messages to bot
- Public channel messages involving bot

#### `AUDIO_OR_LIVE_CHANNEL_MEMBER`

Voice and live channel member events.

```rust
const AUDIO_OR_LIVE_CHANNEL_MEMBER: u32 = 1 << 19;
```

**Events enabled:**
- `audio_or_live_channel_member_enter`
- `audio_or_live_channel_member_exit`

#### `OPEN_FORUM_EVENT`

Public forum thread and post events.

```rust
const OPEN_FORUM_EVENT: u32 = 1 << 18;
```

**Events enabled:**
- `open_forum_thread_create`
- `open_forum_thread_update`
- `open_forum_thread_delete`
- `open_forum_post_create`
- `open_forum_post_delete`
- `open_forum_reply_create`
- `open_forum_reply_delete`

#### `PUBLIC_MESSAGES`

Group and C2C message events.

```rust
const PUBLIC_MESSAGES: u32 = 1 << 25;
```

**Events enabled:**
- `group_message_create`
- `c2c_message_create`

### Privileged Intents

These intents require special approval from QQ and may have additional restrictions:

#### `GUILD_MESSAGES`

All guild message events (privileged).

```rust
const GUILD_MESSAGES: u32 = 1 << 9;
```

**Requirements:**
- Special approval from QQ
- Additional verification for large bots

**Events enabled:**
- All `message_create` events in guilds
- `message_delete` events

#### `FORUMS`

Forum thread and post events (privileged).

```rust
const FORUMS: u32 = 1 << 28;
```

**Requirements:**
- Special approval from QQ
- May require additional permissions

**Events enabled:**
- All forum-related events
- Private forum access

## Constructor Methods

### `new`

Creates an empty intent set.

```rust
pub const fn new() -> Self
```

#### Example

```rust
let intents = Intents::new(); // No intents enabled
```

### `none`

Creates an intent set with no intents enabled (alias for `new`).

```rust
pub const fn none() -> Self
```

### `all`

Creates an intent set with all available intents enabled.

```rust
pub const fn all() -> Self
```

#### Example

```rust
let intents = Intents::all(); // All intents enabled
```

### `default`

Creates the default intent set for most bots (excludes privileged intents).

```rust
pub const fn default() -> Self
```

The default intents include all public intents but exclude `GUILD_MESSAGES` and `FORUMS` which require special approval.

#### Example

```rust
let intents = Intents::default(); // Safe for most bots
```

### `from_bits`

Creates intents from raw bit flags.

```rust
pub const fn from_bits(bits: u32) -> Self
```

#### Parameters

- `bits`: Raw intent bit flags

#### Example

```rust
let intents = Intents::from_bits(0b1011); // Custom bit combination
```

## Intent Management

### `contains`

Checks if a specific intent is enabled.

```rust
pub const fn contains(self, intent: u32) -> bool
```

#### Parameters

- `intent`: The intent flag to check

#### Returns

`true` if the intent is enabled, `false` otherwise.

#### Example

```rust
let intents = Intents::GUILDS | Intents::PUBLIC_GUILD_MESSAGES;
assert!(intents.contains(Intents::GUILDS));
assert!(!intents.contains(Intents::DIRECT_MESSAGE));
```

### `with_intent`

Enables a specific intent.

```rust
pub const fn with_intent(self, intent: u32) -> Self
```

#### Parameters

- `intent`: The intent flag to enable

#### Returns

New `Intents` instance with the intent enabled.

#### Example

```rust
let intents = Intents::new().with_intent(Intents::GUILDS);
```

### `without_intent`

Disables a specific intent.

```rust
pub const fn without_intent(self, intent: u32) -> Self
```

#### Parameters

- `intent`: The intent flag to disable

#### Returns

New `Intents` instance with the intent disabled.

#### Example

```rust
let intents = Intents::all().without_intent(Intents::GUILD_MESSAGES);
```

## Builder Methods

### Guild Intents

```rust
pub const fn with_guilds(self) -> Self
pub const fn with_guild_members(self) -> Self
pub const fn with_guild_messages(self) -> Self
pub const fn with_guild_message_reactions(self) -> Self
```

### Message Intents

```rust
pub const fn with_direct_message(self) -> Self
pub const fn with_public_guild_messages(self) -> Self
pub const fn with_public_messages(self) -> Self
```

### Feature Intents

```rust
pub const fn with_interaction(self) -> Self
pub const fn with_message_audit(self) -> Self
pub const fn with_forums(self) -> Self
pub const fn with_audio_action(self) -> Self
pub const fn with_audio_or_live_channel_member(self) -> Self
pub const fn with_open_forum_event(self) -> Self
```

#### Example

```rust
let intents = Intents::new()
    .with_guilds()
    .with_public_guild_messages()
    .with_direct_message()
    .with_interaction();
```

## Query Methods

### Guild Queries

```rust
pub const fn guilds(self) -> bool
pub const fn guild_members(self) -> bool
pub const fn guild_messages(self) -> bool
pub const fn guild_message_reactions(self) -> bool
```

### Message Queries

```rust
pub const fn direct_message(self) -> bool
pub const fn public_guild_messages(self) -> bool
pub const fn public_messages(self) -> bool
```

### Feature Queries

```rust
pub const fn interaction(self) -> bool
pub const fn message_audit(self) -> bool
pub const fn forums(self) -> bool
pub const fn audio_action(self) -> bool
pub const fn audio_or_live_channel_member(self) -> bool
pub const fn open_forum_event(self) -> bool
```

#### Example

```rust
let intents = Intents::default();

if intents.guilds() {
    println!("Guild events enabled");
}

if intents.direct_message() {
    println!("Direct message events enabled");
}
```

## Utility Methods

### `has_privileged`

Checks if any privileged intents are enabled.

```rust
pub const fn has_privileged(self) -> bool
```

#### Returns

`true` if `GUILD_MESSAGES` or `FORUMS` intents are enabled.

#### Example

```rust
let intents = Intents::default();
assert!(!intents.has_privileged()); // Default excludes privileged

let privileged = Intents::new().with_guild_messages();
assert!(privileged.has_privileged());
```

### `bits`

Gets the raw intent bit flags.

```rust
pub const fn bits(self) -> u32
```

#### Returns

The raw intent bits as a 32-bit unsigned integer.

#### Example

```rust
let intents = Intents::GUILDS | Intents::PUBLIC_GUILD_MESSAGES;
let bits = intents.bits();
println!("Intent bits: {:#032b}", bits);
```

## Bitwise Operations

Intents support standard bitwise operations for combining and manipulating intent sets:

### Bitwise OR (`|`)

Combines intents from multiple sets.

```rust
let intents = Intents::GUILDS | Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE;
```

### Bitwise AND (`&`)

Finds common intents between sets.

```rust
let common = intents1 & intents2;
```

### Bitwise XOR (`^`)

Finds intents that differ between sets.

```rust
let different = intents1 ^ intents2;
```

### Bitwise NOT (`!`)

Inverts all intent flags.

```rust
let inverted = !intents;
```

### Assignment Operators

```rust
let mut intents = Intents::new();
intents |= Intents::GUILDS;        // Add intent
intents &= !Intents::DIRECT_MESSAGE; // Remove intent
```

## Common Usage Patterns

### Basic Bot

```rust
// Simple bot that responds to mentions
let intents = Intents::PUBLIC_GUILD_MESSAGES | Intents::GUILDS;
```

### Moderation Bot

```rust
// Bot with moderation capabilities
let intents = Intents::default()
    .with_guild_members()
    .with_message_audit();
```

### Voice Bot

```rust
// Bot that manages voice channels
let intents = Intents::new()
    .with_guilds()
    .with_audio_action()
    .with_audio_or_live_channel_member();
```

### Forum Bot

```rust
// Bot that manages forum content
let intents = Intents::new()
    .with_guilds()
    .with_open_forum_event()
    .with_forums(); // Requires approval
```

### Comprehensive Bot

```rust
// Bot with full capabilities (requires privileged intents)
let intents = Intents::all();
```

## Privileged Intent Approval

To use privileged intents (`GUILD_MESSAGES`, `FORUMS`), you need:

1. **Application Review**: Submit your bot for review in the QQ Developer Portal
2. **Use Case Justification**: Explain why your bot needs access to these events
3. **Privacy Compliance**: Ensure your bot complies with data protection requirements
4. **Scale Verification**: For large bots (100+ guilds), additional verification may be required

### Requesting Approval

1. Visit the QQ Developer Portal
2. Navigate to your bot's settings
3. Request privileged intent access
4. Provide detailed justification
5. Wait for approval (can take several days)

## Error Handling

### Missing Intents

If your bot doesn't receive expected events, verify your intents:

```rust
let intents = Intents::default();

// Check if required intents are enabled
if !intents.guild_members() {
    println!("Warning: Guild member events not enabled");
}

if !intents.public_guild_messages() {
    println!("Warning: Public guild messages not enabled");
}
```

### Privileged Intent Errors

```rust
impl EventHandler for MyBot {
    async fn error(&self, error: BotError) {
        match error {
            BotError::Forbidden(msg) if msg.contains("intent") => {
                eprintln!("Missing required intents or privileged intent not approved");
            }
            _ => {}
        }
    }
}
```

## Best Practices

### Intent Selection

1. **Minimal Principle**: Only enable intents you actually use
2. **Performance**: Fewer intents = better performance and lower bandwidth
3. **Privacy**: Avoid privileged intents unless absolutely necessary
4. **Documentation**: Document why each intent is needed

### Production Considerations

1. **Testing**: Test with minimal intents in development
2. **Monitoring**: Monitor for missing events that might indicate intent issues
3. **Approval Process**: Plan for privileged intent approval timeline
4. **Fallback**: Design graceful degradation when intents are missing

### Code Organization

```rust
// Define intents as constants for reuse
const BOT_INTENTS: Intents = Intents::new()
    .with_guilds()
    .with_public_guild_messages()
    .with_direct_message();

// Validate intents at startup
fn validate_intents(intents: Intents) -> Result<(), String> {
    if !intents.guilds() {
        return Err("Guild events are required".to_string());
    }
    
    if intents.has_privileged() {
        println!("Warning: Using privileged intents");
    }
    
    Ok(())
}
```

## See Also

- [Intents Guide](/guide/intents) - Comprehensive guide to intent usage
- [`Client`](./client.md) - Bot client configuration
- [`EventHandler`](./event-handler.md) - Event handling with intents
- [Gateway Guide](/guide/gateway) - Gateway connection and intents
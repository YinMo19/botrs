# Intents

`Intents` is a 32-bit flag set that tells the QQ gateway which event categories your bot wants to receive. Subscribing only to what you need keeps gateway traffic small and avoids dispatcher work for events you'd discard.

```rust
pub struct Intents { pub bits: u32 }
```

`Intents` is `Copy`, supports the bitwise operators (`|`, `&`, `^`, `!`), and serializes to its raw `u32` for the gateway `Identify` payload.

## Constructors

- `Intents::new()` — empty (`bits == 0`).
- `Intents::none()` — alias for `new()`, used when you want to opt in field-by-field.
- `Intents::default()` — `all()` with the privileged intents (`GUILD_MESSAGES` and `FORUMS`) cleared. The right starting point for most bots.
- `Intents::all()` — every flag the framework knows about. Use only if your bot has every privilege approved.
- `Intents::from_bits(bits)` — wrap a raw integer received from configuration.

## Flag catalogue

Each row lists the public Rust constant, the underlying bit, the `with_*` builder, and a short description.

| Constant                                | Bit       | Builder                              | What it covers                                          |
|-----------------------------------------|-----------|--------------------------------------|---------------------------------------------------------|
| `Intents::GUILDS`                       | `1 << 0`  | `with_guilds`                        | Guild create / update / delete + channel CRUD.          |
| `Intents::GUILD_MEMBERS`                | `1 << 1`  | `with_guild_members`                 | Member add / update / remove.                           |
| `Intents::GUILD_MESSAGES` (privileged)  | `1 << 9`  | `with_guild_messages`                | All guild messages — needs special approval.            |
| `Intents::GUILD_MESSAGE_REACTIONS`      | `1 << 10` | `with_guild_message_reactions`       | Reactions added / removed in guild messages.            |
| `Intents::DIRECT_MESSAGE`               | `1 << 12` | `with_direct_message`                | Direct-message create / delete events.                  |
| `Intents::OPEN_FORUM_EVENT`             | `1 << 18` | `with_open_forum_event`              | Open-forum activity (public forums).                    |
| `Intents::AUDIO_OR_LIVE_CHANNEL_MEMBER` | `1 << 19` | `with_audio_or_live_channel_member`  | Voice / live channel member changes.                    |
| `Intents::ENTER_AIO`                    | `1 << 23` | `with_enter_aio`                     | User entering AIO (chat panel).                         |
| `Intents::PUBLIC_MESSAGES`              | `1 << 25` | `with_public_messages`               | Group `@bot` messages, C2C messages, friend events.     |
| `Intents::INTERACTION`                  | `1 << 26` | `with_interaction`                   | Interaction (button / app) callbacks.                   |
| `Intents::MESSAGE_AUDIT`                | `1 << 27` | `with_message_audit`                 | Message audit pass / reject.                            |
| `Intents::FORUMS` (privileged)          | `1 << 28` | `with_forums`                        | All forum events — needs special approval.              |
| `Intents::AUDIO_ACTION`                 | `1 << 29` | `with_audio_action`                  | Audio start / finish / on-mic / off-mic.                |
| `Intents::PUBLIC_GUILD_MESSAGES`        | `1 << 30` | `with_public_guild_messages`         | `@bot` mentions and public message-delete events.       |

## Inspection

For each flag there's a matching predicate (no parameters): `intents.guilds()`, `intents.public_guild_messages()`, etc. Two helpers cover common groupings:

- `intents.contains(bits)` — generic membership check against a raw bit.
- `intents.has_privileged()` — `true` when `GUILD_MESSAGES` or `FORUMS` are enabled.

## Mutation

- `with_intent(bits)` / `without_intent(bits)` — generic chainable setters.
- `bits()` — return the raw `u32`.
- `Display` impl prints `Intents(GUILDS | DIRECT_MESSAGE | …)` for logging.

## Examples

```rust
// 1. Sensible defaults — no privileged events.
let default_intents = Intents::default();

// 2. Custom subscription via builder.
let custom = Intents::none()
    .with_guilds()
    .with_public_guild_messages()
    .with_direct_message();

// 3. Bitwise composition.
let public = Intents::from_bits(
    Intents::GUILDS | Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE,
);

// 4. Drop a flag at runtime.
let trimmed = Intents::all().without_intent(Intents::FORUMS);
```

## Privileged intents

`GUILD_MESSAGES` and `FORUMS` require manual approval in the QQ Developer Portal. Bots without that approval will be disconnected by the gateway when they identify with these bits set. `Intents::default()` deliberately omits them so the same code path works for all bots.

```rust
if intents.has_privileged() && !your_bot_is_approved {
    return Err("privileged intents requested without approval".into());
}
```

## See also

- [Bot API](./bot-api.md) — every endpoint that produces events you might subscribe to.
- [Event handler](./event-handler.md) — receives the events selected by these intents.
- [Gateway guide](../guide/gateway.md) — how the framework opens a session with the chosen intents.

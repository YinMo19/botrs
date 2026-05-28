# Intents

`Intents` is a bitflag set you pass to `Client::new`. It tells the gateway which event categories to deliver. You only receive events for categories you opted into, so picking the smallest set you need keeps payload volume down and avoids handlers firing for events you do not consume.

## Constructors

- `Intents::new()` — empty set.
- `Intents::default()` — every public intent except the two privileged ones (`GUILD_MESSAGES` and `FORUMS`). Suitable for most public bots.
- `Intents::all()` — every flag, including privileged ones.

## Toggling flags

Intent flags are exposed as `pub const` `u32` values on the `Intents` type, plus a chainable `with_*` method per flag:

```rust
use botrs::Intents;

let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds();
```

You can also OR raw constants together via `Intents::with_intent(Intents::GUILDS)` if you prefer.

## Flag reference

| Flag                          | Builder                              | Events the gateway will deliver                          |
|-------------------------------|--------------------------------------|----------------------------------------------------------|
| `GUILDS`                      | `with_guilds`                        | `guild_create` / `guild_update` / `guild_delete`         |
| `GUILD_MEMBERS`               | `with_guild_members`                 | `guild_member_add` / `_update` / `_remove`               |
| `GUILD_MESSAGES` *(priv.)*    | `with_guild_messages`                | All channel messages (`message_create` and friends)      |
| `GUILD_MESSAGE_REACTIONS`     | `with_guild_message_reactions`       | Reaction add / remove                                    |
| `DIRECT_MESSAGE`              | `with_direct_message`                | DM `direct_message_create`                               |
| `PUBLIC_GUILD_MESSAGES`       | `with_public_guild_messages`         | At-mention messages (`message_create`) — non-priv.       |
| `PUBLIC_MESSAGES`             | `with_public_messages`               | Group / C2C messages                                     |
| `INTERACTION`                 | `with_interaction`                   | Button / interaction events                              |
| `MESSAGE_AUDIT`               | `with_message_audit`                 | Message audit pass / reject                              |
| `FORUMS` *(priv.)*            | `with_forums`                        | Forum thread / post / reply / publish-audit              |
| `AUDIO_ACTION`                | `with_audio_action`                  | Audio start / finish / on-mic / off-mic                  |
| `OPEN_FORUM_EVENT`            | `with_open_forum_event`              | Open-forum thread / post / reply                         |
| `ENTER_AIO`                   | `with_enter_aio`                     | Single-chat (AIO) entry events                           |
| `AUDIO_OR_LIVE_CHANNEL_MEMBER`| `with_audio_or_live_channel_member`  | Voice / live channel join / leave                        |

The privileged intents (`GUILD_MESSAGES`, `FORUMS`) require approval in the QQ Guild developer portal; without it the gateway will reject the identify.

## Inspecting at runtime

`contains(flag)` checks a single flag, and the named accessors (`intents.guilds()`, `intents.public_guild_messages()`, etc.) return `bool`.

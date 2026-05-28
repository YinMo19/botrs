# BotRS Examples

This directory contains runnable examples for the BotRS public API surface. The examples are grouped by message scene so it is clear which event, intent, parameter type, and send API belong together.

## Layout

```text
examples/
├── api/        # Cross-scene message parameter API examples
├── basic/      # Minimal bot skeleton
├── c2c/        # C2C single-user messages and management events
├── common/     # Shared config and logging helpers
├── direct/     # Guild direct messages
├── events/     # Non-message gateway events
├── group/      # QQ group messages and management events
├── guild/      # Guild channel messages and channel APIs
└── resource/   # Local notes for media examples
```

All examples use `examples/common` except `basic/simple_bot.rs`, which stays standalone as the smallest skeleton.

## Configuration

Examples load credentials in this order:

1. `examples/config.toml` if present.
2. `config.toml` from the repository root.
3. `QQ_BOT_APP_ID`, `QQ_BOT_SECRET`, and optional `QQ_BOT_SANDBOX` environment variables.
4. Command-line arguments: `cargo run --example <name> --features examples -- <app_id> <secret>`.

Create a local config from the template:

```bash
cp examples/config.example.toml examples/config.toml
```

Then fill in:

```toml
[bot]
app_id = "your_bot_app_id"
secret = "your_bot_secret"
sandbox = false
```

## Running

Build all examples:

```bash
cargo check --examples --features examples
```

Run one example:

```bash
cargo run --example guild_reply_text --features examples
```

Pass credentials directly:

```bash
cargo run --example guild_reply_text --features examples -- your_bot_app_id your_bot_secret
```

Enable detailed logs:

```bash
RUST_LOG=debug cargo run --example guild_reply_text --features examples
```

## Guild Channel Examples

| Example | Source | Shows |
|---|---|---|
| `simple_bot` | `basic/simple_bot.rs` | Minimal `Client`, `Token`, `Intents`, and handler wiring |
| `guild_reply_text` | `guild/reply_text.rs` | Replying to guild channel @ messages with `ChannelReplySession::reply` |
| `guild_reply_reference` | `guild/reply_reference.rs` | Quoted channel replies with `MessageParams::message_reference` |
| `guild_command` | `guild/command.rs` | Lightweight text command dispatch in `message_create` |
| `guild_reply_embed` | `guild/reply_embed.rs` | `Embed` payloads |
| `guild_reply_markdown` | `guild/reply_markdown.rs` | Markdown content and template payloads |
| `guild_reply_keyboard` | `guild/reply_keyboard.rs` | Markdown plus keyboard templates or inline keyboard content |
| `guild_reply_ark` | `guild/reply_ark.rs` | ARK template payloads |
| `guild_reply_image` | `guild/reply_image.rs` | Channel image URL messages |
| `guild_recall` | `guild/recall.rs` | Sending and recalling a channel message |
| `guild_pins_message` | `guild/pins_message.rs` | Pin list, add pin, and delete pin |
| `guild_announce` | `guild/announce.rs` | Channel and guild announcements |
| `guild_schedule` | `guild/schedule.rs` | Schedule create, get, update, and delete |
| `guild_api_permission` | `guild/api_permission.rs` | API permission list and permission demand creation |
| `guild_reaction_users` | `guild/reaction_users.rs` | Reaction user pagination |
| `guild_tools` | `guild/tools.rs` | Reading, editing, recalling, announcing, cleaning pins, and setting guides |

Guild channel message examples generally use `Intents::new().with_public_guild_messages()` and send through `session.reply`, `session.send_message`, or other `BotApi` methods exposed through the session.

## Group Examples

| Example | Source | Shows |
|---|---|---|
| `group_reply_text` | `group/reply_text.rs` | Plain text group replies with `GroupMessageParams` |
| `group_reply_file` | `group/reply_file.rs` | `post_group_file` followed by `msg_type: 7` media sending |
| `group_reply_ark` | `group/reply_ark.rs` | Group ARK payloads with `msg_type: 3` |
| `group_reply_markdown` | `group/reply_markdown.rs` | Group Markdown payloads with `msg_type: 2` |
| `group_reply_embed` | `group/reply_embed.rs` | Group Embed payloads with `msg_type: 4` |
| `group_reply_keyboard` | `group/reply_keyboard.rs` | Group Markdown plus keyboard template payloads |
| `group_manage_event` | `group/manage_event.rs` | Robot add/remove and group active-message toggles |

Group examples use `Intents::new().with_public_messages()`. Replies should preserve `msg_id` and `event_id` from the inbound event when those fields are present.

## C2C Examples

| Example | Source | Shows |
|---|---|---|
| `c2c_reply_text` | `c2c/reply_text.rs` | Plain text C2C replies with `C2CMessageParams` |
| `c2c_reply_file` | `c2c/reply_file.rs` | `post_c2c_file` followed by `msg_type: 7` media sending |
| `c2c_reply_ark` | `c2c/reply_ark.rs` | C2C ARK payloads with `msg_type: 3` |
| `c2c_reply_markdown` | `c2c/reply_markdown.rs` | C2C Markdown payloads with `msg_type: 2` |
| `c2c_reply_embed` | `c2c/reply_embed.rs` | C2C Embed payloads with `msg_type: 4` |
| `c2c_reply_keyboard` | `c2c/reply_keyboard.rs` | C2C Markdown plus keyboard template payloads |
| `c2c_manage_event` | `c2c/manage_event.rs` | Friend add/remove and C2C active-message toggles |

C2C examples also use `with_public_messages()`. The target openid comes from `message.author.user_openid`.

## Direct Message Examples

| Example | Source | Shows |
|---|---|---|
| `direct_reply` | `direct/reply.rs` | Direct message replies and creating a direct-message session |
| `direct_reply_rich` | `direct/reply_rich.rs` | Direct Markdown, keyboard, ARK, and Embed replies |

Direct-message receive handlers use `Intents::new().with_direct_message()`. If an example creates a DM session from a guild channel command, it also enables `with_public_guild_messages()`.

## Event Examples

| Example | Source | Shows |
|---|---|---|
| `event_guild_member` | `events/guild_member.rs` | Guild member add, update, and remove |
| `event_open_forum` | `events/open_forum.rs` | Open forum thread, post, and reply events |
| `event_audio_or_live_channel_member` | `events/audio_or_live_channel_member.rs` | Audio/live channel member enter and exit |
| `event_interaction_search` | `events/interaction_search.rs` | Inline search interaction responses |

These examples focus on gateway event parsing and the corresponding `EventHandler` callback names.

## API Example

| Example | Source | Shows |
|---|---|---|
| `api_message_params` | `api/message_params.rs` | `MessageParams`, `GroupMessageParams`, `C2CMessageParams`, and `DirectMessageParams` in one process |

Use `/params` commands in the relevant scene to trigger the API example.

## Message Type Notes

Open-platform group and C2C messages use numeric `msg_type` values:

| `msg_type` | Meaning |
|---|---|
| `0` | Plain text |
| `2` | Markdown |
| `3` | ARK |
| `4` | Embed |
| `7` | Rich media returned by file upload |

Guild channel and direct-message params use `MessageCreateType` for explicit rich types when needed. Plain text can usually omit the type and set only `content`.

## Troubleshooting

- Invalid token: check `app_id`, `secret`, and sandbox setting.
- No events received: check that the example enabled the intent matching the scene you are testing.
- Message send failure: verify bot permissions, target IDs, message type, and platform-side template IDs such as keyboard id `62`.
- File examples: replace the placeholder `file_url` with a reachable URL before running.

# API client

`BotApi` is the REST client used by the current bot runtime path. After a gateway event arrives, handlers usually reach it through the session object to reply, recall messages, upload group/C2C files, manage guild/channel resources, announcements, schedules, pins, reactions, permissions, roles, mute state, and audio controls.

## In a Handler

Reply sessions provide scene-aware send helpers:

```rust
let params = MessageParams::new_text("hi");
session.send_message(params).await?;
```

All sessions also dereference to `BotApi`, so regular REST methods are available directly on `session`:

```rust
let pins = session.get_pins(channel_id).await?;
```

No token is passed in this path. `Client` creates the API client during startup; sessions expose that shared client to handlers through `session.api()` and `session.api_handle()`.

Common calls include:

- Messages: `send_message`, `send_group_message`, `send_c2c_message`, `send_direct_message`, `recall_message`
- Direct-message session: `create_direct_message`
- Guilds/channels/members/roles: `get_guild`, `list_channels`, `create_channel`, `get_guild_member`, `list_roles`
- Files: `post_group_file`, `post_c2c_file`
- Reactions: `put_reaction`, `delete_reaction`, `get_reaction_users`
- Pins: `put_pin`, `delete_pin`, `clean_pins`, `get_pins`
- Announcements: `create_channel_announce`, `delete_channel_announce`, `create_announce`, `create_recommend_announce`, `delete_announce`
- Schedules: `get_schedules`, `get_schedule`, `create_schedule`, `update_schedule`, `delete_schedule`
- Permissions and audio: `get_api_permissions`, `post_permission_demand`, `get_channel_permissions`, `post_audio`, `put_mic`, `delete_mic`

## Standalone Use

If you want a REST-only tool without running the gateway, construct `BotApi` manually:

```rust
let http = HttpClient::new(30, false)?;
let token = Token::from_env()?;
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
let gateway = api.get_gateway().await?;
```

`BotApi` is cloneable. Clones still share the HTTP client and token cache.

## Parameter Structs

Message sending uses parameter structs:

```rust
session.send_message(MessageParams::new_text("channel")).await?;
group_session.send_message(GroupMessageParams::new_text("group")).await?;
c2c_session.send_message(C2CMessageParams::new_text("c2c")).await?;
```

Common rich payloads use constructors such as `new_markdown`, `new_embed`, `new_ark`, `new_keyboard`, and group/C2C `new_media`. Inside reply sessions, prefer the matching `send_*_message` helper. Set params fields directly only for custom protocol combinations not covered by those helpers.

## Errors

All REST methods return `botrs::Result<T>`. In handlers, errors are usually handled locally:

```rust
if let Err(err) = session.send_message(params).await {
    tracing::warn!("send failed: {err}");
}
```

Match on `BotError` when you need variant-specific behavior.

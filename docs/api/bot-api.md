# BotApi

`BotApi` is the synchronous-style facade over the QQ Bot Open API. It owns the HTTP client, builds requests, signs them with a `Token`, and returns parsed model types. Every method is `async` and returns `Result<T, BotError>`.

## Construction

```rust
use botrs::{BotApi, http::HttpClient, Token};

let http = HttpClient::new(/* timeout secs */ 30, /* sandbox */ false)?;
let token = Token::new("app_id", "secret");
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
```

`BotApi` is `Clone` and cheap to clone; the inner HTTP client is reference-counted. For a bot client driven by `Client`, `Context` dereferences to the same `BotApi`, so handler code can call `ctx.send_message(...)` directly.

## Method index

Every method takes `&self` plus endpoint-specific arguments and returns `Result<…>`. The token is stored on `BotApi`. The lists below group the 100+ routes by domain. Look up parameter and response types in [Models](./models/messages.md), [Guilds & Channels](./models/guilds-channels.md), and [Other Types](./models/other-types.md).

### Bot identity

- `get_bot_info` — `/users/@me`, returns `BotInfo`.
- `get_gateway` — gateway URL + recommended shard count.

### Guilds

- `get_guild` / `get_guilds` / `get_guilds_with_pager`
- guild members: `get_guild_member`, `get_guild_members`, `get_guild_members_with_pager`
- role members: `get_guild_role_members`, `get_guild_role_members_with_pager`
- mute: `mute_all`, `cancel_mute_all`, `mute_member`, `mute_multi_member`, `multi_member_mute`, `cancel_mute_multi_member`

### Channels

- `get_channel`, `get_channels`
- `create_channel`, `create_private_channel`, `update_channel` / `patch_channel`, `delete_channel`
- permissions: `get_channel_user_permissions`, `get_channel_role_permissions`, `update_channel_user_permissions`, `update_channel_role_permissions`, `put_channel_permissions`, `put_channel_roles_permissions`

### Roles

- `get_guild_roles`
- `create_guild_role`, `create_guild_role_with_update`, `update_guild_role`, `update_guild_role_with_update`, `delete_guild_role`
- assign / remove: `create_guild_role_member`, `delete_guild_role_member`, `member_add_role`, `member_delete_role`, `delete_member`, `delete_member_with_options`

### Messages (channel)

- `get_message`, `get_messages`
- send: `send_message`.
- edit: `edit_message`.
- recall: `recall_message`.

### Direct messages

- create session: `create_direct_message`.
- send: `send_direct_message`, `post_direct_message`.
- recall: `retract_dm_message`.
- setting guide: `post_dm_setting_guide`, `post_dm_setting_guide_message`.

### Group / C2C messages

- send: `send_group_message`, `send_c2c_message`, `post_group_message_to_create`, `post_c2c_message_to_create`, `post_group_api_message`, `post_c2c_api_message`, `post_group_rich_media_message`, `post_c2c_rich_media_message`.
- recall: `retract_group_message`, `retract_c2c_message`.
- file upload: `post_group_file`, `post_c2c_file`.

### Reactions

- `put_reaction`, `delete_reaction`, `delete_own_message_reaction`
- `create_message_reaction`, `get_reaction_users`, `get_message_reaction_users`

### Pins

- `put_pin`, `delete_pin`, `get_pins`, `clean_pins`

### Announces

- guild: `create_guild_announce`, `delete_guild_announce`, `clean_guild_announces`, `create_guild_recommend_announce`, `create_recommend_announce`
- channel: `create_channel_announce`, `delete_channel_announce`, `clean_channel_announces`
- shorthand: `create_announce`, `delete_announce`

### Schedules

- `get_schedules`, `get_schedule`, `create_schedule`, `create_schedule_with_model`, `update_schedule`, `update_schedule_with_model`, `delete_schedule`

### API permissions

- `get_api_permissions`, `post_permission_demand`, `require_api_permissions`

### Audio / voice

- `post_audio` (uses `AudioControl`), `update_audio`
- `on_microphone`, `off_microphone`, `list_voice_channel_members`

### Setting guide

- `post_setting_guide`, `post_setting_guide_message`

### Interaction

- `put_interaction` — acknowledges a button/interaction event.

### Webhook sessions

- `create_session`, `check_sessions`, `session_list`, `remove_session`, `transport`

### Message setting

- `get_message_setting` — guild push and DM toggles.

### Lifecycle

- `close` — drains in-flight requests and shuts down the client.
- `http` — borrow the underlying `HttpClient` for advanced use.

## Worked examples

**Reply to an @-mention with a keyboard.** Build the keyboard once, attach to `MessageParams`, and dispatch with `send_message`.

```rust
let keyboard = Keyboard {
    content: Some(KeyboardContent {
        rows: Some(vec![KeyboardRow {
            buttons: Some(vec![KeyboardButton {
                id: Some("ok".into()),
                render_data: Some(KeyboardButtonRenderData {
                    label: Some("OK".into()),
                    style: Some(1),
                    ..Default::default()
                }),
                action: Some(KeyboardButtonAction {
                    action_type: Some(ACTION_TYPE_CALLBACK),
                    permission: Some(KeyboardButtonPermission {
                        permission_type: Some(PERMISSION_TYPE_ALL),
                        ..Default::default()
                    }),
                    data: Some("ok".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
        }]),
        ..Default::default()
    }),
    ..Default::default()
};

let mut params = MessageParams::new_text("Choose:")
    .with_reply(message.id.as_deref().unwrap_or(""));
params.keyboard = Some(keyboard);

api.send_message(&channel_id, params).await?;
```

**Paginated member listing.** Use the pager helper to avoid manually threading `after`.

```rust
let pager = ctx
    .get_guild_members_with_pager(&guild_id, &GuildMembersPager::default())
    .await?;
for member in pager.items {
    /* ... */
}
```

**Update channel permissions safely.** The `validate()` helper rejects non-numeric strings before they reach the server.

```rust
let body = UpdateChannelPermissions::new(Some("1024"), Some("0"));
body.validate()?;
api.update_channel_user_permissions(&channel_id, &user_id, &body).await?;
```

## Error handling

Every call returns `Result<T, BotError>`. Match on `BotError` to distinguish:

- `BotError::Http` — transport-level failures (timeout, DNS).
- `BotError::Api { code, message, .. }` — non-2xx response with the QQ-defined error code.
- `BotError::Auth` — token signing or refresh failure.
- `BotError::InvalidData` — local validation failure (e.g. malformed permission string).

For 429 responses, `BotError::Api` carries the `Retry-After` hint when present; the framework does not retry automatically — wrap calls with your own backoff if you need that behavior.

## See also

- [Client](./client.md) — high-level bot loop that owns a `BotApi`.
- [Context](./context.md) — request-scoped wrapper exposing the same routes during event handling.
- [Models](./models/messages.md) — request and response struct definitions.
- [Token](./token.md) — credential management and refresh.

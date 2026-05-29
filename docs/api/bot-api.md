# BotApi

`BotApi` is the shared typed REST client used by a bot while handling events. After a gateway event arrives, handlers usually call it through the event session to send replies, recall messages, upload group/C2C files, manage guild/channel resources, announcements, schedules, pins, reactions, permissions, roles, mute state, or audio controls.

When your bot is driven by `Client`, you normally do not construct `BotApi` yourself. Every event callback receives a session that exposes the shared API client:

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
session.send_message(params).await?;
```

Construct `BotApi` manually only when you want to call REST without running the gateway:

```rust
let http = HttpClient::new(30, false)?;
let token = Token::new("app_id", "secret");
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
```

## Current Surface

`BotApi` covers these paths:

| Area | Methods |
| --- | --- |
| Bot identity and gateway discovery | `get_bot_info`, `get_gateway` |
| Guilds and channels | `get_guild`, `list_bot_guilds`, `get_channel`, `list_channels`, `create_channel`, `update_channel`, `delete_channel`, `create_private_channel` |
| Members, roles, and mute | `get_guild_member`, `list_guild_members`, `delete_guild_member`, `list_roles`, `create_role`, `update_role`, `delete_role`, `add_member_role`, `delete_member_role`, `mute_guild`, `mute_member`, `mute_members` |
| Channel permissions and voice members | `get_channel_permissions`, `update_channel_permissions`, `get_channel_role_permissions`, `update_channel_role_permissions`, `list_voice_channel_members` |
| Guild channel messages | `get_message`, `list_messages`, `send_message`, `update_message`, `recall_message`, `send_setting_guide` |
| Group and C2C messages | `send_group_message`, `send_c2c_message`, `recall_group_message`, `recall_c2c_message` |
| Direct messages | `create_direct_message`, `send_direct_message`, `recall_direct_message`, `send_direct_setting_guide` |
| Group/C2C files | `post_group_file`, `post_c2c_file` |
| Announcements | `create_channel_announce`, `delete_channel_announce`, `create_announce`, `create_recommend_announce`, `delete_announce` |
| Schedules | `get_schedules`, `get_schedule`, `create_schedule`, `update_schedule`, `delete_schedule` |
| API permissions | `get_api_permissions`, `post_permission_demand` |
| Reactions | `put_reaction`, `delete_reaction`, `get_reaction_users` |
| Pins | `put_pin`, `delete_pin`, `clean_pins`, `get_pins` |
| Audio controls | `post_audio`, `put_mic`, `delete_mic` |

## Sending Messages

Message sending uses parameter structs:

```rust
let params = MessageParams::new_text("hello");
session.send_message(params).await?;

let params = GroupMessageParams::new_text("hello group");
group_session.send_message(params).await?;
```

Guild channel messages and DMs use `MessageParams` / `DirectMessageParams`. Group and C2C messages use `GroupMessageParams` / `C2CMessageParams`, matching QQ's open-message shape.

Inside event handlers, prefer the session helpers when they fit: `send_text_message`, `send_markdown_message`, `send_ark_message`, `send_embed_message`, `send_keyboard_message`, and group/C2C `send_media_message`. For lower-level API calls, use params constructors such as `new_markdown`, `new_ark`, `new_embed`, `new_keyboard`, and `new_media`; they fill the protocol message type for you.

## Files and Media

Group and C2C media sending is a two-step flow: upload the file to receive a `Media`, then place that media object into a message parameter struct. `file_type` follows the platform values: commonly 1 image, 2 video, 3 audio, 4 file.

```rust
let media = session
    .post_file(1, "https://example.com/image.png", None)
    .await?;

session.send_media_message(media).await?;
```

When `srv_send_msg` is `Some(true)`, the platform sends the uploaded file directly, so you usually do not need a separate media message.

## Announcements, Schedules, Pins, Permissions

These APIs are direct wrappers around their protocol payloads:

- Announcements can be created from an existing message or from a list of recommended channels.
- Schedules support listing, fetching one item, creating, updating, and deleting.
- Pins support pinning one message, unpinning one message, clearing all pins, and listing pinned message ids.
- API permission requests take `channel_id`, `APIPermissionDemandIdentify`, and a description.

```rust
let identify = APIPermissionDemandIdentify {
    path: "/channels/{channel_id}/messages".to_string(),
    method: "POST".to_string(),
};

session.post_permission_demand(&guild_id, &channel_id, identify, "Need to send replies")
    .await?;
```

## Errors

All methods return `botrs::Result<T>`. In event handlers, handle errors locally because `EventHandler` methods return `()`:

```rust
if let Err(err) = session.send_message(params).await {
    tracing::warn!("send failed: {err}");
}
```

## See Also

- [Sessions](./context.md)
- [Messages](./models/messages.md)
- [Other types](./models/other-types.md)

# BotApi

`BotApi` is the stateless REST client used by a bot while handling events. After a gateway event arrives, handlers usually call it through the event session to send replies, recall channel messages, upload group/C2C files, manage announcements, schedules and pins, inspect reaction users, or create API permission requests.

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
| Guild channel messages | `send_message`, `recall_message` |
| Group and C2C messages | `send_group_message`, `send_c2c_message` |
| Direct messages | `create_direct_message`, `send_direct_message` |
| Group/C2C files | `post_group_file`, `post_c2c_file` |
| Announcements | `create_announce`, `create_recommend_announce`, `delete_announce` |
| Schedules | `get_schedules`, `get_schedule`, `create_schedule`, `update_schedule`, `delete_schedule` |
| API permissions | `get_api_permissions`, `post_permission_demand` |
| Reactions | `put_reaction`, `delete_reaction`, `get_reaction_users` |
| Pins | `put_pin`, `delete_pin`, `get_pins` |

## Sending Messages

Message sending uses parameter structs:

```rust
let params = MessageParams::new_text("hello");
session.send_message(params).await?;

let params = GroupMessageParams::new_text("hello group");
group_session.send_message(params).await?;
```

Guild channel messages and DMs use `MessageParams` / `DirectMessageParams`. Group and C2C messages use `GroupMessageParams` / `C2CMessageParams`, matching QQ's open-message shape. For ark, embed, markdown, keyboard, or media payloads, set the corresponding field on the parameter struct.

## Files and Media

Group and C2C media sending is a two-step flow: upload the file to receive a `Media`, then place that media object into a message parameter struct. `file_type` follows the platform values: commonly 1 image, 2 video, 3 audio, 4 file.

```rust
let media = session
    .post_file(1, "https://example.com/image.png", None)
    .await?;

let mut params = GroupMessageParams::default();
params.msg_type = 7;
params.media = Some(media);
session.send_message(params).await?;
```

When `srv_send_msg` is `Some(true)`, the platform sends the uploaded file directly, so you usually do not need a separate media message.

## Announcements, Schedules, Pins, Permissions

These APIs are direct wrappers around their protocol payloads:

- Announcements can be created from an existing message or from a list of recommended channels.
- Schedules support listing, fetching one item, creating, updating, and deleting.
- Pins support pinning one message, unpinning one message, and listing pinned message ids.
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

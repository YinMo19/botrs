# Message API

The message API uses typed parameter structs. Each send surface has one parameter type and one send method:

| Surface | Parameter type | Send method |
| --- | --- | --- |
| Guild channel | `MessageParams` | `send_message` |
| Group | `GroupMessageParams` | `send_group_message` |
| C2C | `C2CMessageParams` | `send_c2c_message` |
| Direct message | `DirectMessageParams` | `send_direct_message` |

All parameter structs provide `new_text(content)` and `with_reply(message_id)`. Rich payloads are expressed by setting fields on the struct.

```rust
let params = MessageParams::new_text("Hello!").with_reply(&message_id);
api.send_message("channel_id", params).await?;
```

## Field-based payloads

Use struct fields for content that is not plain text:

```rust
let params = MessageParams {
    content: Some("with embed".into()),
    embed: Some(my_embed),
    markdown: Some(my_markdown),
    keyboard: Some(my_keyboard),
    ..Default::default()
};
api.send_message(channel_id, params).await?;
```

This keeps the call site explicit: every optional protocol field has a name, and `..Default::default()` leaves unrelated fields absent from the outgoing JSON.

## Choosing the right parameter type

Use the event payload to choose the ID and parameter type:

- `Message` carries `channel_id`; send with `MessageParams`.
- `GroupMessage` carries `group_openid`; send with `GroupMessageParams`.
- `C2CMessage` carries `author.user_openid`; send with `C2CMessageParams`.
- `DirectMessage` is the DM session returned by `create_direct_message`; send with `DirectMessageParams` and the session `guild_id`.

For plain-text replies, prefer the event model's `reply` method. For richer replies, build the matching params value and call the corresponding `BotApi` method.

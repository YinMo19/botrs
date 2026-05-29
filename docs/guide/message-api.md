# Message API

The message API uses typed parameter structs. Each send surface has one parameter type and one send method:

| Surface | Parameter type | Send method |
| --- | --- | --- |
| Guild channel | `MessageParams` | `send_message` |
| Group | `GroupMessageParams` | `send_group_message` |
| C2C | `C2CMessageParams` | `send_c2c_message` |
| Direct message | `DirectMessageParams` | `send_direct_message` |

All parameter structs provide `new_text(content)`, `new_markdown(content)`, `new_ark(...)`, `new_embed(...)`, `new_keyboard(...)`, and `with_reply(message_id)`. Group and C2C params also provide `new_media(...)`.

```rust
let params = MessageParams::new_text("Hello!").with_reply(&message_id);
api.send_message("channel_id", params).await?;
```

## Rich Payloads

Use constructors for common rich payloads:

```rust
let params = MessageParams::new_embed(my_embed);
api.send_message(channel_id, params).await?;
```

For custom combinations, start from the closest constructor and then set extra fields directly with `..Default::default()` so unrelated protocol fields stay absent from the outgoing JSON.

## Choosing the right parameter type

Use the event payload to choose the ID and parameter type:

- `Message` carries `channel_id`; send with `MessageParams`.
- `GroupMessage` carries `group_openid`; send with `GroupMessageParams`.
- `C2CMessage` carries `author.user_openid`; send with `C2CMessageParams`.
- `DirectMessage` is the DM session returned by `create_direct_message`; send with `DirectMessageParams` and the session `guild_id`.

Inside event handlers, prefer the session helpers: `reply`, `send_markdown_message`, `send_embed_message`, `send_ark_message`, `send_keyboard_message`, and group/C2C `send_media_message`. For standalone `BotApi` calls, build the matching params value and call the corresponding send method.

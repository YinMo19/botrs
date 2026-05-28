# Messages

Message models are split into two groups: inbound gateway payloads and outbound parameter structs. The current implementation focuses on those two paths: events deserialize into usable payloads, and handlers can reply through the framework API.

## Inbound Messages

Guild channel `@bot` messages use `Message`, delivered to `EventHandler::message_create`. Direct-message events also use `Message`, delivered to `direct_message_create`; fields such as `direct_message` and `src_guild_id` identify the DM scene.

Group and C2C messages have their own models:

- `GroupMessage` maps to `GROUP_AT_MESSAGE_CREATE`; the key routing field is `group_openid`.
- `C2CMessage` maps to `C2C_MESSAGE_CREATE`; the key routing field usually comes from `author.user_openid`.

These event models retain platform-provided message id, content, attachments, mentions, timestamp, references, and internal `event_id`. For plain replies, prefer the model's `reply` method. For richer replies, build the matching parameter struct manually.

```rust
if let Some(content) = &message.content {
    if content.trim() == "/ping" {
        message.reply(ctx.api(), "pong").await?;
    }
}
```

## Sending Messages

Outbound sending uses four parameter types:

| Parameter type | Use |
| --- | --- |
| `MessageParams` | Guild channel messages |
| `DirectMessageParams` | Direct-message session messages |
| `GroupMessageParams` | Group messages |
| `C2CMessageParams` | C2C messages |

The most common helpers are `new_text` and `with_reply`:

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
ctx.send_message(&channel_id, params).await?;
```

For complex messages, set fields directly:

- `embed` for embed messages.
- `ark` for ark templates.
- `markdown` for markdown templates or markdown content.
- `keyboard` for button keyboards.
- `media` for media returned by group/C2C file upload.

## DirectMessage

`DirectMessage` is the session DTO returned after creating a direct-message session. The send flow is:

1. Create a session request with `DirectMessageToCreate::new(source_guild_id, recipient_id)`.
2. Call `create_direct_message` to receive `DirectMessage`.
3. Use the returned `guild_id` with `send_direct_message`.

```rust
let dm = DirectMessageToCreate::new(&guild_id, &user_id);
let session = ctx.create_direct_message(&dm).await?;

let params = DirectMessageParams::new_text("hello");
ctx.send_direct_message(&session.guild_id, params).await?;
```

## Open Message msg_type

Group and C2C messages use the platform's numeric `msg_type`. Text generally keeps the default value 0; media uses 7; markdown, ark, embed, and other types follow the platform's protocol values. Guild channel messages use the Rust-modeled `MessageCreateType`.

## See Also

- [BotApi](../bot-api.md)
- [Users and Members](./users-members.md)
- [Other types](./other-types.md)

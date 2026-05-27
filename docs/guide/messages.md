# Messages

Every send method on `BotApi` accepts a typed builder rather than a long list of `Option`s. There are four such builders, one per channel kind:

- `MessageParams` — guild channel messages.
- `GroupMessageParams` — group (`group_openid`) messages.
- `C2CMessageParams` — single-user C2C messages.
- `DirectMessageParams` — DM (private chat) messages.

All four expose `new_text(content)` for the common case, `with_reply(message_id)` to set the `msg_id` reference, and (on `MessageParams` / `DirectMessageParams`) `with_file_image(&bytes)` to attach raw image bytes — they are base64-encoded into the `file_image` field for you. Anything else (embed, ark, markdown, keyboard, media, prompt keyboard, action button, stream) is set with struct-update syntax.

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("hello").with_reply(&message_id);
ctx.send_message(&channel_id, params).await?;
```

For richer payloads, fill the params struct directly:

```rust
use botrs::models::message::{MessageParams, MarkdownPayload, Keyboard};

let params = MessageParams {
    content: Some("with markdown".into()),
    markdown: Some(my_markdown),
    keyboard: Some(my_keyboard),
    ..Default::default()
};
ctx.send_message(&channel_id, params).await?;
```

`with_file_image` takes `&[u8]`, not a URL, and is the right way to send bytes you already have in memory. To reference a remote image instead, set `params.image = Some(url.into())`.

## Sending in each context

The send call has a different name and a different ID parameter for each surface:

| Surface       | Builder              | Send method                          | Identifier         |
|---------------|----------------------|--------------------------------------|--------------------|
| Guild channel | `MessageParams`      | `send_message`           | `channel_id`       |
| Group         | `GroupMessageParams` | `send_group_message`     | `group_openid`     |
| C2C           | `C2CMessageParams`   | `send_c2c_message`       | `openid`           |
| DM            | `DirectMessageParams`| `send_direct_message`               | `guild_id` (DM guild) |

Editing a guild message uses `edit_message(channel_id, message_id, params)`.

## Replying from an event

`Message::reply(&api, content)` is the convenience for replying with plain text in the same channel as the inbound `Message`. The same shape exists on `GroupMessage`, `C2CMessage`, and `DirectMessage`. Internally these construct the matching `*Params` value with `with_reply` set to the inbound message id.

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.is_from_bot() { return; }
    if let Some("!ping") = message.content.as_deref() {
        let _ = message.reply(&ctx, "pong").await;
    }
}
```

For anything beyond plain text, build a `MessageParams` and call `ctx.send_message` with the `channel_id` from the incoming event.

## Recall and audit

- `BotApi::recall_message(channel_id, message_id, hidetip)` deletes a guild message.
- `retract_c2c_message`, `retract_group_message`, `retract_dm_message` are the recall variants for the other surfaces.
- Audit events are surfaced via `EventHandler::message_audit_pass` / `message_audit_reject` and carry a `MessageAudit` payload; you do not have to ack them.

## Removed Legacy API

The pre-0.2 multi-`Option` methods (`post_message`, `post_group_message`, `post_c2c_message`, `post_dms`) and the later `*_with_params` compatibility names have been removed. Use the short methods shown above. See [v0.2.0 migration](/guide/migration-v0.2.0) for the rewrite recipe.

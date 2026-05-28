# Messages

Every send method on `BotApi` accepts a typed builder rather than a long list of `Option`s. There are four such builders, one per channel kind:

- `MessageParams` — guild channel messages.
- `GroupMessageParams` — group (`group_openid`) messages.
- `C2CMessageParams` — single-user C2C messages.
- `DirectMessageParams` — DM (private chat) messages.

All four expose `new_text(content)` for the common case and `with_reply(message_id)` to set the `msg_id` reference. Anything else (embed, ark, markdown, keyboard, image URL, media, prompt keyboard, action button, stream) is set with struct-update syntax.

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

To include an image in a guild channel message, set `params.image = Some(url.into())` with a remote image URL.

## Sending in each context

The send call has a different name and a different ID parameter for each surface:

| Surface       | Builder              | Send method                          | Identifier         |
|---------------|----------------------|--------------------------------------|--------------------|
| Guild channel | `MessageParams`      | `send_message`           | `channel_id`       |
| Group         | `GroupMessageParams` | `send_group_message`     | `group_openid`     |
| C2C           | `C2CMessageParams`   | `send_c2c_message`       | `openid`           |
| DM            | `DirectMessageParams`| `send_direct_message`               | `guild_id` (DM guild) |

## Replying from an event

`message.reply(&ctx, content)` is the convenience for replying with plain text in the same channel as the inbound `Message`. `GroupMessage` and `C2CMessage` provide the same shape for their surfaces. Internally these construct the matching `*Params` value with the inbound message id and event id.

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
    if let Some("!ping") = message.content.as_deref() {
        let _ = message.reply(&ctx, "pong").await;
    }
}
```

For anything beyond plain text, build a `MessageParams` and call `ctx.send_message` with the `channel_id` from the incoming event.

## Recall and audit

- `BotApi::recall_message(channel_id, message_id, hidetip)` deletes a guild message.
- Audit events are surfaced via `EventHandler::message_audit_pass` / `message_audit_reject` and carry a `MessageAudit` payload; you do not have to ack them.

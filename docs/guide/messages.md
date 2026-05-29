# Messages

Every send method on `BotApi` accepts a typed builder rather than a long list of `Option`s. There are four such builders, one per channel kind:

- `MessageParams` — guild channel messages.
- `GroupMessageParams` — group (`group_openid`) messages.
- `C2CMessageParams` — single-user C2C messages.
- `DirectMessageParams` — DM (private chat) messages.

All four expose `new_text(content)`, `new_markdown(content)`, `new_ark(...)`, `new_embed(...)`, and `new_keyboard(...)` for common cases, plus `with_reply(message_id)` to set the `msg_id` reference. `GroupMessageParams` and `C2CMessageParams` also expose `new_media(...)`. More specialized protocol fields can still be set with struct-update syntax.

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("hello").with_reply(&message_id);
session.send_message(params).await?;
```

For common combinations, prefer constructors so the protocol message type is set correctly:

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_keyboard("with markdown", my_keyboard);
session.send_message(params).await?;
```

When a platform field is not covered by a constructor, start from the closest constructor and then override only the extra fields. To include an image in a guild channel message, set `params.image = Some(url.into())` with a remote image URL.

## Sending in each context

The send call has a different name and a different ID parameter for each surface:

| Surface       | Builder              | Send method                          | Identifier         |
|---------------|----------------------|--------------------------------------|--------------------|
| Guild channel | `MessageParams`      | `send_message`           | `channel_id`       |
| Group         | `GroupMessageParams` | `send_group_message`     | `group_openid`     |
| C2C           | `C2CMessageParams`   | `send_c2c_message`       | `openid`           |
| DM            | `DirectMessageParams`| `send_direct_message`               | `guild_id` (DM guild) |

## Replying from an event

`session.reply(content)` replies with plain text in the same event session. Reply sessions also provide `send_text_message`, `send_markdown_message`, `send_ark_message`, `send_embed_message`, `send_keyboard_message`, and, for group/C2C/manage sessions, `send_media_message`. They automatically attach the inbound message id, event id, and open-message `msg_seq` where the platform requires it.

```rust
async fn message_create(&self, mut session: ChannelReplySession) {
    let message = session.message().clone();
    if message.author.bot {
        return;
    }

    if message.content.trim() == "!ping" {
        let _ = session.reply("pong").await;
    }
}
```

```rust
session.send_markdown_message("# hello\n\n- one line").await?;
session.send_embed_message(embed).await?;
```

For template markdown or fields not covered by the helpers, build the matching params struct and call `session.send_message(params)`.

## Recall and audit

- `BotApi::recall_message(channel_id, message_id, hidetip)` deletes a guild message.
- Audit events are surfaced via `EventHandler::message_audit_pass` / `message_audit_reject` and carry a `MessageAudit` payload; you do not have to ack them.

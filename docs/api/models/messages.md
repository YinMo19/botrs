# Messages

Message models are split into two groups: inbound gateway payloads and outbound parameter structs. The current implementation focuses on those two paths: events deserialize into usable payloads, and handlers can reply through the framework API.

## Inbound Messages

Guild channel `@bot` messages use `Message`, delivered to `EventHandler::message_create`. Direct-message events also use `Message`, delivered to `direct_message_create`; fields such as `direct_message` and `src_guild_id` identify the DM scene.

Group and C2C messages have their own models:

- `GroupMessage` maps to both `GROUP_AT_MESSAGE_CREATE` and `GROUP_MESSAGE_CREATE`; the key routing field is `group_openid`.
- `C2CMessage` maps to `C2C_MESSAGE_CREATE`; the key routing field usually comes from `author.user_openid`.

These event models retain platform-provided message id, content, attachments, mentions, timestamp, references, and internal `event_id`. Message id, content, author, routing id, and timestamp are required fields on normal message events; group/C2C messages also carry the required open-message `message_type`. Optional protocol fields stay optional.

Attachments use `MessageAttachment`. When an attachment appears, the filename, content type, size, and URL are plain fields. `id` stays optional because open-message attachment payloads may omit it, while `width` and `height` default to `0` for non-image or payloads that do not include dimensions.

```rust
let message = session.message();
if !message.author.bot && message.content.trim() == "/ping" {
    session.reply("pong").await?;
}
```

## Group receive modes and proactive messages

Read the group's settings with `session.bot_state().await?` or `api.get_group_bot_state(group_openid).await?`. The returned `GroupBotState.recv_msg_setting` uses `botrs::models::group::GroupReceiveMode`.

| QQ group setting | Rust value | Delivery |
| --- | --- | --- |
| Mentions only | `OnlyMention` | Handle mentions in `group_message_create`. |
| Recent context on mention (up to 10 messages) | `MentionAndContext` | Handle the current mention and inspect the context QQ supplies. |
| All group messages | `All` | Handle ordinary messages in `group_full_message_create`. |

Both events use `Intents::default().with_public_messages()`. Match `Event::GroupMessageCreate(s) | Event::GroupFullMessageCreate(s)` to share logic. Read the current command from `s.message().content_without_bot_mention()`. Recursive `msg_elements` carry quoted messages and records. Additional top-level fields are available in `extra`. QQ has not documented a dedicated recent-history field. Element counts cannot identify the receive mode. The state endpoint requires an allowlisted application and may return platform code 11253.

The independent `allow_proactive_msg` flag controls proactive delivery. A scheduled job can call `api.send_group_message(group_openid, GroupMessageParams::new_text(text)).await?`. Use `session.reply(text).await?` to attach the current message ID and reply sequence.

## Sending Messages

Outbound sending uses four parameter types:

| Parameter type | Use |
| --- | --- |
| `MessageParams` | Guild channel messages |
| `DirectMessageParams` | Direct-message session messages |
| `GroupMessageParams` | Group messages |
| `C2CMessageParams` | C2C messages |

Use session helpers inside reply-capable handlers:

```rust
session.reply("pong").await?;
session.send_markdown_message("# hello").await?;
session.send_embed_message(embed).await?;
```

For direct `BotApi` calls or custom fields, use params constructors:

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
session.send_message(params).await?;
```

The rich constructors are `new_markdown`, `new_ark`, `new_embed`, `new_keyboard`, and, for group/C2C messages, `new_media`. Set fields directly only when you need a protocol field not covered by those constructors:

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
let dm_session = session.create_direct_message(&dm).await?;

let params = DirectMessageParams::new_text("hello");
session.send_direct_message(&dm_session.guild_id, params).await?;
```

## Open Message Types

Group and C2C messages still serialize to the platform's open-message type codes, but normal user code should use session helpers or params constructors instead of setting the numeric value by hand. Guild channel and direct-message params use `MessageCreateType` internally.

## See Also

- [BotApi](../bot-api.md)
- [Users and Members](./users-members.md)
- [Other types](./other-types.md)

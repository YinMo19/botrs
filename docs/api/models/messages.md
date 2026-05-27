# Messages

Message data structures for the QQ Guild Bot API. The framework distinguishes incoming events (gateway payloads) from outgoing parameter objects.

## Incoming messages

### `Message`

Guild channel messages, including @-mentions.

```rust
pub struct Message {
    pub id: Option<Snowflake>,
    pub content: Option<String>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub group_id: Option<Snowflake>,
    pub author: Option<MessageUser>,
    pub member: Option<MessageMember>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<MessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub embeds: Vec<Embed>,
    pub ark: Option<Ark>,
    pub direct_message: Option<bool>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub edited_timestamp: Option<Timestamp>,
    pub mention_everyone: Option<bool>,
    pub src_guild_id: Option<Snowflake>,
    pub file_info: Option<String>,
    pub ttl: Option<u32>,
    pub message_scene: Option<MessageScene>,
    pub event_id: Option<String>,
}
```

Most string fields use `Option<Snowflake>` because the QQ Open API may omit them on partial payloads. Convenience methods on `Message`:

- `reply(api, token, content)` — text reply that sets `msg_id` for passive routing.
- `is_from_bot()`, `has_content()`, `has_attachments()`, `has_mentions()` — quick predicates.

### `GroupMessage` and `C2CMessage`

Group messages (group `@bot`) and C2C (private/direct) messages share a similar shape but use OpenID-based identification:

```rust
pub struct GroupMessage {
    pub id: Option<Snowflake>,
    pub content: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<GroupMessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub msg_seq: Option<u64>,
    pub timestamp: Option<Timestamp>,
    pub author: Option<GroupMessageUser>,
    pub group_openid: Option<String>,
    pub event_id: Option<String>,
}
```

`C2CMessage` is structurally identical except its `author`/`mentions` use `C2CMessageUser`.

### `DirectMessage`

`DirectMessage` is the **session** record returned by the direct-message creation API — not the gateway message. Direct-message gateway events use `Message` with `direct_message: Some(true)`.

```rust
pub struct DirectMessage {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub create_time: String,
}
```

## Outgoing parameters

Send-side payloads use builder structs to avoid the long `Option<...>` argument lists of older APIs:

| Struct                  | Used by                                    |
|-------------------------|--------------------------------------------|
| `MessageParams`         | `BotApi::send_message`         |
| `DirectMessageParams`   | `BotApi::send_direct_message`  |
| `GroupMessageParams`    | `BotApi::send_group_message`   |
| `C2CMessageParams`      | `BotApi::send_c2c_message`     |

Each exposes `new_text(content)` and reply/file helpers where they apply. For embed, markdown, ark, keyboard, media, and other optional payloads, set the corresponding field on the params struct and pass it to the matching send method.

```rust
let params = MessageParams::new_text("Pong!").with_reply(message_id);
api.send_message(&channel_id, params).await?;
```

## Supporting types

| Type                | Purpose                                              |
|---------------------|------------------------------------------------------|
| `Embed`             | Embed body (title, description, fields, thumbnail).  |
| `Ark`               | Ark template payload (`template_id` + `kv` array).   |
| `MarkdownPayload`   | Markdown send body (template id, content, params).   |
| `Keyboard`          | Inline keyboard (rows of buttons).                   |
| `MessageReference`  | `{ message_id, ignore_get_message_error }`.          |
| `MessageAttachment` | File attachment metadata (URL, filename, size, …).   |
| `MessageAudit`      | Audit-result payload from the audit gateway events.  |
| `MessageDelete`     | Delete-event payload with `message` and `op_user`.   |
| `MessageScene`      | Scene marker (group, c2c, channel) on inbound events.|

## Message author types

See [Users and Members](./users-members.md#message-author-types) for `MessageUser`, `DirectMessageUser`, `GroupMessageUser`, and `C2CMessageUser`.

## See also

- [Messages guide](../../guide/messages.md) — task-oriented usage.
- [Bot API](../bot-api.md) — every message route.
- [Other types](./other-types.md) — embeds, keyboards, ark, attachments.

# Text Messages

Plain text replies cover four destinations in BotRS, each backed by its own `*Params` struct and send API call. The destination examples:

- Channel @-mentions: [`examples/guild/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_text.rs)
- Channel reply with quoted reference: [`examples/guild/reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_reference.rs)
- Group messages (QQ groups): [`examples/group/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_text.rs)
- C2C (single-user) messages: [`examples/c2c/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_text.rs)
- Direct messages inside guilds: [`examples/direct/reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply.rs)

## Picking the right call

For a guild channel reply, `Message::reply` is the shortest path. For everything else (group, C2C, DMS, or replies that need a `Reference`, `msg_id`, `event_id`, file, embed, …) build the matching `*Params` and call the corresponding `BotApi` method:

| Destination       | Params                | API method                          |
|-------------------|----------------------|-------------------------------------|
| Channel           | `MessageParams`       | `send_message`          |
| Direct message    | `DirectMessageParams` | `send_direct_message`              |
| Group             | `GroupMessageParams`  | `send_group_message`    |
| C2C               | `C2CMessageParams`    | `send_c2c_message`      |

```rust
// Quoted reply (channel) — see examples/guild/reply_reference.rs
let params = MessageParams {
    content: Some("<emoji:4>这是一条引用消息".to_string()),
    message_reference: Some(Reference { message_id: Some(message_id.clone()), ignore_get_message_error: None }),
    ..Default::default()
};
ctx.send_message(channel_id, params).await?;
```

For groups and C2C, set `msg_type: 0` for plain text and pass `msg_id: message.id.clone()` plus `event_id: message.event_id.clone()` when those fields are present. Direct messages need the DM `guild_id` from the inbound `Message` or a session created with `BotApi::create_direct_message`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Builder helpers: `MessageParams::new_text(...).with_reply(...)`, same on `Group/C2C/DirectMessageParams`
- Source files listed above under `examples/`

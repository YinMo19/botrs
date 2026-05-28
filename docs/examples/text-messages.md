# Text Messages

Plain text replies cover four destinations in BotRS, each backed by its own `*Params` struct and send API call. The destination examples:

- Channel @-mentions: [`examples/guild/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_text.rs)
- Channel reply with quoted reference: [`examples/guild/reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_reference.rs)
- Group messages (QQ groups): [`examples/group/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_text.rs)
- C2C (single-user) messages: [`examples/c2c/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_text.rs)
- Direct messages inside guilds: [`examples/direct/reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply.rs)

## Picking the right call

For a plain reply, `session.reply(...)` is the shortest path. For richer payloads (a `Reference`, explicit ids, file, embed, etc.), build the matching `*Params` and call the current session's send method:

| Destination       | Params                | API method                          |
|-------------------|----------------------|-------------------------------------|
| Channel           | `MessageParams`       | `ChannelReplySession::send_message` |
| Direct message    | `DirectMessageParams` | `DirectReplySession::send_message`  |
| Group             | `GroupMessageParams`  | `GroupReplySession::send_message`   |
| C2C               | `C2CMessageParams`    | `C2CReplySession::send_message`     |

```rust
// Quoted reply (channel) — see examples/guild/reply_reference.rs
let params = MessageParams {
    content: Some("<emoji:4>这是一条引用消息".to_string()),
    message_reference: Some(Reference { message_id: Some(message_id.clone()), ignore_get_message_error: None }),
    ..Default::default()
};
session.send_message(params).await?;
```

Reply sessions fill `msg_id`, `event_id`, and `msg_seq` automatically when you leave them unset. Direct messages need the DM `guild_id` from the inbound `Message` or a session created with `BotApi::create_direct_message`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Builder helpers: `MessageParams::new_text(...).with_reply(...)`, same on `Group/C2C/DirectMessageParams`
- Source files listed above under `examples/`

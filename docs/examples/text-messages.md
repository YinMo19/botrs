# Text Messages

Plain text replies cover four destinations in BotRS, each backed by its own `*Params` struct and `post_*_with_params` API call. The destination demos:

- Channel @-mentions: [`demo_at_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply.rs)
- Channel reply with quoted reference: [`demo_at_reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_reference.rs)
- Group messages (QQ groups): [`demo_group_reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_group_reply_text.rs)
- C2C (single-user) messages: [`demo_c2c_reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_c2c_reply_text.rs)
- Direct messages inside guilds: [`demo_dms_reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_dms_reply.rs)

## Picking the right call

For a guild channel reply, `Message::reply` is the shortest path. For everything else (group, C2C, DMS, or replies that need a `Reference`, `msg_id`, `event_id`, file, embed, …) build the matching `*Params` and call the corresponding `BotApi` method:

| Destination       | Params                | API method                          |
|-------------------|----------------------|-------------------------------------|
| Channel           | `MessageParams`       | `post_message_with_params`          |
| Direct message    | `DirectMessageParams` | `post_dms_with_params`              |
| Group             | `GroupMessageParams`  | `post_group_message_with_params`    |
| C2C               | `C2CMessageParams`    | `post_c2c_message_with_params`      |

```rust
// Quoted reply (channel) — see demo_at_reply_reference.rs
let params = MessageParams {
    content: Some("<emoji:4>这是一条引用消息".to_string()),
    message_reference: Some(Reference { message_id: Some(message_id.clone()), ignore_get_message_error: None }),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
```

For groups and C2C, remember to set `msg_type: 0` for plain text and pass `msg_id: message.id.clone()` so the platform threads the reply correctly (see `demo_group_reply_text.rs` and `demo_c2c_reply_text.rs`). DMS additionally needs `guild_id` from the inbound `Message` or a session created with `BotApi::create_direct_message`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Builder helpers: `MessageParams::new_text(...).with_reply(...)`, same on `Group/C2C/DirectMessageParams`
- Source files listed above under `examples/`

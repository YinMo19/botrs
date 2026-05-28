# Event Handling

Each gateway event is one method on `EventHandler`. You implement only the ones you care about; everything else has an empty default. Examples covering the non-message event surface:

- Guild member join/update/leave: [`examples/events/guild_member.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/guild_member.rs) (`guild_member_add`, `guild_member_update`, `guild_member_remove`)
- Open forum threads/posts/replies: [`examples/events/open_forum.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/open_forum.rs) (`open_forum_thread_create`, `open_forum_post_create`, `open_forum_reply_create`, ...)
- Voice / live channel enter/exit: [`examples/events/audio_or_live_channel_member.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/audio_or_live_channel_member.rs) (`audio_or_live_channel_member_enter` / `_exit`)
- C2C friend add/remove + active-message toggle: [`examples/c2c/manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/manage_event.rs) (`friend_add`, `friend_del`, `c2c_msg_reject`, `c2c_msg_receive`)
- Group robot add/remove + active-message toggle: [`examples/group/manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/manage_event.rs) (`group_add_robot`, `group_del_robot`, `group_msg_reject`, `group_msg_receive`)

## Wiring

Each event family has its own intent flag. Forgetting the flag means the handler is silent, not an error. Mix the flags you need:

```rust
let intents = Intents::new()
    .with_guild_members()                 // examples/events/guild_member.rs
    .with_open_forum_event()              // examples/events/open_forum.rs
    .with_audio_or_live_channel_member()  // examples/events/audio_or_live_channel_member.rs
    .with_public_messages();              // c2c/group manage events
```

Inside the handler, the `Context` argument dereferences to `BotApi`, so you can react by calling APIs directly on `ctx` (e.g. `ctx.create_direct_message` + `ctx.send_direct_message` to greet a new member, or `ctx.send_group_message` with `event_id: event.event_id.clone()` to acknowledge a robot-add event).

## See also

- Guide: [`docs/guide/intents.md`](../guide/intents.md), [`docs/guide/client-handler.md`](../guide/client-handler.md)
- Full list of `EventHandler` methods: `src/client.rs`
- Examples listed above under `examples/`

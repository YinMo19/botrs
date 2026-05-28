# 事件处理

每个网关事件对应 `EventHandler` 上的一个方法。只实现你关心的方法，其余有空默认实现。覆盖非消息事件的示例：

- 频道成员加入/更新/离开：[`examples/events/guild_member.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/guild_member.rs)（`guild_member_add`、`guild_member_update`、`guild_member_remove`）
- 开放论坛主题/帖子/回复：[`examples/events/open_forum.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/open_forum.rs)（`open_forum_thread_create`、`open_forum_post_create`、`open_forum_reply_create` ...）
- 语音/直播子频道进出：[`examples/events/audio_or_live_channel_member.rs`](https://github.com/YinMo19/botrs/blob/main/examples/events/audio_or_live_channel_member.rs)（`audio_or_live_channel_member_enter` / `_exit`）
- C2C 好友添加/删除及主动消息开关：[`examples/c2c/manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/manage_event.rs)（`friend_add`、`friend_del`、`c2c_msg_reject`、`c2c_msg_receive`）
- 群机器人添加/移除及主动消息开关：[`examples/group/manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/manage_event.rs)（`group_add_robot`、`group_del_robot`、`group_msg_reject`、`group_msg_receive`）

## 接线

每个事件族都有专属 intent 标志，没开标志不会报错，只是 handler 拿不到事件。按需组合：

```rust
let intents = Intents::new()
    .with_guild_members()                 // examples/events/guild_member.rs
    .with_open_forum_event()              // examples/events/open_forum.rs
    .with_audio_or_live_channel_member()  // examples/events/audio_or_live_channel_member.rs
    .with_public_messages();              // c2c/group 管理事件
```

handler 内部，`Context` 会解引用到 `BotApi`，可以直接在 `ctx` 上回调 API（例如 `ctx.create_direct_message` + `ctx.send_direct_message` 欢迎新成员；或 `ctx.send_group_message` 配合 `event_id: event.event_id.clone()` 来回应机器人入群事件）。

## 参见

- 指南：[`docs/zh/guide/intents.md`](../guide/intents.md)、[`docs/zh/guide/client-handler.md`](../guide/client-handler.md)
- `EventHandler` 全部方法：`src/client.rs`
- 上面列出的 `examples/` 中的示例

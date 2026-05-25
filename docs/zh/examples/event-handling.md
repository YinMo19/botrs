# 事件处理

每个网关事件对应 `EventHandler` 上的一个方法。只实现你关心的方法，其余有空默认实现。覆盖非消息事件的 demo：

- 频道成员加入/更新/离开：[`demo_guild_member_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_guild_member_event.rs)（`guild_member_add`、`guild_member_update`、`guild_member_remove`）
- 开放论坛主题/帖子/回复：[`demo_open_forum_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_open_forum_event.rs)（`open_forum_thread_create`、`open_forum_post_create`、`open_forum_reply_create` …）
- 语音/直播子频道进出：[`demo_audio_or_live_channel_member.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_audio_or_live_channel_member.rs)（`audio_or_live_channel_member_enter` / `_exit`）
- C2C 好友添加/删除及主动消息开关：[`demo_c2c_manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_c2c_manage_event.rs)（`friend_add`、`friend_del`、`c2c_msg_reject`、`c2c_msg_receive`）
- 群机器人添加/移除及主动消息开关：[`demo_group_manage_event.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_group_manage_event.rs)（`group_add_robot`、`group_del_robot`、`group_msg_reject`、`group_msg_receive`）

## 接线

每个事件族都有专属 intent 标志，没开标志不会报错，只是 handler 拿不到事件。按需组合：

```rust
let intents = Intents::default()
    .with_guild_members()                 // demo_guild_member_event.rs
    .with_open_forum_event()              // demo_open_forum_event.rs
    .with_audio_or_live_channel_member()  // demo_audio_or_live_channel_member.rs
    .with_public_messages();              // c2c/group 管理事件
```

handler 内部，`Context` 参数携带 `ctx.api` 和 `ctx.token`，可以直接回调 API（例如 `ctx.api.create_dms` + `post_dms_with_params` 欢迎新成员；或 `post_group_message_with_params` 配合 `event_id: event.event_id.clone()` 来回应机器人入群事件——这些用法都在对应 demo 里）。

## 参见

- 指南：[`docs/zh/guide/intents.md`](../guide/intents.md)、[`docs/zh/guide/client-handler.md`](../guide/client-handler.md)
- `EventHandler` 全部方法：`src/client.rs`
- 上面列出的 `examples/` 中的 demo

# 文本消息

纯文本回复在 BotRS 中覆盖四种目的地，每种都有自己的 `*Params` 结构体和发送 API。对应示例：

- 频道 @ 回复：[`examples/guild/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_text.rs)
- 带引用块的频道回复：[`examples/guild/reply_reference.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_reference.rs)
- 群消息（QQ 群）：[`examples/group/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_text.rs)
- C2C（单聊）消息：[`examples/c2c/reply_text.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_text.rs)
- 频道内私信：[`examples/direct/reply.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply.rs)

## 选用合适的调用

频道回复最短的写法是 `Message::reply`。其他场景（群、C2C、私信，或需要 `Reference`、`msg_id`、`event_id`、附件、embed 的回复）请构造对应的 `*Params` 并调用对应的 `BotApi` 方法：

| 目的地       | Params                | API 方法                              |
|--------------|-----------------------|---------------------------------------|
| 频道         | `MessageParams`       | `send_message`            |
| 私信         | `DirectMessageParams` | `send_direct_message`                |
| 群           | `GroupMessageParams`  | `send_group_message`      |
| C2C          | `C2CMessageParams`    | `send_c2c_message`        |

```rust
// 引用回复（频道）— 见 examples/guild/reply_reference.rs
let params = MessageParams {
    content: Some("<emoji:4>这是一条引用消息".to_string()),
    message_reference: Some(Reference { message_id: Some(message_id.clone()), ignore_get_message_error: None }),
    ..Default::default()
};
ctx.send_message(channel_id, params).await?;
```

群与 C2C 记得把 `msg_type` 设为 `0`（纯文本），并在字段存在时把 `msg_id: message.id.clone()`、`event_id: message.event_id.clone()` 一起传给平台。私信还需要从入站 `Message` 拿 DM `guild_id`，或使用 `BotApi::create_direct_message` 创建会话。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Builder 辅助：`MessageParams::new_text(...).with_reply(...)`，`Group/C2C/DirectMessageParams` 上有同名方法
- 上面列出的 `examples/` 源文件

# 消息 API

消息 API 使用类型化参数结构体。每个发送场景对应一个参数类型和一个发送方法：

| 场景 | 参数类型 | 发送方法 |
| --- | --- | --- |
| 频道 | `MessageParams` | `send_message` |
| 群 | `GroupMessageParams` | `send_group_message` |
| C2C | `C2CMessageParams` | `send_c2c_message` |
| 私信 | `DirectMessageParams` | `send_direct_message` |

所有参数结构体都提供 `new_text(content)` 和 `with_reply(message_id)`。富文本载荷通过结构体字段表达。

```rust
let params = MessageParams::new_text("你好").with_reply(&message_id);
api.send_message("channel_id", params).await?;
```

## 基于字段的载荷

非纯文本内容直接设置结构体字段：

```rust
let params = MessageParams {
    content: Some("带 embed".into()),
    embed: Some(my_embed),
    markdown: Some(my_markdown),
    keyboard: Some(my_keyboard),
    ..Default::default()
};
api.send_message(channel_id, params).await?;
```

这样调用点会很明确：每个可选协议字段都有名字，`..Default::default()` 会让无关字段不进入发出的 JSON。

## 选择正确的参数类型

根据事件 payload 选择 ID 和参数类型：

- `Message` 带 `channel_id`；使用 `MessageParams`。
- `GroupMessage` 带 `group_openid`；使用 `GroupMessageParams`。
- `C2CMessage` 带 `author.user_openid`；使用 `C2CMessageParams`。
- `DirectMessage` 是 `create_direct_message` 返回的私信会话；使用 `DirectMessageParams` 和会话 `guild_id`。

纯文本回复优先使用事件模型自带的 `reply` 方法。需要更丰富的回复时，构造对应 params 并调用相应的 `BotApi` 方法。

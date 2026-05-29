# 消息 API

消息 API 使用类型化参数结构体。每个发送场景对应一个参数类型和一个发送方法：

| 场景 | 参数类型 | 发送方法 |
| --- | --- | --- |
| 频道 | `MessageParams` | `send_message` |
| 群 | `GroupMessageParams` | `send_group_message` |
| C2C | `C2CMessageParams` | `send_c2c_message` |
| 私信 | `DirectMessageParams` | `send_direct_message` |

所有参数结构体都提供 `new_text(content)`、`new_markdown(content)`、`new_ark(...)`、`new_embed(...)`、`new_keyboard(...)` 和 `with_reply(message_id)`。群和 C2C 参数还提供 `new_media(...)`。

```rust
let params = MessageParams::new_text("你好").with_reply(&message_id);
api.send_message("channel_id", params).await?;
```

## 富消息载荷

常见富消息使用构造器：

```rust
let params = MessageParams::new_embed(my_embed);
api.send_message(channel_id, params).await?;
```

自定义组合先从最接近的构造器开始，再直接设置额外字段，并用 `..Default::default()` 让无关协议字段不进入发出的 JSON。

## 选择正确的参数类型

根据事件 payload 选择 ID 和参数类型：

- `Message` 带 `channel_id`；使用 `MessageParams`。
- `GroupMessage` 带 `group_openid`；使用 `GroupMessageParams`。
- `C2CMessage` 带 `author.user_openid`；使用 `C2CMessageParams`。
- `DirectMessage` 是 `create_direct_message` 返回的私信会话；使用 `DirectMessageParams` 和会话 `guild_id`。

事件处理器里优先使用 session helper：`reply`、`send_markdown_message`、`send_embed_message`、`send_ark_message`、`send_keyboard_message`，以及群/C2C 的 `send_media_message`。独立调用 `BotApi` 时，构造对应 params 并调用相应发送方法。

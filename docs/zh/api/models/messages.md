# 消息

消息模型分成两类：网关推送进来的事件 payload，以及发送消息时使用的参数结构体。当前实现重点保证这两条链路可用：事件能正确解析，handler 能用框架 API 正常回复。

## 收到的消息

频道 `@bot` 消息使用 `Message`，对应 `EventHandler::message_create`。私信事件也使用 `Message`，对应 `direct_message_create`，可通过 `direct_message`、`src_guild_id` 等字段识别私信场景。

群聊和 C2C 是独立模型：

- `GroupMessage` 对应 `GROUP_AT_MESSAGE_CREATE`，核心定位字段是 `group_openid`。
- `C2CMessage` 对应 `C2C_MESSAGE_CREATE`，核心定位字段通常来自 `author.user_openid`。

这些事件模型都保留了平台可能给出的 message id、content、attachments、mentions、timestamp、reference 和内部 `event_id`。如果只是做回复，优先使用对应 reply session 的 `session.reply(...)`；更复杂的回复则手动构造对应的参数结构体。

```rust
let message = session.message();
if let Some(content) = &message.content {
    if content.trim() == "/ping" {
        session.reply("pong").await?;
    }
}
```

## 发送消息

发送侧使用四个参数类型：

| 参数类型 | 用途 |
| --- | --- |
| `MessageParams` | 频道消息 |
| `DirectMessageParams` | 私信会话消息 |
| `GroupMessageParams` | 群聊消息 |
| `C2CMessageParams` | C2C 消息 |

最常用的是 `new_text` 和 `with_reply`：

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
session.send_message(params).await?;
```

复杂消息直接在参数结构体上设置对应字段：

- `embed` 用于 embed 消息。
- `ark` 用于 ark 模板。
- `markdown` 用于 markdown 模板或 markdown 内容。
- `keyboard` 用于按钮键盘。
- `media` 用于群/C2C 文件上传后返回的媒体描述。

## DirectMessage 的含义

`DirectMessage` 是创建私信会话后返回的 session DTO。发送私信的流程是：

1. 用 `DirectMessageToCreate::new(source_guild_id, recipient_id)` 创建会话请求。
2. 调用 `create_direct_message` 得到 `DirectMessage`。
3. 用返回的 `guild_id` 调用 `send_direct_message`。

```rust
let dm = DirectMessageToCreate::new(&guild_id, &user_id);
let dm_session = session.create_direct_message(&dm).await?;

let params = DirectMessageParams::new_text("hello");
session.send_direct_message(&dm_session.guild_id, params).await?;
```

## Open message 的 msg_type

群和 C2C 消息沿用开放平台的数字 `msg_type`。文本一般保持默认值 0；媒体消息使用 7；markdown、ark、embed 等类型按平台协议值填写。频道消息则使用 `MessageCreateType`，在 Rust 侧有 enum 建模。

## 参见

- [消息指南](../../guide/messages.md)
- [Bot API](../bot-api.md)
- [其他类型](./other-types.md)

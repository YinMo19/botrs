# 消息

QQ 频道机器人 API 中的消息数据结构。框架将网关事件载荷（入站消息）与发送参数对象（出站请求体）分开建模。

## 入站消息

### `Message`

频道（包含 @ 提及）消息。

```rust
pub struct Message {
    pub id: Option<Snowflake>,
    pub content: Option<String>,
    pub channel_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub group_id: Option<Snowflake>,
    pub author: Option<MessageUser>,
    pub member: Option<MessageMember>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<MessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub embeds: Vec<Embed>,
    pub ark: Option<Ark>,
    pub direct_message: Option<bool>,
    pub seq: Option<u64>,
    pub seq_in_channel: Option<String>,
    pub timestamp: Option<Timestamp>,
    pub edited_timestamp: Option<Timestamp>,
    pub mention_everyone: Option<bool>,
    pub src_guild_id: Option<Snowflake>,
    pub file_info: Option<String>,
    pub ttl: Option<u32>,
    pub message_scene: Option<MessageScene>,
    pub event_id: Option<String>,
}
```

字符串字段大量使用 `Option<Snowflake>`，因为 QQ 接口可能在部分载荷中省略它们。`Message` 提供的便捷方法：

- `reply(api, token, content)` —— 文本回复，会自动设置 `msg_id` 走被动消息路径。
- `is_from_bot()`、`has_content()`、`has_attachments()`、`has_mentions()` —— 常用判断。

### `GroupMessage` 与 `C2CMessage`

群消息（群 `@bot`）和 C2C（私聊）消息结构相似，都使用 OpenID 标识用户：

```rust
pub struct GroupMessage {
    pub id: Option<Snowflake>,
    pub content: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub mentions: Vec<GroupMessageUser>,
    pub attachments: Vec<MessageAttachment>,
    pub msg_seq: Option<u64>,
    pub timestamp: Option<Timestamp>,
    pub author: Option<GroupMessageUser>,
    pub group_openid: Option<String>,
    pub event_id: Option<String>,
}
```

`C2CMessage` 形状相同，只是 `author`/`mentions` 的类型变成 `C2CMessageUser`。

### `DirectMessage`

`DirectMessage` 是私信**会话**对象（由创建私信会话的接口返回），并不是网关上的消息事件。私信网关事件仍然使用 `Message`，并把 `direct_message` 置为 `Some(true)`。

```rust
pub struct DirectMessage {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub create_time: Option<String>,
}
```

`DirectMessageSession` 是 `DirectMessage` 的别名。

## 出站参数对象

发送侧使用构建器结构体，避免老接口里一长串 `Option<...>` 参数：

| 结构体                  | 对应接口                                   |
|-------------------------|--------------------------------------------|
| `MessageParams`         | `BotApi::post_message_with_params`         |
| `DirectMessageParams`   | `BotApi::post_direct_message_with_params`  |
| `GroupMessageParams`    | `BotApi::post_group_message_with_params`   |
| `C2CMessageParams`      | `BotApi::post_c2c_message_with_params`     |

每个结构体都暴露 `new_text(content)`，并在适用场景提供回复和文件辅助方法。embed、markdown、ark、keyboard、media 等可选载荷请直接设置参数结构体里的对应字段，然后交给对应的 `post_*_with_params` 方法。

```rust
let params = MessageParams::new_text("Pong!").with_reply(message_id);
api.post_message_with_params(&token, &channel_id, params).await?;
```

## 配套类型

| 类型                | 用途                                                 |
|---------------------|------------------------------------------------------|
| `Embed`             | 嵌入式消息（标题、描述、字段、缩略图）。             |
| `Ark`               | Ark 模板消息（`template_id` + `kv` 数组）。          |
| `MarkdownPayload`   | Markdown 发送体（模板 id、内容、参数等）。           |
| `Keyboard`          | 行内键盘（按钮组合）。                               |
| `MessageReference`  | `{ message_id, ignore_get_message_error }`。         |
| `MessageAttachment` | 附件元信息（URL、文件名、大小……）。                  |
| `MessageAudit`      | 审核网关事件的载荷。                                 |
| `MessageDelete`     | 撤回事件载荷，包含 `message` 与 `op_user`。          |
| `MessageScene`      | 入站事件中的场景标记（群聊、C2C、频道）。            |

## 消息作者类型

参见 [用户与成员 - 消息作者类型](./users-members.md#消息作者类型) 了解 `MessageUser`、`DirectMessageUser`、`GroupMessageUser`、`C2CMessageUser` 的结构。

## 参见

- [消息指南](../../guide/messages.md) —— 任务导向的使用说明。
- [Bot API](../bot-api.md) —— 全部 `post_*_with_params` 路由。
- [其他类型](./other-types.md) —— 嵌入、键盘、Ark、附件等附属结构。

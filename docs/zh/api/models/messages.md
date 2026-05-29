# 消息

消息模型分成两类：网关推送进来的事件 payload，以及发送消息时使用的参数结构体。当前实现重点保证这两条链路可用：事件能正确解析，handler 能用框架 API 正常回复。

## 收到的消息

频道 `@bot` 消息使用 `Message`，对应 `EventHandler::message_create`。私信事件也使用 `Message`，对应 `direct_message_create`，可通过 `direct_message`、`src_guild_id` 等字段识别私信场景。

群聊和 C2C 是独立模型：

- `GroupMessage` 对应 `GROUP_AT_MESSAGE_CREATE`，核心定位字段是 `group_openid`。
- `C2CMessage` 对应 `C2C_MESSAGE_CREATE`，核心定位字段通常来自 `author.user_openid`。

这些事件模型都保留了平台给出的 message id、content、attachments、mentions、timestamp、reference 和内部 `event_id`。常规消息事件里的 message id、content、author、路由 id、timestamp 是必有字段；群/C2C 消息还会携带必有的开放消息 `message_type`。协议本身可缺省的字段仍然保持可选。

附件使用 `MessageAttachment`。只要附件出现，filename、content type、size、url 都是普通字段。`id` 仍是可选字段，因为开放消息附件可能不带该值；`width` / `height` 在非图片或平台未给尺寸时默认为 `0`。

```rust
let message = session.message();
if !message.author.bot && message.content.trim() == "/ping" {
    session.reply("pong").await?;
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

在可回复事件里优先使用 session helper：

```rust
session.reply("pong").await?;
session.send_markdown_message("# hello").await?;
session.send_embed_message(embed).await?;
```

直接调用 `BotApi` 或需要自定义字段时，使用 params 构造器：

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
session.send_message(params).await?;
```

富消息构造器包括 `new_markdown`、`new_ark`、`new_embed`、`new_keyboard`，群/C2C 还支持 `new_media`。只有在 helper 和构造器覆盖不到时，再直接设置字段：

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

## Open message 类型

群和 C2C 消息最终仍会序列化为开放平台的消息类型码，但应用代码应使用 session helper 或 params 构造器，不需要手写数字。频道和私信参数内部使用 `MessageCreateType`。

## 参见

- [消息指南](../../guide/messages.md)
- [Bot API](../bot-api.md)
- [其他类型](./other-types.md)

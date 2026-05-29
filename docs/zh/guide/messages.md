# 消息

`BotApi` 上的所有发送方法都接受类型化参数结构体。共有四个结构体，分别对应四种消息场景：

- `MessageParams` —— 频道消息。
- `GroupMessageParams` —— 群（`group_openid`）消息。
- `C2CMessageParams` —— 单聊 C2C 消息。
- `DirectMessageParams` —— 私信（DM）消息。

四者都提供常见场景的 `new_text(content)`、`new_markdown(content)`、`new_ark(...)`、`new_embed(...)`、`new_keyboard(...)`，以及设置 `msg_id` 引用的 `with_reply(message_id)`。`GroupMessageParams` 和 `C2CMessageParams` 还提供 `new_media(...)`。更特殊的协议字段仍可通过结构体字段直接赋值。

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("你好").with_reply(&message_id);
session.send_message(params).await?;
```

常见组合优先使用构造器，这样协议消息类型会被正确设置：

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_keyboard("带 markdown", my_keyboard);
session.send_message(params).await?;
```

构造器覆盖不到的平台字段，可以从最接近的构造器开始，再只覆盖额外字段。频道消息需要带图片时，把远程图片 URL 写入 `params.image = Some(url.into())`。

## 各场景的发送方法

不同场景的 ID 字段与发送方法不同：

| 场景 | 参数类型 | 发送方法 | ID 参数 |
| --- | --- | --- | --- |
| 频道 | `MessageParams` | `send_message` | `channel_id` |
| 群 | `GroupMessageParams` | `send_group_message` | `group_openid` |
| C2C | `C2CMessageParams` | `send_c2c_message` | `openid` |
| 私信 (DM) | `DirectMessageParams` | `send_direct_message` | `guild_id`（DM 会话） |

## 在事件中回复

`session.reply(content)` 在当前事件会话中回复纯文本。Reply session 还提供 `send_text_message`、`send_markdown_message`、`send_ark_message`、`send_embed_message`、`send_keyboard_message`，群/C2C/manage session 还提供 `send_media_message`。这些方法会自动带上入站消息 id、event id，以及平台要求的 open-message `msg_seq`。

```rust
async fn message_create(&self, mut session: ChannelReplySession) {
    let message = session.message().clone();
    if message.author.bot {
        return;
    }

    if message.content.trim() == "!ping" {
        let _ = session.reply("pong").await;
    }
}
```

```rust
session.send_markdown_message("# hello\n\n- one line").await?;
session.send_embed_message(embed).await?;
```

模板 markdown 或 helper 覆盖不到的字段组合，仍然自行构造对应 params，然后调用 `session.send_message(params)`。

## 撤回与审核

- `BotApi::recall_message(channel_id, message_id, hidetip)` 撤回频道消息。
- 审核结果通过 `EventHandler::message_audit_pass` / `message_audit_reject` 投递，载荷为 `MessageAudit`，处理器读取事件即可。

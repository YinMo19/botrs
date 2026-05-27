# 消息

`BotApi` 上的所有发送方法都接受类型化的构建器，而不是一长串 `Option`。共有四个构建器，分别对应四种消息场景：

- `MessageParams` —— 频道消息。
- `GroupMessageParams` —— 群（`group_openid`）消息。
- `C2CMessageParams` —— 单聊 C2C 消息。
- `DirectMessageParams` —— 私信（DM）消息。

四者都提供 `new_text(content)`、`with_reply(message_id)`（设置 `msg_id`）。其余字段（embed、ark、markdown、keyboard、图片 URL、media、prompt keyboard、action button、stream 等）通过结构体字段直接赋值。

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("你好").with_reply(&message_id);
ctx.send_message(&channel_id, params).await?;
```

更复杂的载荷直接初始化字段：

```rust
use botrs::models::message::{MessageParams, MarkdownPayload, Keyboard};

let params = MessageParams {
    content: Some("带 markdown".into()),
    markdown: Some(my_markdown),
    keyboard: Some(my_keyboard),
    ..Default::default()
};
ctx.send_message(&channel_id, params).await?;
```

频道消息需要带图片时，把远程图片 URL 写入 `params.image = Some(url.into())`。

## 各场景的发送方法

不同场景的 ID 字段与发送方法不同：

| 场景       | 构建器                | 发送方法                              | ID 参数            |
|------------|----------------------|--------------------------------------|--------------------|
| 频道       | `MessageParams`      | `send_message`           | `channel_id`       |
| 群         | `GroupMessageParams` | `send_group_message`     | `group_openid`     |
| C2C        | `C2CMessageParams`   | `send_c2c_message`       | `openid`           |
| 私信 (DM)  | `DirectMessageParams`| `send_direct_message`               | `guild_id`（DM 频道） |

编辑频道消息使用 `edit_message(channel_id, message_id, params)`。

## 在事件中回复

`Message::reply(&api, content)` 是回复入站 `Message` 同频道纯文本的便捷方法。`GroupMessage`、`C2CMessage`、`DirectMessage` 各自具备同名方法。框架内部会构造相应的 `*Params` 并把 `msg_id` 指向入站消息。

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.is_from_bot() { return; }
    if let Some("!ping") = message.content.as_deref() {
        let _ = message.reply(&ctx, "pong").await;
    }
}
```

要发送非纯文本，请自行构造 `MessageParams` 并使用入站事件中的 `channel_id` 调用 `ctx.send_message`。

## 撤回与审核

- `BotApi::recall_message(channel_id, message_id, hidetip)` 撤回频道消息。
- `retract_c2c_message`、`retract_group_message`、`retract_dm_message` 是另外三种场景的撤回。
- 审核结果通过 `EventHandler::message_audit_pass` / `message_audit_reject` 投递，载荷为 `MessageAudit`，无需主动 ACK。

## 已移除的旧 API

0.2 之前的多 `Option` 接口（`post_message`、`post_group_message`、`post_c2c_message`、`post_dms`）以及后来的 `*_with_params` 兼容名已经移除。请使用上文展示的短方法，迁移步骤见 [v0.2.0 迁移](/zh/guide/migration-v0.2.0)。

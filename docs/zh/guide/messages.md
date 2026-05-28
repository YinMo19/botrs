# 消息

`BotApi` 上的所有发送方法都接受类型化参数结构体。共有四个结构体，分别对应四种消息场景：

- `MessageParams` —— 频道消息。
- `GroupMessageParams` —— 群（`group_openid`）消息。
- `C2CMessageParams` —— 单聊 C2C 消息。
- `DirectMessageParams` —— 私信（DM）消息。

四者都提供 `new_text(content)` 和 `with_reply(message_id)`。富文本载荷（embed、ark、markdown、keyboard、图片 URL、群/C2C media）通过结构体字段直接赋值。

```rust
use botrs::models::message::MessageParams;

let params = MessageParams::new_text("你好").with_reply(&message_id);
ctx.send_message(&channel_id, params).await?;
```

更复杂的载荷直接初始化字段：

```rust
use botrs::models::message::MessageParams;

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

| 场景 | 参数类型 | 发送方法 | ID 参数 |
| --- | --- | --- | --- |
| 频道 | `MessageParams` | `send_message` | `channel_id` |
| 群 | `GroupMessageParams` | `send_group_message` | `group_openid` |
| C2C | `C2CMessageParams` | `send_c2c_message` | `openid` |
| 私信 (DM) | `DirectMessageParams` | `send_direct_message` | `guild_id`（DM 会话） |

## 在事件中回复

`message.reply(&ctx, content)` 是回复入站 `Message` 同频道纯文本的便捷方法。`GroupMessage` 和 `C2CMessage` 也提供相同形态的方法。框架内部会构造相应的 `*Params`，并带上入站消息 id 和 event id。

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
    if let Some("!ping") = message.content.as_deref() {
        let _ = message.reply(&ctx, "pong").await;
    }
}
```

要发送非纯文本，请自行构造 `MessageParams` 并使用入站事件中的 `channel_id` 调用 `ctx.send_message`。

## 撤回与审核

- `BotApi::recall_message(channel_id, message_id, hidetip)` 撤回频道消息。
- 审核结果通过 `EventHandler::message_audit_pass` / `message_audit_reject` 投递，载荷为 `MessageAudit`，处理器读取事件即可。

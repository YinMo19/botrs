# 交互式消息

内联键盘（Markdown 卡片下的按钮）见 [`demo_at_reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_keyboard.rs)。基于消息文本的轻量斜杠命令分发见 [`demo_at_reply_command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_command.rs)。

## 键盘

键盘作为普通 `MessageParams` 的一部分与 Markdown 一起发送。可以通过 id 引用服务端模板，也可以用 `KeyboardContent`、`KeyboardRow`、`KeyboardButton`、`KeyboardButtonRenderData`、`KeyboardButtonAction`、`KeyboardButtonPermission` 内联定义行/按钮。两种写法在 demo 的 `send_template_keyboard` 与 `send_self_defined_keyboard` 中并存。

```rust
use botrs::models::message::{
    Keyboard, KeyboardButton, KeyboardButtonAction, KeyboardButtonRenderData,
    KeyboardContent, KeyboardRow, MarkdownPayload, MessageParams,
};

let keyboard = Keyboard {
    id: None,
    content: Some(KeyboardContent {
        rows: Some(vec![KeyboardRow { buttons: Some(vec![KeyboardButton {
            id: Some("1".into()),
            render_data: Some(KeyboardButtonRenderData { label: Some("button".into()), visited_label: Some("BUTTON".into()), style: Some(0) }),
            action: Some(KeyboardButtonAction { action_type: Some(2), data: Some("/搜索".into()), enter: true, ..Default::default() }),
            group_id: None,
        }]) }]),
        style: None,
    }),
};
let params = MessageParams {
    markdown: Some(MarkdownPayload { content: Some("# title \n## body".into()), ..Default::default() }),
    keyboard: Some(keyboard),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
```

## 命令分发

`demo_at_reply_command.rs` 展示了一个极小的 `CommandRegistry`：把 `Vec<&str>` 别名映射到 handler 函数，在 `message_create` 中分发。框架本身没有命令系统——demo 就是推荐做法：按前缀切分、调用匹配的 handler、再用 `Message::reply` 或 `MessageParams` 回复。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- 命令细节：[命令处理器](./command-handler.md)
- Demo：`examples/demo_at_reply_keyboard.rs`、`examples/demo_at_reply_command.rs`

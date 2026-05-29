# 交互式消息

内联键盘需要与 Markdown 一起发送。频道键盘见 [`examples/guild/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_keyboard.rs)，群和 C2C 键盘模板见 [`examples/group/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_keyboard.rs)、[`examples/c2c/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_keyboard.rs)。基于消息文本的轻量斜杠命令分发见 [`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs)。

## 键盘

键盘与 Markdown 一起发送。可以通过 id 引用服务端模板，也可以用 `KeyboardContent`、`KeyboardRow`、`KeyboardButton`、`KeyboardButtonRenderData`、`KeyboardButtonAction`、`KeyboardButtonPermission` 内联定义行/按钮。群与 C2C session 使用 `KeyboardPayload`，频道和私信用 `Keyboard`。

```rust
use botrs::models::message::{
    Keyboard, KeyboardButton, KeyboardButtonAction, KeyboardButtonRenderData,
    KeyboardContent, KeyboardRow,
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
session.send_keyboard_message("# title\n\n## body", keyboard).await?;
```

独立调用 `BotApi` 时，使用对应的 params 构造器，例如 `MessageParams::new_keyboard(content, keyboard)`。群和 C2C 使用带 `KeyboardPayload` 的 `GroupMessageParams::new_keyboard` / `C2CMessageParams::new_keyboard`。

## 命令分发

`examples/guild/command.rs` 展示了一个极小的 `CommandRegistry`：把别名映射到 handler 函数，在 `message_create` 中分发。框架本身没有命令系统；按前缀切分、调用匹配的 handler、再用 `session.reply` 或语义化 `send_*_message` helper 回复即可。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- 命令细节：[命令处理器](./command-handler.md)
- 示例：`examples/guild/reply_keyboard.rs`、`examples/group/reply_keyboard.rs`、`examples/c2c/reply_keyboard.rs`、`examples/guild/command.rs`

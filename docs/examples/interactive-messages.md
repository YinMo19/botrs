# Interactive Messages

Inline keyboards are sent together with Markdown. Guild channel keyboards live in [`examples/guild/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_keyboard.rs); group and C2C keyboard templates live in [`examples/group/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_keyboard.rs) and [`examples/c2c/reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_keyboard.rs). Lightweight slash-style command parsing on incoming text is in [`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs).

## Keyboards

A keyboard is sent alongside Markdown. You can either reference a server-side template by id, or define rows + buttons inline using `KeyboardContent`, `KeyboardRow`, `KeyboardButton`, `KeyboardButtonRenderData`, `KeyboardButtonAction`, and `KeyboardButtonPermission`. Group and C2C sessions use `KeyboardPayload`; channel and DM sessions use `Keyboard`.

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

For standalone `BotApi` calls, use the matching params constructor, for example `MessageParams::new_keyboard(content, keyboard)`. Group and C2C use `GroupMessageParams::new_keyboard` / `C2CMessageParams::new_keyboard` with `KeyboardPayload`.

## Command dispatch

`examples/guild/command.rs` shows a tiny `CommandRegistry` that maps aliases to handler functions and dispatches inside `message_create`. BotRS does not include a command framework; split on prefix, run the matched handler, then reply via `session.reply` or `session.send_message`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Command details: [Command Handler](./command-handler.md)
- Examples: `examples/guild/reply_keyboard.rs`, `examples/group/reply_keyboard.rs`, `examples/c2c/reply_keyboard.rs`, `examples/guild/command.rs`

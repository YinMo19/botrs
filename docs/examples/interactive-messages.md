# Interactive Messages

Inline keyboards (buttons under a Markdown card) live in [`demo_at_reply_keyboard.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_keyboard.rs). Lightweight slash-style command parsing on incoming text is in [`demo_at_reply_command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_command.rs).

## Keyboards

A keyboard is sent as part of a normal `MessageParams` alongside Markdown. You can either reference a server-side template by id, or define rows + buttons inline using `KeyboardContent`, `KeyboardRow`, `KeyboardButton`, `KeyboardButtonRenderData`, `KeyboardButtonAction`, `KeyboardButtonPermission`. Both flavours appear in the demo's `send_template_keyboard` and `send_self_defined_keyboard` helpers.

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

## Command dispatch

`demo_at_reply_command.rs` shows a tiny `CommandRegistry` that maps `Vec<&str>` aliases to handler functions and dispatches inside `message_create`. There is no built-in command framework — the demo is the recommended pattern: split on prefix, run the matched handler, then reply via `Message::reply` or `MessageParams`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Command details: [Command Handler](./command-handler.md)
- Demos: `examples/demo_at_reply_keyboard.rs`, `examples/demo_at_reply_command.rs`

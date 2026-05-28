# Rich Messages

Embeds, Markdown, ARK templates, and keyboards use the message parameter struct for the destination you are sending to. Guild channel and direct messages use `MessageParams` / `DirectMessageParams`; group and C2C messages use `GroupMessageParams` / `C2CMessageParams` with a numeric `msg_type`.

Working examples:

- Guild channel: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_ark.rs)
- Group: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_ark.rs)
- C2C: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_ark.rs)
- Direct messages: [`reply_rich.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply_rich.rs)

## Pattern

Build a payload (`Embed`, `MarkdownPayload`, `Ark`) using its struct-init form, drop it into the matching params field, then call the current session's `send_message`.

```rust
use botrs::models::message::{Embed, EmbedField, MessageParams};

let embed = Embed {
    title: Some("embed消息".to_string()),
    prompt: "消息透传显示".to_string(),
    fields: Some(vec![EmbedField { name: Some("hello world".to_string()), ..Default::default() }]),
    ..Default::default()
};
let params = MessageParams { embed: Some(embed), ..Default::default() };
session.send_message(params).await?;
```

The same shape works for `markdown: Some(MarkdownPayload { ... })` and `ark: Some(Ark { template_id: Some(37), kv: Some(vec![ArkKv { ... }]) })`. Markdown supports both `custom_template_id` + `params` (template form) and raw `content`. For group and C2C sends, set `msg_type` to `2` for Markdown, `3` for ARK, and `4` for Embed.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Combined Markdown + Keyboard: [Interactive Messages](./interactive-messages.md)
- Examples: `examples/guild`, `examples/group`, `examples/c2c`, and `examples/direct/reply_rich.rs`

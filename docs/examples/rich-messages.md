# Rich Messages

Embeds, Markdown, and ARK templates are all sent through the same `MessageParams` struct — set the relevant optional field, leave the others `None`. Working demos:

- Embed: [`demo_at_reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_embed.rs)
- Markdown (template + free content): [`demo_at_reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_markdown.rs)
- ARK card: [`demo_at_reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_ark.rs)

## Pattern

Build a payload (`Embed`, `MarkdownPayload`, `Ark`) using its struct-init form, drop it into the matching `MessageParams` field, then call `ctx.send_message(channel_id, params)`.

```rust
use botrs::models::message::{Embed, EmbedField, MessageParams};

let embed = Embed {
    title: Some("embed消息".to_string()),
    prompt: "消息透传显示".to_string(),
    fields: Some(vec![EmbedField { name: Some("hello world".to_string()), ..Default::default() }]),
    ..Default::default()
};
let params = MessageParams { embed: Some(embed), ..Default::default() };
ctx.send_message(channel_id, params).await?;
```

The same shape works for `markdown: Some(MarkdownPayload { ... })` and `ark: Some(Ark { template_id: Some(37), kv: Some(vec![ArkKv { ... }]) })`. Markdown supports both `custom_template_id` + `params` (template form) and a raw `content: Some("# title …".into())` (free form); both styles appear side-by-side in `demo_at_reply_markdown.rs`.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Combined Markdown + Keyboard: [Interactive Messages](./interactive-messages.md)
- Demos: `demo_at_reply_embed.rs`, `demo_at_reply_markdown.rs`, `demo_at_reply_ark.rs`

# Rich Messages

Embeds, Markdown, ARK templates, and keyboards can be sent directly from the current reply session. Use the lower-level params structs only when you need custom protocol fields.

Working examples:

- Guild channel: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_ark.rs)
- Group: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_ark.rs)
- C2C: [`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_embed.rs), [`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_markdown.rs), [`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_ark.rs)
- Direct messages: [`reply_rich.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply_rich.rs)

## Pattern

Build a payload (`Embed`, `Ark`, keyboard payload), then pass it to the semantic session helper.

```rust
use botrs::models::message::{Embed, EmbedField};

let embed = Embed {
    title: Some("embed消息".to_string()),
    prompt: "消息透传显示".to_string(),
    fields: Some(vec![EmbedField { name: Some("hello world".to_string()), ..Default::default() }]),
    ..Default::default()
};
session.send_embed_message(embed).await?;
```

Raw markdown is also a session helper:

```rust
session.send_markdown_message("# title\n\nbody").await?;
```

Use params constructors such as `MessageParams::new_keyboard(...)`, `GroupMessageParams::new_ark(...)`, or `C2CMessageParams::new_embed(...)` when you are sending outside a reply session. Set fields manually only for custom combinations such as template markdown params not covered by the helpers.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Combined Markdown + Keyboard: [Interactive Messages](./interactive-messages.md)
- Examples: `examples/guild`, `examples/group`, `examples/c2c`, and `examples/direct/reply_rich.rs`

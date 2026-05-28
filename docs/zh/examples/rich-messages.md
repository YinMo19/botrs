# 富文本消息

Embed、Markdown、ARK 模板和键盘都通过对应目的地的参数结构体发送。频道与私信用 `MessageParams` / `DirectMessageParams`，群与 C2C 用 `GroupMessageParams` / `C2CMessageParams` 并设置数字 `msg_type`。

可运行的示例：

- 频道：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_ark.rs)
- 群：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_ark.rs)
- C2C：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_ark.rs)
- 私信：[`reply_rich.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply_rich.rs)

## 套路

用结构体初始化的方式构造 payload（`Embed`、`MarkdownPayload`、`Ark`），塞进对应 params 字段，再调用当前 session 的 `send_message`。

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

自由格式 markdown 有 session helper：

```rust
session.send_markdown_message("# title\n\nbody").await?;
```

需要 `custom_template_id` + `params` 或 markdown + keyboard 同发时，再使用 `markdown: Some(MarkdownPayload { ... })`。手动构造群与 C2C 富消息时，Markdown 用 `msg_type: 2`，ARK 用 `msg_type: 3`，Embed 用 `msg_type: 4`。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Markdown + 按钮组合：[交互式消息](./interactive-messages.md)
- 示例：`examples/guild`、`examples/group`、`examples/c2c`、`examples/direct/reply_rich.rs`

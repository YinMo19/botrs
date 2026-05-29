# 富文本消息

Embed、Markdown、ARK 模板和键盘可以直接从当前 reply session 发送。只有需要自定义协议字段时，才使用更底层的 params 结构体。

可运行的示例：

- 频道：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_ark.rs)
- 群：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_ark.rs)
- C2C：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_ark.rs)
- 私信：[`reply_rich.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply_rich.rs)

## 套路

用结构体初始化的方式构造 payload（`Embed`、`Ark`、键盘 payload），再传给语义化 session helper。

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

自由格式 markdown 也有 session helper：

```rust
session.send_markdown_message("# title\n\nbody").await?;
```

不在 reply session 内发送时，使用 `MessageParams::new_keyboard(...)`、`GroupMessageParams::new_ark(...)`、`C2CMessageParams::new_embed(...)` 这类构造器。需要模板 markdown 参数等 helper 覆盖不到的组合时，再手动设置字段。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Markdown + 按钮组合：[交互式消息](./interactive-messages.md)
- 示例：`examples/guild`、`examples/group`、`examples/c2c`、`examples/direct/reply_rich.rs`

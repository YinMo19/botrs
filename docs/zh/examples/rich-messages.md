# 富文本消息

Embed、Markdown、ARK 模板和键盘都通过对应目的地的参数结构体发送。频道与私信用 `MessageParams` / `DirectMessageParams`，群与 C2C 用 `GroupMessageParams` / `C2CMessageParams` 并设置数字 `msg_type`。

可运行的示例：

- 频道：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_ark.rs)
- 群：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_ark.rs)
- C2C：[`reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_embed.rs)、[`reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_markdown.rs)、[`reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_ark.rs)
- 私信：[`reply_rich.rs`](https://github.com/YinMo19/botrs/blob/main/examples/direct/reply_rich.rs)

## 套路

用结构体初始化的方式构造 payload（`Embed`、`MarkdownPayload`、`Ark`），塞进 `MessageParams` 对应字段，再调用 `ctx.send_message(channel_id, params)`。

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

`markdown: Some(MarkdownPayload { ... })` 和 `ark: Some(Ark { template_id: Some(37), kv: Some(vec![ArkKv { ... }]) })` 的写法完全一样。Markdown 支持两种形态：`custom_template_id` + `params`（模板）和直接 `content`（自由格式）。群与 C2C 发送时，Markdown 用 `msg_type: 2`，ARK 用 `msg_type: 3`，Embed 用 `msg_type: 4`。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Markdown + 按钮组合：[交互式消息](./interactive-messages.md)
- 示例：`examples/guild`、`examples/group`、`examples/c2c`、`examples/direct/reply_rich.rs`

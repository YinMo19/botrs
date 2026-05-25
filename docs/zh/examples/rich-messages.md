# 富文本消息

Embed、Markdown、ARK 模板都通过同一个 `MessageParams` 发送——填写对应的可选字段，其余留 `None` 即可。可运行的 demo：

- Embed：[`demo_at_reply_embed.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_embed.rs)
- Markdown（模板 + 自由内容）：[`demo_at_reply_markdown.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_markdown.rs)
- ARK 卡片：[`demo_at_reply_ark.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_ark.rs)

## 套路

用结构体初始化的方式构造 payload（`Embed`、`MarkdownPayload`、`Ark`），塞进 `MessageParams` 对应字段，再调用 `BotApi::post_message_with_params(&token, channel_id, params)`。

```rust
use botrs::models::message::{Embed, EmbedField, MessageParams};

let embed = Embed {
    title: Some("embed消息".to_string()),
    prompt: "消息透传显示".to_string(),
    fields: Some(vec![EmbedField { name: Some("hello world".to_string()), ..Default::default() }]),
    ..Default::default()
};
let params = MessageParams { embed: Some(embed), ..Default::default() };
ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
```

`markdown: Some(MarkdownPayload { ... })` 和 `ark: Some(Ark { template_id: Some(37), kv: Some(vec![ArkKv { ... }]) })` 的写法完全一样。Markdown 支持两种形态：`custom_template_id` + `params`（模板）和直接 `content: Some("# 标题 …".into())`（自由格式），两者在 `demo_at_reply_markdown.rs` 中并排出现。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Markdown + 按钮组合：[交互式消息](./interactive-messages.md)
- Demo：`demo_at_reply_embed.rs`、`demo_at_reply_markdown.rs`、`demo_at_reply_ark.rs`

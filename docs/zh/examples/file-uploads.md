# 文件上传

按目的地分为两条独立流程：

- **频道 @ 回复带图片附件**——把文件读成 `Vec<u8>`，调用 `MessageParams::new_text(...).with_file_image(&bytes)`。见 [`demo_at_reply_file_data.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_file_data.rs)。
- **群 / C2C 富媒体**——先用 `BotApi::post_group_file` / `BotApi::post_c2c_file` 上传一个 URL，把响应反序列化成 `Media`，再发一条 `msg_type: 7` 的后续消息。见 [`demo_group_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_group_reply_file.rs) 和 [`demo_c2c_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_c2c_reply_file.rs)。

## 频道附件

```rust
let bytes = std::fs::read("examples/resource/test.png")?;
let params = MessageParams::new_text("here you go").with_file_image(&bytes);
ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
```

## 群 / C2C 两步走

```rust
let upload = ctx.api.post_group_file(&ctx.token, group_openid, /* file_type */ 1, file_url, None).await?;
let media: botrs::models::message::Media = serde_json::from_value(upload)?;
let params = botrs::models::message::GroupMessageParams {
    msg_type: 7, // 富媒体
    msg_id: message.id.clone(),
    media: Some(media),
    ..Default::default()
};
ctx.api.post_group_message_with_params(&ctx.token, group_openid, params).await?;
```

`post_c2c_file` + `post_c2c_message_with_params` 模式完全一致。`file_type` 为 `1` 表示图片，其他取值见 demo。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Demo：`examples/demo_at_reply_file_data.rs`、`examples/demo_group_reply_file.rs`、`examples/demo_c2c_reply_file.rs`

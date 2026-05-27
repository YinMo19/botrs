# 文件上传

按目的地分为两条独立流程：

- **频道图片 URL**——把远程图片 URL 写入 `MessageParams::image`。
- **群 / C2C 富媒体**——先用 `BotApi::post_group_file` / `BotApi::post_c2c_file` 上传一个 URL，再把返回的 `Media` 发成一条 `msg_type: 7` 的后续消息。见 [`demo_group_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_group_reply_file.rs) 和 [`demo_c2c_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_c2c_reply_file.rs)。

## 频道图片

```rust
let params = MessageParams {
    content: Some("here you go".into()),
    image: Some("https://example.com/image.png".into()),
    ..Default::default()
};
ctx.send_message(channel_id, params).await?;
```

## 群 / C2C 两步走

```rust
let media = ctx.post_group_file(group_openid, /* file_type */ 1, file_url, None).await?;
let params = botrs::models::message::GroupMessageParams {
    msg_type: 7, // 富媒体
    msg_id: message.id.clone(),
    media: Some(media),
    ..Default::default()
};
ctx.send_group_message(group_openid, params).await?;
```

`post_c2c_file` + `send_c2c_message` 模式完全一致。`file_type` 为 `1` 表示图片，其他取值见 demo。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- Demo：`examples/demo_group_reply_file.rs`、`examples/demo_c2c_reply_file.rs`

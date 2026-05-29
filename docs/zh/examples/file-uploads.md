# 文件上传

按目的地分为两条独立流程：

- **频道图片 URL**——把远程图片 URL 写入 `MessageParams::image`。见 [`examples/guild/reply_image.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_image.rs)。
- **群 / C2C 富媒体**——先用 reply session 的 `post_file` 上传一个 URL，再用 `send_media_message` 发送返回的 `Media`。见 [`examples/group/reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_file.rs) 和 [`examples/c2c/reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_file.rs)。

## 频道图片

```rust
let params = MessageParams {
    content: Some("here you go".into()),
    image: Some("https://example.com/image.png".into()),
    ..Default::default()
};
session.send_message(params).await?;
```

## 群 / C2C 两步走

```rust
let media = session.post_file(/* file_type */ 1, file_url, None).await?;
session.send_media_message(media).await?;
```

`C2CReplySession::post_file` + `send_media_message` 模式完全一致。`file_type` 为 `1` 表示图片，其他取值见示例源码。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- 示例：`examples/guild/reply_image.rs`、`examples/group/reply_file.rs`、`examples/c2c/reply_file.rs`

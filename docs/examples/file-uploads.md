# File Uploads

Two distinct flows depending on the destination:

- **Channel image by URL** — set `MessageParams::image` to a remote image URL. See [`examples/guild/reply_image.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reply_image.rs).
- **Group / C2C rich media** — upload a URL via the reply session's `post_file`, then send the returned `Media` in a follow-up message with `msg_type: 7`. See [`examples/group/reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/group/reply_file.rs) and [`examples/c2c/reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/c2c/reply_file.rs).

## Channel Image

```rust
let params = MessageParams {
    content: Some("here you go".into()),
    image: Some("https://example.com/image.png".into()),
    ..Default::default()
};
session.send_message(params).await?;
```

## Group / C2C two-step

```rust
let media = session.post_file(/* file_type */ 1, file_url, None).await?;
let params = botrs::models::message::GroupMessageParams {
    msg_type: 7, // 富媒体
    media: Some(media),
    ..Default::default()
};
session.send_message(params).await?;
```

`C2CReplySession::post_file` + `send_message` follow the exact same pattern. `file_type` is `1` for image; see the example source for other constants.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Examples: `examples/guild/reply_image.rs`, `examples/group/reply_file.rs`, `examples/c2c/reply_file.rs`

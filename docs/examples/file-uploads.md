# File Uploads

Two distinct flows depending on the destination:

- **Channel image by URL** — set `MessageParams::image` to a remote image URL.
- **Group / C2C rich media** — upload a URL via `BotApi::post_group_file` / `BotApi::post_c2c_file`, then send the returned `Media` in a follow-up message with `msg_type: 7`. See [`demo_group_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_group_reply_file.rs) and [`demo_c2c_reply_file.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_c2c_reply_file.rs).

## Channel Image

```rust
let params = MessageParams {
    content: Some("here you go".into()),
    image: Some("https://example.com/image.png".into()),
    ..Default::default()
};
ctx.send_message(channel_id, params).await?;
```

## Group / C2C two-step

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

`post_c2c_file` + `send_c2c_message` follow the exact same pattern. `file_type` is `1` for image, see the demo file for other constants.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Demos: `examples/demo_group_reply_file.rs`, `examples/demo_c2c_reply_file.rs`

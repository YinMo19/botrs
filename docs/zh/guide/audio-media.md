# 音频与媒体

音频相关能力分成两类：gateway 音频事件，以及群/C2C 的媒体文件发送。

## 音频事件

开启对应 intents 后，平台会通过 gateway 推送音频状态和语音/直播子频道成员变更：

- `audio_start`
- `audio_finish`
- `on_mic`
- `off_mic`
- `audio_or_live_channel_member_enter`
- `audio_or_live_channel_member_exit`

`Audio` 会携带 channel id、guild id、audio url、文本描述和内部 event id。`PublicAudio` 用于语音/直播子频道成员进入和离开事件，`PublicAudioType` 用来区分 voice 与 live。

这些事件适合做日志、统计、状态同步，或触发普通消息回复。

## 群与 C2C 媒体上传

群和 C2C 消息可以先上传媒体，再发送引用该媒体的消息。

```rust
let media = session.post_file(1, image_url, None).await?;

let mut params = GroupMessageParams::default();
params.msg_type = 7;
params.media = Some(media);
session.send_message(params).await?;
```

`file_type` 使用平台协议值：常见值为 1 图片、2 视频、3 语音、4 文件。C2C 使用 `post_c2c_file`，参数形态相同。

`srv_send_msg` 传 `Some(true)` 时，平台会在上传后直接发送；传 `None` 或 `Some(false)` 时，通常把返回的 `Media` 放进消息参数里自行发送。

# 音频与媒体

框架的音频支持分两层：网关投递给 `EventHandler` 的事件，以及 `BotApi` 上控制播放和上传媒体的 REST 方法。

## 音频事件

启用 `Intents::AUDIO_ACTION` 后会收到四个回调：

- `audio_start(&self, ctx, audio: Audio)` —— 播放开始。
- `audio_finish(&self, ctx, audio: Audio)` —— 播放结束。
- `on_mic(&self, ctx, audio: Audio)` —— 机器人已上麦。
- `off_mic(&self, ctx, audio: Audio)` —— 机器人已下麦。

`Audio` 包含 `channel_id`、`guild_id`、`audio_url`、`text`、`event_id`，均为 `Option<String>`。`Audio` 内部还持有 `BotApi` 引用，可通过 `audio.api()` 访问，便于在不传 `Context` 的情况下发起 REST 请求。

语音／直播子频道成员进出事件需要启用 `Intents::AUDIO_OR_LIVE_CHANNEL_MEMBER`，并实现 `audio_or_live_channel_member_enter` / `_exit`。载荷为 `PublicAudio { guild_id, channel_id, channel_type: Option<PublicAudioType>, user_id }`，`PublicAudioType` 取值 `Voice = 2` 或 `Live = 5`。

## 控制播放

`BotApi::post_audio(channel_id, &AudioControl)` 用于更新某语音子频道的音频会话，`AudioControl` 即请求体：

```rust
use botrs::audio::{AudioControl, AudioStatus};

let control = AudioControl {
    audio_url: "https://example.com/track.mp3".into(),
    text: "正在播放".into(),
    status: AudioStatus::Start, // Start | Pause | Resume | Stop
};
ctx.post_audio(&channel_id, &control).await?;
```

麦位控制：

- `BotApi::on_microphone(channel_id)` —— 上麦。
- `BotApi::off_microphone(channel_id)` —— 下麦。

## 富媒体上传

群／C2C 消息可以先上传媒体，再发送引用该媒体的消息：

```rust
use botrs::models::message::GroupMessageParams;

// 1=图片 2=视频 3=语音 4=文件
let media = ctx
    .post_group_file(&group_openid, 1, image_url, None)
    .await?;

let params = GroupMessageParams {
    msg_type: 7, // media
    media: Some(media),
    ..Default::default()
};
ctx.send_group_message(&group_openid, params).await?;
```

C2C 与之对应的方法是 `post_c2c_file(openid, file_type, url, srv_send_msg)`。传入 `srv_send_msg = Some(true)` 时平台会直接转发为消息，否则把返回的 `Media` 描述塞进自己的 `*MessageParams`。

频道消息若手上已有图片字节，使用 `MessageParams::with_file_image(&bytes)` 最方便 —— 框架会自动 base64 编码到 `file_image` 字段。

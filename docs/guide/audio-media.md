# Audio and media

The framework exposes two layers of audio support: events the gateway delivers to your `EventHandler`, and REST methods on `BotApi` for controlling playback and uploading media.

## Audio events

Subscribing to `Intents::AUDIO_ACTION` enables four callbacks:

- `audio_start(&self, ctx, audio: Audio)` — playback began.
- `audio_finish(&self, ctx, audio: Audio)` — playback finished.
- `on_mic(&self, ctx, audio: Audio)` — bot is on-mic in the voice subchannel.
- `off_mic(&self, ctx, audio: Audio)` — bot is off-mic.

`Audio` carries `channel_id`, `guild_id`, `audio_url`, `text`, `event_id`, all `Option<String>`. It also keeps an internal `BotApi` reference accessible via `audio.api()`, which is convenient when the handler wants to make REST calls without dragging the whole `Context`.

For voice / live-channel member traffic, enable `Intents::AUDIO_OR_LIVE_CHANNEL_MEMBER` and implement `audio_or_live_channel_member_enter` / `_exit`. The payload is `PublicAudio { guild_id, channel_id, channel_type: Option<PublicAudioType>, user_id }` where `PublicAudioType` is `Voice = 2` or `Live = 5`.

## Controlling playback

`BotApi::post_audio(&token, channel_id, &AudioControl)` updates an audio session in a voice channel. `AudioControl` is the request body:

```rust
use botrs::audio::{AudioControl, AudioStatus};

let control = AudioControl {
    audio_url: "https://example.com/track.mp3".into(),
    text: "now playing".into(),
    status: AudioStatus::Start,   // Start | Pause | Resume | Stop
};
ctx.api.post_audio(&ctx.token, &channel_id, &control).await?;
```

Mic control on a voice channel:

- `BotApi::on_microphone(&token, channel_id)` — bot joins the mic.
- `BotApi::off_microphone(&token, channel_id)` — bot leaves.

## Uploading rich media

For group and C2C messages you can upload media first, then send a message that references it.

```rust
use botrs::models::message::GroupMessageParams;

// 1 = image, 2 = video, 3 = audio (voice), 4 = file
let media = ctx.api
    .post_group_file(&ctx.token, &group_openid, 1, image_url, None)
    .await?;

let params = GroupMessageParams {
    msg_type: 7, // media
    media: Some(media),
    ..Default::default()
};
ctx.api.post_group_message_with_params(&ctx.token, &group_openid, params).await?;
```

The C2C surface mirrors this with `post_c2c_file(&token, openid, file_type, url, srv_send_msg)`. Pass `srv_send_msg = Some(true)` to have the platform forward the upload as a message immediately, otherwise reuse the returned `Media` descriptor in your own `*MessageParams`.

For guild channel messages with raw image bytes already in memory, prefer `MessageParams::with_file_image(&bytes)` — the framework base64-encodes them into the `file_image` field for you.

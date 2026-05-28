# Audio and media

Audio-related functionality has two parts: gateway audio events, and media upload for group/C2C messages.

## Audio Events

With the corresponding intents enabled, the platform sends audio state and voice/live channel member changes through the gateway:

- `audio_start`
- `audio_finish`
- `on_mic`
- `off_mic`
- `audio_or_live_channel_member_enter`
- `audio_or_live_channel_member_exit`

`Audio` carries channel id, guild id, audio URL, text description, and internal event id. `PublicAudio` represents voice/live channel member enter and exit events, with `PublicAudioType` distinguishing voice from live.

These events are useful for logging, metrics, state sync, or triggering normal message replies.

## Media Upload for Group and C2C

Group and C2C messages can upload media first, then send a message that references the returned media.

```rust
let media = ctx
    .post_group_file(&group_openid, 1, image_url, None)
    .await?;

let mut params = GroupMessageParams::default();
params.msg_type = 7;
params.media = Some(media);
ctx.send_group_message(&group_openid, params).await?;
```

`file_type` follows platform values: commonly 1 image, 2 video, 3 audio, 4 file. C2C uses `post_c2c_file` with the same parameter shape.

When `srv_send_msg` is `Some(true)`, the platform sends the uploaded file directly. When it is `None` or `Some(false)`, place the returned `Media` into your own message params.

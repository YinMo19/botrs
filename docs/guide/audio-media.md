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
let media = session.post_file(1, image_url, None).await?;
session.send_media_message(media).await?;
```

`file_type` follows platform values: commonly 1 image, 2 video, 3 audio, 4 file. C2C uses `post_c2c_file` with the same parameter shape.

When `srv_send_msg` is `Some(true)`, the platform sends the uploaded file directly. When it is `None` or `Some(false)`, pass the returned `Media` to `send_media_message` or `GroupMessageParams::new_media` / `C2CMessageParams::new_media`.

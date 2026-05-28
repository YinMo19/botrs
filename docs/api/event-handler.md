# EventHandler

`EventHandler` is the async trait you implement to react to gateway events. Every method has an empty default body, so you only override what you care about — even an empty struct will compile.

```rust
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _session: ReadySession) {}
    async fn message_create(&self, _session: ChannelReplySession) {}
    /* … */
}
```

The framework dispatches whichever method matches the gateway event type. Every public callback receives a session object. Reply-capable message callbacks use specialized reply sessions, while lifecycle and non-reply callbacks use event sessions whose payload is available through `session.event()`. Every method returns `()`; if you need to surface an error, log it locally — gateway-event errors do not interrupt the connection.

## Method catalogue

Methods are grouped by the gateway category that produces them.

### Lifecycle

- `ready(ReadySession)` — called once after a successful identify handshake; bot info is available through `session.bot_info()`.
- `resumed(ResumeSession)` — called after a successful resume.
- `unknown_event(UnknownEventSession)` — catch-all for event types without a first-class callback.

### Channel messages

- `message_create(ChannelReplySession)` — guild channel messages and `@mentions`.
- `message_delete(MessageDeleteSession)` — guild message delete event.
- `public_message_delete(MessageDeleteSession)` — public-domain delete.

### Direct messages

- `direct_message_create(DirectReplySession)`
- `direct_message_delete(MessageDeleteSession)`

### Group / C2C

- `group_message_create(GroupReplySession)`
- `c2c_message_create(C2CReplySession)`
- `friend_add(C2CManageSession)` / `friend_del(C2CManageSession)`
- `c2c_msg_reject(C2CManageSession)` / `c2c_msg_receive(C2CManageSession)`
- `subscribe_message_status(SubscribeMessageStatusSession)`
- `enter_aio(EnterAioSession)`

### Group management

- `group_add_robot` / `group_del_robot`
- `group_msg_reject` / `group_msg_receive`

### Reactions

- `message_reaction_add(ReactionSession)` / `message_reaction_remove(ReactionSession)`

### Interaction

- `interaction_create(InteractionSession)` — buttons and application interaction payloads.

### Audio / voice

- `audio_start` / `audio_finish` / `on_mic` / `off_mic` — `Audio` payloads.
- `audio_or_live_channel_member_enter` / `audio_or_live_channel_member_exit` — `PublicAudio` payloads.

### Guild / channel

- `guild_create` / `guild_update` / `guild_delete`
- `channel_create` / `channel_update` / `channel_delete`
- `guild_member_add` / `guild_member_update` / `guild_member_remove`

### Audit

- `message_audit_pass` / `message_audit_reject`

### Forum

- `forum_thread_create` / `forum_thread_update` / `forum_thread_delete`
- `forum_post_create` / `forum_post_delete`
- `forum_reply_create` / `forum_reply_delete`
- `forum_publish_audit_result(ForumAuditSession)`
- Open-forum equivalents: `open_forum_thread_*`, `open_forum_post_*`, `open_forum_reply_*`.

## Skeleton implementation

```rust
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, session: ReadySession) {
        info!("logged in as {}", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                let _ = session.reply("Pong!").await;
            }
        }
    }
}
```

`MyBot` only handles two events; every other gateway event silently lands in the empty defaults.

## Concurrency

Each handler invocation runs on the runtime task that dispatched the event. Long-running work should be spawned to avoid blocking subsequent dispatches:

```rust
let api = session.api_handle();
tokio::spawn(async move {
    if let Err(e) = run_long_task(&api).await {
        warn!("background task failed: {e}");
    }
});
```

Sessions expose the shared API through `session.api()` and `Deref<Target = BotApi>`. Use `session.api_handle()` when a background task needs to own the API client.

## See also

- [Client](./client.md) — how a `Client<H: EventHandler>` is constructed and started.
- [Bot API](./bot-api.md) — the REST API available through sessions.
- [Intents](./intents.md) — controls which of these methods can ever be called.

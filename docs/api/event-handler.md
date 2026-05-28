# EventHandler

`EventHandler` is the async trait you implement to react to gateway events. Every method has an empty default body, so you only override what you care about — even an empty struct will compile.

```rust
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _ctx: Context, _ready: Ready) {}
    async fn message_create(&self, _ctx: Context, _message: Message) {}
    /* … */
}
```

The framework dispatches whichever method matches the gateway event type, passing a fresh `Context` (cheap to clone) plus the typed payload. Every method returns `()`; if you need to surface an error, log it locally — gateway-event errors do not interrupt the connection.

## Method catalogue

Methods are grouped by the gateway category that produces them.

### Lifecycle

- `ready(ctx, Ready)` — called once after a successful identify handshake; `ctx.bot_info` is populated.
- `resumed(ctx)` — called after a successful resume.
- `unknown_event(ctx, GatewayEvent)` — catch-all for event types without a first-class callback.

### Channel messages

- `message_create(ctx, Message)` — guild channel messages and `@mentions`.
- `message_delete(ctx, MessageDelete)` — guild message delete event.
- `public_message_delete(ctx, MessageDelete)` — public-domain delete.

### Direct messages

- `direct_message_create(ctx, Message)`
- `direct_message_delete(ctx, MessageDelete)`

### Group / C2C

- `group_message_create(ctx, GroupMessage)`
- `c2c_message_create(ctx, C2CMessage)`
- `friend_add(ctx, C2CManageEvent)` / `friend_del(ctx, C2CManageEvent)`
- `c2c_msg_reject` / `c2c_msg_receive`
- `subscribe_message_status(ctx, SubscribeMessageStatusData)`
- `enter_aio(ctx, EnterAioEvent)`

### Group management

- `group_add_robot` / `group_del_robot`
- `group_msg_reject` / `group_msg_receive`

### Reactions

- `message_reaction_add(ctx, Reaction)` / `message_reaction_remove(ctx, Reaction)`

### Interaction

- `interaction_create(ctx, Interaction)` — buttons and application interaction payloads.

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
- `forum_publish_audit_result(ctx, ForumAuditResult)`
- Open-forum equivalents: `open_forum_thread_*`, `open_forum_post_*`, `open_forum_reply_*`.

## Skeleton implementation

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("logged in as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                let _ = message.reply(&ctx, "Pong!").await;
            }
        }
    }
}
```

`MyBot` only handles two events; every other gateway event silently lands in the empty defaults.

## Concurrency

Each handler invocation runs on the runtime task that dispatched the event. Long-running work should be spawned to avoid blocking subsequent dispatches:

```rust
let ctx = ctx.clone();
tokio::spawn(async move {
    if let Err(e) = run_long_task(&ctx).await {
        warn!("background task failed: {e}");
    }
});
```

`Context` is `Clone`, and `BotApi` is `Arc`-wrapped internally, so this pattern is allocation-free for the common case.

## See also

- [Client](./client.md) — how a `Client<H: EventHandler>` is constructed and started.
- [Context](./context.md) — what each handler method receives.
- [Intents](./intents.md) — controls which of these methods can ever be called.

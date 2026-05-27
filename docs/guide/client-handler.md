# Client and event handler

`Client` and `EventHandler` are the two pieces every bot is built around. `Client` owns the gateway connection and the HTTP layer; `EventHandler` is the trait you implement to react to events the gateway delivers.

## Constructing the client

```rust
let client = Client::new(token, intents, handler, is_sandbox)?;
```

The arguments:

- `token: Token` — your `Token::new(app_id, secret)` (or `Token::from_env()`).
- `intents: Intents` — see [Intents](/guide/intents).
- `handler: H where H: EventHandler` — your handler value (consumed by value, stored in an `Arc` internally).
- `is_sandbox: bool` — `true` selects the sandbox base URL, `false` production.

`Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` is the same constructor with an extra HTTP timeout in seconds; the default is `botrs::DEFAULT_TIMEOUT` (30 s).

After construction, call `client.start().await`. The future resolves only after the gateway connection ends and the event channel drains. Internally `start` validates the token, fetches `/users/@me` and the gateway URL, spawns a session manager that runs each shard, and pumps events back into the event handler.

## The handler trait

`EventHandler` has one `async fn` per dispatched gateway event. Every method has a default empty implementation, so you only override the events you care about. Annotate the impl with `#[async_trait::async_trait]`.

```rust
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("ready as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
        if message.content.as_deref() == Some("!ping") {
            let _ = message.reply(&ctx, "pong").await;
        }
    }
}
```

## Available callbacks

The complete set of callbacks (each takes `&self, Context, <payload>`):

- Lifecycle: `ready`, `resumed`, `error(BotError)`, `unknown_event(GatewayEvent)`.
- Messages: `message_create`, `message_delete`, `direct_message_create`, `direct_message_delete`, `group_message_create`, `c2c_message_create`.
- Reactions: `message_reaction_add`, `message_reaction_remove`.
- Interactions: `interaction_create`.
- Audit: `message_audit_pass`, `message_audit_reject`.
- Guilds and channels: `guild_create`, `guild_update`, `guild_delete`, `channel_create`, `channel_update`, `channel_delete`.
- Members: `guild_member_add`, `guild_member_update`, `guild_member_remove`.
- Audio: `audio_start`, `audio_finish`, `on_mic`, `off_mic`, `audio_or_live_channel_member_enter`, `audio_or_live_channel_member_exit`.
- Forum: `forum_thread_create`/`_update`/`_delete`, `forum_post_create`/`_delete`, `forum_reply_create`/`_delete`, `forum_publish_audit_result`, plus the `open_forum_*` mirrors.
- C2C and group management: `friend_add`, `friend_del`, `c2c_msg_reject`, `c2c_msg_receive`, `subscribe_message_status`, `enter_aio`, `group_add_robot`, `group_del_robot`, `group_msg_reject`, `group_msg_receive`.

A callback only fires if the matching `Intents` flag is enabled — see the [Intents](/guide/intents) page for the mapping.

## The Context parameter

Every callback receives a `Context`:

```rust
pub struct Context {
    api: Arc<BotApi>,
    pub bot_info: Option<BotInfo>,
}
```

The inner API client is the same `BotApi` the client built. `Context` dereferences to `BotApi`, so calls such as `ctx.send_message(...)` and `ctx.get_guild(...)` work directly. `bot_info` is filled in from `/users/@me` once the client starts.

If you need an explicit API reference, call `ctx.api()`. For event replies, `message.reply(&ctx, "pong")` accepts the context because it dereferences to `BotApi`.

## The error callback

If event dispatch fails, the framework calls `EventHandler::error(&self, error: BotError)` once and continues processing the next event. The default implementation logs at `error!`. Override it if you want custom behavior — but the framework does not retry on your behalf, so handler-level retry must come from your own code.

## Spawning work from a handler

A handler call should return promptly so the next event can be dispatched. For long-running work, clone `Context` and move it into `tokio::spawn`:

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    let context = ctx.clone();
    tokio::spawn(async move {
        let params = MessageParams::new_text("done");
        let _ = context.send_message("channel", params).await;
    });
}
```

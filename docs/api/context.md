# Context

`Context` is the request-scoped object passed to every `EventHandler` callback. It bundles the API client, the bot's authentication token, and the cached `BotInfo` so handlers don't need to plumb credentials manually.

```rust
pub struct Context {
    pub api: Arc<BotApi>,
    pub token: Token,
    pub bot_info: Option<BotInfo>,
}
```

`Context` is `Clone`-cheap; share it across spawned tasks.

## Construction

You rarely build `Context` yourself — `Client` constructs it from the same `BotApi` it uses to drive the gateway. The two public helpers are kept for tests and custom embeddings:

- `Context::new(api, token)` — empty `bot_info`.
- `Context::with_bot_info(bot_info)` — builder; populated automatically once the framework receives the gateway `READY`.

## What it gives you

`Context` re-exposes most of `BotApi` so you can write `ctx.send_message(channel, content)` instead of `ctx.api.post_message_with_params(&ctx.token, channel, params)`. The wrapped methods fall into the same categories listed in [Bot API](./bot-api.md):

- **Send shortcuts** — `send_message`, `send_message_with_embed`, `reply_message`, `send_group_message`, `send_c2c_message`. These wrap `MessageParams` for you.
- **Read shortcuts** — `get_channel(s)`, `get_guild(s)`, `get_message(s)`, `get_pins`, `get_message_setting`, `get_message_reaction_users`.
- **Member / role** — `get_guild_member(s)`, `kick_member`, `delete_member_with_options`, `create_guild_role_member`, `delete_guild_role_member`, `member_add_role`, `member_delete_role`.
- **Channel admin** — `create_channel`, `create_private_channel`, `update_channel` / `patch_channel`, `delete_channel`, `put_channel_permissions`, `put_channel_roles_permissions`.
- **Roles** — `create_guild_role`, `update_guild_role`, `delete_guild_role`.
- **Voice / audio** — `update_audio`, `post_audio`, `mute_*`, `cancel_mute_*`, `on_microphone`, `off_microphone`, `list_voice_channel_members`.
- **Reactions / pins** — `put_reaction`, `delete_reaction`, `put_pin`, `delete_pin`, `clean_pins`, `create_message_reaction`, `delete_own_message_reaction`.
- **Announces** — `create_*_announce`, `delete_*_announce`, `clean_*_announces`.
- **Recall / patch** — `recall_message`, `retract_dm_message`, `retract_c2c_message`, `retract_group_message`, `patch_message_with_params`.
- **Setting guides** — `post_setting_guide(_message)`, `post_dm_setting_guide(_message)`.
- **Files** — `post_group_file`, `post_c2c_file`.
- **Webhook sessions** — `create_session`, `check_sessions`, `session_list`, `remove_session`.
- **Interaction** — `put_interaction`.

For routes that aren't wrapped, fall back to `ctx.api.<method>(&ctx.token, …)`.

## Example: replying with embed

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.is_from_bot() { return; }

    let embed = Embed::new()
        .with_title("Reply")
        .with_description("Hello from BotRS!")
        .with_color(0x00ff00);

    if let Err(err) = ctx.send_message_with_embed(
        message.channel_id.as_deref().unwrap_or(""),
        embed,
    ).await {
        warn!("send failed: {err}");
    }
}
```

## Example: kicking a misbehaving user

```rust
ctx.kick_member(&guild_id, &user_id, Some(1), Some("spam")).await?;
```

`kick_member` accepts `add_blacklist_days: Option<u8>` and `reason: Option<&str>` — both default to `None` if you pass it.

## Concurrency

Because `Context` is cheap to clone and `Arc<BotApi>` is shared, fanning work out to spawned tasks is trivial:

```rust
let context = ctx.clone();
tokio::spawn(async move {
    let _ = context.send_message(&channel, "background work done").await;
});
```

The token inside `Context` refreshes itself automatically through `BotApi`'s shared cache; you do not need to pass refreshed tokens between tasks.

## See also

- [Client](./client.md) — owner of the `Context` instances passed to handlers.
- [BotApi](./bot-api.md) — full route catalogue when `Context`'s shortcut isn't enough.
- [Event handler](./event-handler.md) — the trait whose methods receive `Context`.
- [Token](./token.md) — credential model embedded in `Context`.

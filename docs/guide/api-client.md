# API client

`BotApi` is the typed wrapper around QQ Guild's REST API. The `Client` builds one when it starts and exposes it on the `Context` your handlers receive. You can also build one yourself when you need REST access without a gateway.

## From a handler

Inside any `EventHandler` callback, `ctx.api` is an `Arc<BotApi>` and `ctx.token` is the active `Token`. Pass them by reference into any API call:

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let params = MessageParams::new_text("hi");
    let _ = ctx.api
        .post_message_with_params(&ctx.token, msg.channel_id.as_deref().unwrap_or(""), params)
        .await;
}
```

`Context` also offers a few high-level shortcuts that wrap common `BotApi` calls so you can keep handler code short:

- `ctx.send_message(channel_id, content)`
- `ctx.send_message_with_embed(channel_id, content, embed)`
- `ctx.reply_message(channel_id, content, message_id)`
- `ctx.send_group_message(group_openid, content)`
- `ctx.send_c2c_message(openid, content)`
- `ctx.get_guild(guild_id)`, `ctx.get_channel(channel_id)`, `ctx.get_message_setting(guild_id)`
- `ctx.add_reaction(...)`, `ctx.remove_reaction(...)`, plus `recall_message`, `retract_*`, etc.

For anything they don't cover, use `ctx.api` directly.

## Outside a handler

`BotApi::new(http)` takes an `HttpClient`; build one with `HttpClient::new(timeout_secs, is_sandbox)`. Pass a `Token` on every call.

```rust
use botrs::{BotApi, Intents, Token};
use botrs::http::HttpClient;

let http = HttpClient::new(30, false)?;
let api = BotApi::new(http);
let token = Token::from_env()?;

let me = api.get_bot_info(&token).await?;
let guilds = api.get_guilds(&token, None, None, None).await?;
```

When you need to share one client across tasks, wrap it in an `Arc` (the `Client` already does this — `ctx.api` is `Arc<BotApi>`, so cloning is cheap).

## Method shape

`BotApi` exposes endpoint families as `async fn`s that return `Result<T, BotError>`:

- Bot identity: `get_bot_info`, `get_gateway`.
- Guilds: `get_guild`, `get_guilds`, `get_guilds_with_pager`, `get_message_setting`.
- Channels: `get_channel`, `get_channels`, `create_channel`, `update_channel`, `delete_channel`, `create_private_channel`.
- Members and roles: `get_guild_member`, `get_guild_members`, `delete_member`, `get_guild_roles`, `create_guild_role`, `update_guild_role`, `delete_guild_role`, `create_guild_role_member`, `delete_guild_role_member`.
- Messages: `post_message_with_params`, `patch_message_with_params`, `recall_message`, `get_message`, `get_messages`, `get_messages_with_params`, plus the `_group_`, `_c2c_`, `_dms_` variants.
- Reactions, pins, announcements, schedules, mute, audio (`post_audio`, `on_microphone`, `off_microphone`), forum threads (`get_threads`, `create_thread`, `delete_thread`, `get_thread`).

## Errors

Every method returns `botrs::Result<T>`. Errors arrive as `BotError::Api { code, message }` for QQ-side rejections, `BotError::RateLimit { retry_after }` when QQ replies 429, `BotError::AuthenticationFailed` / `Forbidden` / `NotFound` for the obvious HTTP statuses, and `BotError::Http` / `BotError::Timeout` for transport failures. See [error handling](/guide/error-handling) for the full enum.

# API client

`BotApi` is the typed wrapper around QQ Guild's REST API. The `Client` builds one when it starts, stores the token in it, and exposes it through the `Context` your handlers receive. You can also build one yourself when you need REST access without a gateway.

## From a handler

Inside any `EventHandler` callback, `ctx` dereferences to `BotApi`, so REST calls do not need an explicit token argument:

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let params = MessageParams::new_text("hi");
    let _ = ctx.send_message(msg.channel_id.as_deref().unwrap_or(""), params)
        .await;
}
```

Common calls stay short because they are just `BotApi` methods on `ctx`:

- `ctx.send_message(channel_id, params)`
- `ctx.send_group_message(group_openid, params)`
- `ctx.send_c2c_message(openid, params)`
- `ctx.send_direct_message(guild_id, params)`
- `ctx.get_guild(guild_id)`, `ctx.get_channel(channel_id)`, `ctx.get_message_setting(guild_id)`
- `ctx.put_reaction(...)`, `ctx.delete_reaction(...)`, plus `recall_message`, `retract_*`, etc.

For helper APIs that require a `BotApi` reference, pass `&ctx` or call `ctx.api()`.

## Outside a handler

`BotApi::new(http, token)` takes an `HttpClient` and stores the token for all later calls.

```rust
use botrs::{BotApi, Intents, Token};
use botrs::http::HttpClient;

let http = HttpClient::new(30, false)?;
let token = Token::from_env()?;
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
let guilds = api.get_guilds(None, None, None).await?;
```

When you need to share one standalone API client across tasks, wrap `BotApi` in an `Arc`. In handlers, cloning `Context` is usually enough because it already holds the shared client.

## Method shape

`BotApi` exposes endpoint families as `async fn`s that return `Result<T, BotError>`:

- Bot identity: `get_bot_info`, `get_gateway`.
- Guilds: `get_guild`, `get_guilds`, `get_guilds_with_pager`, `get_message_setting`.
- Channels: `get_channel`, `get_channels`, `create_channel`, `update_channel`, `delete_channel`, `create_private_channel`.
- Members and roles: `get_guild_member`, `get_guild_members`, `delete_member`, `get_guild_roles`, `create_guild_role`, `update_guild_role`, `delete_guild_role`, `create_guild_role_member`, `delete_guild_role_member`.
- Messages: `send_message`, `edit_message`, `recall_message`, `get_message`, `get_messages`, `get_messages_with_params`, plus the `_group_`, `_c2c_`, `_dms_` variants.
- Reactions, pins, announcements, schedules, mute, audio (`post_audio`, `on_microphone`, `off_microphone`), forum threads (`get_threads`, `create_thread`, `delete_thread`, `get_thread`).

## Errors

Every method returns `botrs::Result<T>`. Errors arrive as `BotError::Api { code, message }` for QQ-side rejections, `BotError::RateLimit { retry_after }` when QQ replies 429, `BotError::AuthenticationFailed` / `Forbidden` / `NotFound` for the obvious HTTP statuses, and `BotError::Http` / `BotError::Timeout` for transport failures. See [error handling](/guide/error-handling) for the full enum.

# API client

`BotApi` is the REST client used by the current bot runtime path. After a gateway event arrives, handlers usually call it through `Context` to reply, recall messages, upload group/C2C files, manage announcements, schedules and pins, inspect reaction users, and create API permission requests.

## In a Handler

The `Context` received by an event callback can be used directly as a `BotApi`:

```rust
let params = MessageParams::new_text("hi");
ctx.send_message(channel_id, params).await?;
```

No token is passed in this path. `Client` creates the API client during startup; `Context` exposes that shared client to handlers.

Common calls include:

- Messages: `send_message`, `send_group_message`, `send_c2c_message`, `send_direct_message`, `recall_message`
- Direct-message session: `create_direct_message`
- Files: `post_group_file`, `post_c2c_file`
- Reactions: `put_reaction`, `delete_reaction`, `get_reaction_users`
- Pins: `put_pin`, `delete_pin`, `get_pins`
- Announcements: `create_announce`, `create_recommend_announce`, `delete_announce`
- Schedules: `get_schedules`, `get_schedule`, `create_schedule`, `update_schedule`, `delete_schedule`
- Permissions: `get_api_permissions`, `post_permission_demand`

## Standalone Use

If you want a REST-only tool without running the gateway, construct `BotApi` manually:

```rust
let http = HttpClient::new(30, false)?;
let token = Token::from_env()?;
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
let gateway = api.get_gateway().await?;
```

`BotApi` is cloneable. Clones still share the HTTP client and token cache.

## Parameter Structs

Message sending uses parameter structs:

```rust
ctx.send_message(&channel_id, MessageParams::new_text("channel")).await?;
ctx.send_group_message(&group_openid, GroupMessageParams::new_text("group")).await?;
ctx.send_c2c_message(&openid, C2CMessageParams::new_text("c2c")).await?;
```

Rich payloads are field combinations: ark, embed, markdown, keyboard, media, and related fields are set directly on the corresponding params value. This keeps protocol fields explicit at the call site.

## Errors

All REST methods return `botrs::Result<T>`. In handlers, errors are usually handled locally:

```rust
if let Err(err) = ctx.send_message(&channel_id, params).await {
    tracing::warn!("send failed: {err}");
}
```

Match on `BotError` when you need variant-specific behavior.

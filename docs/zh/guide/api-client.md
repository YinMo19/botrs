# API 客户端

`BotApi` 是对 QQ 频道 REST API 的类型化封装。`Client` 在启动时会创建一个 `BotApi`，把 token 存在其中，并通过 `Context` 暴露给事件处理器；如果需要在网关之外调用 REST，也可以自行构造。

## 在事件处理器中

任何 `EventHandler` 回调里，`ctx` 都会解引用到 `BotApi`，REST 调用不需要显式传 token：

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let params = MessageParams::new_text("你好");
    let _ = ctx.send_message(msg.channel_id.as_deref().unwrap_or(""), params)
        .await;
}
```

常用调用直接写在 `ctx` 上即可：

- `ctx.send_message(channel_id, params)`
- `ctx.send_group_message(group_openid, params)`
- `ctx.send_c2c_message(openid, params)`
- `ctx.send_direct_message(guild_id, params)`
- `ctx.get_guild(guild_id)`、`ctx.get_channel(channel_id)`、`ctx.get_message_setting(guild_id)`
- `ctx.put_reaction(...)`、`ctx.delete_reaction(...)`，以及 `recall_message`、`retract_*` 等

需要 `BotApi` 引用的辅助 API 可以传 `&ctx`，或调用 `ctx.api()` 显式借出。

## 在事件处理器之外

`BotApi::new(http, token)` 接受一个 `HttpClient`，并把 token 存起来供后续所有调用使用。

```rust
use botrs::{BotApi, Intents, Token};
use botrs::http::HttpClient;

let http = HttpClient::new(30, false)?;
let token = Token::from_env()?;
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
let guilds = api.get_guilds(None, None, None).await?;
```

需要在多个任务中共享独立 API 客户端时，用 `Arc` 包裹 `BotApi`。在 handler 中通常克隆 `Context` 即可，因为它内部已经持有共享客户端。

## 方法形态

`BotApi` 把每类端点封装成返回 `Result<T, BotError>` 的 `async fn`：

- 机器人身份：`get_bot_info`、`get_gateway`。
- 频道：`get_guild`、`get_guilds`、`get_guilds_with_pager`、`get_message_setting`。
- 子频道：`get_channel`、`get_channels`、`create_channel`、`update_channel`、`delete_channel`、`create_private_channel`。
- 成员与角色：`get_guild_member`、`get_guild_members`、`delete_member`、`get_guild_roles`、`create_guild_role`、`update_guild_role`、`delete_guild_role`、`create_guild_role_member`、`delete_guild_role_member`。
- 消息：`send_message`、`edit_message`、`recall_message`、`get_message`、`get_messages`、`get_messages_with_params`，以及 `_group_`、`_c2c_`、`_dms_` 等变体。
- 表情、置顶、公告、日程、禁言、音频（`post_audio`、`on_microphone`、`off_microphone`），以及论坛主题（`get_threads`、`create_thread`、`delete_thread`、`get_thread`）。

## 错误

所有方法返回 `botrs::Result<T>`。常见错误：QQ 侧拒绝是 `BotError::Api { code, message }`；429 限流是 `BotError::RateLimit { retry_after }`；HTTP 401/403/404 分别映射到 `AuthenticationFailed` / `Forbidden` / `NotFound`；传输层失败是 `BotError::Http` 或 `BotError::Timeout`。完整变体见 [错误处理](/zh/guide/error-handling)。

# API 客户端

`BotApi` 是对 QQ 频道 REST API 的类型化封装。`Client` 在启动时会创建一个 `BotApi`，并通过 `Context` 暴露给事件处理器；如果需要在网关之外调用 REST，也可以自行构造。

## 在事件处理器中

任何 `EventHandler` 回调里，`ctx.api` 都是 `Arc<BotApi>`，`ctx.token` 是当前的 `Token`，按引用传给任意 API：

```rust
async fn message_create(&self, ctx: Context, msg: Message) {
    let params = MessageParams::new_text("你好");
    let _ = ctx.api
        .post_message_with_params(&ctx.token, msg.channel_id.as_deref().unwrap_or(""), params)
        .await;
}
```

`Context` 还封装了若干常用 `BotApi` 调用，便于保持回调简短：

- `ctx.send_message(channel_id, content)`
- `ctx.send_message_with_embed(channel_id, content, embed)`
- `ctx.reply_message(channel_id, content, message_id)`
- `ctx.send_group_message(group_openid, content)`
- `ctx.send_c2c_message(openid, content)`
- `ctx.get_guild(guild_id)`、`ctx.get_channel(channel_id)`、`ctx.get_message_setting(guild_id)`
- `ctx.add_reaction(...)`、`ctx.remove_reaction(...)`，以及 `recall_message`、`retract_*` 等

未覆盖的接口请直接走 `ctx.api`。

## 在事件处理器之外

`BotApi::new(http)` 接受一个 `HttpClient`，用 `HttpClient::new(timeout_secs, is_sandbox)` 创建它。每次调用都需要传入 `Token`。

```rust
use botrs::{BotApi, Intents, Token};
use botrs::http::HttpClient;

let http = HttpClient::new(30, false)?;
let api = BotApi::new(http);
let token = Token::from_env()?;

let me = api.get_bot_info(&token).await?;
let guilds = api.get_guilds(&token, None, None, None).await?;
```

需要在多个任务中共享时用 `Arc` 包裹（`Client` 已经这么做了 —— `ctx.api` 即 `Arc<BotApi>`，克隆代价极小）。

## 方法形态

`BotApi` 把每类端点封装成返回 `Result<T, BotError>` 的 `async fn`：

- 机器人身份：`get_bot_info`、`get_gateway`。
- 频道：`get_guild`、`get_guilds`、`get_guilds_with_pager`、`get_message_setting`。
- 子频道：`get_channel`、`get_channels`、`create_channel`、`update_channel`、`delete_channel`、`create_private_channel`。
- 成员与角色：`get_guild_member`、`get_guild_members`、`delete_member`、`get_guild_roles`、`create_guild_role`、`update_guild_role`、`delete_guild_role`、`create_guild_role_member`、`delete_guild_role_member`。
- 消息：`post_message_with_params`、`patch_message_with_params`、`recall_message`、`get_message`、`get_messages`、`get_messages_with_params`，以及 `_group_`、`_c2c_`、`_dms_` 等变体。
- 表情、置顶、公告、日程、禁言、音频（`PostAudio`、`PutMic`、`DeleteMic`），以及论坛主题（`get_threads`、`create_thread`、`delete_thread`、`get_thread`）。

crate 还在同一 HTTP 层之上导出了 `PascalCase` 入口（`api.PostMessage`、`api.Channels` 等），方便你直接对照 QQ Bot API 文档中的方法名，而不是 Rust 习惯的 snake_case。

## 错误

所有方法返回 `botrs::Result<T>`。常见错误：QQ 侧拒绝是 `BotError::Api { code, message }`；429 限流是 `BotError::RateLimit { retry_after }`；HTTP 401/403/404 分别映射到 `AuthenticationFailed` / `Forbidden` / `NotFound`；传输层失败是 `BotError::Http` 或 `BotError::Timeout`。完整变体见 [错误处理](/zh/guide/error-handling)。

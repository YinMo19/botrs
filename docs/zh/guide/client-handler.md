# 客户端与事件处理器

`Client` 与 `EventHandler` 是每个机器人都会用到的两个核心类型。`Client` 持有网关连接和 HTTP 层；`EventHandler` 是你实现的 trait，用于响应网关投递的事件。

## 构造客户端

```rust
let client = Client::new(token, intents, handler, is_sandbox)?;
```

参数含义：

- `token: Token` —— `Token::new(app_id, secret)`（或 `Token::from_env()`）。
- `intents: Intents` —— 见 [Intents](/zh/guide/intents)。
- `handler: H where H: EventHandler` —— 你的处理器实例（按值传入，框架内部存放在 `Arc` 中）。
- `is_sandbox: bool` —— `true` 走沙箱基础地址，`false` 走正式环境。

`Client::with_config(token, intents, handler, timeout_secs, is_sandbox)` 是同一构造函数，多了一个 HTTP 超时（秒），默认值为 `botrs::DEFAULT_TIMEOUT`（30 秒）。

构造完成后调用 `client.start().await`。该 future 在网关连接结束、事件通道关闭后才会返回。`start` 内部会：校验 token、调用 `/users/@me` 与网关 URL 接口、启动一个会话管理器为每个 shard 维持连接，并把事件回送给处理器。

## 处理器 trait

`EventHandler` 中每个网关事件对应一个 `async fn`，都有空的默认实现，因此只需要重写关心的事件。impl 块需要标注 `#[async_trait::async_trait]`。

```rust
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, session: ReadySession) {
        tracing::info!("ready as {}", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }
        if message.content.as_deref() == Some("!ping") {
            let _ = session.reply("pong").await;
        }
    }
}
```

## 可重写的回调

每个回调都接收一个 session 参数：

- 生命周期：`ready`、`resumed`、`error(BotError)`、`unknown_event(GatewayEvent)`。
- 消息：`message_create`、`message_delete`、`direct_message_create`、`direct_message_delete`、`group_message_create`、`c2c_message_create`。
- 表情：`message_reaction_add`、`message_reaction_remove`。
- 交互：`interaction_create`。
- 审核：`message_audit_pass`、`message_audit_reject`。
- 频道与子频道：`guild_create`/`_update`/`_delete`、`channel_create`/`_update`/`_delete`。
- 成员：`guild_member_add`、`guild_member_update`、`guild_member_remove`。
- 音频：`audio_start`、`audio_finish`、`on_mic`、`off_mic`、`audio_or_live_channel_member_enter`、`audio_or_live_channel_member_exit`。
- 论坛：`forum_thread_create`/`_update`/`_delete`、`forum_post_create`/`_delete`、`forum_reply_create`/`_delete`、`forum_publish_audit_result`，以及对应的 `open_forum_*` 公开论坛事件。
- C2C 与群管理：`friend_add`、`friend_del`、`c2c_msg_reject`、`c2c_msg_receive`、`subscribe_message_status`、`enter_aio`、`group_add_robot`、`group_del_robot`、`group_msg_reject`、`group_msg_receive`。

只有当对应的 `Intents` 标志启用时，回调才会被触发，事件与标志的对应关系见 [Intents](/zh/guide/intents)。

## Session 参数

每个回调都会收到一个 session 对象。消息创建回调用 `ChannelReplySession`、`GroupReplySession`、`C2CReplySession`、`DirectReplySession` 这类 reply session；其他回调用 `ReadySession`、`MemberSession`、`MessageDeleteSession` 等 event session。

通用事件通过 `session.event()` 读取载荷，回复 session 通过 `session.message()` 读取入站消息。每个 session 都通过 `session.api()`、`session.api_handle()` 和 `Deref<Target = BotApi>` 暴露共享 REST 客户端。

## 错误回调

事件分派若发生错误，框架会调用一次 `EventHandler::error(&self, error: BotError)` 然后继续处理下一个事件。默认实现以 `error!` 级别记录日志，可按需重写。框架本身不会替你重试，处理器内部的重试必须自行实现。

## 在处理器中派发任务

回调应尽快返回，避免阻塞下一个事件。处理耗时任务时，把拥有型 API handle 移入 `tokio::spawn`：

```rust
async fn message_create(&self, session: ChannelReplySession) {
    let api = session.api_handle();
    tokio::spawn(async move {
        let params = MessageParams::new_text("done");
        let _ = api.send_message("channel", params).await;
    });
}
```

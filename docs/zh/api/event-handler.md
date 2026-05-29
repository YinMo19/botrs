# EventHandler

`EventHandler` 是你需要实现的 async trait，用于响应网关事件。每个方法都有一个空的默认实现，因此只需要重写自己关心的事件——一个空结构体也能编译通过。

```rust
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _session: ReadySession) {}
    async fn message_create(&self, _session: ChannelReplySession) {}
    /* … */
}
```

框架会根据网关事件类型分发到对应方法。所有公开回调都会收到 session 对象；可回复的消息事件使用专用 reply session，生命周期和普通事件使用 event session，并通过 `session.event()` 读取载荷。所有方法返回 `()`；如果需要上报错误，请在本地记录日志——网关事件的回调错误不会中断连接。

## 方法目录

按事件类别分组列出。

### 生命周期

- `ready(ReadySession)` —— identify 握手成功后调用一次；bot 信息可通过 `session.bot_info()` 读取。
- `resumed(ResumeSession)` —— resume 成功后调用。
- `unknown_event(UnknownEventSession)` —— 没有一等回调的事件类型会进入这个兜底方法。

### 频道消息

- `message_create(ChannelReplySession)` —— 频道消息及 `@mention`。
- `message_delete(MessageDeleteSession)` —— 频道消息被删除。
- `public_message_delete(MessageDeleteSession)` —— 公域消息删除。

### 私信

- `direct_message_create(DirectReplySession)`
- `direct_message_delete(MessageDeleteSession)`

### 群 / C2C

- `group_message_create(GroupReplySession)`
- `c2c_message_create(C2CReplySession)`
- `friend_add(C2CManageSession)` / `friend_del(C2CManageSession)`
- `c2c_msg_reject(C2CManageSession)` / `c2c_msg_receive(C2CManageSession)`
- `subscribe_message_status(SubscribeMessageStatusSession)`
- `enter_aio(EnterAioSession)`

### 群管理

- `group_add_robot` / `group_del_robot`
- `group_msg_reject` / `group_msg_receive`

### 表情回应

- `message_reaction_add(ReactionSession)` / `message_reaction_remove(ReactionSession)`

### 互动

- `interaction_create(InteractionSession)` —— 按钮、应用互动 payload。

### 音频 / 语音

- `audio_start` / `audio_finish` / `on_mic` / `off_mic` —— 载荷为 `Audio`。
- `audio_or_live_channel_member_enter` / `audio_or_live_channel_member_exit` —— 载荷为 `PublicAudio`。

### 频道 / 子频道

- `guild_create` / `guild_update` / `guild_delete`
- `channel_create` / `channel_update` / `channel_delete`
- `guild_member_add` / `guild_member_update` / `guild_member_remove`

### 审核

- `message_audit_pass` / `message_audit_reject`

### 论坛

- `forum_thread_create` / `forum_thread_update` / `forum_thread_delete`
- `forum_post_create` / `forum_post_delete`
- `forum_reply_create` / `forum_reply_delete`
- `forum_publish_audit_result(ForumAuditSession)`
- 开放论坛等价方法：`open_forum_thread_*`、`open_forum_post_*`、`open_forum_reply_*`。

## 最小实现

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
        if message.author.bot {
            return;
        }

        if message.content.trim() == "!ping" {
            let _ = session.reply("Pong!").await;
        }
    }
}
```

`MyBot` 只处理两种事件，其他网关事件都会走默认的空实现。

## 并发

每个回调都在派发该事件的运行时任务上执行。耗时操作应当 spawn 出去，避免阻塞后续派发：

```rust
let api = session.api_handle();
tokio::spawn(async move {
    if let Err(e) = run_long_task(&api).await {
        warn!("background task failed: {e}");
    }
});
```

Session 通过 `session.api()` 以及 `Deref<Target = BotApi>` 暴露共享 API。后台任务需要拥有 API 客户端时，使用 `session.api_handle()`。

## 参见

- [Client](./client.md) —— 如何构造和启动 `Client<H: EventHandler>`。
- [Bot API](./bot-api.md) —— session 中可用的 REST API。
- [Intents](./intents.md) —— 决定哪些方法可能被触发。

# EventHandler

`EventHandler` 是你需要实现的 async trait，用于响应网关事件。每个方法都有一个空的默认实现，因此只需要重写自己关心的事件——一个空结构体也能编译通过。

```rust
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn ready(&self, _ctx: Context, _ready: Ready) {}
    async fn message_create(&self, _ctx: Context, _message: Message) {}
    /* … */
}
```

框架会根据网关事件类型分发到对应方法，传入新构造的 `Context`（克隆成本很低）以及类型化的载荷。所有方法返回 `()`；如果需要上报错误，请在本地记录日志——网关事件的回调错误不会中断连接。

## 方法目录

按事件类别分组列出。

### 生命周期

- `ready(ctx, Ready)` —— identify 握手成功后调用一次；此时 `ctx.bot_info` 已被填充。
- `resumed(ctx)` —— resume 成功后调用。
- `error_notify(ctx, BotError)` —— 网关层错误被捕获时触发。
- `plain_event(ctx, GatewayEvent)` —— 通过 `register_handlers` 注册的全量原始事件兜底回调。

### 频道消息

- `message_create(ctx, Message)` —— 频道消息及 `@mention`。
- `message_delete(ctx, MessageDelete)` —— 频道消息被删除。
- `public_message_delete(ctx, MessageDelete)` —— 公域消息删除。
- `at_message_create(ctx, Message)` —— 单独配置 `@mention` 时使用的别名事件。

### 私信

- `direct_message_create(ctx, Message)`
- `direct_message_delete(ctx, MessageDelete)`

### 群 / C2C

- `group_message_create(ctx, GroupMessage)`
- `c2c_message_create(ctx, C2CMessage)`
- `friend_add(ctx, C2CManageEvent)` / `friend_del(ctx, C2CManageEvent)`
- `c2c_msg_reject` / `c2c_msg_receive`
- `subscribe_message_status(ctx, SubscribeMessageStatusData)`
- `enter_aio(ctx, EnterAioEvent)`

### 群管理

- `group_add_robot` / `group_del_robot`
- `group_msg_reject` / `group_msg_receive`

### 表情回应

- `message_reaction_add(ctx, Reaction)` / `message_reaction_remove(ctx, Reaction)`

### 互动

- `interaction_create(ctx, Interaction)` —— 按钮、应用互动事件；通过 `ctx.put_interaction(...)` 应答。

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
- `forum_publish_audit_result(ctx, ForumAuditResult)`
- 开放论坛等价方法：`open_forum_thread_*`、`open_forum_post_*`、`open_forum_reply_*`。

## 最小实现

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("logged in as {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() { return; }
        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}
```

`MyBot` 只处理两种事件，其他网关事件都会走默认的空实现。

## 并发

每个回调都在派发该事件的运行时任务上执行。耗时操作应当 spawn 出去，避免阻塞后续派发：

```rust
let ctx = ctx.clone();
tokio::spawn(async move {
    if let Err(e) = run_long_task(&ctx).await {
        warn!("background task failed: {e}");
    }
});
```

`Context` 是 `Clone` 的，`BotApi` 内部用 `Arc` 包裹，所以常见路径几乎不产生分配。

## 参见

- [Client](./client.md) —— 如何构造和启动 `Client<H: EventHandler>`。
- [Context](./context.md) —— 每个方法都会收到的上下文对象。
- [Intents](./intents.md) —— 决定哪些方法可能被触发。

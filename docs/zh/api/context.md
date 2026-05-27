# Context

`Context` 是传给每个 `EventHandler` 回调的请求作用域对象。它持有共享 API 客户端和缓存的 `BotInfo`；认证 token 存在 `BotApi` 内部。

```rust
pub struct Context {
    api: Arc<BotApi>,
    pub bot_info: Option<BotInfo>,
}
```

`Context` 实现了 `Clone`，复制成本很低；它还实现了 `Deref<Target = BotApi>`，所以 handler 里可以直接在 `ctx` 上调用 `BotApi` 方法。

## 构造

无需自行构造 `Context`。`Client` 会用驱动网关的同一个 `BotApi` 创建它，并在网关 `READY` 数据可用后填充 `bot_info`。

## 提供的能力

因为 `Context` 会解引用到 `BotApi`，[Bot API](./bot-api.md) 里列出的 REST 方法都可以直接调用：

```rust
ctx.send_message(channel_id, MessageParams::new_text("pong")).await?;
ctx.send_group_message(group_openid, GroupMessageParams::new_text("pong")).await?;
ctx.recall_message(channel_id, message_id, Some(true)).await?;
```

需要 `BotApi` 引用的辅助 API 可以直接传 `&ctx`。如果要显式借用，`ctx.api()` 返回 `&BotApi`。

## 示例：使用 embed 回复

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.author.as_ref().and_then(|author| author.bot).unwrap_or_default() { return; }

    let params = MessageParams {
        embed: Some(embed),
        ..Default::default()
    };

    if let Err(err) = ctx
        .send_message(message.channel_id.as_deref().unwrap_or(""), params)
        .await
    {
        warn!("send failed: {err}");
    }
}
```

## 示例：删除成员

```rust
ctx.delete_member(&guild_id, &user_id, Some(true), Some(1)).await?;
```

`delete_member` 接受 `add_blacklist: Option<bool>` 与 `delete_history_msg_days: Option<i32>`，使用平台默认值时传 `None`。

## 并发

`Context` clone 成本极低，且内部 `Arc<BotApi>` 是共享的，把任务派出去非常容易：

```rust
let context = ctx.clone();
tokio::spawn(async move {
    let params = MessageParams::new_text("background work done");
    let _ = context.send_message(&channel, params).await;
});
```

`BotApi` 持有的 token 会通过共享缓存自动续期，不需要在任务之间手动传递最新 token。

## 参见

- [Client](./client.md) —— 创建并把 `Context` 传给事件回调的对象。
- [BotApi](./bot-api.md) —— 完整路由目录。
- [事件处理器](./event-handler.md) —— 接收 `Context` 的 trait。
- [Token](./token.md) —— 嵌入 `BotApi` 的凭证模型。

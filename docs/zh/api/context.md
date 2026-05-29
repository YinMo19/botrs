# Sessions

事件回调现在是 session-first。`Context` 退为框架内部运行时细节，应用侧 handler 收到的是 session 对象。

可回复的消息事件使用专用 session：

- `ChannelReplySession`
- `DirectReplySession`
- `GroupReplySession`
- `C2CReplySession`

其他事件使用 `EventSession<T>` 的类型别名，例如 `ReadySession`、`MessageDeleteSession`、`MemberSession`、`ReactionSession` 和 `OpenForumSession`。

## 读取事件数据

使用对应 session 的访问方法：

```rust
async fn ready(&self, session: ReadySession) {
    tracing::info!("ready as {}", session.event().user.username);
}

async fn message_create(&self, mut session: ChannelReplySession) {
    let message = session.message().clone();
    if message.author.bot {
        return;
    }

    if message.content.trim() == "!ping" {
        let _ = session.reply("pong").await;
    }
}
```

对于通用事件 session，`session.event()` 返回类型化网关载荷。

## 调用 API

每个 session 都暴露共享 REST 客户端：

```rust
let api = session.api();
```

Session 也会解引用到 `BotApi`，所以仍然可以直接调用 REST 方法：

```rust
let pins = session.get_pins(channel_id).await?;
```

需要让工作超过当前回调生命周期时，把拥有型 API handle 移入任务：

```rust
let api = session.api_handle();
tokio::spawn(async move {
    let _ = api.get_bot_info().await;
});
```

## bot_info

`session.bot_info()` 返回启动阶段获取到的 bot 信息。

```rust
let bot_name = session
    .bot_info()
    .map(|bot| bot.username.as_str())
    .unwrap_or("bot");
```

## 参见

- [BotApi](./bot-api.md)
- [EventHandler](./event-handler.md)
- [Client](./client.md)

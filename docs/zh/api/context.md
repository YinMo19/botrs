# Context

`Context` 是传给每个 `EventHandler` 回调的请求作用域对象。它把 API 客户端、机器人令牌、缓存的 `BotInfo` 打包在一起，避免在各个处理器里手动传递凭证。

```rust
pub struct Context {
    pub api: Arc<BotApi>,
    pub token: Token,
    pub bot_info: Option<BotInfo>,
}
```

`Context` 实现了 `Clone`，复制成本很低，可以放心地在 spawn 出的任务之间共享。

## 构造

通常无需自己构造 `Context` —— `Client` 会用驱动网关的同一个 `BotApi` 帮你创建。两个公开辅助方法主要用于测试或嵌入定制：

- `Context::new(api, token)` —— `bot_info` 为空。
- `Context::with_bot_info(bot_info)` —— 链式 setter；框架收到网关 `READY` 后会自动填充。

## 提供的能力

`Context` 把 `BotApi` 上多数路由再包了一层，方便你写 `ctx.send_message(channel, content)`，而不必每次都 `ctx.api.post_message_with_params(&ctx.token, channel, params)`。封装的方法与 [Bot API](./bot-api.md) 中的分类一一对应：

- **发送类捷径** —— `send_message`、`send_message_with_embed`、`reply_message`、`send_group_message`、`send_c2c_message`。它们替你构建 `MessageParams`。
- **读取类捷径** —— `get_channel(s)`、`get_guild(s)`、`get_message(s)`、`get_pins`、`get_message_setting`、`get_message_reaction_users`。
- **成员 / 角色** —— `get_guild_member(s)`、`kick_member`、`delete_member_with_options`、`create_guild_role_member`、`delete_guild_role_member`、`member_add_role`、`member_delete_role`。
- **子频道管理** —— `create_channel`、`create_private_channel`、`update_channel` / `patch_channel`、`delete_channel`、`put_channel_permissions`、`put_channel_roles_permissions`。
- **角色** —— `create_guild_role`、`update_guild_role`、`delete_guild_role`。
- **语音 / 音频** —— `update_audio`、`post_audio`、`mute_*`、`cancel_mute_*`、`on_microphone`、`off_microphone`、`list_voice_channel_members`。
- **回应 / 精华** —— `put_reaction`、`delete_reaction`、`put_pin`、`delete_pin`、`clean_pins`、`create_message_reaction`、`delete_own_message_reaction`。
- **公告** —— `create_*_announce`、`delete_*_announce`、`clean_*_announces`。
- **撤回 / 编辑** —— `recall_message`、`retract_dm_message`、`retract_c2c_message`、`retract_group_message`、`patch_message_with_params`。
- **设置引导** —— `post_setting_guide(_message)`、`post_dm_setting_guide(_message)`。
- **文件** —— `post_group_file`、`post_c2c_file`。
- **Webhook 会话** —— `create_session`、`check_sessions`、`session_list`、`remove_session`。
- **互动** —— `put_interaction`。

未被封装的接口可直接 `ctx.api.<method>(&ctx.token, …)` 调用。

## 示例：使用 embed 回复

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if message.is_from_bot() { return; }

    let embed = Embed::new()
        .with_title("Reply")
        .with_description("Hello from BotRS!")
        .with_color(0x00ff00);

    if let Err(err) = ctx.send_message_with_embed(
        message.channel_id.as_deref().unwrap_or(""),
        embed,
    ).await {
        warn!("send failed: {err}");
    }
}
```

## 示例：踢出成员

```rust
ctx.kick_member(&guild_id, &user_id, Some(1), Some("spam")).await?;
```

`kick_member` 接受 `add_blacklist_days: Option<u8>` 和 `reason: Option<&str>`，省略时传 `None` 即可。

## 并发

`Context` clone 成本极低，且内部 `Arc<BotApi>` 是共享的，把任务派出去非常容易：

```rust
let context = ctx.clone();
tokio::spawn(async move {
    let _ = context.send_message(&channel, "background work done").await;
});
```

`Context` 持有的 token 通过 `BotApi` 的共享缓存自动续期，不需要在任务之间手动传递最新的 token。

## 参见

- [Client](./client.md) —— 创建并把 `Context` 传给事件回调的对象。
- [BotApi](./bot-api.md) —— 当 `Context` 的捷径不够用时，可以查阅完整路由目录。
- [事件处理器](./event-handler.md) —— 接收 `Context` 的 trait。
- [Token](./token.md) —— 嵌入 `Context` 的凭证模型。

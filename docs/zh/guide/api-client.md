# API 客户端

`BotApi` 是当前 bot 运行链路使用的 REST 客户端。收到网关事件以后，handler 通常通过 session 调用它发送回复、撤回消息、上传群/C2C 文件，并管理频道与子频道资源、公告、日程、精华、表情回应、权限、身份组、禁言和音频控制。

## 在 handler 里使用

可回复 session 提供与当前场景匹配的发送 helper：

```rust
let params = MessageParams::new_text("hi");
session.send_message(params).await?;
```

所有 session 也会解引用到 `BotApi`，因此普通 REST 方法可以直接写在 `session` 上：

```rust
let pins = session.get_pins(channel_id).await?;
```

这条链路里不需要传 token。`Client` 启动时已经创建了 API 客户端，session 通过 `session.api()` 与 `session.api_handle()` 把它交给 handler。

常见调用包括：

- 消息：`send_message`、`send_group_message`、`send_c2c_message`、`send_direct_message`、`recall_message`
- 私信会话：`create_direct_message`
- 频道/子频道/成员/身份组：`get_guild`、`list_channels`、`create_channel`、`get_guild_member`、`list_roles`
- 文件：`post_group_file`、`post_c2c_file`
- 表情回应：`put_reaction`、`delete_reaction`、`get_reaction_users`
- 精华：`put_pin`、`delete_pin`、`clean_pins`、`get_pins`
- 公告：`create_channel_announce`、`delete_channel_announce`、`create_announce`、`create_recommend_announce`、`delete_announce`
- 日程：`get_schedules`、`get_schedule`、`create_schedule`、`update_schedule`、`delete_schedule`
- 权限与音频：`get_api_permissions`、`post_permission_demand`、`get_channel_permissions`、`post_audio`、`put_mic`、`delete_mic`

## 独立使用

如果你只想写一个 REST 工具，不跑网关，可以手动创建 `BotApi`：

```rust
let http = HttpClient::new(30, false)?;
let token = Token::from_env()?;
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
let gateway = api.get_gateway().await?;
```

`BotApi` 可以 clone。clone 之后仍共享 HTTP client 和 token 缓存。

## 参数结构体

发送消息统一使用参数结构体：

```rust
session.send_message(MessageParams::new_text("channel")).await?;
group_session.send_message(GroupMessageParams::new_text("group")).await?;
c2c_session.send_message(C2CMessageParams::new_text("c2c")).await?;
```

常见富消息使用 `new_markdown`、`new_embed`、`new_ark`、`new_keyboard` 和群/C2C 的 `new_media` 构造器。处于 reply session 内时，优先使用对应的 `send_*_message` helper。只有 helper 和构造器覆盖不到的协议组合，才直接设置 params 字段。

## 错误

所有 REST 方法返回 `botrs::Result<T>`。在 handler 里一般就地处理错误：

```rust
if let Err(err) = session.send_message(params).await {
    tracing::warn!("send failed: {err}");
}
```

需要区分错误类型时，再匹配 `BotError`。

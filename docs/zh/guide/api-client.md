# API 客户端

`BotApi` 是当前 bot 运行链路使用的 REST 客户端。收到网关事件以后，handler 通常通过 `Context` 调用它发送回复、撤回消息、上传群/C2C 文件、处理公告/日程/精华/表情回应和 API 权限申请。

## 在 handler 里使用

事件回调拿到的 `Context` 可以直接当 `BotApi` 用：

```rust
let params = MessageParams::new_text("hi");
ctx.send_message(channel_id, params).await?;
```

这条链路里不需要传 token。`Client` 启动时已经创建了 API 客户端，`Context` 只是把它交给 handler。

常见调用包括：

- 消息：`send_message`、`send_group_message`、`send_c2c_message`、`send_direct_message`、`recall_message`
- 私信会话：`create_direct_message`
- 文件：`post_group_file`、`post_c2c_file`
- 表情回应：`put_reaction`、`delete_reaction`、`get_reaction_users`
- 精华：`put_pin`、`delete_pin`、`get_pins`
- 公告：`create_announce`、`create_recommend_announce`、`delete_announce`
- 日程：`get_schedules`、`get_schedule`、`create_schedule`、`update_schedule`、`delete_schedule`
- 权限：`get_api_permissions`、`post_permission_demand`

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
ctx.send_message(&channel_id, MessageParams::new_text("channel")).await?;
ctx.send_group_message(&group_openid, GroupMessageParams::new_text("group")).await?;
ctx.send_c2c_message(&openid, C2CMessageParams::new_text("c2c")).await?;
```

富文本能力通过字段组合：ark、embed、markdown、keyboard、media 等都直接放到对应 params 上。这样既能表达协议字段，也能让调用点保持清晰。

## 错误

所有 REST 方法返回 `botrs::Result<T>`。在 handler 里一般就地处理错误：

```rust
if let Err(err) = ctx.send_message(&channel_id, params).await {
    tracing::warn!("send failed: {err}");
}
```

需要区分错误类型时，再匹配 `BotError`。

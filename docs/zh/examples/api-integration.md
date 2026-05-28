# 调用 QQ 开放接口

下面这些示例展示了 `BotApi` 在消息收发之外的常见用法。每个示例都是自包含的 event handler，通过聊天命令触发 API 调用。

| 示例                                                                                                                  | 涉及 API                                                                                                                              |
|-----------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| [`guild/api_permission.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/api_permission.rs)              | `BotApi::get_api_permissions`、`BotApi::post_permission_demand`、`APIPermissionDemandIdentify`                                          |
| [`guild/announce.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/announce.rs)                          | `BotApi::create_announce`、`BotApi::delete_announce`、`BotApi::create_recommend_announce`、`RecommendChannel`、`AnnouncesType`         |
| [`guild/schedule.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/schedule.rs)                          | `BotApi::create_schedule`、`get_schedule`、`update_schedule`、`delete_schedule`、`RemindType`                                          |
| [`guild/pins_message.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/pins_message.rs)                  | `BotApi::get_pins`、`BotApi::put_pin`、`BotApi::delete_pin`                                                                            |
| [`guild/reaction_users.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/reaction_users.rs)              | `BotApi::get_reaction_users`、`EmojiType::System`，按 `cookie` 字段翻页                                                                |
| [`guild/recall.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/recall.rs)                              | `Context::recall_message`（也可走 `BotApi::recall_message`）                                                                           |
| [`api/message_params.rs`](https://github.com/YinMo19/botrs/blob/main/examples/api/message_params.rs)                  | `MessageParams`、`GroupMessageParams`、`C2CMessageParams`、`DirectMessageParams`                                                       |

## 套路

所有 REST 调用都可以直接写在 `ctx` 上，因为 `Context` 会解引用到 `BotApi`；token 存在 `BotApi` 内部。错误以 `botrs::BotError` 返回，框架不会自动重试，请就地处理。

```rust
// examples/guild/recall.rs —— 发完立即撤回
let resp = message.reply(&ctx, "this will vanish").await?;
if let Some(message_id) = resp.id {
    ctx.recall_message(channel_id, &message_id, /* hidetip */ true).await?;
}
```

对于 `get_reaction_users` 这类分页 API，按示例里的写法用返回的 `cookie` 与 `is_end` 自行驱动循环。

## 参见

- 指南：[`docs/zh/guide/api-client.md`](../guide/api-client.md)
- 可用方法的权威来源：`src/api/mod.rs` 以及 `src/api/` 目录下的实现文件
- 上面表格里列出的 `examples/` 示例路径

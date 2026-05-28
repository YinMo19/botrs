# BotApi

`BotApi` 是 bot 在事件处理过程中使用的无状态 REST 客户端。事件进来以后，handler 通常通过事件 session 调用它完成回复、撤回、上传群/C2C 文件、维护公告/日程/精华、查询表情回应用户，以及申请 API 权限。

在 `Client` 驱动的 bot 中通常不需要手动创建 `BotApi`。每个事件回调收到的 session 已经暴露同一个 API 客户端，可以直接调用：

```rust
let params = MessageParams::new_text("pong").with_reply(message_id);
session.send_message(params).await?;
```

只需要独立调用 REST、而不运行网关时，可以手动构造：

```rust
let http = HttpClient::new(30, false)?;
let token = Token::new("app_id", "secret");
let api = BotApi::new(http, token);

let me = api.get_bot_info().await?;
```

## 当前能力

`BotApi` 覆盖以下路径：

| 场景 | 方法 |
| --- | --- |
| bot 身份和网关发现 | `get_bot_info`、`get_gateway` |
| 频道消息 | `send_message`、`recall_message` |
| 群与 C2C 消息 | `send_group_message`、`send_c2c_message` |
| 私信 | `create_direct_message`、`send_direct_message` |
| 群/C2C 文件 | `post_group_file`、`post_c2c_file` |
| 公告 | `create_announce`、`create_recommend_announce`、`delete_announce` |
| 日程 | `get_schedules`、`get_schedule`、`create_schedule`、`update_schedule`、`delete_schedule` |
| API 权限 | `get_api_permissions`、`post_permission_demand` |
| 表情回应 | `put_reaction`、`delete_reaction`、`get_reaction_users` |
| 精华消息 | `put_pin`、`delete_pin`、`get_pins` |

## 消息发送

发送消息通过参数结构体完成：

```rust
let params = MessageParams::new_text("hello");
session.send_message(params).await?;

let params = GroupMessageParams::new_text("hello group");
group_session.send_message(params).await?;
```

频道消息和私信使用 `MessageParams` / `DirectMessageParams`。群和 C2C 使用 `GroupMessageParams` / `C2CMessageParams`，对应 QQ 开放平台的 open message 形态。需要发送 ark、embed、markdown、keyboard 或 media 时，在对应参数结构体上设置字段即可。

## 文件与媒体

群和 C2C 文件发送分两步：先上传得到 `Media`，再把它放进消息参数里发送。`file_type` 使用平台定义的数字，常用值是 1 图片、2 视频、3 语音、4 文件。

```rust
let media = session
    .post_file(1, "https://example.com/image.png", None)
    .await?;

let mut params = GroupMessageParams::default();
params.msg_type = 7;
params.media = Some(media);
session.send_message(params).await?;
```

如果 `srv_send_msg` 传 `Some(true)`，平台会在上传后直接发送，通常就不需要再手动构造一条 media 消息。

## 公告、日程、精华和权限

这些 API 的使用方式比较直接：

- 公告可以从已有消息创建，也可以创建推荐频道公告。
- 日程支持列表、单个查询、创建、更新和删除。
- 精华消息支持置顶、取消置顶和查询。
- API 权限申请通过 `post_permission_demand` 直接传 `channel_id`、`APIPermissionDemandIdentify` 和描述文本。

```rust
let identify = APIPermissionDemandIdentify {
    path: "/channels/{channel_id}/messages".to_string(),
    method: "POST".to_string(),
};

session.post_permission_demand(&guild_id, &channel_id, identify, "需要发送回复")
    .await?;
```

## 错误处理

所有方法返回 `botrs::Result<T>`。在事件处理器里，通常就地记录错误并返回即可，因为 `EventHandler` 方法本身不返回 `Result`：

```rust
if let Err(err) = session.send_message(params).await {
    tracing::warn!("send failed: {err}");
}
```

## 参见

- [Sessions](./context.md)
- [消息模型](./models/messages.md)
- [其他类型](./models/other-types.md)

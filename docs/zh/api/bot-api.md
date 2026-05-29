# BotApi

`BotApi` 是 bot 在事件处理过程中使用的共享类型化 REST 客户端。事件进来以后，handler 通常通过事件 session 调用它完成回复、撤回、上传群/C2C 文件、管理频道与子频道资源、公告、日程、精华、表情回应、权限、身份组、禁言和音频控制。

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
| 频道与子频道资源 | `get_guild`、`list_bot_guilds`、`get_channel`、`list_channels`、`create_channel`、`update_channel`、`delete_channel`、`create_private_channel` |
| 成员、身份组与禁言 | `get_guild_member`、`list_guild_members`、`delete_guild_member`、`list_roles`、`create_role`、`update_role`、`delete_role`、`add_member_role`、`delete_member_role`、`mute_guild`、`mute_member`、`mute_members` |
| 子频道权限与语音成员 | `get_channel_permissions`、`update_channel_permissions`、`get_channel_role_permissions`、`update_channel_role_permissions`、`list_voice_channel_members` |
| 频道消息 | `get_message`、`list_messages`、`send_message`、`update_message`、`recall_message`、`send_setting_guide` |
| 群与 C2C 消息 | `send_group_message`、`send_c2c_message`、`recall_group_message`、`recall_c2c_message` |
| 私信 | `create_direct_message`、`send_direct_message`、`recall_direct_message`、`send_direct_setting_guide` |
| 群/C2C 文件 | `post_group_file`、`post_c2c_file` |
| 公告 | `create_channel_announce`、`delete_channel_announce`、`create_announce`、`create_recommend_announce`、`delete_announce` |
| 日程 | `get_schedules`、`get_schedule`、`create_schedule`、`update_schedule`、`delete_schedule` |
| API 权限 | `get_api_permissions`、`post_permission_demand` |
| 表情回应 | `put_reaction`、`delete_reaction`、`get_reaction_users` |
| 精华消息 | `put_pin`、`delete_pin`、`clean_pins`、`get_pins` |
| 音频控制 | `post_audio`、`put_mic`、`delete_mic` |

## 消息发送

发送消息通过参数结构体完成：

```rust
let params = MessageParams::new_text("hello");
session.send_message(params).await?;

let params = GroupMessageParams::new_text("hello group");
group_session.send_message(params).await?;
```

频道消息和私信使用 `MessageParams` / `DirectMessageParams`。群和 C2C 使用 `GroupMessageParams` / `C2CMessageParams`，对应 QQ 开放平台的 open message 形态。

在事件处理器里，优先使用 session helper：`send_text_message`、`send_markdown_message`、`send_ark_message`、`send_embed_message`、`send_keyboard_message`，以及群/C2C 的 `send_media_message`。直接调用底层 API 时，使用 `new_markdown`、`new_ark`、`new_embed`、`new_keyboard`、`new_media` 等 params 构造器，它们会自动填好协议消息类型。

## 文件与媒体

群和 C2C 文件发送分两步：先上传得到 `Media`，再把它放进消息参数里发送。`file_type` 使用平台定义的数字，常用值是 1 图片、2 视频、3 语音、4 文件。

```rust
let media = session
    .post_file(1, "https://example.com/image.png", None)
    .await?;

session.send_media_message(media).await?;
```

如果 `srv_send_msg` 传 `Some(true)`，平台会在上传后直接发送，通常就不需要再手动构造一条 media 消息。

## 公告、日程、精华和权限

这些 API 的使用方式比较直接：

- 公告可以从已有消息创建，也可以创建推荐频道公告。
- 日程支持列表、单个查询、创建、更新和删除。
- 精华消息支持置顶、取消置顶、清空和查询。
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

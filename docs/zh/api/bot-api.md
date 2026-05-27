# BotApi

`BotApi` 是 QQ 频道开放接口的同步风格门面：它持有 HTTP 客户端，构造请求，使用 `Token` 完成签名，并把响应解析成模型类型返回。所有方法都是 `async` 的，返回 `Result<T, BotError>`。

## 构造

```rust
use botrs::{BotApi, http::HttpClient, Token};

let http = HttpClient::new(/* 超时秒 */ 30, /* 是否沙箱 */ false)?;
let api = BotApi::new(http);
let token = Token::new("app_id", "secret");

let me = api.get_bot_info(&token).await?;
```

`BotApi` 实现了 `Clone`，开销很小（内部 HTTP 客户端是引用计数的）。当 bot 通过 `Client` 驱动时，同一个 `BotApi` 实例会通过 `Context::api` 暴露给事件处理器。

## 方法目录

每个方法的签名都是 `&self, token: &Token, …`，返回 `Result<…>`。下面按业务域归类列出全部 100+ 路由。具体的参数与响应结构请查阅 [消息模型](./models/messages.md)、[频道与子频道](./models/guilds-channels.md)、[其他类型](./models/other-types.md)。

### 机器人身份

- `get_bot_info` —— `/users/@me`，返回 `BotInfo`。
- `get_gateway` —— 网关地址 + 推荐分片数。

### Guild

- `get_guild` / `get_guilds` / `get_guilds_with_pager`
- 成员：`get_guild_member`、`get_guild_members`、`get_guild_members_with_pager`
- 角色成员：`get_guild_role_members`、`get_guild_role_members_with_pager`
- 禁言：`mute_all`、`cancel_mute_all`、`mute_member`、`mute_multi_member`、`multi_member_mute`、`cancel_mute_multi_member`

### 子频道

- `get_channel`、`get_channels`
- `create_channel`、`create_private_channel`、`update_channel` / `patch_channel`、`delete_channel`
- 权限：`get_channel_user_permissions`、`get_channel_role_permissions`、`update_channel_user_permissions`、`update_channel_role_permissions`、`put_channel_permissions`、`put_channel_roles_permissions`

### 角色

- `get_guild_roles`
- `create_guild_role`、`create_guild_role_with_update`、`update_guild_role`、`update_guild_role_with_update`、`delete_guild_role`
- 赋予 / 移除：`create_guild_role_member`、`delete_guild_role_member`、`member_add_role`、`member_delete_role`、`delete_member`、`delete_member_with_options`

### 频道消息

- `get_message`、`get_messages`、`get_messages_with_params`
- 发送：`post_message_with_params`、`post_message_to_create`。
- 编辑：`patch_message_with_params`、`patch_message_to_create`。
- 撤回：`recall_message`。

### 私信

- 创建会话：`create_direct_message`。
- 发送：`post_dms_with_params`、`post_direct_message`。
- 撤回：`retract_dm_message`。
- 设置引导：`post_dm_setting_guide`、`post_dm_setting_guide_message`。

### 群 / C2C 消息

- 发送：`post_group_message_with_params`、`post_c2c_message_with_params`、`post_group_message_to_create`、`post_c2c_message_to_create`、`post_group_api_message`、`post_c2c_api_message`、`post_group_rich_media_message`、`post_c2c_rich_media_message`。
- 撤回：`retract_group_message`、`retract_c2c_message`。
- 文件上传：`post_group_file`、`post_c2c_file`。

### 表情回应

- `put_reaction`、`delete_reaction`、`delete_own_message_reaction`
- `create_message_reaction`、`get_reaction_users`、`get_message_reaction_users`

### 精华消息

- `put_pin`、`delete_pin`、`get_pins`、`clean_pins`

### 公告

- 频道：`create_guild_announce`、`delete_guild_announce`、`clean_guild_announces`、`create_guild_recommend_announce`、`create_recommend_announce`
- 子频道：`create_channel_announce`、`delete_channel_announce`、`clean_channel_announces`
- 简写：`create_announce`、`delete_announce`

### 日程

- `get_schedules`、`get_schedule`、`create_schedule`、`create_schedule_with_model`、`update_schedule`、`update_schedule_with_model`、`delete_schedule`

### API 权限

- `get_api_permissions`、`post_permission_demand`、`require_api_permissions`

### 音频 / 语音

- `post_audio`（使用 `AudioControl`）、`update_audio`
- `on_microphone`、`off_microphone`、`list_voice_channel_members`

### 设置引导

- `post_setting_guide`、`post_setting_guide_message`

### 互动

- `put_interaction` —— 应答按钮 / 互动事件。

### Webhook 会话

- `create_session`、`check_sessions`、`session_list`、`remove_session`、`transport`

### 消息推送配置

- `get_message_setting` —— 频道推送和私信开关。

### 生命周期

- `close` —— 等待请求结束并关闭底层客户端。
- `http` —— 借出底层 `HttpClient`，便于做高级定制。

## 完整示例

**回应 @ 提及，并附带按钮键盘。** 一次构建键盘，挂到 `MessageParams` 上，再交给 `_with_params` 系列方法。

```rust
let keyboard = Keyboard {
    content: Some(KeyboardContent {
        rows: Some(vec![KeyboardRow {
            buttons: Some(vec![KeyboardButton {
                id: Some("ok".into()),
                render_data: Some(KeyboardButtonRenderData {
                    label: Some("OK".into()),
                    style: Some(1),
                    ..Default::default()
                }),
                action: Some(KeyboardButtonAction {
                    action_type: Some(ACTION_TYPE_CALLBACK),
                    permission: Some(KeyboardButtonPermission {
                        permission_type: Some(PERMISSION_TYPE_ALL),
                        ..Default::default()
                    }),
                    data: Some("ok".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }]),
        }]),
        ..Default::default()
    }),
    ..Default::default()
};

let mut params = MessageParams::new_text("Choose:")
    .with_reply(message.id.as_deref().unwrap_or(""));
params.keyboard = Some(keyboard);

api.post_message_with_params(&token, &channel_id, params).await?;
```

**分页拉取成员列表。** 用 pager 辅助方法，避免手动维护 `after` 游标。

```rust
let pager = ctx.api.get_guild_members_with_pager(
    &ctx.token, &guild_id, GuildMembersPager::default(),
).await?;
for member in pager.items {
    /* ... */
}
```

**安全地更新子频道权限。** `validate()` 会在请求发出前检查权限串能不能被解析为整数。

```rust
let body = UpdateChannelPermissions::new(Some("1024"), Some("0"));
body.validate()?;
api.update_channel_user_permissions(&token, &channel_id, &user_id, &body).await?;
```

## 错误处理

所有方法都返回 `Result<T, BotError>`，可以按变体匹配：

- `BotError::Http` —— 传输层错误（超时、DNS 等）。
- `BotError::Api { code, message, .. }` —— 非 2xx 响应及其 QQ 错误码。
- `BotError::Auth` —— Token 签名或刷新失败。
- `BotError::InvalidData` —— 本地校验失败（例如非法的权限字符串）。

收到 429 时 `BotError::Api` 会携带可用的 `Retry-After`；框架不会自动重试，你可以根据自身节流需求自行包一层退避。

## 参见

- [Client](./client.md) —— 持有 `BotApi` 的高层事件循环。
- [Context](./context.md) —— 事件回调中的请求作用域包装器，暴露同样的接口。
- [模型](./models/messages.md) —— 请求与响应的结构体定义。
- [Token](./token.md) —— 凭证管理与刷新。

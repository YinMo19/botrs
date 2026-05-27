# 其他类型

QQ 频道机器人 API 使用的辅助模型类型。完整字段定义请参考源码；本文重点是结构概览和指向使用这些类型的接口。

## 音频与语音

```rust
pub struct AudioControl {            // /audio_control 的请求体
    pub audio_url: String,
    pub text: String,
    pub status: AudioStatus,         // 数值：0..=3
}

pub enum AudioStatus { Start = 0, Pause = 1, Resume = 2, Stop = 3 }
```

`Audio` 是网关事件载荷（`AUDIO_START` / `AUDIO_FINISH` 等），通过 `Audio::api()` 暴露 BotApi 句柄。`PublicAudio` 表示语音/直播子频道的成员事件，附带 `PublicAudioType`（`Voice = 2`、`Live = 5`）。

## 论坛主题

网关论坛事件的载荷。字段类型保留协议中的裸字符串/数值：

```rust
pub struct Thread     { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub thread_info: ThreadInfo, pub event_id: Option<String> }
pub struct ThreadInfo { pub thread_id: Option<String>, pub title: Option<String>, pub content: Option<String>, pub date_time: Option<String> }
pub struct Post       { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub post_info: PostInfo, pub event_id: Option<String> }
pub struct Reply      { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub reply_info: ReplyInfo, pub event_id: Option<String> }
pub struct ForumAuditResult { pub task_id: String, pub guild_id: String, pub channel_id: String, pub author_id: String, pub thread_id: String, pub post_id: String, pub reply_id: String, pub publish_type: u32, pub result: u32, pub err_msg: String, pub date_time: String, /* … */ }
```

`OpenThread` 是开放论坛变体，会根据触发的具体子事件在 `thread_info`、`post_info`、`reply_info` 中携带其一。

`title` 和 `content` 是协议定义的 paragraph 树 JSON。需要结构化解析时可用 `Content::new(&serde_json::from_str(&info.title.as_deref().unwrap_or("{}"))?)`。

## 日程

```rust
pub struct Schedule {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    pub start_timestamp: String,        // unix 秒
    pub end_timestamp: String,
    pub jump_channel_id: Snowflake,
    pub remind_type: String,            // RemindType::to_wire_string()
    pub creator: Option<Member>,
}
```

`RemindType` 取值 `None=0` … `Before2Days=8`，加 `Unknown(u8)` 兜底。`ScheduleWrapper { schedule: Option<Schedule> }` 是创建/更新接口的请求体。

## API 权限

```rust
pub struct APIPermissions { pub api_list: Vec<APIPermission> /* JSON: "apis" */ }
pub struct APIPermission  { pub path: String, pub method: String, pub desc: String, pub auth_status: i32 }

pub struct APIPermissionDemand          { pub guild_id: Snowflake, pub channel_id: Snowflake, pub api_identify: Option<APIPermissionDemandIdentify>, pub title: String, pub desc: String }
pub struct APIPermissionDemandToCreate  { pub channel_id: Snowflake, pub api_identify: Option<APIPermissionDemandIdentify>, pub desc: String }
pub struct APIPermissionDemandIdentify  { pub path: String, pub method: String }
```

协议中所有带 `omitempty` 的字段都使用了 `skip_serializing_if`，零值不会出现在 JSON 中。

## 精华、公告、推送配置

- `PinsMessage { guild_id, channel_id, message_ids: Vec<Snowflake> }` —— `get_pins` 的返回值。
- `Announce` —— 见 [`announce.rs`](https://github.com/YinMo19/botrs/blob/main/src/models/announce.rs)；配套有 `RecommendChannel`、`ChannelAnnouncesToCreate`、`GuildAnnouncesToCreate`。
- `MessageSetting { disable_create_dm: bool, disable_push_msg: bool, channel_ids: Vec<Snowflake>, channel_push_max_num: i32 }` —— 每个字段都跳过零值，与协议保持一致。

## 互动事件

```rust
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    pub interaction_type: InteractionType,         // JSON: "type"
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub group_open_id: Option<String>,
    pub user_open_id: Option<String>,
    pub group_member_open_id: Option<String>,
    pub timestamp: Option<String>,
    pub version: Option<i32>,
    pub event_id: Option<String>,
    pub scene: Option<String>,
    pub chat_type: Option<i32>,
    pub channel_type: Option<i32>,
}
```

`InteractionType` 与 `InteractionDataType` 对应文档定义的整数码。`BotApi::put_interaction` 用于回执互动事件。

## 表情回应

```rust
pub struct MessageReaction { pub user_id: Snowflake, pub channel_id: Snowflake, pub guild_id: Snowflake, pub target: ReactionTarget, pub emoji: ReactionEmoji }
pub struct ReactionTarget  { pub id: String, #[serde(rename = "type")] pub target_type: ReactionTargetType }
pub enum   ReactionTargetType { Message = 0, Subject = 1, Bot = 2 }
pub struct ReactionEmoji   { pub id: String, #[serde(rename = "type")] pub emoji_type: u32 }
pub struct ReactionUsers   { pub users: Vec<User>, pub cookie: String, pub is_end: bool }
```

## 管理类事件

`GroupManageEvent` / `C2CManageEvent` 携带 `event_id`、`timestamp`，以及定位被影响群/成员的 OpenID。`ManageEventType` 是字符串枚举（`group_add_robot`、`friend_del`……），网关分发器使用它路由事件。`EnterAioEvent { user_openid: String, from_source: String }` 与 `SubscribeMessageStatusData { group_openid, openid, result: Vec<SubscribeMsgTemplateResult>, /* … */ }` 是其余的管理类载荷。

## 网关与会话

```rust
pub struct Ready             { pub user: User, pub session_id: String, pub shard: [u32; 2], pub version: u32 }
pub struct ConnectionSession { /* 内部：id、shard、网关 URL、last seq…… */ }
pub enum   ConnectionState   { Disconnected, Connecting, Connected, Reconnecting, Closed }
pub struct Session           { pub id: String, pub url: String, pub token: String, pub shards: u32, pub last_seq: u64 }
pub struct SessionStartLimit { pub total: u32, pub remaining: u32, pub reset_after: u64, pub max_concurrency: u32 }
```

## Trait

`HasId` 定义 `fn id(&self) -> Option<&Snowflake>`，所有持有 ID 的 DTO 都实现了它；`HasName` 是与之并列的可显示名称 trait。两者在频道、子频道、角色、日程、成员、表情等类型上有完整实现。

## 参见

- [消息](./messages.md) —— 消息侧的辅助类型（embed、ark、keyboard）。
- [频道与子频道](./guilds-channels.md) —— 频道、子频道、角色、禁言相关结构。
- [Bot API](../bot-api.md) —— 产生或使用这些类型的接口。

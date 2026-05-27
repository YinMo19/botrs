# Other Types

Supporting model types used by the QQ Bot Open API. Refer to source for full field-level definitions; this page summarises shapes and points to the routes that produce or consume them.

## Audio and voice

```rust
pub struct AudioControl {            // request body for /audio_control
    pub audio_url: String,
    pub text: String,
    pub status: AudioStatus,         // numeric: 0..=3
}

pub enum AudioStatus { Start = 0, Pause = 1, Resume = 2, Stop = 3 }
```

`Audio` is the gateway event payload (`AUDIO_START` / `AUDIO_FINISH` etc.). `PublicAudio` carries voice/live channel member events with a `PublicAudioType` (`Voice = 2`, `Live = 5`). Use the event callback `Context` for REST calls.

## Forum threads

Gateway forum events deliver these payloads. Field types reflect the protocol's bare strings/integers:

```rust
pub struct Thread     { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub thread_info: ThreadInfo, pub event_id: Option<String> }
pub struct ThreadInfo { pub thread_id: Option<String>, pub title: Option<String>, pub content: Option<String>, pub date_time: Option<String> }
pub struct Post       { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub post_info: PostInfo, pub event_id: Option<String> }
pub struct Reply      { pub guild_id: Option<String>, pub channel_id: Option<String>, pub author_id: Option<String>, pub reply_info: ReplyInfo, pub event_id: Option<String> }
pub struct ForumAuditResult { pub task_id: String, pub guild_id: String, pub channel_id: String, pub author_id: String, pub thread_id: String, pub post_id: String, pub reply_id: String, pub publish_type: u32, pub result: u32, pub err_msg: String, pub date_time: String, /* … */ }
```

`OpenThread` is the open-forum variant and may carry `thread_info`, `post_info`, **or** `reply_info` depending on which sub-event fired.

`title` and `content` are JSON-encoded paragraph trees as defined by the QQ Bot Open API; parse with `Content::new(&serde_json::from_str(&info.title.as_deref().unwrap_or("{}"))?)` if you need a structured view.

## Schedules

```rust
pub struct Schedule {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    pub start_timestamp: String,        // unix seconds
    pub end_timestamp: String,
    pub jump_channel_id: Snowflake,
    pub remind_type: String,            // RemindType::to_wire_string()
    pub creator: Option<Member>,
}
```

`RemindType` enumerates `None=0` … `Before2Days=8` plus `Unknown(u8)`. `ScheduleWrapper { schedule: Option<Schedule> }` is the create/update body.

## API permissions

```rust
pub struct APIPermissions { pub api_list: Vec<APIPermission> /* JSON: "apis" */ }
pub struct APIPermission  { pub path: String, pub method: String, pub desc: String, pub auth_status: i32 }

pub struct APIPermissionDemand          { pub guild_id: Snowflake, pub channel_id: Snowflake, pub api_identify: Option<APIPermissionDemandIdentify>, pub title: String, pub desc: String }
pub struct APIPermissionDemandToCreate  { pub channel_id: Snowflake, pub api_identify: Option<APIPermissionDemandIdentify>, pub desc: String }
pub struct APIPermissionDemandIdentify  { pub path: String, pub method: String }
```

Every field with `omitempty` in the protocol uses `skip_serializing_if` so zero values stay off the wire.

## Pins, announces, message setting

- `PinsMessage { guild_id, channel_id, message_ids: Vec<Snowflake> }` — return value of `get_pins`.
- `Announce` — see [`announce.rs`](https://github.com/YinMo19/botrs/blob/main/src/models/announce.rs); `RecommendChannel` and the create-side `ChannelAnnouncesToCreate` / `GuildAnnouncesToCreate` accompany it.
- `MessageSetting { disable_create_dm: bool, disable_push_msg: bool, channel_ids: Vec<Snowflake>, channel_push_max_num: i32 }` — every field omits its zero value to match the protocol.

## Interaction

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

`InteractionType` and `InteractionDataType` map to the documented integer codes. `BotApi::put_interaction` acknowledges the event.

## Reactions

```rust
pub struct MessageReaction { pub user_id: Snowflake, pub channel_id: Snowflake, pub guild_id: Snowflake, pub target: ReactionTarget, pub emoji: Emoji }
pub struct ReactionTarget  { pub id: String, #[serde(rename = "type")] pub target_type: ReactionTargetType }
pub enum   ReactionTargetType { Message = 0, Subject = 1, Bot = 2 }
pub struct Emoji            { pub id: String, #[serde(rename = "type")] pub emoji_type: i32 }
pub struct ReactionUsers   { pub users: Vec<User>, pub cookie: String, pub is_end: bool }
```

## Management events

`GroupManageEvent` / `C2CManageEvent` carry `event_id`, `timestamp`, plus the OpenIDs needed to identify the affected group / member. `ManageEventType` is a string enum (`group_add_robot`, `friend_del`, …) used by the gateway router. `EnterAioEvent { user_openid: String, from_source: String }` and `SubscribeMessageStatusData { group_openid, openid, result: Vec<SubscribeMsgTemplateResult>, /* … */ }` are the remaining management payloads.

## Gateway / session

```rust
pub struct Ready             { pub user: User, pub session_id: String, pub shard: [u32; 2], pub version: u32 }
pub struct Session           { pub id: String, pub url: String, pub token: Token, pub intent: Intents, pub last_seq: u64, /* … */ }
pub struct SessionStartLimit { pub total: u32, pub remaining: u32, pub reset_after: u64, pub max_concurrency: u32 }
```

## Traits

`HasId` exposes `fn id(&self) -> Option<&Snowflake>` for any DTO that owns an id; `HasName` is the parallel trait for displayable types. Both are blanket-implemented for the relevant guild, channel, role, schedule, member, and emoji types.

## See also

- [Messages](./messages.md) — message-side support types (embeds, ark, keyboards).
- [Guilds & Channels](./guilds-channels.md) — guild, channel, role, mute structs.
- [Bot API](../bot-api.md) — the routes that produce and consume these types.

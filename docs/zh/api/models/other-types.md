# 其他类型

这一页记录除了消息、用户、guild/channel 以外的常用模型：网关启动信息、日程、权限、精华、公告、表情回应、互动事件、音频事件、管理事件和论坛事件。

## 网关

`get_gateway` 返回 `GatewayResponse`，其中包含 websocket URL、建议 shard 数和 session start limit。`Client::start` 会使用它来创建内部 gateway session；应用代码通常不需要直接处理。

`Ready` 是 `READY` 事件的 payload。它包含 session id、版本号、当前 bot 的 `User`，以及可选 shard 信息。

## 日程

日程模型服务于 `get_schedules`、`get_schedule`、`create_schedule`、`update_schedule`、`delete_schedule`。创建或更新时使用 `Schedule::new(...)`，提醒时间使用 `RemindType`，最终会转成平台需要的字符串值。

```rust
let schedule = Schedule::new(
    "Daily sync",
    "1767225600",
    "1767229200",
    Some(channel_id.to_string()),
    RemindType::Before15Minutes,
);
ctx.create_schedule(&schedule_channel_id, &schedule).await?;
```

## API 权限

权限查询使用 `get_api_permissions`。申请权限使用 `post_permission_demand`，调用时传目标子频道、接口标识和说明文本。

`post_permission_demand` 会根据这些参数生成请求体；调用方只需要关心目标接口和说明文本。

## 精华与公告

精华消息用 `PinsMessage` 表示当前 pinned message id 列表。常用流程是置顶、取消置顶和查询。

公告模型包含 `Announce`、`RecommendChannel` 和 `AnnouncesType`。常见流程是从已有消息创建公告，或构造推荐频道列表创建推荐公告。

## 表情回应

reaction 事件和 REST 查询共享同一套核心类型：

- `Reaction` 是 handler 收到的事件对象。
- `MessageReaction` 是平台 reaction payload 的 DTO。
- `ReactionTarget` 和 `ReactionTargetType` 表示表态目标。
- `ReactionUsers` 是 `get_reaction_users` 的响应。
- emoji 使用 `models::emoji::Emoji` 与 `EmojiType`。

查询 reaction 用户时，把上一次响应里的 `cookie` 和本次 `limit` 传给 `get_reaction_users`。当响应的 `is_end` 为 true 时分页结束。

## 互动事件

`Interaction` 对应 `interaction_create`，用于读取平台传来的按钮或应用互动 payload。

## 音频、管理和论坛事件

音频相关模型用于 gateway 事件：`Audio`、`PublicAudio` 和 `PublicAudioType`。

管理类事件仍会分发：

- 群：`GroupManageEvent`
- C2C：`C2CManageEvent`
- AIO：`EnterAioEvent`
- 订阅消息授权状态：`SubscribeMessageStatusData`

论坛事件包含普通论坛事件和 open forum 事件，handler 可以读取主题、帖子、回复和审核结果。

## 参见

- [BotApi](../bot-api.md)
- [EventHandler](../event-handler.md)
- [消息](./messages.md)

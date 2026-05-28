# Intents

`Intents` 是传给 `Client::new` 的位标志集合，用于告诉网关需要投递哪些事件类别。只有显式开启的事件才会被推送，因此选择最小够用的集合既能减少流量，也能避免你不消费的事件触发处理器。

## 构造方式

- `Intents::new()` —— 空集。
- `Intents::default()` —— 除两个特权 intent（`GUILD_MESSAGES` 与 `FORUMS`）之外的全部公开 intent，适合大多数公域机器人。
- `Intents::all()` —— 全部标志，包括特权位。

## 切换标志

每个 intent 既以 `pub const` 的 `u32` 形式暴露在 `Intents` 类型上，也提供同名的链式 `with_*` 方法：

```rust
use botrs::Intents;

let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds();
```

也可以用 `Intents::with_intent(Intents::GUILDS)` 把原始常量按位组合。

## 标志一览

| 标志                          | 构建器                                | 网关投递的事件                                          |
|-------------------------------|--------------------------------------|--------------------------------------------------------|
| `GUILDS`                      | `with_guilds`                        | `guild_create` / `guild_update` / `guild_delete`        |
| `GUILD_MEMBERS`               | `with_guild_members`                 | `guild_member_add` / `_update` / `_remove`              |
| `GUILD_MESSAGES`（特权）      | `with_guild_messages`                | 频道全量消息（`message_create` 等）                     |
| `GUILD_MESSAGE_REACTIONS`     | `with_guild_message_reactions`       | 表情添加 / 移除                                         |
| `DIRECT_MESSAGE`              | `with_direct_message`                | 私聊 `direct_message_create`                            |
| `PUBLIC_GUILD_MESSAGES`       | `with_public_guild_messages`         | @ 消息 `message_create`，非特权                         |
| `PUBLIC_MESSAGES`             | `with_public_messages`               | 群聊 / C2C 消息                                         |
| `INTERACTION`                 | `with_interaction`                   | 按钮 / interaction 事件                                 |
| `MESSAGE_AUDIT`               | `with_message_audit`                 | 消息审核通过 / 拒绝                                     |
| `FORUMS`（特权）              | `with_forums`                        | 论坛主题 / 帖子 / 回复 / 发布审核                       |
| `AUDIO_ACTION`                | `with_audio_action`                  | 音频开始 / 结束 / 上下麦                                |
| `OPEN_FORUM_EVENT`            | `with_open_forum_event`              | 公开论坛主题 / 帖子 / 回复                              |
| `ENTER_AIO`                   | `with_enter_aio`                     | 单聊（AIO）进入事件                                     |
| `AUDIO_OR_LIVE_CHANNEL_MEMBER`| `with_audio_or_live_channel_member`  | 语音 / 直播子频道成员进出                                |

特权 intent（`GUILD_MESSAGES`、`FORUMS`）需要在开发者后台申请权限，否则网关在 identify 时会拒绝连接。

## 运行时查询

`contains(flag)` 用于检查某一位；同名访问器（如 `intents.guilds()`、`intents.public_guild_messages()`）返回 `bool`。

# Intents

`Intents` 是一个 32 位标志集，告诉 QQ 网关你的机器人希望接收哪些事件类别。只订阅自己需要的事件可以减少网关流量，也避免框架为不会处理的事件做无用功。

```rust
pub struct Intents { pub bits: u32 }
```

`Intents` 实现了 `Copy`，支持位运算 (`|`、`&`、`^`、`!`)，并直接以原始 `u32` 序列化进网关 `Identify` 载荷。

## 构造

- `Intents::new()` —— 空集合（`bits == 0`）。
- `Intents::default()` —— `all()` 去掉特权 intent (`GUILD_MESSAGES` 和 `FORUMS`)，多数机器人都从这里开始。
- `Intents::all()` —— 框架已知的全部位。仅当机器人拥有所有特权时使用。
- `Intents::from_bits(bits)` —— 把配置或外部传入的整数包成 `Intents`。

## 标志位一览

每行包含 Rust 常量、底层位、`with_*` 构建器以及简短说明。

| 常量                                    | 位        | 构建器                                | 含义                                                    |
|-----------------------------------------|-----------|--------------------------------------|---------------------------------------------------------|
| `Intents::GUILDS`                       | `1 << 0`  | `with_guilds`                        | 频道创建/更新/删除以及子频道 CRUD。                     |
| `Intents::GUILD_MEMBERS`                | `1 << 1`  | `with_guild_members`                 | 成员加入/更新/退出。                                    |
| `Intents::GUILD_MESSAGES`（特权）       | `1 << 9`  | `with_guild_messages`                | 全部频道消息——需要特权审批。                            |
| `Intents::GUILD_MESSAGE_REACTIONS`      | `1 << 10` | `with_guild_message_reactions`       | 频道消息表情回应增/删。                                 |
| `Intents::DIRECT_MESSAGE`               | `1 << 12` | `with_direct_message`                | 私信创建/删除事件。                                     |
| `Intents::OPEN_FORUM_EVENT`             | `1 << 18` | `with_open_forum_event`              | 公域论坛活动事件。                                      |
| `Intents::AUDIO_OR_LIVE_CHANNEL_MEMBER` | `1 << 19` | `with_audio_or_live_channel_member`  | 语音/直播子频道的成员变化。                             |
| `Intents::ENTER_AIO`                    | `1 << 23` | `with_enter_aio`                     | 用户进入 AIO 聊天面板。                                 |
| `Intents::PUBLIC_MESSAGES`              | `1 << 25` | `with_public_messages`               | 群 `@bot` 消息、C2C 消息、好友事件。                    |
| `Intents::INTERACTION`                  | `1 << 26` | `with_interaction`                   | 互动（按钮/应用）回调。                                 |
| `Intents::MESSAGE_AUDIT`                | `1 << 27` | `with_message_audit`                 | 消息审核通过/拒绝。                                     |
| `Intents::FORUMS`（特权）               | `1 << 28` | `with_forums`                        | 全部论坛事件——需要特权审批。                            |
| `Intents::AUDIO_ACTION`                 | `1 << 29` | `with_audio_action`                  | 音频开始/结束/上麦/下麦事件。                           |
| `Intents::PUBLIC_GUILD_MESSAGES`        | `1 << 30` | `with_public_guild_messages`         | `@bot` 消息及公域消息删除事件。                         |

## 查询

对每个标志都有匹配的无参谓词：`intents.guilds()`、`intents.public_guild_messages()` 等。两个常用的辅助方法：

- `intents.contains(bits)` —— 通用位归属判断。
- `intents.has_privileged()` —— 当 `GUILD_MESSAGES` 或 `FORUMS` 被启用时返回 `true`。

## 修改

- `with_intent(bits)` / `without_intent(bits)` —— 通用链式 setter。
- `bits()` —— 返回原始 `u32`。
- `Display` 实现会输出 `Intents(GUILDS | DIRECT_MESSAGE | …)`，便于日志记录。

## 示例

```rust
// 1. 推荐默认值，不开启特权事件。
let default_intents = Intents::default();

// 2. 通过 builder 自定义订阅。
let custom = Intents::new()
    .with_guilds()
    .with_public_guild_messages()
    .with_direct_message();

// 3. 直接位组合。
let public = Intents::from_bits(
    Intents::GUILDS | Intents::PUBLIC_GUILD_MESSAGES | Intents::DIRECT_MESSAGE,
);

// 4. 运行时摘除某个标志。
let trimmed = Intents::all().without_intent(Intents::FORUMS);
```

## 特权 intent

`GUILD_MESSAGES` 和 `FORUMS` 需要 QQ 开放平台手动审批通过。未获得审批的机器人在 identify 阶段会被网关断开。`Intents::default()` 默认不开启它们，方便所有机器人共用同一份代码路径。

```rust
if intents.has_privileged() && !your_bot_is_approved {
    return Err("特权 intent 未获审批".into());
}
```

## 参见

- [Bot API](./bot-api.md) —— 可能产出这些事件的全部接口。
- [事件处理器](./event-handler.md) —— 接收这些 intent 选中的事件。
- [网关指南](../guide/gateway.md) —— 框架如何使用所选 intent 建立会话。

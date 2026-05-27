# 用户与成员

QQ 频道机器人 API 中的用户和成员数据结构。所有结构体都实现了 `Serialize` + `Deserialize`，并通过 `#[serde(default)]` 容忍缺失字段，便于反序列化部分载荷。

## `User`

频道接口返回的标准用户对象。

```rust
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub avatar: String,           // 头像哈希，未设置时为空
    pub bot: bool,
    pub union_openid: String,     // 跨应用标识
    pub union_user_account: String,
}
```

`Snowflake` 是 `String` 的类型别名。`User::avatar_url()` 用哈希拼出 CDN 地址，`User::mention()` 返回 `<@!id>` 的 mention 串。

## `Member`

频道作用域下的 `User` 包装。

```rust
pub struct Member {
    #[serde(flatten)]
    pub user: User,
    pub nick: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: Timestamp,
    pub deaf: bool,
    pub mute: bool,
}
```

`Timestamp` 是 RFC 3339 字符串。`display_name()` 优先返回 `nick`，否则返回 `user.username`；`has_role(&id)`/`has_any_role(&ids)`/`has_all_roles(&ids)` 用于角色判断。

> 注：成员相关网关事件使用另一份 `Member` —— 见 [`models::guild::Member`](./guilds-channels.md)，它把 `guild_id` 和 `op_user_id` 单独保留为字段。

## `BotInfo`

`/users/@me` 返回的机器人自身信息。

```rust
pub struct BotInfo {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub bot: bool,
}
```

`Ready::user` 在 `EventHandler::ready` 回调中暴露这个对象。

## 消息作者类型

不同会话场景下的作者结构略有差异，所有字段均为 `Option`，因为接口可能省略。

| 类型                 | 出现位置                    | 关键字段                                  |
|----------------------|-----------------------------|-------------------------------------------|
| `MessageUser`        | 频道消息（`Message`）       | `id`、`username`、`avatar`、`bot`         |
| `DirectMessageUser`  | 私信                        | `id`、`username`、`avatar`                |
| `GroupMessageUser`   | 群 `@bot` 消息              | `id`、`member_openid`、`union_openid`     |
| `C2CMessageUser`     | C2C（私聊）消息             | `id`、`user_openid`                       |

群聊和 C2C 不暴露真实的频道用户 ID，使用 `*_openid` 标识用户。

## 成员相关操作

常用方法挂在 `BotApi` / `Context` 上：

- `get_guild_member(guild_id, user_id)` —— 获取单个成员。
- `get_guild_members(guild_id, limit, after)` —— 分页列表，下页传上页最后一个 `user.id` 作 `after`。
- `create_guild_role_member` / `delete_guild_role_member` —— 角色赋予和移除。
- `delete_member` —— 移除成员，可选拉黑和历史消息删除设置。
- `mute_member`、`mute_all`、`cancel_mute_all`、`on_microphone`、`off_microphone` —— 语音控制。

每个方法都返回 `Result<T>`，QQ 接口错误会被映射为 `BotError`。

```rust
let member = ctx.get_guild_member(guild_id, user_id).await?;
if member.has_role(&moderator_role_id) {
    // 执行管理操作
}
```

## 参见

- [消息](./messages.md) —— 消息结构以及消息作者类型在场景中的使用。
- [频道与子频道](./guilds-channels.md) —— 频道侧的 `Member` 变体和角色数据。
- [Client API](../client.md) —— `Context` 暴露的用户作用域辅助方法。

# 用户与成员

用户模型只保留会在核心链路里出现的几种形态：bot 自身、READY 事件里的 user、reaction user 列表、消息作者、guild member 事件，以及日程 creator。

## User 与 BotInfo

`User` 是通用用户结构，包含 id、username、avatar、bot 标记和 union 字段。`Ready::user` 使用这个类型。

`BotInfo` 是 `/users/@me` 的响应，除了基础用户字段，还保留平台返回的 `share_url` 和 `welcome_msg`。handler 派发时的 `session.bot_info()` 就来自这个接口。

## Member 形态

当前有两个 member 形态：

- `botrs::models::guild::Member`，也就是 `models::guild::Member`，用于 guild member 网关事件。它有 `guild_id`、`user`、`nick`、`roles`、`joined_at` 和 `op_user_id`。
- `models::user::Member`，它 flatten 一个 `User`，并带 nick、roles、joined_at、deaf、mute。当前主要用于日程 creator 这类响应数据。

两者不要混用。事件处理器里看到的 `guild_member_add` / `guild_member_update` / `guild_member_remove` 参数是第一种。

## 消息作者

不同消息场景的作者字段并不相同：

- 频道消息使用 `MessageUser`，字段接近普通 guild user。
- 群消息使用 `GroupMessageUser`，重点是 `member_openid` 和 `union_openid`。
- C2C 消息使用 `C2CMessageUser`，重点是 `user_openid`。
- 频道消息里的 member 信息使用 `MessageMember`，只包含 nick、roles、joined_at。

私信事件使用 `Message`；私信会话使用 `DirectMessage`。

## 使用建议

常规消息事件里的 `message.author` 是必有字段。member 详情和部分平台特定用户字段仍可能按事件类型缺省或使用默认值。

```rust
let is_bot = message.author.bot;
```

群和 C2C 回复时优先使用事件模型中的 openid 字段。

## 参见

- [消息](./messages.md)
- [频道与子频道](./guilds-channels.md)
- [其他类型](./other-types.md)

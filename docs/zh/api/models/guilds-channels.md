# 频道与子频道

crate 保留 guild、channel 和 guild member 模型，是因为网关会推送这些事件。它们用于解析 `GUILD_*`、`CHANNEL_*`、`GUILD_MEMBER_*` 事件，并交给 `EventHandler`。

这些模型主要用于事件读取、日志、指标和业务判断。

## Guild

`Guild` 描述频道服务器本身，例如 id、名称、图标、owner、成员数量、描述和绑定的 union 信息。`GUILD_CREATE` 这类网关 payload 可能会带上 `channels`，因此模型里保留了 `Vec<Channel>`。

典型使用方式是在 handler 里读取事件内容：

```rust
async fn guild_create(&self, session: GuildSession) {
    let guild = session.event();
    tracing::info!("joined guild {}", guild.name);
}
```

## Channel

`Channel` 描述子频道。模型保留了平台字段，包括 `type`、`sub_type`、父频道、owner、可见性和发言权限等。

类型字段使用 Rust enum 包装协议数字：

- `ChannelType`: text、voice、category、live、application、forum，以及未知值。
- `ChannelSubType`: chat、notice、guide、team game，以及未知值。
- `PrivateType`: public、only admin、admin and member，以及未知值。
- `SpeakPermission`: invalid、public、admin and member，以及未知值。

这些 enum 都保留未知数字，避免平台新增类型时反序列化失败。

## Guild member 事件

`botrs::Member` 对应网关里的 guild member 事件。它包含 guild id、可选 user、昵称、角色 id 列表、加入时间和操作者 id。

```rust
async fn guild_member_add(&self, session: MemberSession) {
    let member = session.event();
    if let Some(user) = &member.user {
        tracing::info!("{} joined {}", user.username, member.guild_id);
    }
}
```

注意它和 `models::user::Member` 不是同一个类型。后者是 `User` 的包装，目前主要出现在日程 creator 这类响应字段里。

## 参见

- [EventHandler](../event-handler.md)
- [用户与成员](./users-members.md)
- [BotApi](../bot-api.md)

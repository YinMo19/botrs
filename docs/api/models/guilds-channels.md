# Guilds and Channels

The crate keeps guild, channel, and guild member models because the gateway delivers those events. They deserialize `GUILD_*`, `CHANNEL_*`, and `GUILD_MEMBER_*` payloads before dispatching to `EventHandler`.

These models are primarily for reading event data, logging, metrics, and business checks.

## Guild

`Guild` describes the guild itself: id, name, icon, owner fields, member counts, description, and union binding fields. `GUILD_CREATE` payloads may include `channels`, so the model keeps `Vec<Channel>`.

Typical usage is reading the event in a handler:

```rust
async fn guild_create(&self, session: GuildSession) {
    let guild = session.event();
    tracing::info!("joined guild {}", guild.name);
}
```

## Channel

`Channel` describes a sub-channel. It keeps platform fields including `type`, `sub_type`, parent channel, owner, visibility, and speak permission.

Type fields wrap protocol numbers in Rust enums:

- `ChannelType`: text, voice, category, live, application, forum, plus unknown values.
- `ChannelSubType`: chat, notice, guide, team game, plus unknown values.
- `PrivateType`: public, only admin, admin and member, plus unknown values.
- `SpeakPermission`: invalid, public, admin and member, plus unknown values.

These enums retain unknown numeric values so new platform values do not break deserialization.

## Guild Member Events

`botrs::models::guild::Member` maps to guild member gateway events. It includes guild id, optional user, nickname, role ids, join time, and operator id.

```rust
async fn guild_member_add(&self, session: MemberSession) {
    let member = session.event();
    if let Some(user) = &member.user {
        tracing::info!("{} joined {}", user.username, member.guild_id);
    }
}
```

This is distinct from `models::user::Member`, which wraps `User` and mainly appears in response fields such as schedule creator.

## See Also

- [EventHandler](../event-handler.md)
- [Users and Members](./users-members.md)
- [BotApi](../bot-api.md)

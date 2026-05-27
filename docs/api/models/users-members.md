# Users and Members

User and member data structures for the QQ Guild Bot API. All structs are `Serialize` + `Deserialize` and accept missing fields via `#[serde(default)]` so they survive partial payloads.

## `User`

The canonical user record returned by guild endpoints.

```rust
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub avatar: String,           // avatar hash, empty when unset
    pub bot: bool,
    pub union_openid: String,     // cross-app identifier
    pub union_user_account: String,
}
```

`Snowflake` is a type alias for `String`. Use `User::avatar_url()` to build a CDN URL from the hash, and `User::mention()` for `<@!id>`.

## `Member`

A guild-scoped wrapper around `User`.

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

`Timestamp` is `String` (RFC 3339). Inspect `nick`, `user.username`, and `roles` directly when deriving display labels or checking role membership.

> Note: gateway events deserialize a different `Member` shape — see [`models::guild::Member`](./guilds-channels.md), which keeps `guild_id` and `op_user_id` as separate fields used by member-add/remove events.

## `BotInfo`

Returned by `/users/@me`.

```rust
pub struct BotInfo {
    pub id: Snowflake,
    pub username: String,
    pub avatar: Option<String>,
    pub bot: bool,
}
```

`Ready::user` exposes one of these to your `EventHandler::ready` handler.

## Message author types

Different message channels carry different author shapes. All four use `Option` fields because the API may omit them.

| Type                | Used by                         | Notable fields                              |
|---------------------|---------------------------------|---------------------------------------------|
| `MessageUser`       | guild messages (`Message`)      | `id`, `username`, `avatar`, `bot`           |
| `DirectMessageUser` | direct messages                 | `id`, `username`, `avatar`                  |
| `GroupMessageUser`  | group `@bot` messages           | `id`, `member_openid`, `union_openid`       |
| `C2CMessageUser`    | C2C (private) messages          | `id`, `user_openid`                         |

Group and C2C messages identify users with `*_openid` rather than a numeric guild user id, since the bot does not see the underlying QQ accounts.

## Working with members

The high-traffic operations live on `BotApi` / `Context`:

- `get_guild_member(guild_id, user_id)` — fetch a single member.
- `get_guild_members(guild_id, limit, after)` — paginated listing; pass the previous page's last `user.id` as `after`.
- `create_guild_role_member` / `delete_guild_role_member` — role assignment.
- `delete_member` — removes a member, with optional blacklist and history-delete settings.
- `mute_member`, `mute_all`, `cancel_mute_all`, `on_microphone`, `off_microphone` — voice controls.

Each method returns `Result<T>` and propagates QQ API errors as `BotError`.

```rust
let member = ctx.get_guild_member(guild_id, user_id).await?;
if member.roles.contains(&moderator_role_id) {
    // perform privileged action
}
```

## See also

- [Messages](./messages.md) — message structures and the message author types in context.
- [Guilds & Channels](./guilds-channels.md) — guild-side `Member` variant and role data.
- [Client API](../client.md) — how `Context` exposes user-scoped helpers.

# Guilds and Channels

Guild and channel data structures for the QQ Guild Bot API. `Snowflake` is `String`. Most string and numeric fields use `#[serde(default)]` so partial gateway payloads deserialize cleanly.

## `Guild`

Guild metadata returned by the guild endpoints and gateway events.

```rust
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: String,
    pub owner_id: Snowflake,
    #[serde(rename = "owner")]
    pub is_owner: bool,
    pub member_count: i32,
    pub max_members: i64,
    pub description: String,
    pub joined_at: String,                  // RFC 3339
    pub channels: Vec<Channel>,             // gateway only
    pub union_world_id: String,
    pub union_org_id: String,
    pub op_user_id: Snowflake,
}
```

`is_owner` maps to the protocol's `owner` boolean. `channels` is populated only on `GUILD_CREATE` gateway payloads.

## `Channel`

Channel metadata. Skip-serializing-when-zero is applied to the optional fields so request bodies for create/update endpoints stay minimal.

```rust
pub struct Channel {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub channel_type: ChannelType,          // JSON: "type"
    pub sub_type: ChannelSubType,
    pub position: i64,
    pub parent_id: Snowflake,
    pub owner_id: Snowflake,
    pub private_type: PrivateType,
    pub private_user_ids: Vec<String>,
    pub speak_permission: SpeakPermission,
    pub application_id: Snowflake,
    pub permissions: String,
    pub op_user_id: Snowflake,
}
```

For modify/create endpoints use `ChannelValueObject`, which wraps every field in `Option<T>` and omits zero values from the JSON body.

### Channel enums

| Enum                  | Variants                                                                |
|-----------------------|-------------------------------------------------------------------------|
| `ChannelType`         | `Text=0`, `Voice=2`, `Category=4`, `Live=10005`, `Application=10006`, `Forum=10007`, plus `Unknown(u32)` for forward-compat |
| `ChannelSubType`      | `Chat=0`, `Notice=1`, `Guide=2`, `TeamGame=3`                          |
| `PrivateType`         | `Public=0`, `OnlyAdmin=1`, `AdminAndMember=2`                          |
| `SpeakPermission`     | `Invalid=0`, `Public=1`, `AdminAndMember=2`                            |

Each enum implements `From<u8>`/`From<u32>` for raw numeric round-trips. Use the enum variants directly, for example `ChannelType::Text` or `SpeakPermission::Public`.

Predicates on `Channel`: `is_text()`, `is_voice()`, `is_group()`, `is_live()`, `is_application()`, `is_discussion()`, `is_public()`, `is_admin_only()`, `everyone_can_speak()`, `mention()` (returns `<#id>`).

## Channel permissions

Two read-side DTOs — one per principal type — and a single update body:

```rust
pub struct ChannelPermissions {       // user-scoped
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub permissions: String,          // bitset as decimal string
}

pub struct ChannelRolesPermissions {  // role-scoped
    pub channel_id: Snowflake,
    pub role_id: Snowflake,
    pub permissions: String,
}

pub struct UpdateChannelPermissions {
    pub add: Option<String>,          // bits to set
    pub remove: Option<String>,       // bits to clear
}
```

`UpdateChannelPermissions::validate()` confirms the strings parse as `u64` before the request is sent.

## Roles

```rust
pub struct GuildRole {
    pub id: Snowflake,
    pub name: String,
    pub color: u32,                   // ARGB-as-decimal
    pub hoist: u32,                   // 0 / 1
    pub member_count: u32,            // JSON: "number", omitempty
    pub member_limit: u32,            // omitempty
}

pub struct GuildRoles {
    pub guild_id: Snowflake,
    pub roles: Vec<GuildRole>,
    #[serde(rename = "role_num_limit")]
    pub num_limit: String,
}
```

Update flow uses three structs together:

```rust
pub struct UpdateRoleFilter { pub name: u32, pub color: u32, pub hoist: u32 }
pub struct UpdateRoleInfo   { pub name: String, pub color: u32, pub hoist: u32 }
pub struct UpdateRole {
    pub guild_id: String,
    pub filter: UpdateRoleFilter,     // 1 = update this field
    #[serde(rename = "info")]
    pub update: GuildRole,
}
```

`UpdateRoleFilter::default()` enables every field (all `1`).

## Member (guild side)

The gateway uses a guild-scoped `Member` distinct from `models::user::Member`.

```rust
pub struct Member {
    pub guild_id: Snowflake,
    pub user: Option<User>,
    pub nick: String,
    pub roles: Vec<Snowflake>,
    pub joined_at: Timestamp,
    pub op_user_id: Snowflake,        // omitempty
}
```

`MemberAddRoleBody { channel: Option<Channel> }` is the body for the channel-administrator role assignment endpoint.

## Mute

```rust
pub struct UpdateGuildMute {
    pub mute_end_timestamp: String,   // omitempty
    pub mute_seconds: String,         // omitempty
    pub user_ids: Vec<String>,        // omitempty, batch-mute only
}
```

Constructors:

- `UpdateGuildMute::new(end, seconds)` — single member.
- `UpdateGuildMute::new_multi(user_ids, end, seconds)` — batch.
- `UpdateGuildMute::cancel()` / `cancel_multi(user_ids)` — clear mute.

## See also

- [Bot API](../bot-api.md) — every guild/channel/role/mute endpoint.
- [Users and Members](./users-members.md) — the `User` shape referenced from `Member`.
- [Other types](./other-types.md) — announces, schedules, pins, API permission demands.

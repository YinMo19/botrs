# 频道与子频道

QQ 频道机器人 API 中的频道（Guild）与子频道（Channel）数据结构。`Snowflake` 是 `String` 别名；多数字符串和数值字段都标注了 `#[serde(default)]`，这样部分网关载荷也能正常反序列化。

## `Guild`

频道接口和网关事件中的频道元信息。

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
    pub channels: Vec<Channel>,             // 仅网关
    pub union_world_id: String,
    pub union_org_id: String,
    pub op_user_id: Snowflake,
}
```

`is_owner` 在协议里叫 `owner`。`channels` 只在 `GUILD_CREATE` 网关事件里被填充。

## `Channel`

子频道元信息。零值字段使用 `skip_serializing_if`，让创建/更新接口的请求体保持精简。

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

修改/创建接口使用 `ChannelValueObject`，它把所有字段都包成 `Option<T>`，零值会被跳过。

### 子频道相关枚举

| 枚举                  | 取值                                                                       |
|-----------------------|----------------------------------------------------------------------------|
| `ChannelType`         | `Text=0`、`Voice=2`、`Category=4`、`Live=10005`、`Application=10006`、`Forum=10007`，外加 `Unknown(u32)` 兜底 |
| `ChannelSubType`      | `Chat=0`、`Notice=1`、`Guide=2`、`TeamGame=3`                              |
| `PrivateType`         | `Public=0`、`OnlyAdmin=1`、`AdminAndMember=2`                              |
| `SpeakPermission`     | `Invalid=0`、`Public=1`、`AdminAndMember=2`                                |

每个枚举都实现了 `From<u8>`/`From<u32>` 用于数值往返。请直接使用枚举 variant，例如 `channel.channel_type == ChannelType::Text` 或 `channel.speak_permission == SpeakPermission::Public`。

## 子频道权限

读取侧分两份 DTO（按主体类型），更新侧共用一份请求体：

```rust
pub struct ChannelPermissions {       // 用户权限
    pub channel_id: Snowflake,
    pub user_id: Snowflake,
    pub permissions: String,          // 权限位的十进制字符串
}

pub struct ChannelRolesPermissions {  // 角色权限
    pub channel_id: Snowflake,
    pub role_id: Snowflake,
    pub permissions: String,
}

pub struct UpdateChannelPermissions {
    pub add: Option<String>,          // 需要置位的位
    pub remove: Option<String>,       // 需要清零的位
}
```

`UpdateChannelPermissions::validate()` 会检查字符串能否解析为 `u64`，避免发送非法权限串。

## 角色

```rust
pub struct GuildRole {
    pub id: Snowflake,
    pub name: String,
    pub color: u32,                   // ARGB 颜色（十进制）
    pub hoist: u32,                   // 0 / 1
    pub member_count: u32,            // JSON: "number"，omitempty
    pub member_limit: u32,            // omitempty
}

pub struct GuildRoles {
    pub guild_id: Snowflake,
    pub roles: Vec<GuildRole>,
    #[serde(rename = "role_num_limit")]
    pub num_limit: String,
}
```

更新流程涉及三个结构体：

```rust
pub struct UpdateRoleFilter { pub name: u32, pub color: u32, pub hoist: u32 }
pub struct UpdateRoleInfo   { pub name: String, pub color: u32, pub hoist: u32 }
pub struct UpdateRole {
    pub guild_id: String,
    pub filter: UpdateRoleFilter,     // 1 表示需要更新该字段
    #[serde(rename = "info")]
    pub update: GuildRole,
}
```

`UpdateRoleFilter::default()` 会把每个字段都置为 `1`，相当于全部更新。

## Member（频道侧）

网关事件使用频道作用域的 `Member`，与 `models::user::Member` 不同：

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

`MemberAddRoleBody { channel: Option<Channel> }` 是分配子频道管理员身份组接口的请求体。

## 禁言

```rust
pub struct UpdateGuildMute {
    pub mute_end_timestamp: String,   // omitempty
    pub mute_seconds: String,         // omitempty
    pub user_ids: Vec<String>,        // omitempty，仅批量禁言时填
}
```

构造器：

- `UpdateGuildMute::new(end, seconds)` —— 单人。
- `UpdateGuildMute::new_multi(user_ids, end, seconds)` —— 批量。
- `UpdateGuildMute::cancel()` / `cancel_multi(user_ids)` —— 解除禁言。

## 参见

- [Bot API](../bot-api.md) —— 频道、子频道、角色、禁言相关接口。
- [用户与成员](./users-members.md) —— `Member` 内部 `User` 字段的结构。
- [其他类型](./other-types.md) —— 公告、日程、精华、API 权限申请等。

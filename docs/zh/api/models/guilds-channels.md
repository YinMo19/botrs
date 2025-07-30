# 频道与子频道模型 API 参考

该模块为 QQ 频道机器人 API 中的频道（服务器）和子频道提供数据结构。

## 频道类型

### `Guild`

表示机器人可以访问的 QQ 频道（服务器）。

```rust
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner_id: String,
    pub owner: bool,
    pub member_count: Option<u32>,
    pub max_members: Option<u32>,
    pub description: Option<String>,
    pub joined_at: Option<String>,
    pub features: Vec<String>,
    pub op_user_id: Option<String>,
}
```

#### 字段

- `id`: 频道的唯一标识符
- `name`: 频道名称
- `icon`: 频道图标 URL（可选）
- `owner_id`: 频道所有者的用户 ID
- `owner`: 机器人是否为频道所有者
- `member_count`: 当前成员数量
- `max_members`: 允许的最大成员数
- `description`: 频道描述
- `joined_at`: 机器人加入频道的时间戳
- `features`: 启用的频道功能列表
- `op_user_id`: 操作员用户 ID

#### 示例

```rust
async fn handle_guild_create(ctx: Context, guild: Guild) {
    println!("加入频道: {} (ID: {})", guild.name, guild.id);
    println!("成员数量: {}", guild.member_count.unwrap_or(0));

    if guild.owner {
        println!("机器人是该频道的所有者");
    }
}
```

### `GuildRole`

表示频道内的身份组。

```rust
pub struct GuildRole {
    pub id: String,
    pub name: String,
    pub color: Option<u32>,
    pub hoist: Option<bool>,
    pub number: Option<u32>,
    pub member_limit: Option<u32>,
}
```

#### 字段

- `id`: 身份组的唯一标识符
- `name`: 身份组名称
- `color`: 身份组颜色（十六进制值）
- `hoist`: 是否在成员列表中单独显示
- `number`: 身份组位置/优先级
- `member_limit`: 可拥有此身份组的最大成员数

#### 示例

```rust
async fn list_guild_roles(ctx: Context, guild_id: &str) -> Result<()> {
    let roles = ctx.get_guild_roles(guild_id).await?;

    for role in roles.roles {
        println!("身份组: {} (ID: {})", role.name, role.id);
        if let Some(color) = role.color {
            println!("  颜色: #{:06X}", color);
        }
        if let Some(limit) = role.member_limit {
            println!("  成员限制: {}", limit);
        }
    }

    Ok(())
}
```

### `GuildRoles`

频道身份组信息的容器。

```rust
pub struct GuildRoles {
    pub guild_id: String,
    pub roles: Vec<GuildRole>,
    pub role_num_limit: Option<u32>,
}
```

#### 字段

- `guild_id`: 这些身份组所属频道的 ID
- `roles`: 频道中的身份组列表
- `role_num_limit`: 频道中允许的最大身份组数量

## 子频道类型

### `Channel`

表示频道内的子频道。

```rust
pub struct Channel {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub sub_type: Option<ChannelSubType>,
    pub position: Option<u32>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub private_type: Option<u32>,
    pub speak_permission: Option<u32>,
    pub application_id: Option<String>,
    pub permissions: Option<String>,
}
```

#### 字段

- `id`: 子频道的唯一标识符
- `guild_id`: 该子频道所属频道的 ID
- `name`: 子频道名称
- `channel_type`: 子频道类型（文字、语音等）
- `sub_type`: 子频道子类型，用于额外分类
- `position`: 子频道在子频道列表中的位置
- `parent_id`: 父子频道 ID（用于子频道分类）
- `owner_id`: 子频道所有者的 ID
- `private_type`: 子频道的隐私设置
- `speak_permission`: 发言权限要求
- `application_id`: 关联的应用程序 ID
- `permissions`: 子频道特定权限

#### 示例

```rust
async fn handle_channel_create(ctx: Context, channel: Channel) {
    println!("创建新子频道: {} (类型: {:?})", channel.name, channel.channel_type);

    match channel.channel_type {
        ChannelType::Text => {
            println!("在频道 {} 中创建了文字子频道", channel.guild_id);
        }
        ChannelType::Voice => {
            println!("在频道 {} 中创建了语音子频道", channel.guild_id);
        }
        ChannelType::Category => {
            println!("在频道 {} 中创建了分类", channel.guild_id);
        }
        _ => {
            println!("创建了其他类型的子频道");
        }
    }
}
```

### `ChannelType`

不同子频道类型的枚举。

```rust
pub enum ChannelType {
    Text = 0,
    Voice = 1,
    Category = 4,
    Announcement = 5,
    Forum = 10,
    Live = 11,
    Application = 12,
}
```

#### 变体

- `Text`: 用于消息的文字子频道
- `Voice`: 用于音频通信的语音子频道
- `Category`: 用于组织子频道的分类
- `Announcement`: 公告子频道
- `Forum`: 用于话题讨论的论坛子频道
- `Live`: 直播子频道
- `Application`: 应用程序特定子频道

#### 示例

```rust
async fn create_text_channel(ctx: Context, guild_id: &str, name: &str) -> Result<Channel> {
    let channel = ctx.create_channel(
        guild_id,
        name,
        ChannelType::Text,
        None, // sub_type
        None, // position
        None, // parent_id
        None, // private_type
        None, // speak_permission
        None, // application_id
    ).await?;

    println!("创建文字子频道: {}", channel.name);
    Ok(channel)
}
```

### `ChannelSubType`

用于额外分类的子频道子类型枚举。

```rust
pub enum ChannelSubType {
    Chat = 0,
    Announcement = 1,
    Guide = 2,
    Game = 3,
}
```

#### 变体

- `Chat`: 普通聊天子频道
- `Announcement`: 公告专用子频道
- `Guide`: 指南或帮助子频道
- `Game`: 游戏相关子频道

## 成员类型

### `Member`

表示频道的成员。

```rust
pub struct Member {
    pub user: Option<User>,
    pub nick: Option
<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
}
```

#### 字段

- `user`: 该成员的用户信息
- `nick`: 成员在频道中的昵称
- `roles`: 分配给成员的身份组 ID 列表
- `joined_at`: 成员加入频道的时间戳
- `deaf`: 成员在语音子频道中是否被拒听
- `mute`: 成员在语音子频道中是否被静音

#### 示例

```rust
async fn handle_member_update(ctx: Context, member: Member) {
    if let Some(user) = &member.user {
        println!("成员更新: {}", user.username.as_deref().unwrap_or("未知"));

        if let Some(nick) = &member.nick {
            println!("昵称: {}", nick);
        }

        println!("身份组: {:?}", member.roles);
    }
}
```

### `User`

表示用户信息。

```rust
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub union_openid: Option<String>,
    pub union_user_account: Option<String>,
}
```

#### 字段

- `id`: 唯一用户标识符
- `username`: 用户的显示名称
- `avatar`: 头像图片 URL
- `bot`: 该用户是否为机器人
- `union_openid`: 用于跨平台识别的联合 OpenID
- `union_user_account`: 联合用户账户标识符

## 子频道管理

### 创建子频道

```rust
async fn setup_guild_channels(ctx: Context, guild_id: &str) -> Result<()> {
    // 创建分类
    let category = ctx.create_channel(
        guild_id,
        "常规",
        ChannelType::Category,
        None,
        Some(0), // 位置在顶部
        None,
        None,
        None,
        None,
    ).await?;

    // 在分类下创建文字子频道
    let general_chat = ctx.create_channel(
        guild_id,
        "闲聊",
        ChannelType::Text,
        Some(ChannelSubType::Chat),
        Some(1),
        Some(&category.id), // 父分类
        None,
        None,
        None,
    ).await?;

    let announcements = ctx.create_channel(
        guild_id,
        "公告",
        ChannelType::Text,
        Some(ChannelSubType::Announcement),
        Some(2),
        Some(&category.id),
        None,
        None,
        None,
    ).await?;

    println!("创建分类 '{}' 及其子频道:", category.name);
    println!("  - {}", general_chat.name);
    println!("  - {}", announcements.name);

    Ok(())
}
```

### 子频道权限

```rust
async fn manage_channel_permissions(ctx: Context, channel_id: &str, user_id: &str) -> Result<()> {
    // 获取用户的当前权限
    let permissions = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    println!("用户权限: {}", permissions.permissions);

    // 获取身份组权限
    let role_id = "role_id_here";
    let role_permissions = ctx.get_channel_role_permissions(channel_id, role_id).await?;
    println!("身份组权限: {}", role_permissions.permissions);

    Ok(())
}
```

### 频道成员管理

```rust
async fn manage_guild_members(ctx: Context, guild_id: &str) -> Result<()> {
    // 获取频道成员
    let members = ctx.get_guild_members(guild_id, Some(100), None).await?;
    println!("频道有 {} 个成员", members.len());

    for member in &members {
        if let Some(user) = &member.user {
            println!("成员: {}", user.username.as_deref().unwrap_or("未知"));
            println!("  身份组: {:?}", member.roles);

            if let Some(joined) = &member.joined_at {
                println!("  加入时间: {}", joined);
            }
        }
    }

    // 获取特定成员
    let user_id = "specific_user_id";
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    if let Some(user) = &member.user {
        println!("找到成员: {}", user.username.as_deref().unwrap_or("未知"));
    }

    Ok(())
}
```

### 身份组管理

```rust
async fn manage_roles(ctx: Context, guild_id: &str) -> Result<()> {
    // 创建新身份组
    let new_role = ctx.create_guild_role(
        guild_id,
        "管理员",
        Some(0x0099ff), // 蓝色
        Some(true),     // 单独显示
        Some(100),      // 成员限制
    ).await?;

    println!("创建身份组: {} (ID: {})", new_role.name, new_role.id);

    // 为用户分配身份组
    let user_id = "user_id_here";
    ctx.add_guild_role_member(guild_id, &new_role.id, user_id, None).await?;
    println!("为用户分配身份组");

    // 更新身份组
    let updated_role = ctx.update_guild_role(
        guild_id,
        &new_role.id,
        "高级管理员",
        Some(0xff9900), // 橙色
        Some(true),
        Some(50), // 减少成员限制
    ).await?;

    println!("更新身份组: {}", updated_role.name);

    // 移除用户的身份组
    ctx.remove_guild_role_member(
guild_id, &new_role.id, user_id, None).await?;
    println!("移除用户的身份组");
    
    // 删除身份组
    ctx.delete_guild_role(guild_id, &new_role.id).await?;
    println!("删除身份组");
    
    Ok(())
}
```

## 常见使用模式

### 频道发现

```rust
async fn explore_guilds(ctx: Context) -> Result<()> {
    let guilds = ctx.get_guilds(None, None).await?;
    
    for guild in guilds {
        println!("频道: {} (ID: {})", guild.name, guild.id);
        
        // 获取该频道的子频道
        let channels = ctx.get_channels(&guild.id).await?;
        println!("  子频道 ({}):", channels.len());
        
        for channel in channels {
            let type_name = match channel.channel_type {
                ChannelType::Text => "文字",
                ChannelType::Voice => "语音",
                ChannelType::Category => "分类",
                ChannelType::Announcement => "公告",
                ChannelType::Forum => "论坛",
                ChannelType::Live => "直播",
                ChannelType::Application => "应用",
            };
            
            println!("    {} - {} ({})", channel.name, type_name, channel.id);
        }
    }
    
    Ok(())
}
```

### 子频道组织

```rust
async fn organize_channels(ctx: Context, guild_id: &str) -> Result<()> {
    let channels = ctx.get_channels(guild_id).await?;
    
    // 按分类分组子频道
    let mut categories = std::collections::HashMap::new();
    let mut orphaned_channels = Vec::new();
    
    for channel in channels {
        match (channel.channel_type, &channel.parent_id) {
            (ChannelType::Category, _) => {
                categories.insert(channel.id.clone(), (channel, Vec::new()));
            }
            (_, Some(parent_id)) => {
                if let Some((_, children)) = categories.get_mut(parent_id) {
                    children.push(channel);
                }
            }
            (_, None) => {
                orphaned_channels.push(channel);
            }
        }
    }
    
    // 打印组织结构
    for (_, (category, children)) in categories {
        println!("分类: {}", category.name);
        for child in children {
            println!("  └─ {}", child.name);
        }
    }
    
    if !orphaned_channels.is_empty() {
        println!("未分类的子频道:");
        for channel in orphaned_channels {
            println!("  - {}", channel.name);
        }
    }
    
    Ok(())
}
```

## 相关文档

- [客户端 API](../client.md) - 频道和子频道操作的主要客户端
- [上下文 API](../context.md) - API 访问的上下文对象
- [消息](./messages.md) - 消息类型和处理
- [用户与成员](./users-members.md) - 用户和成员管理
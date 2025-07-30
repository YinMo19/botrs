# 用户与成员模型 API 参考

该模块为 QQ 频道机器人 API 中的用户信息、频道成员和相关实体提供数据结构。

## 核心类型

### `User`

表示系统中的 QQ 用户。

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
- `bot`: 该用户是否为机器人账户
- `union_openid`: 用户的跨平台标识符
- `union_user_account`: 联合账户标识符

#### 示例

```rust
async fn handle_user_info(user: User) {
    println!("用户: {}", user.username.as_deref().unwrap_or("未知"));
    
    if user.bot.unwrap_or(false) {
        println!("这是一个机器人账户");
    }
    
    if let Some(avatar) = &user.avatar {
        println!("头像 URL: {}", avatar);
    }
}
```

### `Member`

表示频道成员，包含用户信息以及频道特定数据。

```rust
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
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

#### 方法

##### `display_name`

获取成员的显示名称（昵称或用户名）。

```rust
pub fn display_name(&self) -> &str
```

**返回值：** 成员的昵称（如果有），否则返回用户名，如果都没有则返回 "未知"。

**示例：**
```rust
let display_name = member.display_name();
println!("成员显示名称: {}", display_name);
```

##### `is_bot`

检查成员是否为机器人。

```rust
pub fn is_bot(&self) -> bool
```

**示例：**
```rust
if member.is_bot() {
    println!("这个成员是机器人");
}
```

##### `has_role`

检查成员是否拥有特定身份组。

```rust
pub fn has_role(&self, role_id: &str) -> bool
```

**参数：**
- `role_id`: 要检查的身份组 ID

**示例：**
```rust
if member.has_role("admin_role_id") {
    println!("成员拥有管理员身份组");
}
```

### `MessageUser`

消息中的用户信息的简化版本。

```rust
pub struct MessageUser {
    pub id: String,
    pub username: Option<String>,
    pub bot: Option<bool>,
    pub avatar: Option<String>,
}
```

#### 字段

- `id`: 用户唯一标识符
- `username`: 用户显示名称
- `bot`: 是否为机器人账户
- `avatar`: 头像 URL

#### 方法

##### `from_data`

从 JSON 数据创建 MessageUser 实例。

```rust
pub fn from_data(data: serde_json::Value) -> Self
```

**示例：**
```rust
let user = MessageUser::from_data(json_data);
```

### `MessageMember`

消息中的成员信息。

```rust
pub struct MessageMember {
    pub nick: Option<String>,
    pub roles: Option<Vec<String>>,
    pub joined_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

#### 字段

- `nick`: 成员昵称
- `roles`: 身份组列表
- `joined_at`: 加入时间

## 群聊和 C2C 用户类型

### `GroupMessageUser`

群消息中的用户信息。

```rust
pub struct GroupMessageUser {
    pub id: Option<String>,
    pub member_openid: Option<String>,
    pub union_openid: Option<String>,
}
```

#### 字段

- `id`: 用户 ID（可选）
- `member_openid`: 成员 OpenID
- `union_openid`: 联合 OpenID

### `C2CMessageUser`

C2C 消息中的用户信息。

```rust
pub struct C2CMessageUser {
    pub user_openid: Option<String>,
}
```

#### 字段

- `user_openid`: 用户 OpenID

### `DirectMessageUser`

私信中的用户信息。

```rust
pub struct DirectMessageUser {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
}
```

#### 字段

- `id`: 用户 ID
- `username`: 用户名
- `avatar`: 头像 URL

### `DirectMessageMember`

私信中的成员信息。

```rust
pub struct DirectMessageMember {
    pub nick: Option<String>,
    pub roles: Option<Vec<String>>,
    pub joined_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

## 用户管理操作

### 获取用户信息

```rust
async fn get_user_details(ctx: Context, user_id: &str) -> Result<()> {
    // 获取用户基本信息
    let user = ctx.get_current_user().await?;
    println!("当前用户: {}", user.username.as_deref().unwrap_or("未知"));
    
    // 在特定频道中获取用户信息
    let guild_id = "guild_id_here";
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    
    if let Some(user) = &member.user {
        println!("频道成员: {}", user.username.as_deref().unwrap_or("未知"));
        
        if let Some(nick) = &member.nick {
            println!("昵称: {}", nick);
        }
        
        println!("身份组: {:?}", member.roles);
    }
    
    Ok(())
}
```

### 成员管理

```rust
async fn manage_member(ctx: Context, guild_id: &str, user_id: &str) -> Result<()> {
    // 获取成员信息
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    println!("成员显示名称: {}", member.display_name());
    
    // 检查成员权限
    if member.has_role("moderator_role_id") {
        println!("成员拥有版主权限");
    }
    
    // 为成员添加身份组
    let role_id = "new_role_id";
    ctx.add_guild_role_member(guild_id, role_id, user_id, None).await?;
    println!("已为成员添加身份组");
    
    // 从成员移除身份组
    ctx.remove_guild_role_member(guild_id, role_id, user_id, None).await?;
    println!("已从成员移除身份组");
    
    Ok(())
}
```

### 批量用户操作

```rust
async fn bulk_user_operations(ctx: Context, guild_id: &str) -> Result<()> {
    // 获取所有成员
    let members = ctx.get_guild_members(guild_id, Some(1000), None).await?;
    
    let mut bot_count = 0;
    let mut human_count = 0;
    let mut admin_count = 0;
    
    for member in &members {
        if member.is_bot() {
            bot_count += 1;
        } else {
            human_count += 1;
        }
        
        if member.has_role("admin_role_id") {
            admin_count += 1;
        }
    }
    
    println!("频道统计:");
    println!("  机器人: {}", bot_count);
    println!("  人类用户: {}", human_count);
    println!("  管理员: {}", admin_count);
    println!("  总成员: {}", members.len());
    
    // 找到最近加入的成员
    let mut recent_members: Vec<_> = members.iter()
        .filter(|m| m.joined_at.is_some())
        .collect();
    
    recent_members.sort_by(|a, b| {
        b.joined_at.as_ref().unwrap().cmp(a.joined_at.as_ref().unwrap())
    });
    
    println!("最近加入的 5 个成员:");
    for member in recent_members.iter().take(5) {
        println!("  - {}", member.display_name());
        if let Some(joined) = &member.joined_at {
            println!("    加入时间: {}", joined.format("%Y-%m-%d %H:%M:%S"));
        }
    }
    
    Ok(())
}
```

### 用户权限检查

```rust
async fn check_user_permissions(ctx: Context, guild_id: &str, user_id: &str) -> Result<()> {
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    
    // 检查特定身份组
    let admin_roles = ["admin", "moderator", "owner"];
    let is_admin = admin_roles.iter().any(|role| member.has_role(role));
    
    if is_admin {
        println!("用户 {} 拥有管理权限", member.display_name());
    } else {
        println!("用户 {} 是普通成员", member.display_name());
    }
    
    // 检查用户状态
    if member.deaf.unwrap_or(false) {
        println!("用户在语音中被拒听");
    }
    
    if member.mute.unwrap_or(false) {
        println!("用户在语音中被静音");
    }
    
    // 获取用户在特定子频道的权限
    let channel_id = "channel_id_here";
    let permissions = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    println!("用户在子频道中的权限: {}", permissions.permissions);
    
    Ok(())
}
```

## 消息中的用户处理

### 处理用户提及

```rust
use botrs::{Context, EventHandler, Message};

struct UserMentionHandler;

#[async_trait::async_trait]
impl EventHandler for UserMentionHandler {
    async fn message_create(&self, ctx: Context, message: Message) {
        if message.has_mentions() {
            println!("消息包含 {} 个用户提及", message.mentions.len());
            
            for mentioned_user in &message.mentions {
                println!("提及用户: {}", 
                    mentioned_user.username.as_deref().unwrap_or("未知"));
                
                if mentioned_user.bot.unwrap_or(false) {
                    println!("  这是一个机器人");
                }
            }
            
            // 回复提及消息
            let response = format!(
                "检测到 {} 个用户提及", 
                message.mentions.len()
            );
            
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("回复失败: {}", e);
            }
        }
    }
}
```

### 用户信息展示

```rust
async fn display_user_card(ctx: Context, user: User, member: Option<Member>) -> String {
    let mut card = format!("用户信息卡片\n");
    card.push_str(&format!("ID: {}\n", user.id));
    card.push_str(&format!("用户名: {}\n", 
        user.username.as_deref().unwrap_or("未设置")));
    
    if user.bot.unwrap_or(false) {
        card.push_str("类型: 机器人\n");
    } else {
        card.push_str("类型: 用户\n");
    }
    
    if let Some(avatar) = &user.avatar {
        card.push_str(&format!("头像: {}\n", avatar));
    }
    
    if let Some(member) = member {
        if let Some(nick) = &member.nick {
            card.push_str(&format!("昵称: {}\n", nick));
        }
        
        if !member.roles.is_empty() {
            card.push_str(&format!("身份组数量: {}\n", member.roles.len()));
        }
        
        if let Some(joined) = &member.joined_at {
            card.push_str(&format!("加入时间: {}\n", 
                joined.format("%Y-%m-%d %H:%M:%S")));
        }
    }
    
    card
}
```

## 常见使用模式

### 用户验证

```rust
async fn verify_user_access(ctx: Context, guild_id: &str, user_id: &str, required_role: &str) -> Result<bool> {
    let member = ctx.get_guild_member(guild_id, user_id).await?;
    
    // 检查用户是否为机器人
    if member.is_bot() {
        return Ok(false);
    }
    
    // 检查用户是否拥有所需身份组
    if !member.has_role(required_role) {
        return Ok(false);
    }
    
    // 检查用户状态
    if member.mute.unwrap_or(false) || member.deaf.unwrap_or(false) {
        return Ok(false);
    }
    
    Ok(true)
}
```

### 用户活动分析

```rust
async fn analyze_user_activity(ctx: Context, guild_id: &str) -> Result<()> {
    let members = ctx.get_guild_members(guild_id, Some(1000), None).await?;
    
    // 按加入时间分组
    let now = chrono::Utc::now();
    let mut new_members = 0;
    let mut recent_members = 0;
    let mut old_members = 0;
    
    for member in &members {
        if let Some(joined) = &member.joined_at {
            let days_ago = (now - *joined).num_days();
            
            if days_ago <= 7 {
                new_members += 1;
            } else if days_ago <= 30 {
                recent_members += 1;
            } else {
                old_members += 1;
            }
        }
    }
    
    println!("成员活动分析:");
    println!("  新成员 (7天内): {}", new_members);
    println!("  近期成员 (30天内): {}", recent_members);
    println!("  老成员 (30天以上): {}", old_members);
    
    // 身份组分布
    let mut role_distribution = std::collections::HashMap::new();
    for member in &members {
        for role_id in &member.roles {
            *role_distribution.entry(role_id.clone()).or_insert(0) += 1;
        }
    }
    
    println!("身份组分布:");
    for (role_id, count) in role_distribution {
        println!("  {}: {} 人", role_id, count);
    }
    
    Ok(())
}
```

## 相关文档

- [客户端 API](../client.md) - 用户和成员操作的主要客户端
- [上下文 API](../context.md) - API 访问的上下文对象
- [消息](./messages.md) - 消息中的用户信息处理
- [频道与子频道](./guilds-channels.md) - 频道成员管理
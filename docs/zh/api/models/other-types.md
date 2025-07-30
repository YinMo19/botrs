# 其他类型 API 参考

该模块涵盖了在 QQ 频道机器人 API 中使用的其他数据结构和工具类型。

## 音频和语音类型

### `Audio`

表示语音子频道中的音频控制和状态。

```rust
pub struct Audio {
    pub audio_control: Option<AudioControl>,
    pub audio_status: Option<AudioStatus>,
}
```

#### 字段

- `audio_control`: 音频播放的控制操作
- `audio_status`: 子频道中音频的当前状态

### `AudioControl`

音频控制操作。

```rust
pub struct AudioControl {
    pub audio_url: Option<String>,
    pub text: Option<String>,
    pub status: Option<u32>,
}
```

#### 字段

- `audio_url`: 要播放的音频文件 URL
- `text`: 音频的文本描述
- `status`: 音频播放状态（0: 暂停，1: 播放）

### `AudioStatus`

当前音频播放状态。

```rust
pub struct AudioStatus {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<u32>,
}
```

#### 字段

- `start_time`: 音频开始时间
- `end_time`: 音频结束时间
- `status`: 播放状态

#### 示例

```rust
async fn control_audio(ctx: Context, channel_id: &str) -> Result<()> {
    let audio_control = AudioControl {
        audio_url: Some("https://example.com/audio.mp3".to_string()),
        text: Some("播放背景音乐".to_string()),
        status: Some(1), // 开始播放
    };

    let audio = Audio {
        audio_control: Some(audio_control),
        audio_status: None,
    };

    // 控制音频播放
    ctx.control_audio(channel_id, audio).await?;
    println!("音频播放已开始");

    Ok(())
}
```

## 时间和日期类型

### `Timestamp`

表示时间戳，基于 `chrono::DateTime<chrono::Utc>`。

```rust
pub type Timestamp = chrono::DateTime<chrono::Utc>;
```

#### 示例

```rust
use chrono::Utc;

fn create_timestamp() -> Timestamp {
    Utc::now()
}

fn format_timestamp(ts: &Timestamp) -> String {
    ts.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
```

### `Snowflake`

QQ API 中使用的雪花 ID 类型。

```rust
pub type Snowflake = String;
```

#### 字段说明

雪花 ID 是 QQ API 中用于唯一标识资源的字符串格式 ID，包括：
- 用户 ID
- 频道 ID
- 子频道 ID
- 消息 ID
- 身份组 ID

#### 示例

```rust
fn parse_snowflake_timestamp(snowflake: &Snowflake) -> Option<Timestamp> {
    // 雪花 ID 包含时间戳信息
    // 实际解析逻辑取决于 QQ 的雪花算法实现
    snowflake.parse::<u64>().ok().map(|id| {
        // 简化示例 - 实际实现会更复杂
        let timestamp = (id >> 22) + 1420070400000; // QQ epoch
        chrono::DateTime::from_timestamp_millis(timestamp as i64)
            .unwrap_or_default()
            .with_timezone(&chrono::Utc)
    })
}
```

## API 响应类型

### `ApiResponse<T>`

标准 API 响应包装器。

```rust
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}
```

#### 字段

- `code`: 响应状态码
- `message`: 响应消息
- `data`: 响应数据（可选）

#### 示例

```rust
async fn handle_api_response<T>(response: ApiResponse<T>) -> Result<T, String> {
    match response.code {
        0 => {
            if let Some(data) = response.data {
                Ok(data)
            } else {
                Err("响应数据为空".to_string())
            }
        }
        _ => Err(format!("API 错误 {}: {}", response.code, response.message))
    }
}
```

### `PaginatedResponse<T>`

分页响应类型。

```rust
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: Option<u32>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}
```

#### 字段

- `items`: 当前页的项目列表
- `total`: 总项目数（如果可用）
- `has_more`: 是否还有更多页面
- `next_cursor`: 下一页的游标

#### 示例

```rust
async fn fetch_all_items<T>(
    ctx: Context,
    fetch_fn: impl Fn(&str) -> Result<PaginatedResponse<T>>
) -> Result<Vec<T>> {
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let response = fetch_fn(cursor.as_deref().unwrap_or(""))?;
        all_items.extend(response.items);

        if !response.has_more {
            break;
        }

        cursor = response.next_cursor;
    }

    Ok(all_items)
}
```

## 权限类型

### `Permission`

表示权限值。

```rust
pub type Permission = u64;
```

#### 常见权限常量

```rust
pub mod permissions {
    use super::Permission;

    pub const VIEW_CHANNEL: Permission = 1 << 0;
    pub const MANAGE_CHANNEL: Permission = 1 << 1;
    pub const SEND_MESSAGES: Permission = 1 << 2;
    pub const MANAGE_MESSAGES: Permission = 1 << 3;
    pub const EMBED_LINKS: Permission = 1 << 4;
    pub const ATTACH_FILES: Permission = 1 << 5;
    pub const READ_MESSAGE_HISTORY: Permission = 1 << 6;
    pub const MENTION_EVERYONE: Permission = 1 << 7;
    pub const CONNECT: Permission = 1 << 8;
    pub const SPEAK: Permission = 1 << 9;
    pub const MUTE_MEMBERS: Permission = 1 << 10;
    pub const DEAFEN_MEMBERS: Permission = 1 << 11;
}
```

#### 示例

```rust
use crate::permissions::*;

fn check_permissions(user_permissions: Permission, required: Permission) -> bool {
    (user_permissions & required) == required
}

fn has_basic_permissions(permissions: Permission) -> bool {
    check_permissions(permissions, VIEW_CHANNEL | SEND_MESSAGES)
}

fn can_moderate(permissions: Permission) -> bool {
    check_permissions(permissions, MANAGE_MESSAGES | MUTE_MEMBERS)
}

async fn validate_user_action(
    ctx: Context,
    channel_id: &str,
    user_id: &str,
    action: &str
) -> Result<bool> {
    let perms = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    let permission_value = perms.permissions.parse::<Permission>().unwrap_or(0);

    match action {
        "send_message" => Ok(check_permissions(permission_value, SEND_MESSAGES)),
        "attach_file" => Ok(check_permissions(permission_value, ATTACH_FILES)),
        "moderate" => Ok(can_moderate(permission_value)),
        _ => Ok(false),
    }
}
```

## 枚举类型

### `EventType`

事件类型枚举。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Ready,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    DirectMessageCreate,
    GroupMessageCreate,
    C2CMessageCreate,
    MemberAdd,
    MemberUpdate,
    MemberRemove,
    Unknown(String),
}
```

#### 示例

```rust
impl EventType {
    pub fn from_string(event: &str) -> Self {
        match event {
            "READY" => EventType::Ready,
            "GUILD_CREATE" => EventType::GuildCreate,
            "MESSAGE_CREATE" => EventType::MessageCreate,
            "DIRECT_MESSAGE_CREATE" => EventType::DirectMessageCreate,
            "GROUP_AT_MESSAGE_CREATE" => EventType::GroupMessageCreate,
            "C2C_MESSAGE_CREATE" => EventType::C2CMessageCreate,
            _ => EventType::Unknown(event.to_string()),
        }
    }

    pub fn is_message_event(&self) -> bool {
        matches!(self, 
            EventType::MessageCreate |
            EventType::DirectMessageCreate |
            EventType::GroupMessageCreate |
            EventType::C2CMessageCreate
        )
    }
}
```

### `ChannelType`

子频道类型枚举（重新导出以便引用）。

```rust
#[derive(Debug, Clone, PartialEq)]
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

## 错误类型

### `BotError`

主要错误类型。

```rust
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("HTTP 错误: {0}")]
    Http(u16),
    
    #[error("频率限制: 请在 {0} 秒后重试")]
    RateLimit(u64),
    
    #[error("无效数据: {0}")]
    InvalidData(String),
    
    #[error("认证失败: {0}")]
    Authentication(String),
    
    #[error("权限不足")]
    Insufficient Permissions,
    
    #[error("网络错误: {0}")]
    Network(String),
    
    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("WebSocket 错误: {0}")]
    WebSocket(String),
    
    #[error("超时")]
    Timeout,
    
    #[error("未知错误: {0}")]
    Unknown(String),
}
```

#### 示例

```rust
impl BotError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, 
            BotError::Network(_) |
            BotError::Timeout |
            BotError::Http(500..=599)
        )
    }

    pub fn retry_delay(&self) -> Option<std::time::Duration> {
        match self {
            BotError::RateLimit(seconds) => Some(std::time::Duration::from_secs(*seconds)),
            BotError::Network(_) => Some(std::time::Duration::from_secs(5)),
            BotError::Timeout => Some(std::time::Duration::from_secs(3)),
            _ => None,
        }
    }
}
```

## 配置类型

### `ClientConfig`

客户端配置。

```rust
pub struct ClientConfig {
    pub timeout: std::time::Duration,
    pub max_retries: u32,
    pub retry_delay: std::time::Duration,
    pub enable_cache: bool,
    pub cache_size: usize,
    pub user_agent: String,
}
```

#### 默认实现

```rust
impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
            retry_delay: std::time::Duration::from_secs(1),
            enable_cache: true,
            cache_size: 1000,
            user_agent: format!("BotRS/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}
```

#### 示例

```rust
fn create_custom_config() -> ClientConfig {
    ClientConfig {
        timeout: std::time::Duration::from_secs(60),
        max_retries: 5,
        retry_delay: std::time::Duration::from_millis(500),
        enable_cache: true,
        cache_size: 2000,
        user_agent: "MyBot/1.0".to_string(),
    }
}
```

## 实用工具类型

### `HasId`

为具有 ID 的类型提供的 trait。

```rust
pub trait HasId {
    fn id(&self) -> &str;
}
```

#### 实现示例

```rust
impl HasId for Message {
    fn id(&self) -> &str {
        self.id.as_deref().unwrap_or("")
    }
}

impl HasId for Channel {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for Guild {
    fn id(&self) -> &str {
        &self.id
    }
}
```

### `Cacheable`

可缓存类型的 trait。

```rust
pub trait Cacheable: HasId + Clone {
    fn cache_key(&self) -> String {
        format!("{}:{}", std::any::type_name::<Self>(), self.id())
    }

    fn is_expired(&self, max_age: std::time::Duration) -> bool {
        // 默认实现 - 具体类型可以重写
        false
    }
}
```

## 常见使用模式

### 错误处理

```rust
async fn handle_api_call<F, T>(operation: F) -> Result<T, BotError>
where
    F: FnOnce() -> Result<T, BotError>,
{
    let mut retries = 3;
    
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                if error.is_retryable() && retries > 0 {
                    retries -= 1;
                    
                    if let Some(delay) = error.retry_delay() {
                        tokio::time::sleep(delay).await;
                    }
                    
                    continue;
                }
                
                return Err(error);
            }
        }
    }
}
```

### 权限检查

```rust
async fn ensure_permissions(
    ctx: Context,
    channel_id: &str,
    user_id: &str,
    required: Permission,
) -> Result<(), BotError> {
    let perms = ctx.get_channel_user_permissions(channel_id, user_id).await?;
    let permission_value = perms.permissions.parse::<Permission>()
        .map_err(|e| BotError::InvalidData(format!("权限解析失败: {}", e)))?;
    
    if !check_permissions(permission_value, required) {
        return Err(BotError::InsufficientPermissions);
    }
    
    Ok(())
}
```

### 分页数据处理

```rust
async fn collect_all_members(ctx: Context, guild_id: &str) -> Result<Vec<Member>> {
    let mut all_members = Vec::new();
    let mut after: Option<String> = None;
    let limit = 100;
    
    loop {
        let members = ctx.get_guild_members(guild_id, Some(limit), after.as_deref()).await?;
        
        if members.is_empty() {
            break;
        }
        
        // 获取最后一个成员的 ID 用于分页
        if let Some(last_member) = members.last() {
            if let Some(user) = &last_member.user {
                after = Some(user.id.clone());
            }
        }
        
        all_members.extend(members);
        
        // 如果返回的数量小于限制，说明已经是最后一页
        if members.len() < limit as usize {
            break;
        }
    }
    
    Ok(all_members)
}
```

## 相关文档

- [客户端 API](../client.md) - 主要客户端接口
- [错误类型](../error-types.md) - 详细的错误处理
- [消息](./messages.md) - 消息相关类型
- [频道与子频道](./guilds-channels.md) - 频道管理类型
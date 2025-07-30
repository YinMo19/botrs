# Intent 权限 API 参考

`Intents` 结构体控制机器人从网关接收哪些事件。Intent 系统是一个权限系统，允许您控制机器人通过网关连接接收的事件类型，有助于减少带宽和处理开销。

## 概述

```rust
pub struct Intents {
    pub bits: u32,
}
```

Intent 是一个位标志系统，每个位代表一组相关的事件。通过组合不同的 Intent，您可以精确控制机器人需要处理的事件类型。

## 创建 Intent

### `new`

创建一个空的 Intent 集合。

```rust
pub const fn new() -> Self
```

#### 示例

```rust
use botrs::Intents;

let intents = Intents::new();
```

### `none`

创建一个没有启用任何 Intent 的集合。

```rust
pub const fn none() -> Self
```

#### 示例

```rust
let intents = Intents::none();
```

### `all`

创建一个启用所有可用 Intent 的集合。

```rust
pub const fn all() -> Self
```

#### 示例

```rust
let intents = Intents::all();
```

### `default`

创建一个包含常用 Intent 的默认集合。

```rust
impl Default for Intents
```

默认 Intent 包括：
- 频道事件
- 公开频道消息
- 私信消息

#### 示例

```rust
let intents = Intents::default();
```

## 可用的 Intent 类型

### 频道相关

#### `GUILDS`

接收频道创建、更新、删除事件。

```rust
pub const GUILDS: u32 = 1 << 0;
```

#### `GUILD_MEMBERS`

接收频道成员加入、离开、更新事件。

```rust
pub const GUILD_MEMBERS: u32 = 1 << 1;
```

#### `GUILD_MESSAGES`

接收频道内的消息事件（需要机器人被 @ 提及）。

```rust
pub const GUILD_MESSAGES: u32 = 1 << 9;
```

#### `GUILD_MESSAGE_REACTIONS`

接收频道消息的表情回应事件。

```rust
pub const GUILD_MESSAGE_REACTIONS: u32 = 1 << 10;
```

### 消息相关

#### `DIRECT_MESSAGE`

接收私信消息事件。

```rust
pub const DIRECT_MESSAGE: u32 = 1 << 12;
```

#### `GROUP_AND_C2C_EVENT`

接收群组和用户对用户事件。

```rust
pub const GROUP_AND_C2C_EVENT: u32 = 1 << 25;
```

#### `INTERACTION`

接收交互事件（按钮点击、选择菜单等）。

```rust
pub const INTERACTION: u32 = 1 << 26;
```

#### `MESSAGE_AUDIT`

接收消息审核事件。

```rust
pub const MESSAGE_AUDIT: u32 = 1 << 27;
```

### 论坛相关

#### `FORUMS_EVENT`

接收论坛相关事件。

```rust
pub const FORUMS_EVENT: u32 = 1 << 28;
```

#### `AUDIO_OR_LIVE_CHANNEL_MEMBER`

接收音频或直播频道成员事件。

```rust
pub const AUDIO_OR_LIVE_CHANNEL_MEMBER: u32 = 1 << 29;
```

### 特殊权限

#### `PUBLIC_GUILD_MESSAGES`

接收公开频道消息（不需要 @ 提及）。

```rust
pub const PUBLIC_GUILD_MESSAGES: u32 = 1 << 30;
```

## Intent 构建方法

### `with_guilds`

启用频道事件。

```rust
pub const fn with_guilds(self) -> Self
```

#### 示例

```rust
let intents = Intents::new().with_guilds();
```

### `with_guild_members`

启用频道成员事件。

```rust
pub const fn with_guild_members(self) -> Self
```

### `with_guild_messages`

启用频道消息事件（@ 提及）。

```rust
pub const fn with_guild_messages(self) -> Self
```

### `with_public_guild_messages`

启用公开频道消息事件。

```rust
pub const fn with_public_guild_messages(self) -> Self
```

### `with_guild_message_reactions`

启用消息表情回应事件。

```rust
pub const fn with_guild_message_reactions(self) -> Self
```

### `with_direct_message`

启用私信事件。

```rust
pub const fn with_direct_message(self) -> Self
```

### `with_group_and_c2c_event`

启用群组和C2C事件。

```rust
pub const fn with_group_and_c2c_event(self) -> Self
```

### `with_interaction`

启用交互事件。

```rust
pub const fn with_interaction(self) -> Self
```

### `with_message_audit`

启用消息审核事件。

```rust
pub const fn with_message_audit(self) -> Self
```

### `with_forums_event`

启用论坛事件。

```rust
pub const fn with_forums_event(self) -> Self
```

### `with_audio_or_live_channel_member`

启用音频/直播频道成员事件。

```rust
pub const fn with_audio_or_live_channel_member(self) -> Self
```

## Intent 检查方法

### `contains`

检查是否包含指定的 Intent。

```rust
pub const fn contains(self, other: Self) -> bool
```

#### 示例

```rust
let intents = Intents::default().with_guilds();
assert!(intents.contains(Intents::new().with_guilds()));
```

### `is_empty`

检查是否为空（没有启用任何 Intent）。

```rust
pub const fn is_empty(self) -> bool
```

### `is_all`

检查是否启用了所有 Intent。

```rust
pub const fn is_all(self) -> bool
```

## 位运算操作

### `insert`

添加指定的 Intent。

```rust
pub fn insert(&mut self, other: Self)
```

### `remove`

移除指定的 Intent。

```rust
pub fn remove(&mut self, other: Self)
```

### `toggle`

切换指定的 Intent。

```rust
pub fn toggle(&mut self, other: Self)
```

## 使用示例

### 基础消息机器人

```rust
use botrs::{Client, Intents, Token};

// 只接收 @ 提及的消息
let intents = Intents::default()
    .with_guild_messages();

let client = Client::new(token, intents, handler, false)?;
```

### 全功能机器人

```rust
// 接收所有消息和事件
let intents = Intents::default()
    .with_guilds()
    .with_guild_members()
    .with_guild_messages()
    .with_public_guild_messages()
    .with_direct_message()
    .with_interaction();
```

### 私信机器人

```rust
// 只处理私信
let intents = Intents::new()
    .with_direct_message();
```

### 群组机器人

```rust
// 处理群组和C2C消息
let intents = Intents::new()
    .with_group_and_c2c_event();
```

### 论坛机器人

```rust
// 处理论坛事件
let intents = Intents::default()
    .with_forums_event()
    .with_guild_messages();
```

### 管理机器人

```rust
// 处理频道管理事件
let intents = Intents::default()
    .with_guilds()
    .with_guild_members()
    .with_message_audit();
```

### 音频机器人

```rust
// 处理音频频道事件
let intents = Intents::default()
    .with_audio_or_live_channel_member()
    .with_guild_messages();
```

## 条件组合

### 动态 Intent

```rust
fn build_intents(enable_public_messages: bool, enable_dm: bool) -> Intents {
    let mut intents = Intents::default()
        .with_guilds()
        .with_guild_messages();
    
    if enable_public_messages {
        intents = intents.with_public_guild_messages();
    }
    
    if enable_dm {
        intents = intents.with_direct_message();
    }
    
    intents
}
```

### 环境相关 Intent

```rust
fn production_intents() -> Intents {
    Intents::default()
        .with_guild_messages()
        .with_direct_message()
        .with_interaction()
}

fn development_intents() -> Intents {
    Intents::all() // 开发环境接收所有事件
}

fn get_intents() -> Intents {
    if cfg!(debug_assertions) {
        development_intents()
    } else {
        production_intents()
    }
}
```

## 位运算示例

### 手动位操作

```rust
use botrs::Intents;

// 使用位运算直接构建
let intents = Intents { 
    bits: Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGE 
};

// 检查特定位
if intents.bits & Intents::GUILDS != 0 {
    println!("启用了频道事件");
}

// 添加新 Intent
let mut intents = Intents::default();
intents.bits |= Intents::INTERACTION;

// 移除 Intent
intents.bits &= !Intents::INTERACTION;
```

### 集合操作

```rust
let base_intents = Intents::default();
let extra_intents = Intents::new()
    .with_interaction()
    .with_forums_event();

// 合并 Intent
let combined = Intents { 
    bits: base_intents.bits | extra_intents.bits 
};

// 交集
let intersection = Intents { 
    bits: base_intents.bits & extra_intents.bits 
};

// 差集
let difference = Intents { 
    bits: base_intents.bits & !extra_intents.bits 
};
```

## 调试和诊断

### Intent 显示

```rust
impl std::fmt::Display for Intents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        
        if self.bits & Self::GUILDS != 0 {
            parts.push("GUILDS");
        }
        if self.bits & Self::GUILD_MEMBERS != 0 {
            parts.push("GUILD_MEMBERS");
        }
        if self.bits & Self::GUILD_MESSAGES != 0 {
            parts.push("GUILD_MESSAGES");
        }
        if self.bits & Self::PUBLIC_GUILD_MESSAGES != 0 {
            parts.push("PUBLIC_GUILD_MESSAGES");
        }
        if self.bits & Self::DIRECT_MESSAGE != 0 {
            parts.push("DIRECT_MESSAGE");
        }
        
        write!(f, "{}", parts.join(" | "))
    }
}

// 使用
let intents = Intents::default().with_guilds();
println!("启用的 Intent: {}", intents);
```

### Intent 验证

```rust
fn validate_intents(intents: Intents) -> Result<(), String> {
    if intents.is_empty() {
        return Err("至少需要启用一个 Intent".to_string());
    }
    
    // 检查权限组合
    if intents.contains(Intents::new().with_public_guild_messages()) && 
       !intents.contains(Intents::new().with_guilds()) {
        return Err("PUBLIC_GUILD_MESSAGES 需要同时启用 GUILDS".to_string());
    }
    
    Ok(())
}
```

## 性能考虑

### Intent 对性能的影响

```rust
// 高效：只启用必需的事件
let efficient_intents = Intents::new()
    .with_guild_messages()  // 只接收 @ 消息
    .with_direct_message(); // 和私信

// 低效：接收所有事件
let inefficient_intents = Intents::all(); // 会接收大量不需要的事件
```

### 带宽优化

```rust
// 针对聊天机器人优化
let chat_bot_intents = Intents::default()
    .with_guild_messages()
    .with_direct_message();

// 针对管理机器人优化
let admin_bot_intents = Intents::default()
    .with_guilds()
    .with_guild_members()
    .with_message_audit();

// 针对音乐机器人优化
let music_bot_intents = Intents::default()
    .with_guild_messages()
    .with_audio_or_live_channel_member();
```

## 最佳实践

1. **只启用必需的 Intent**：减少不必要的事件处理开销
2. **使用链式调用**：提高代码可读性
3. **环境区分**：开发和生产环境使用不同的 Intent 配置
4. **文档记录**：清楚记录为什么需要特定的 Intent
5. **定期审查**：随着功能变化调整 Intent 配置

## 另请参阅

- [`Client`](./client.md) - 配置客户端时使用 Intent
- [`EventHandler`](./event-handler.md) - 不同 Intent 对应的事件处理方法
- [Intent 系统指南](/zh/guide/intents.md) - Intent 系统详细说明
- [性能优化指南](/zh/guide/performance.md) - Intent 对性能的影响
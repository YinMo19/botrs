# 客户端与事件处理

本指南涵盖 BotRS 的核心概念：`Client` 和 `EventHandler`。这两个组件构成了每个机器人应用程序的基础，处理连接、身份验证和事件处理。

## 理解客户端

`Client` 是机器人的主要协调器。它管理到 QQ 服务器的 WebSocket 连接，处理身份验证，并将事件分派给您的事件处理器。

### 客户端生命周期

```rust
use botrs::{Client, EventHandler, Intents, Token};

// 1. 使用凭据创建令牌
let token = Token::new("你的应用ID", "你的密钥");

// 2. 配置 intent（要接收的事件）
let intents = Intents::default().with_public_guild_messages();

// 3. 创建事件处理器
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 定义如何处理事件
}

// 4. 创建并启动客户端
let mut client = Client::new(token, intents, MyBot, false)?;
client.start().await?; // 这会阻塞直到机器人停止
```

### 客户端配置

#### 环境选择

```rust
// 生产环境
let client = Client::new(token, intents, handler, false)?;

// 沙盒环境（用于测试）
let client = Client::new(token, intents, handler, true)?;
```

#### 连接管理

客户端自动处理：
- WebSocket 连接建立
- 与 QQ 服务器的身份验证
- 心跳维护
- 网络问题时的自动重连
- 速率限制合规

## 理解事件处理器

`EventHandler` trait 定义机器人如何响应来自 QQ 频道的事件。您实现此 trait 来定义机器人的行为。

### 基本事件处理器

```rust
use botrs::{Context, EventHandler, Message, Ready};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 机器人连接时调用一次
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人 {} 已就绪！", ready.user.username);
    }

    // 有人提及您的机器人时调用
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}
```

### 带状态的事件处理器

对于更复杂的机器人，您可以在事件处理器中维护状态：

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

struct StatefulBot {
    // 事件之间的共享状态
    user_data: Arc<RwLock<HashMap<String, UserInfo>>>,
    config: BotConfig,
}

impl StatefulBot {
    fn new(config: BotConfig) -> Self {
        Self {
            user_data: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    async fn get_user_info(&self, user_id: &str) -> Option<UserInfo> {
        let data = self.user_data.read().await;
        data.get(user_id).cloned()
    }
    
    async fn update_user_info(&self, user_id: String, info: UserInfo) {
        let mut data = self.user_data.write().await;
        data.insert(user_id, info);
    }
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        // 访问共享状态
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                // 更新用户信息
                let info = UserInfo {
                    last_message: chrono::Utc::now(),
                    message_count: self.get_user_info(user_id)
                        .await
                        .map(|u| u.message_count + 1)
                        .unwrap_or(1),
                };
                self.update_user_info(user_id.clone(), info).await;
            }
        }
    }
}
```

## Context 参数

每个事件处理器方法都接收一个 `Context` 参数，该参数提供对基本机器人功能的访问：

```rust
pub struct Context {
    pub api: BotApi,     // 用于发出请求的 API 客户端
    pub token: Token,    // 身份验证令牌
    // 其他上下文数据...
}
```

### 使用 Context

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // 发送消息
    let params = MessageParams::new_text("你好！");
    ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
    
    // 获取频道信息
    let guild = ctx.api.get_guild(&ctx.token, &guild_id).await?;
    
    // 管理频道权限
    ctx.api.modify_channel_permissions(&ctx.token, &channel_id, &permissions).await?;
}
```

## 事件类型

### 核心事件

#### Ready 事件
```rust
async fn ready(&self, ctx: Context, ready: Ready) {
    // 机器人已连接并就绪
    // 访问机器人用户信息：ready.user
    // 访问初始频道列表：ready.guilds
}
```

#### 消息事件
```rust
// 带 @提及 的频道消息
async fn message_create(&self, ctx: Context, message: Message) {
    // 处理频道中的 @ 提及
}

// 私信
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    // 处理私人消息
}

// 群消息
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    // 处理群聊消息
}
```

### 频道事件

```rust
// 频道生命周期
async fn guild_create(&self, ctx: Context, guild: Guild) {
    // 机器人加入频道或频道变为可用
}

async fn guild_update(&self, ctx: Context, guild: Guild) {
    // 频道信息更改
}

async fn guild_delete(&self, ctx: Context, guild: Guild) {
    // 机器人离开频道或频道变为不可用
}
```

### 子频道事件

```rust
async fn channel_create(&self, ctx: Context, channel: Channel) {
    // 创建新子频道
}

async fn channel_update(&self, ctx: Context, channel: Channel) {
    // 子频道更新
}

async fn channel_delete(&self, ctx: Context, channel: Channel) {
    // 子频道删除
}
```

### 成员事件

```rust
async fn guild_member_add(&self, ctx: Context, member: Member) {
    // 新成员加入
}

async fn guild_member_update(&self, ctx: Context, member: Member) {
    // 成员信息更新
}

async fn guild_member_remove(&self, ctx: Context, member: Member) {
    // 成员离开或被移除
}
```

## 事件处理器中的错误处理

### 基本错误处理

```rust
async fn message_create(&self, ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        match self.process_command(content).await {
            Ok(response) => {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("发送回复失败: {}", e);
                }
            }
            Err(e) => {
                eprintln!("处理命令时出错: {}", e);
                let _ = message.reply(&ctx.api, &ctx.token, "抱歉，出现了错误！").await;
            }
        }
    }
}
```

### 集中式错误处理

```rust
async fn error(&self, error: BotError) {
    match error {
        BotError::Network(e) => {
            eprintln!("网络错误: {}", e);
            // 也许实现重连逻辑
        }
        BotError::RateLimited(info) => {
            println!("速率限制 {} 秒", info.retry_after);
            // 等待和重试逻辑
        }
        BotError::Authentication(e) => {
            eprintln!("认证错误: {}", e);
            // 处理认证问题
        }
        _ => {
            eprintln!("意外错误: {}", error);
        }
    }
}
```

## 最佳实践

### 性能

1. **保持事件处理器轻量级**
   ```rust
   async fn message_create(&self, ctx: Context, message: Message) {
       // 在后台生成繁重的工作
       let api = ctx.api.clone();
       let token = ctx.token.clone();
       
       tokio::spawn(async move {
           // 繁重的计算在这里
           let result = heavy_computation().await;
           // 将结果发送回频道
       });
   }
   ```

2. **为状态使用适当的数据结构**
   ```rust
   // 对于读密集型工作负载
   use std::sync::Arc;
   use tokio::sync::RwLock;
   
   // 对于简单的原子操作
   use std::sync::atomic::{AtomicU64, Ordering};
   
   // 对于并发集合
   use dashmap::DashMap;
   ```

### 错误恢复

1. **优雅降级**
   ```rust
   async fn message_create(&self, ctx: Context, message: Message) {
       match self.get_user_permissions(&ctx, &message).await {
           Ok(perms) if perms.can_execute_commands() => {
               // 执行命令
           }
           Ok(_) => {
               // 用户没有权限
               let _ = message.reply(&ctx.api, &ctx.token, "权限拒绝").await;
           }
           Err(_) => {
               // 后备：允许命令但记录错误
               eprintln!("检查权限失败，允许命令");
           }
       }
   }
   ```

2. **对瞬时故障的重试逻辑**
   ```rust
   async fn send_with_retry(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), BotError> {
       for attempt in 1..=3 {
           match ctx.api.post_message_with_params(
               &ctx.token, 
               channel_id, 
               MessageParams::new_text(content)
           ).await {
               Ok(response) => return Ok(()),
               Err(BotError::Network(_)) if attempt < 3 => {
                   tokio::time::sleep(Duration::from_millis(1000 * attempt)).await;
                   continue;
               }
               Err(e) => return Err(e),
           }
       }
       unreachable!()
   }
   ```

### 资源管理

1. **限制并发操作**
   ```rust
   use tokio::sync::Semaphore;
   
   struct MyBot {
       semaphore: Arc<Semaphore>,
   }
   
   impl MyBot {
       fn new() -> Self {
           Self {
               semaphore: Arc::new(Semaphore::new(10)), // 最多 10 个并发操作
           }
       }
   }
   
   #[async_trait::async_trait]
   impl EventHandler for MyBot {
       async fn message_create(&self, ctx: Context, message: Message) {
           let _permit = self.semaphore.acquire().await.unwrap();
           // 以有限并发处理消息
       }
   }
   ```

## 完整示例

这是一个演示这些概念的综合示例：

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token, BotError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Clone)]
struct UserStats {
    message_count: u64,
    last_active: chrono::DateTime<chrono::Utc>,
}

struct ComprehensiveBot {
    stats: Arc<RwLock<HashMap<String, UserStats>>>,
    start_time: chrono::DateTime<chrono::Utc>,
}

impl ComprehensiveBot {
    fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(HashMap::new())),
            start_time: chrono::Utc::now(),
        }
    }
    
    async fn update_user_stats(&self, user_id: &str) {
        let mut stats = self.stats.write().await;
        let entry = stats.entry(user_id.to_string()).or_insert(UserStats {
            message_count: 0,
            last_active: chrono::Utc::now(),
        });
        entry.message_count += 1;
        entry.last_active = chrono::Utc::now();
    }
    
    async fn handle_command(&self, ctx: &Context, message: &Message, command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match command {
            "ping" => Ok("Pong! 🏓".to_string()),
            "uptime" => {
                let uptime = chrono::Utc::now() - self.start_time;
                Ok(format!("机器人运行时间：{} 秒", uptime.num_seconds()))
            }
            "stats" => {
                if let Some(author) = &message.author {
                    if let Some(user_id) = &author.id {
                        let stats = self.stats.read().await;
                        if let Some(user_stats) = stats.get(user_id) {
                            Ok(format!("发送消息数：{}，最后活跃：{}", 
                                     user_stats.message_count, 
                                     user_stats.last_active.format("%Y-%m-%d %H:%M:%S")))
                        } else {
                            Ok("无统计数据".to_string())
                        }
                    } else {
                        Ok("无法识别用户".to_string())
                    }
                } else {
                    Ok("无作者信息".to_string())
                }
            }
            "help" => Ok("可用命令：!ping, !uptime, !stats, !help".to_string()),
            _ => Ok(format!("未知命令：{}。输入 !help 查看可用命令。", command)),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ComprehensiveBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("🤖 机器人已就绪！登录为：{}", ready.user.username);
        info!("📊 连接到 {} 个频道", ready.guilds.len());
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        // 跳过机器人消息
        if message.is_from_bot() {
            return;
        }
        
        // 更新用户统计
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                self.update_user_stats(user_id).await;
            }
        }
        
        // 处理命令
        if let Some(content) = &message.content {
            let content = content.trim();
            if let Some(command_text) = content.strip_prefix('!') {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if !parts.is_empty() {
                    let command = parts[0];
                    let args = &parts[1..];
                    
                    match self.handle_command(&ctx, &message, command, args).await {
                        Ok(response) => {
                            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                                warn!("发送回复失败: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("处理命令 '{}' 时出错: {}", command, e);
                            let _ = message.reply(&ctx.api, &ctx.token, "抱歉，出现了错误！").await;
                        }
                    }
                }
            }
        }
    }
    
    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        info!("📥 加入频道：{}", guild.name.as_deref().unwrap_or("未知"));
    }
    
    async fn guild_delete(&self, _ctx: Context, guild: Guild) {
        info!("📤 离开频道：{}", guild.name.as_deref().unwrap_or("未知"));
    }
    
    async fn error(&self, error: BotError) {
        match error {
            BotError::Network(ref e) => {
                warn!("🌐 网络错误: {}", e);
            }
            BotError::RateLimited(ref info) => {
                warn!("⏰ 速率限制 {} 秒", info.retry_after);
            }
            BotError::Authentication(ref e) => {
                error!("🔐 认证错误: {}", e);
            }
            _ => {
                error!("❌ 意外错误: {}", error);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,comprehensive_bot=info")
        .init();
    
    // 加载配置
    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID")?,
        std::env::var("QQ_BOT_SECRET")?,
    );
    
    // 配置 intent
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds();
    
    // 创建并启动机器人
    let mut client = Client::new(token, intents, ComprehensiveBot::new(), false)?;
    
    info!("🚀 启动综合机器人...");
    client.start().await?;
    
    Ok(())
}
```

此示例演示了：
- 带用户统计的状态事件处理
- 带错误处理的命令处理
- 适当的日志记录和监控
- 异步操作的资源管理
- 全面的事件覆盖

## 下一步

- [消息与回复](./messages.md) - 了解发送不同类型的消息
- [Intent 系统](./intents.md) - 理解事件过滤和权限
- [配置](./configuration.md) - 高级配置选项
- [错误处理](./error-handling.md) - 健壮的错误处理模式
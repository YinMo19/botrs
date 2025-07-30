# WebSocket 网关指南

WebSocket 网关是 BotRS 与 QQ 频道服务器建立实时连接的核心组件。它负责维护持久连接、处理心跳、管理会话状态，以及接收和分发实时事件。本指南将详细介绍网关的工作原理和最佳实践。

## 概述

QQ 频道机器人使用 WebSocket 连接来接收实时事件，如消息创建、成员加入、频道更新等。BotRS 的网关组件自动处理连接管理、身份验证、心跳维护和事件分发。

```rust
use botrs::{Client, EventHandler, Intents, Token};

// 网关连接通过 Client 自动管理
let mut client = Client::new(token, intents, handler, false)?;
client.start().await?; // 启动网关连接
```

## 网关架构

### 连接生命周期

1. **连接建立**: 客户端连接到 QQ 的 WebSocket 端点
2. **身份验证**: 发送包含令牌和 Intent 的 Identify 负载
3. **就绪事件**: 服务器返回 Ready 事件，包含会话信息
4. **事件循环**: 持续接收和处理事件
5. **心跳维护**: 定期发送心跳以保持连接活跃
6. **重连处理**: 连接断开时自动重新连接

### 网关端点

```rust
// 生产环境
const GATEWAY_URL: &str = "wss://api.sgroup.qq.com/websocket";

// 沙盒环境
const SANDBOX_GATEWAY_URL: &str = "wss://sandbox.api.sgroup.qq.com/websocket";
```

## 连接管理

### 自动重连

BotRS 实现了智能重连机制：

```rust
// 重连策略配置
pub struct ReconnectConfig {
    pub max_attempts: usize,     // 最大重连尝试次数
    pub initial_delay: Duration, // 初始延迟
    pub max_delay: Duration,     // 最大延迟
    pub backoff_factor: f64,     // 退避因子
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
        }
    }
}
```

### 连接状态监控

```rust
use botrs::{Client, ConnectionState};

// 检查连接状态
if client.is_connected() {
    println!("网关已连接");
} else {
    println!("网关未连接");
}

// 获取详细会话信息
if let Some(session) = client.get_session_info() {
    println!("会话 ID: {}", session.session_id);
    println!("序列号: {}", session.sequence_number);
    println!("分片信息: {:?}", session.shard);
}
```

## 心跳机制

### 心跳工作原理

网关要求客户端定期发送心跳来维持连接：

```rust
// 心跳间隔由服务器在 Hello 事件中指定
pub struct HeartbeatConfig {
    pub interval: Duration,      // 心跳间隔
    pub timeout: Duration,       // 心跳超时
    pub max_missed: usize,       // 最大失败次数
}

// 心跳负载格式
#[derive(Serialize)]
struct HeartbeatPayload {
    op: u8,           // 操作码 1
    d: Option<u64>,   // 最后接收的序列号
}
```

### 心跳失败处理

```rust
// 心跳失败时的处理逻辑
async fn handle_heartbeat_failure(&mut self) {
    warn!("心跳失败，尝试重新连接");
    
    // 关闭当前连接
    self.close_connection().await;
    
    // 触发重连
    self.schedule_reconnect().await;
}
```

## 事件处理

### 事件分发流程

```rust
// 网关事件处理流程
async fn handle_gateway_event(&self, event: GatewayEvent) {
    match event {
        GatewayEvent::Ready(ready) => {
            info!("网关就绪: {}", ready.user.username);
            self.handler.ready(self.context.clone(), ready).await;
        }
        GatewayEvent::MessageCreate(message) => {
            self.handler.message_create(self.context.clone(), message).await;
        }
        GatewayEvent::GuildCreate(guild) => {
            self.handler.guild_create(self.context.clone(), guild).await;
        }
        // ... 其他事件类型
    }
}
```

### 事件过滤和预处理

```rust
use botrs::{EventHandler, Context, Message};

struct FilteringHandler {
    allowed_guilds: Vec<String>,
    command_prefix: String,
}

#[async_trait::async_trait]
impl EventHandler for FilteringHandler {
    async fn message_create(&self, ctx: Context, message: Message) {
        // 过滤：只处理允许的频道
        if let Some(guild_id) = &message.guild_id {
            if !self.allowed_guilds.contains(guild_id) {
                return;
            }
        }
        
        // 预处理：检查命令前缀
        if let Some(content) = &message.content {
            if content.starts_with(&self.command_prefix) {
                self.handle_command(&ctx, &message, content).await;
            }
        }
    }
    
    async fn handle_command(&self, ctx: &Context, message: &Message, content: &str) {
        let command = content.strip_prefix(&self.command_prefix).unwrap_or("");
        
        match command.trim() {
            "ping" => {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
            "status" => {
                let status = if ctx.client.is_connected() {
                    "在线"
                } else {
                    "离线"
                };
                let _ = message.reply(&ctx.api, &ctx.token, &format!("状态: {}", status)).await;
            }
            _ => {}
        }
    }
}
```

## Intent 系统与网关

### Intent 对网关的影响

Intent 决定网关将接收哪些事件类型：

```rust
use botrs::Intents;

// 基础事件
let basic_intents = Intents::default()
    .with_guilds()              // 频道事件
    .with_guild_messages();     // @ 消息事件

// 完整事件（高带宽）
let full_intents = Intents::all();

// 自定义事件组合
let custom_intents = Intents::new()
    .with_public_guild_messages()  // 公开消息
    .with_direct_message()         // 私信
    .with_guild_members();         // 成员事件
```

### Intent 验证

```rust
async fn validate_intents_permissions(
    api: &BotApi,
    token: &Token,
    intents: Intents
) -> Result<(), BotError> {
    // 检查机器人是否有相应的权限
    let app_info = api.get_current_application(token).await?;
    
    if intents.contains(Intents::new().with_public_guild_messages()) {
        // 需要消息内容权限
        if !app_info.flags.contains("MESSAGE_CONTENT_INTENT") {
            return Err(BotError::InsufficientPermissions(
                "需要消息内容 Intent 权限".to_string()
            ));
        }
    }
    
    Ok(())
}
```

## 分片支持

### 分片配置

当机器人加入大量频道时，可能需要使用分片来分散负载：

```rust
#[derive(Clone)]
pub struct ShardConfig {
    pub shard_id: u32,      // 当前分片 ID
    pub shard_count: u32,   // 总分片数
}

// 创建分片客户端
async fn create_sharded_client(
    token: Token,
    intents: Intents,
    handler: impl EventHandler + Clone,
    shard_config: ShardConfig
) -> Result<Client<impl EventHandler>, BotError> {
    let client = Client::new_with_shard(
        token,
        intents,
        handler,
        false,
        Some(shard_config)
    )?;
    
    Ok(client)
}
```

### 多分片管理

```rust
use tokio::task::JoinSet;

async fn run_sharded_bot(
    token: Token,
    intents: Intents,
    handler: impl EventHandler + Clone + 'static,
    shard_count: u32
) -> Result<(), BotError> {
    let mut join_set = JoinSet::new();
    
    for shard_id in 0..shard_count {
        let token = token.clone();
        let intents = intents;
        let handler = handler.clone();
        
        join_set.spawn(async move {
            let shard_config = ShardConfig { shard_id, shard_count };
            let mut client = create_sharded_client(
                token, intents, handler, shard_config
            ).await?;
            
            info!("启动分片 {}/{}", shard_id + 1, shard_count);
            client.start().await
        });
        
        // 避免同时连接过多分片
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    
    // 等待所有分片完成
    while let Some(result) = join_set.join_next().await {
        if let Err(e) = result? {
            error!("分片错误: {}", e);
        }
    }
    
    Ok(())
}
```

## 错误处理和恢复

### 网关错误类型

```rust
#[derive(Debug)]
pub enum GatewayError {
    ConnectionFailed(String),     // 连接失败
    AuthenticationFailed,         // 身份验证失败
    InvalidSession,               // 会话无效
    HeartbeatTimeout,            // 心跳超时
    RateLimited(u64),            // 速率限制
    InvalidIntents,              // Intent 无效
    UnknownOpcode(u8),           // 未知操作码
}
```

### 错误恢复策略

```rust
async fn handle_gateway_error(&mut self, error: GatewayError) -> Result<(), BotError> {
    match error {
        GatewayError::InvalidSession => {
            warn!("会话无效，重新进行身份验证");
            self.reset_session().await;
            self.reconnect(true).await
        }
        GatewayError::HeartbeatTimeout => {
            warn!("心跳超时，重新连接");
            self.reconnect(false).await
        }
        GatewayError::RateLimited(retry_after) => {
            warn!("网关速率限制，{}秒后重试", retry_after);
            tokio::time::sleep(Duration::from_secs(retry_after)).await;
            self.reconnect(false).await
        }
        GatewayError::AuthenticationFailed => {
            error!("身份验证失败，检查令牌");
            Err(BotError::Authentication("网关认证失败".to_string()))
        }
        _ => {
            warn!("网关错误: {:?}，尝试重连", error);
            self.reconnect(false).await
        }
    }
}
```

## 性能优化

### 网关性能监控

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct GatewayMetrics {
    events_received: AtomicU64,
    events_processed: AtomicU64,
    reconnection_count: AtomicU64,
    last_heartbeat: std::sync::RwLock<Option<Instant>>,
    connection_start: std::sync::RwLock<Option<Instant>>,
}

impl GatewayMetrics {
    pub fn new() -> Self {
        Self {
            events_received: AtomicU64::new(0),
            events_processed: AtomicU64::new(0),
            reconnection_count: AtomicU64::new(0),
            last_heartbeat: std::sync::RwLock::new(None),
            connection_start: std::sync::RwLock::new(None),
        }
    }
    
    pub fn record_event_received(&self) {
        self.events_received.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_event_processed(&self) {
        self.events_processed.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_events_per_second(&self) -> f64 {
        let events = self.events_processed.load(Ordering::Relaxed) as f64;
        if let Some(start) = *self.connection_start.read().unwrap() {
            let duration = start.elapsed().as_secs_f64();
            if duration > 0.0 {
                return events / duration;
            }
        }
        0.0
    }
}
```

### 事件处理优化

```rust
use tokio::sync::mpsc;
use tokio::task;

pub struct OptimizedEventHandler {
    event_sender: mpsc::UnboundedSender<GatewayEvent>,
}

impl OptimizedEventHandler {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        
        // 在后台任务中处理事件
        task::spawn(async move {
            while let Some(event) = receiver.recv().await {
                // 并发处理事件
                task::spawn(async move {
                    Self::process_event(event).await;
                });
            }
        });
        
        Self {
            event_sender: sender,
        }
    }
    
    pub fn handle_event(&self, event: GatewayEvent) {
        // 非阻塞地发送事件到处理队列
        if let Err(_) = self.event_sender.send(event) {
            warn!("事件队列已满，丢弃事件");
        }
    }
    
    async fn process_event(event: GatewayEvent) {
        // 实际的事件处理逻辑
        match event {
            GatewayEvent::MessageCreate(message) => {
                // 处理消息
            }
            _ => {}
        }
    }
}
```

## 调试和诊断

### 网关日志配置

```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber::{EnvFilter, fmt};

// 配置详细的网关日志
fn setup_gateway_logging() {
    let filter = EnvFilter::new("botrs::gateway=debug,botrs::client=info");
    
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

// 在事件处理器中添加诊断日志
#[async_trait::async_trait]
impl EventHandler for DiagnosticHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("网关就绪事件:");
        info!("  用户: {}", ready.user.username);
        info!("  会话 ID: {}", ready.session_id);
        info!("  API 版本: {}", ready.version);
        
        if let Some(shard) = ready.shard {
            info!("  分片: {}/{}", shard.0, shard.1);
        }
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        debug!("收到消息事件:");
        debug!("  消息 ID: {}", message.id);
        debug!("  频道 ID: {}", message.channel_id);
        debug!("  作者: {:?}", message.author.as_ref().map(|a| &a.username));
    }
}
```

### 连接诊断工具

```rust
pub struct GatewayDiagnostics {
    start_time: Instant,
    connection_attempts: AtomicU64,
    successful_connections: AtomicU64,
    total_events: AtomicU64,
}

impl GatewayDiagnostics {
    pub fn print_status(&self) {
        let uptime = self.start_time.elapsed();
        let attempts = self.connection_attempts.load(Ordering::Relaxed);
        let successful = self.successful_connections.load(Ordering::Relaxed);
        let events = self.total_events.load(Ordering::Relaxed);
        
        println!("=== 网关诊断信息 ===");
        println!("运行时间: {:?}", uptime);
        println!("连接尝试: {}", attempts);
        println!("成功连接: {}", successful);
        println!("连接成功率: {:.2}%", 
                (successful as f64 / attempts as f64) * 100.0);
        println!("接收事件: {}", events);
        println!("事件速率: {:.2} 事件/秒", 
                events as f64 / uptime.as_secs_f64());
    }
}
```

## 最佳实践

### 网关配置建议

1. **Intent 选择**: 只启用必需的 Intent 以减少带宽使用
2. **心跳监控**: 实现心跳超时检测和自动恢复
3. **重连策略**: 使用指数退避避免频繁重连
4. **错误日志**: 记录详细的网关错误信息
5. **性能监控**: 监控事件处理延迟和吞吐量

### 生产环境配置

```rust
pub struct ProductionGatewayConfig {
    pub heartbeat_timeout: Duration,
    pub reconnect_attempts: usize,
    pub event_buffer_size: usize,
    pub metrics_enabled: bool,
    pub debug_logging: bool,
}

impl Default for ProductionGatewayConfig {
    fn default() -> Self {
        Self {
            heartbeat_timeout: Duration::from_secs(60),
            reconnect_attempts: 5,
            event_buffer_size: 1000,
            metrics_enabled: true,
            debug_logging: false,
        }
    }
}
```

网关是机器人与 QQ 频道平台交互的关键组件。通过理解其工作原理并遵循最佳实践，您可以构建出稳定、高效的机器人应用程序。

## 另请参阅

- [客户端与事件处理指南](/zh/guide/client-handler.md) - 客户端配置和事件处理
- [Intent 系统指南](/zh/guide/intents.md) - Intent 权限详细说明
- [错误处理指南](/zh/guide/error-handling.md) - 网关错误处理策略
- [`Client` API 参考](/zh/api/client.md) - 客户端 API 文档
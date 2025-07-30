# 错误恢复示例

本示例展示如何在 BotRS 机器人中实现强大的错误处理和恢复机制，确保机器人在各种异常情况下都能保持稳定运行。

## 概述

在生产环境中，机器人可能会遇到各种错误：网络连接问题、API 速率限制、服务器临时不可用等。本示例展示如何优雅地处理这些错误并实现自动恢复功能。

## 错误类型分析

### 网络相关错误

```rust
use botrs::{BotError, Context, EventHandler, Message};
use tokio::time::{sleep, Duration};
use tracing::{warn, error, info};

async fn handle_network_error(
    ctx: &Context,
    channel_id: &str,
    content: &str,
    error: &BotError
) -> Result<(), BotError> {
    match error {
        BotError::Network(msg) => {
            warn!("网络错误: {}", msg);
            // 实现指数退避重试
            exponential_backoff_retry(|| async {
                let params = MessageParams::new_text(content);
                ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
            }, 3).await
        }
        BotError::Timeout => {
            warn!("请求超时，使用更长的超时时间重试");
            // 使用更长的超时时间重试
            retry_with_extended_timeout(ctx, channel_id, content).await
        }
        _ => Err(error.clone()),
    }
}
```

### API 速率限制处理

```rust
async fn handle_rate_limit(
    operation: impl Fn() -> Pin<Box<dyn Future<Output = Result<Message, BotError>> + Send>>,
    max_retries: usize
) -> Result<Message, BotError> {
    let mut retries = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(BotError::RateLimited(retry_after)) => {
                if retries >= max_retries {
                    error!("速率限制重试次数已达上限");
                    return Err(BotError::RateLimited(retry_after));
                }
                
                warn!("遇到速率限制，等待 {} 秒后重试 ({}/{})", 
                      retry_after, retries + 1, max_retries);
                
                sleep(Duration::from_secs(retry_after)).await;
                retries += 1;
            }
            Err(other_error) => return Err(other_error),
        }
    }
}
```

## 重试机制实现

### 指数退避重试

```rust
use std::pin::Pin;
use std::future::Future;

async fn exponential_backoff_retry<T, F, Fut, E>(
    operation: F,
    max_attempts: usize
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut delay = Duration::from_millis(500); // 初始延迟 500ms
    let max_delay = Duration::from_secs(30);    // 最大延迟 30 秒
    
    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    info!("操作在第 {} 次尝试后成功", attempt);
                }
                return Ok(result);
            }
            Err(error) => {
                if attempt == max_attempts {
                    error!("操作在 {} 次尝试后仍然失败: {}", max_attempts, error);
                    return Err(error);
                }
                
                warn!("第 {} 次尝试失败: {}，{}ms 后重试", attempt, error, delay.as_millis());
                sleep(delay).await;
                
                // 指数退避：每次失败后延迟时间翻倍
                delay = std::cmp::min(delay * 2, max_delay);
            }
        }
    }
    
    unreachable!()
}
```

### 智能重试策略

```rust
#[derive(Clone)]
pub struct RetryConfig {
    pub max_attempts: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
    pub jitter: bool, // 添加随机性避免雷群效应
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
            jitter: true,
        }
    }
}

async fn smart_retry<T, F, Fut>(
    operation: F,
    config: RetryConfig,
    is_retryable: impl Fn(&BotError) -> bool,
) -> Result<T, BotError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, BotError>>,
{
    let mut delay = config.initial_delay;
    
    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                // 检查错误是否可重试
                if !is_retryable(&error) {
                    warn!("遇到不可重试的错误: {}", error);
                    return Err(error);
                }
                
                if attempt == config.max_attempts {
                    error!("智能重试达到最大次数限制: {}", error);
                    return Err(error);
                }
                
                // 添加随机抖动
                let actual_delay = if config.jitter {
                    let jitter_range = delay.as_millis() / 4; // 25% 抖动
                    let jitter = fastrand::u64(0..=jitter_range);
                    delay + Duration::from_millis(jitter)
                } else {
                    delay
                };
                
                warn!("第 {} 次尝试失败，{}ms 后重试", attempt, actual_delay.as_millis());
                sleep(actual_delay).await;
                
                // 计算下次延迟
                delay = std::cmp::min(
                    Duration::from_millis(
                        (delay.as_millis() as f64 * config.backoff_factor) as u64
                    ),
                    config.max_delay
                );
            }
        }
    }
    
    unreachable!()
}

// 定义哪些错误可以重试
fn is_retryable_error(error: &BotError) -> bool {
    match error {
        BotError::Network(_) => true,
        BotError::Timeout => true,
        BotError::RateLimited(_) => true,
        BotError::ServerError(_) => true,
        BotError::Authentication(_) => false, // 认证错误不应重试
        BotError::Forbidden => false,         // 权限错误不应重试
        BotError::NotFound => false,          // 资源不存在不应重试
        _ => false,
    }
}
```

## 连接恢复机制

### 自动重连处理

```rust
use botrs::{Client, ConnectionState};
use std::sync::Arc;
use tokio::sync::Notify;

pub struct ResilienceBotClient<H: EventHandler> {
    client: Client<H>,
    reconnect_notify: Arc<Notify>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
    reconnect_config: RetryConfig,
}

impl<H: EventHandler> ResilienceBotClient<H> {
    pub fn new(
        client: Client<H>,
        reconnect_config: Option<RetryConfig>
    ) -> Self {
        Self {
            client,
            reconnect_notify: Arc::new(Notify::new()),
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            reconnect_config: reconnect_config.unwrap_or_default(),
        }
    }
    
    pub async fn start_with_recovery(&mut self) -> Result<(), BotError> {
        self.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        loop {
            if !self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            
            info!("尝试启动机器人连接");
            
            match self.client.start().await {
                Ok(_) => {
                    info!("机器人正常停止");
                    break;
                }
                Err(error) => {
                    error!("机器人连接失败: {}", error);
                    
                    if !self.should_reconnect(&error) {
                        error!("遇到不可恢复的错误，停止重连");
                        return Err(error);
                    }
                    
                    if let Err(e) = self.wait_for_reconnect().await {
                        error!("重连等待失败: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn wait_for_reconnect(&self) -> Result<(), BotError> {
        smart_retry(
            || async { 
                info!("准备重新连接");
                Ok(()) 
            },
            self.reconnect_config.clone(),
            |_| true, // 重连准备总是可重试的
        ).await
    }
    
    fn should_reconnect(&self, error: &BotError) -> bool {
        match error {
            BotError::Authentication(_) => false,
            BotError::InvalidInput(_) => false,
            _ => true,
        }
    }
    
    pub fn stop(&self) {
        self.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.reconnect_notify.notify_one();
    }
}
```

### 健康检查机制

```rust
use std::time::Instant;
use tokio::time::interval;

pub struct HealthChecker {
    last_heartbeat: Arc<std::sync::RwLock<Option<Instant>>>,
    last_message: Arc<std::sync::RwLock<Option<Instant>>>,
    check_interval: Duration,
    heartbeat_timeout: Duration,
    message_timeout: Duration,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            last_heartbeat: Arc::new(std::sync::RwLock::new(None)),
            last_message: Arc::new(std::sync::RwLock::new(None)),
            check_interval: Duration::from_secs(30),
            heartbeat_timeout: Duration::from_secs(120),
            message_timeout: Duration::from_secs(300),
        }
    }
    
    pub async fn start_monitoring(&self) -> Result<(), BotError> {
        let mut interval = interval(self.check_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.perform_health_check().await {
                error!("健康检查失败: {}", e);
                return Err(e);
            }
        }
    }
    
    async fn perform_health_check(&self) -> Result<(), BotError> {
        let now = Instant::now();
        
        // 检查心跳
        if let Some(last_heartbeat) = *self.last_heartbeat.read().unwrap() {
            if now.duration_since(last_heartbeat) > self.heartbeat_timeout {
                return Err(BotError::Custom("心跳超时".to_string()));
            }
        }
        
        // 检查消息活动
        if let Some(last_message) = *self.last_message.read().unwrap() {
            if now.duration_since(last_message) > self.message_timeout {
                warn!("长时间未收到消息，可能存在连接问题");
            }
        }
        
        info!("健康检查通过");
        Ok(())
    }
    
    pub fn update_heartbeat(&self) {
        *self.last_heartbeat.write().unwrap() = Some(Instant::now());
    }
    
    pub fn update_message_activity(&self) {
        *self.last_message.write().unwrap() = Some(Instant::now());
    }
}
```

## 状态管理和持久化

### 应用状态管理

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct BotState {
    pub last_message_id: Option<String>,
    pub processed_messages: u64,
    pub error_count: u64,
    pub last_error: Option<String>,
    pub uptime_start: Option<chrono::DateTime<chrono::Utc>>,
    pub user_data: HashMap<String, UserData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub last_interaction: chrono::DateTime<chrono::Utc>,
    pub message_count: u64,
    pub preferences: HashMap<String, String>,
}

impl BotState {
    pub fn new() -> Self {
        Self {
            last_message_id: None,
            processed_messages: 0,
            error_count: 0,
            last_error: None,
            uptime_start: Some(chrono::Utc::now()),
            user_data: HashMap::new(),
        }
    }
    
    pub async fn save_to_file(&self, path: &str) -> Result<(), BotError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| BotError::Custom(format!("序列化状态失败: {}", e)))?;
        
        fs::write(path, json).await
            .map_err(|e| BotError::Custom(format!("保存状态文件失败: {}", e)))?;
        
        Ok(())
    }
    
    pub async fn load_from_file(path: &str) -> Result<Self, BotError> {
        match fs::read_to_string(path).await {
            Ok(json) => {
                serde_json::from_str(&json)
                    .map_err(|e| BotError::Custom(format!("反序列化状态失败: {}", e)))
            }
            Err(_) => {
                info!("状态文件不存在，创建新状态");
                Ok(Self::new())
            }
        }
    }
    
    pub fn record_message(&mut self, message_id: String, user_id: String) {
        self.last_message_id = Some(message_id);
        self.processed_messages += 1;
        
        let user_data = self.user_data.entry(user_id).or_insert_with(|| UserData {
            last_interaction: chrono::Utc::now(),
            message_count: 0,
            preferences: HashMap::new(),
        });
        
        user_data.last_interaction = chrono::Utc::now();
        user_data.message_count += 1;
    }
    
    pub fn record_error(&mut self, error: &str) {
        self.error_count += 1;
        self.last_error = Some(error.to_string());
    }
}
```

### 定期状态保存

```rust
pub struct StatePersistence {
    state: Arc<tokio::sync::RwLock<BotState>>,
    save_path: String,
    save_interval: Duration,
}

impl StatePersistence {
    pub fn new(save_path: String, save_interval: Duration) -> Self {
        Self {
            state: Arc::new(tokio::sync::RwLock::new(BotState::new())),
            save_path,
            save_interval,
        }
    }
    
    pub async fn load_initial_state(&self) -> Result<(), BotError> {
        let loaded_state = BotState::load_from_file(&self.save_path).await?;
        *self.state.write().await = loaded_state;
        info!("已加载初始状态");
        Ok(())
    }
    
    pub async fn start_auto_save(&self) -> Result<(), BotError> {
        let state = self.state.clone();
        let save_path = self.save_path.clone();
        let mut interval = interval(self.save_interval);
        
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                
                let current_state = state.read().await.clone();
                if let Err(e) = current_state.save_to_file(&save_path).await {
                    error!("自动保存状态失败: {}", e);
                } else {
                    info!("状态已自动保存");
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn get_state(&self) -> BotState {
        self.state.read().await.clone()
    }
    
    pub async fn update_state<F>(&self, updater: F)
    where
        F: FnOnce(&mut BotState),
    {
        let mut state = self.state.write().await;
        updater(&mut *state);
    }
}
```

## 错误恢复事件处理器

```rust
use botrs::{Context, EventHandler, Message, Ready, DirectMessage, GroupMessage};

pub struct ErrorRecoveryHandler {
    retry_config: RetryConfig,
    health_checker: Arc<HealthChecker>,
    state_persistence: Arc<StatePersistence>,
}

impl ErrorRecoveryHandler {
    pub fn new() -> Self {
        let health_checker = Arc::new(HealthChecker::new());
        let state_persistence = Arc::new(StatePersistence::new(
            "bot_state.json".to_string(),
            Duration::from_secs(60), // 每分钟保存一次
        ));
        
        Self {
            retry_config: RetryConfig::default(),
            health_checker,
            state_persistence,
        }
    }
    
    async fn safe_send_message(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<Message, BotError> {
        smart_retry(
            || async {
                let params = MessageParams::new_text(content);
                ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
            },
            self.retry_config.clone(),
            is_retryable_error,
        ).await
    }
    
    async fn handle_operation_error(&self, operation: &str, error: &BotError) {
        error!("操作 '{}' 失败: {}", operation, error);
        
        // 记录错误到状态
        self.state_persistence.update_state(|state| {
            state.record_error(&format!("{}: {}", operation, error));
        }).await;
        
        // 根据错误类型执行不同的恢复策略
        match error {
            BotError::RateLimited(retry_after) => {
                warn!("遇到速率限制，暂停操作 {} 秒", retry_after);
                sleep(Duration::from_secs(*retry_after)).await;
            }
            BotError::Network(_) => {
                warn!("网络错误，检查连接状态");
                // 可以触发连接健康检查
            }
            BotError::Authentication(_) => {
                error!("认证错误，需要人工干预");
                // 可以发送警报通知管理员
            }
            _ => {}
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ErrorRecoveryHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("错误恢复机器人已就绪: {}", ready.user.username);
        
        // 启动健康检查
        let health_checker = self.health_checker.clone();
        tokio::spawn(async move {
            if let Err(e) = health_checker.start_monitoring().await {
                error!("健康检查器启动失败: {}", e);
            }
        });
        
        // 启动状态持久化
        if let Err(e) = self.state_persistence.load_initial_state().await {
            error!("加载初始状态失败: {}", e);
        }
        
        if let Err(e) = self.state_persistence.start_auto_save().await {
            error!("启动自动保存失败: {}", e);
        }
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        self.health_checker.update_message_activity();
        
        // 更新状态
        if let Some(author) = &message.author {
            self.state_persistence.update_state(|state| {
                state.record_message(message.id.clone(), author.id.clone());
            }).await;
        }
        
        if message.is_from_bot() {
            return;
        }
        
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };
        
        // 使用安全的消息发送方法
        match content {
            "!status" => {
                let state = self.state_persistence.get_state().await;
                let status_msg = format!(
                    "机器人状态:\n• 已处理消息: {}\n• 错误次数: {}\n• 运行时间: {:?}",
                    state.processed_messages,
                    state.error_count,
                    state.uptime_start.map(|t| chrono::Utc::now() - t)
                );
                
                if let Err(e) = self.safe_send_message(&ctx, &message.channel_id, &status_msg).await {
                    self.handle_operation_error("发送状态消息", &e).await;
                }
            }
            "!ping" => {
                if let Err(e) = self.safe_send_message(&ctx, &message.channel_id, "Pong!").await {
                    self.handle_operation_error("发送 ping 回复", &e).await;
                }
            }
            "!test_error" => {
                // 故意触发错误用于测试
                if let Err(e) = ctx.api.get_guild(&ctx.token, "invalid_guild_id").await {
                    self.handle_operation_error("测试错误", &e).await;
                    
                    if let Err(e) = self.safe_send_message(
                        &ctx, 
                        &message.channel_id, 
                        "已触发测试错误，请查看日志"
                    ).await {
                        self.handle_operation_error("发送错误测试回复", &e).await;
                    }
                }
            }
            _ => {}
        }
    }
    
    async fn direct_message_create(&self, ctx: Context, dm: DirectMessage) {
        self.health_checker.update_message_activity();
        
        // 私信也使用相同的错误恢复机制
        if let Some(content) = &dm.content {
            if content.trim() == "!health" {
                let health_msg = "私信功能正常，错误恢复机制运行中";
                
                if let (Some(guild_id), Err(e)) = (&dm.guild_id, self.safe_send_message(
                    &ctx, 
                    &dm.channel_id, 
                    health_msg
                ).await) {
                    self.handle_operation_error("发送私信健康检查回复", &e).await;
                }
            }
        }
    }
    
    async fn group_message_create(&self, ctx: Context, group_msg: GroupMessage) {
        self.health_checker.update_message_activity();
        
        // 群组消息错误处理
        if let Some(content) = &group_msg.content {
            if content.contains("error_test") {
                // 测试群组消息错误恢复
                if let Err(e) = group_msg.reply(&ctx.api, &ctx.token, "群组错误恢复测试").await {
                    self.handle_operation_error("群组消息回复", &e).await;
                }
            }
        }
    }
    
    async fn error(&self, error: botrs::BotError) {
        self.handle_operation_error("事件处理器", &error).await;
    }
}
```

## 监控和告警

### 错误监控

```rust
use std::sync::atomic::{AtomicU64, Ordering};

pub struct ErrorMonitor {
    error_count: AtomicU64,
    last_error_time: Arc<std::sync::RwLock<Option<Instant>>>,
    error_threshold: u64,
    time_window: Duration,
}

impl ErrorMonitor {
    pub fn new(error_threshold: u64, time_window: Duration) -> Self {
        Self {
            error_count: AtomicU64::new(0),
            last_error_time: Arc::new(std::sync::RwLock::new(None)),
            error_threshold,
            time_window,
        }
    }
    
    pub fn record_error(&self) -> bool {
        let now = Instant::now();
        let mut last_error = self.last_error_time.write().unwrap();
        
        // 检查是否在时间窗口内
        if let Some(last_time) = *last_error {
            if now.duration_since(last_time) > self.time_window {
                // 重置计数器
                self.error_count.store(0, Ordering::SeqCst);
            }
        }
        
        *last_error = Some(now);
        let current_count = self.error_count.fetch_add(1, Ordering::SeqCst) + 1;
        
        // 检查是否超过阈值
        current_count >= self.error_threshold
    }
    
    pub fn get_error_rate(&self) -> f64 {
        let count = self.error_count.load(Ordering::SeqCst);
        count as f64 / self.time_window.as_secs() as f64
    }
}
```

## 完整示例程序

```rust
use botrs::{Client, Intents, Token};
use tokio::signal;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,error_recovery=info")
        .init();
    
    info!("启动错误恢复示例机器人");
    
    // 加载配置
    let token = Token::from_env()?;
    token.validate()?;
    
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_group_and_c2c_event();
    
    // 创建错误恢复处理器
    let handler = ErrorRecoveryHandler::new();
    
    // 创建恢复性客户端
    let client = Client::new(token, intents, handler, false)?;
    let mut resilient_client = ResilienceBotClient::new(client, None);
    
    // 设置优雅关闭
    let shutdown_signal = async {
        signal::ctrl_c().await.expect("安装 Ctrl+C 处理器失败");
        info!("收到关闭信号");
    };
    
    // 启动机器人与关闭信号竞争
    tokio::select! {
        result = resilient_client.start_with_recovery() => {
            match result {
                Ok(_) => info!("机器人正常停止"),
                Err(e) => error!("机器人启动失败: {}", e),
            }
        }
        _ = shutdown_signal => {
            info!("正在优雅关闭机器人");
            resilient_client.stop();
        }
    }
    
    info!("错误恢复示例机器人已停止");
    Ok(())
}
```

## 最佳实践

1. **分层错误处理**: 在不同层级实现相应的错误处理策略
2. **智能重试**: 根据错误类型选择合适的重试策略
3. **状态持久化**: 保存重要状态以便恢复后继续工作
4. **监控告警**: 实时监控错误率和系统健康状态
5. **优雅降级**: 在部分功能失效时保持核心功能可用

通过实现完善的错误恢复机制，您的机器人将能够在各种异常情况下保持稳定运行，提供可靠的服务质量。

## 另请参阅

- [错误处理指南](/zh/guide/error-handling.md) - 错误处理系统详细说明
- [API 集成示例](/zh/examples/api-integration.md) - API 调用错误处理
- [事件处理示例](/zh/examples/event-handling.md) - 事件系统错误处理
- [`BotError` API 参考](/zh/api/error-types.md) - 错误类型详细文档
# 错误处理

在开发 QQ 机器人时，正确的错误处理至关重要。BotRS 提供了全面的错误类型和处理机制，帮助你构建稳定可靠的机器人应用。

## 错误类型概述

BotRS 使用 `BotError` 枚举来表示所有可能的错误情况：

```rust
use botrs::{BotError, Result};

pub enum BotError {
    Authentication(String),
    Network(String),
    Gateway(String),
    Api(String),
    Serialization(String),
    InvalidInput(String),
    RateLimited(String),
    InternalError(String),
}
```

### 错误类型详解

#### 认证错误 (Authentication)
```rust
// 无效的应用 ID 或密钥
let token = Token::new("invalid_app_id", "invalid_secret");
match token.validate() {
    Err(BotError::Authentication(msg)) => {
        eprintln!("认证失败: {}", msg);
    }
    Ok(_) => println!("认证成功"),
}
```

#### 网络错误 (Network)
```rust
// 网络连接问题
async fn handle_network_error(ctx: Context, message: Message) {
    match message.reply(&ctx.api, &ctx.token, "回复内容").await {
        Err(BotError::Network(msg)) => {
            eprintln!("网络错误: {}", msg);
            // 可以尝试重新发送或记录错误
        }
        Ok(_) => println!("消息发送成功"),
    }
}
```

#### 网关错误 (Gateway)
```rust
// WebSocket 连接问题
async fn start_bot_with_retry(mut client: Client<MyHandler>) -> Result<()> {
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;

    loop {
        match client.start().await {
            Err(BotError::Gateway(msg)) => {
                retry_count += 1;
                eprintln!("网关错误 (尝试 {}/{}): {}", retry_count, MAX_RETRIES, msg);
                
                if retry_count >= MAX_RETRIES {
                    return Err(BotError::Gateway(format!("超过最大重试次数: {}", msg)));
                }
                
                // 等待后重试
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            other => return other,
        }
    }
}
```

#### API 错误 (Api)
```rust
// API 调用失败
async fn safe_api_call(ctx: &Context, guild_id: &str) -> Result<()> {
    match ctx.get_guild(guild_id).await {
        Err(BotError::Api(msg)) => {
            if msg.contains("403") {
                eprintln!("权限不足: {}", msg);
            } else if msg.contains("404") {
                eprintln!("频道不存在: {}", msg);
            } else {
                eprintln!("API 错误: {}", msg);
            }
        }
        Ok(guild) => {
            println!("获取频道信息成功: {}", guild.name);
        }
    }
    Ok(())
}
```

#### 速率限制错误 (RateLimited)
```rust
// 处理速率限制
async fn send_with_rate_limit_handling(
    ctx: &Context,
    channel_id: &str,
    content: &str,
) -> Result<()> {
    loop {
        match ctx.send_message(channel_id, &MessageParams::new_text(content)).await {
            Err(BotError::RateLimited(msg)) => {
                eprintln!("触发速率限制: {}", msg);
                
                // 解析重试时间（简化版本）
                let retry_after = parse_retry_after(&msg).unwrap_or(5);
                
                println!("等待 {} 秒后重试...", retry_after);
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            result => return result.map(|_| ()),
        }
    }
}

fn parse_retry_after(error_msg: &str) -> Option<u64> {
    // 从错误消息中解析重试时间
    // 实际实现需要根据 API 返回的具体格式
    Some(5) // 默认 5 秒
}
```

## 错误处理策略

### 1. 优雅降级

```rust
async fn robust_message_handler(ctx: Context, message: Message) {
    if let Some(content) = &message.content {
        // 主要功能
        if let Err(e) = try_primary_action(&ctx, &message, content).await {
            eprintln!("主要功能失败: {}", e);
            
            // 降级到简单回复
            if let Err(e2) = try_fallback_action(&ctx, &message).await {
                eprintln!("降级功能也失败: {}", e2);
                // 记录严重错误但不崩溃
            }
        }
    }
}

async fn try_primary_action(
    ctx: &Context,
    message: &Message,
    content: &str,
) -> Result<()> {
    // 尝试发送富文本消息
    let embed = create_rich_embed(content)?;
    let params = MessageParams {
        embed: Some(embed),
        ..Default::default()
    };
    ctx.send_message(&message.channel_id, &params).await?;
    Ok(())
}

async fn try_fallback_action(ctx: &Context, message: &Message) -> Result<()> {
    // 降级到简单文本回复
    message.reply(&ctx.api, &ctx.token, "服务暂时不可用，请稍后再试").await?;
    Ok(())
}
```

### 2. 重试机制

```rust
use std::time::Duration;

async fn retry_with_backoff<F, T>(
    mut operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
{
    let mut delay = initial_delay;
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_retries {
                    return Err(e);
                }
                
                match &e {
                    BotError::Network(_) | BotError::Gateway(_) => {
                        println!("重试 {}/{} 在 {:?} 后", attempt + 1, max_retries, delay);
                        tokio::time::sleep(delay).await;
                        delay *= 2; // 指数退避
                    }
                    _ => return Err(e), // 不重试其他错误类型
                }
            }
        }
    }
    
    unreachable!()
}

// 使用示例
async fn send_important_message(ctx: &Context, channel_id: &str, content: &str) -> Result<()> {
    retry_with_backoff(
        || {
            let ctx = ctx.clone();
            let channel_id = channel_id.to_string();
            let content = content.to_string();
            Box::pin(async move {
                let params = MessageParams::new_text(&content);
                ctx.send_message(&channel_id, &params).await
            })
        },
        3, // 最多重试 3 次
        Duration::from_secs(1), // 初始延迟 1 秒
    ).await
}
```

### 3. 错误聚合和报告

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ErrorStats {
    pub count: u64,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub sample_message: String,
}

pub struct ErrorTracker {
    errors: Arc<Mutex<HashMap<String, ErrorStats>>>,
}

impl ErrorTracker {
    pub fn new() -> Self {
        Self {
            errors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn record_error(&self, error: &BotError) {
        let error_type = match error {
            BotError::Authentication(_) => "authentication",
            BotError::Network(_) => "network",
            BotError::Gateway(_) => "gateway",
            BotError::Api(_) => "api",
            BotError::RateLimited(_) => "rate_limited",
            _ => "other",
        };
        
        let mut errors = self.errors.lock().await;
        let stats = errors.entry(error_type.to_string()).or_insert(ErrorStats {
            count: 0,
            last_seen: chrono::Utc::now(),
            sample_message: error.to_string(),
        });
        
        stats.count += 1;
        stats.last_seen = chrono::Utc::now();
        stats.sample_message = error.to_string();
    }
    
    pub async fn get_error_summary(&self) -> Vec<(String, ErrorStats)> {
        let errors = self.errors.lock().await;
        errors.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    
    pub async fn should_alert(&self, error_type: &str, threshold: u64) -> bool {
        let errors = self.errors.lock().await;
        errors.get(error_type).map(|stats| stats.count >= threshold).unwrap_or(false)
    }
}

// 在事件处理器中使用
struct MyBot {
    error_tracker: ErrorTracker,
}

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn error(&self, error: BotError) {
        // 记录错误
        self.error_tracker.record_error(&error).await;
        
        // 检查是否需要告警
        if self.error_tracker.should_alert("network", 10).await {
            eprintln!("警告: 网络错误次数过多");
            // 发送告警通知
        }
        
        eprintln!("处理错误: {}", error);
    }
}
```

## 特定场景的错误处理

### 消息发送失败

```rust
async fn safe_send_message(
    ctx: &Context,
    channel_id: &str,
    content: &str,
) -> Result<Option<Message>> {
    let params = MessageParams::new_text(content);
    
    match ctx.send_message(channel_id, &params).await {
        Ok(message) => Ok(Some(message)),
        Err(BotError::RateLimited(msg)) => {
            eprintln!("触发速率限制: {}", msg);
            Ok(None) // 返回 None 而不是错误
        }
        Err(BotError::Api(msg)) if msg.contains("403") => {
            eprintln!("权限不足，无法发送消息");
            Ok(None)
        }
        Err(e) => Err(e), // 其他错误继续传播
    }
}
```

### 文件上传错误

```rust
async fn upload_file_safely(
    ctx: &Context,
    channel_id: &str,
    file_data: &[u8],
    filename: &str,
) -> Result<()> {
    // 检查文件大小
    const MAX_FILE_SIZE: usize = 25 * 1024 * 1024; // 25MB
    if file_data.len() > MAX_FILE_SIZE {
        return Err(BotError::InvalidInput(
            "文件大小超过限制 (25MB)".to_string()
        ));
    }
    
    // 检查文件类型
    let allowed_extensions = ["jpg", "jpeg", "png", "gif", "pdf", "txt"];
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    if !allowed_extensions.contains(&extension.to_lowercase().as_str()) {
        return Err(BotError::InvalidInput(
            format!("不支持的文件类型: {}", extension)
        ));
    }
    
    // 尝试上传
    match upload_file_to_api(ctx, file_data, filename).await {
        Ok(file_info) => {
            let params = MessageParams::new_text("文件上传成功")
                .with_file_image(&file_info);
            ctx.send_message(channel_id, &params).await?;
        }
        Err(BotError::Network(_)) => {
            // 网络错误时提供用户友好的消息
            let params = MessageParams::new_text("网络不稳定，文件上传失败，请稍后重试");
            ctx.send_message(channel_id, &params).await?;
        }
        Err(e) => return Err(e),
    }
    
    Ok(())
}

async fn upload_file_to_api(
    ctx: &Context,
    file_data: &[u8],
    filename: &str,
) -> Result<String> {
    // 模拟文件上传 API 调用
    // 实际实现会调用相应的 API
    todo!("实现文件上传 API 调用")
}
```

### 权限检查错误

```rust
async fn check_user_permission(
    ctx: &Context,
    channel_id: &str,
    user_id: &str,
    required_permission: &str,
) -> Result<bool> {
    match ctx.get_channel_user_permissions(channel_id, user_id).await {
        Ok(permissions) => {
            // 解析权限
            Ok(has_permission(&permissions.permissions, required_permission))
        }
        Err(BotError::Api(msg)) if msg.contains("403") => {
            eprintln!("机器人没有权限查看用户权限");
            Ok(false) // 保守处理，假设用户没有权限
        }
        Err(BotError::Api(msg)) if msg.contains("404") => {
            eprintln!("频道或用户不存在");
            Ok(false)
        }
        Err(e) => Err(e),
    }
}

fn has_permission(permissions: &str, required: &str) -> bool {
    // 简化的权限检查实现
    let perm_value: u64 = permissions.parse().unwrap_or(0);
    let required_bit = match required {
        "send_messages" => 1 << 11,
        "manage_messages" => 1 << 13,
        "kick_members" => 1 << 1,
        _ => 0,
    };
    (perm_value & required_bit) != 0
}
```

## 监控和日志记录

### 结构化日志记录

```rust
use tracing::{error, warn, info, debug};

async fn log_errors_properly(ctx: Context, message: Message) {
    match process_message(&ctx, &message).await {
        Ok(_) => {
            info!(
                message_id = %message.id,
                channel_id = %message.channel_id,
                "消息处理成功"
            );
        }
        Err(e) => {
            match &e {
                BotError::RateLimited(_) => {
                    warn!(
                        error = %e,
                        message_id = %message.id,
                        "触发速率限制"
                    );
                }
                BotError::Network(_) => {
                    error!(
                        error = %e,
                        message_id = %message.id,
                        "网络错误"
                    );
                }
                _ => {
                    error!(
                        error = %e,
                        message_id = %message.id,
                        error_type = ?std::mem::discriminant(&e),
                        "消息处理失败"
                    );
                }
            }
        }
    }
}

async fn process_message(ctx: &Context, message: &Message) -> Result<()> {
    // 消息处理逻辑
    Ok(())
}
```

### 健康检查

```rust
pub struct HealthChecker {
    last_successful_api_call: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    consecutive_failures: Arc<Mutex<u32>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            last_successful_api_call: Arc::new(Mutex::new(None)),
            consecutive_failures: Arc::new(Mutex::new(0)),
        }
    }
    
    pub async fn record_success(&self) {
        *self.last_successful_api_call.lock().await = Some(chrono::Utc::now());
        *self.consecutive_failures.lock().await = 0;
    }
    
    pub async fn record_failure(&self) {
        *self.consecutive_failures.lock().await += 1;
    }
    
    pub async fn is_healthy(&self) -> bool {
        let last_success = self.last_successful_api_call.lock().await;
        let failures = *self.consecutive_failures.lock().await;
        
        // 如果连续失败次数过多，认为不健康
        if failures > 5 {
            return false;
        }
        
        // 如果很久没有成功的 API 调用，认为不健康
        if let Some(last) = *last_success {
            let duration = chrono::Utc::now() - last;
            duration.num_minutes() < 5
        } else {
            false
        }
    }
    
    pub async fn get_status(&self) -> HealthStatus {
        HealthStatus {
            is_healthy: self.is_healthy().await,
            last_successful_call: *self.last_successful_api_call.lock().await,
            consecutive_failures: *self.consecutive_failures.lock().await,
        }
    }
}

#[derive(Debug)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub last_successful_call: Option<chrono::DateTime<chrono::Utc>>,
    pub consecutive_failures: u32,
}
```

## 最佳实践

### 1. 错误分类和处理策略

```rust
fn get_error_handling_strategy(error: &BotError) -> ErrorHandlingStrategy {
    match error {
        BotError::RateLimited(_) => ErrorHandlingStrategy::Retry,
        BotError::Network(_) => ErrorHandlingStrategy::Retry,
        BotError::Gateway(_) => ErrorHandlingStrategy::Reconnect,
        BotError::Authentication(_) => ErrorHandlingStrategy::Fatal,
        BotError::Api(msg) if msg.contains("403") => ErrorHandlingStrategy::Skip,
        BotError::Api(msg) if msg.contains("404") => ErrorHandlingStrategy::Skip,
        _ => ErrorHandlingStrategy::Log,
    }
}

enum ErrorHandlingStrategy {
    Retry,      // 重试操作
    Reconnect,  // 重新连接
    Skip,       // 跳过当前操作
    Fatal,      // 致命错误，停止程序
    Log,        // 记录错误但继续
}
```

### 2. 用户友好的错误消息

```rust
fn user_friendly_error_message(error: &BotError) -> String {
    match error {
        BotError::RateLimited(_) => "请求过于频繁，请稍后再试".to_string(),
        BotError::Network(_) => "网络连接不稳定，请检查网络设置".to_string(),
        BotError::Api(msg) if msg.contains("403") => "权限不足，无法执行此操作".to_string(),
        BotError::Api(msg) if msg.contains("404") => "找不到指定的资源".to_string(),
        BotError::InvalidInput(_) => "输入格式不正确，请检查后重试".to_string(),
        _ => "服务暂时不可用，请稍后再试".to_string(),
    }
}

async fn handle_user_command_with_friendly_errors(
    ctx: Context,
    message: Message,
    command: &str,
) {
    match execute_command(&ctx, &message, command).await {
        Ok(_) => {
            // 成功处理
        }
        Err(e) => {
            let friendly_msg = user_friendly_error_message(&e);
            if let Err(reply_error) = message.reply(&ctx.api, &ctx.token, &friendly_msg).await {
                eprintln!("无法发送错误消息: {}", reply_error);
            }
        }
    }
}

async fn execute_command(ctx: &Context, message: &Message, command: &str) -> Result<()> {
    // 命令执行逻辑
    Ok(())
}
```

### 3. 错误恢复机制

```rust
pub struct BotManager {
    client: Option<Client<MyHandler>>,
    token: Token,
    intents: Intents,
    handler: MyHandler,
    restart_count: u32,
}

impl BotManager {
    pub async fn run_with_auto_restart(&mut self) -> Result<()> {
        const MAX_RESTARTS: u32 = 5;
        
        loop {
            match self.start_bot().await {
                Ok(_) => {
                    info!("机器人正常退出");
                    break;
                }
                Err(e) => {
                    self.restart_count += 1;
                    error!("机器人错误 (重启 {}/{}): {}", self.restart_count, MAX_RESTARTS, e);
                    
                    if self.restart_count >= MAX_RESTARTS {
                        error!("超过最大重启次数，停止尝试");
                        return Err(e);
                    }
                    
                    // 等待后重启
                    let wait_time = std::cmp::min(300, 30 * self.restart_count); // 最多等待 5 分钟
                    warn!("等待 {} 秒后重启", wait_time);
                    tokio::time::sleep(Duration::from_secs(wait_time as u64)).await;
                }
            }
        }
        
        Ok(())
    }
    
    async fn start_bot(&mut self) -> Result<()> {
        let mut client = Client::new(
            self.token.clone(),
            self.intents,
            self.handler.clone(),
            false,
        )?;
        
        client.start().await
    }
}
```

## 总结

有效的错误处理是构建稳定 QQ 机器人的关键。BotRS 提供了完整的错误类型系统，配合适当的处理策略，可以确保机器人在各种异常情况下都能保持稳定运行。

关键原则：

1. **分类处理**: 根据错误类型采用不同的处理策略
2. **优雅降级**: 在主要功能失败时提供备选方案
3. **重试机制**: 对临时性错误实施智能重试
4. **用户友好**: 向用户提供清晰易懂的错误信息
5. **监控日志**: 记录和监控错误情况以便问题诊断
6. **自动恢复**: 实现错误后的自动恢复机制

通过遵循这些最佳实践，你的机器人将能够优雅地处理各种错误情况，为用户提供稳定可靠的服务。

## 下一步

- 查看 [API 客户端使用](./api-client.md) 了解如何正确使用 API
- 阅读 [WebSocket 网关](./gateway.md) 学习连接管理
- 探索 [示例代码](../../examples/error-recovery.md) 查看实际的错误处理实现
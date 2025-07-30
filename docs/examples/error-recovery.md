# Error Recovery Example

This example demonstrates advanced error recovery patterns and resilient bot design using BotRS.

## Overview

Building a robust QQ Guild bot requires comprehensive error handling and recovery mechanisms. This example shows how to implement retry logic, circuit breakers, graceful degradation, and automatic recovery strategies.

## Basic Error Recovery

### Retry with Exponential Backoff

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token, BotError};
use std::time::Duration;
use tokio::time::sleep;

struct ResilientBot {
    max_retries: u32,
    base_delay: Duration,
}

impl ResilientBot {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(1000),
        }
    }

    async fn retry_with_backoff<F, T>(&self, mut operation: F) -> Result<T, BotError>
    where
        F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, BotError>> + Send>>,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error);
                    
                    if attempt < self.max_retries {
                        let delay = self.base_delay * 2_u32.pow(attempt);
                        tracing::warn!(
                            "Operation failed (attempt {}/{}), retrying in {:?}",
                            attempt + 1,
                            self.max_retries + 1,
                            delay
                        );
                        sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}

#[async_trait::async_trait]
impl EventHandler for ResilientBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("Resilient bot ready: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content.trim() == "!test-retry" {
                let result = self.retry_with_backoff(|| {
                    let ctx = ctx.clone();
                    let message = message.clone();
                    Box::pin(async move {
                        message.reply(&ctx.api, &ctx.token, "This might fail but we'll retry!").await
                    })
                }).await;

                match result {
                    Ok(_) => tracing::info!("Message sent successfully after retries"),
                    Err(e) => tracing::error!("Failed to send message after all retries: {}", e),
                }
            }
        }
    }

    async fn error(&self, error: BotError) {
        tracing::error!("Bot error occurred: {}", error);
        
        // Implement error-specific recovery strategies
        match &error {
            BotError::Network(_) => {
                tracing::info!("Network error detected, implementing recovery strategy");
                // Network-specific recovery logic
            }
            BotError::RateLimited(_) => {
                tracing::info!("Rate limit hit, backing off");
                // Rate limit recovery logic
            }
            BotError::Gateway(_) => {
                tracing::info!("Gateway error, preparing for reconnection");
                // Gateway recovery logic
            }
            _ => {
                tracing::warn!("Unhandled error type: {}", error);
            }
        }
    }
}
```

## Circuit Breaker Pattern

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum CircuitState {
    Closed,
    Open(Instant),
    HalfOpen,
}

pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
    failure_count: Arc<Mutex<u32>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_threshold,
            recovery_timeout,
            failure_count: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn call<F, T>(&self, operation: F) -> Result<T, BotError>
    where
        F: std::future::Future<Output = Result<T, BotError>>,
    {
        // Check circuit state
        let mut state = self.state.lock().await;
        match *state {
            CircuitState::Open(opened_at) => {
                if Instant::now().duration_since(opened_at) > self.recovery_timeout {
                    *state = CircuitState::HalfOpen;
                    tracing::info!("Circuit breaker transitioning to half-open");
                } else {
                    return Err(BotError::InternalError("Circuit breaker is open".to_string()));
                }
            }
            CircuitState::Closed | CircuitState::HalfOpen => {}
        }
        drop(state);

        // Execute operation
        match operation.await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(error) => {
                self.on_failure().await;
                Err(error)
            }
        }
    }

    async fn on_success(&self) {
        let mut failure_count = self.failure_count.lock().await;
        *failure_count = 0;

        let mut state = self.state.lock().await;
        if matches!(*state, CircuitState::HalfOpen) {
            *state = CircuitState::Closed;
            tracing::info!("Circuit breaker closed after successful operation");
        }
    }

    async fn on_failure(&self) {
        let mut failure_count = self.failure_count.lock().await;
        *failure_count += 1;

        if *failure_count >= self.failure_threshold {
            let mut state = self.state.lock().await;
            *state = CircuitState::Open(Instant::now());
            tracing::warn!("Circuit breaker opened after {} failures", self.failure_threshold);
        }
    }
}
```

## Graceful Degradation

```rust
use std::collections::HashMap;

pub struct FeatureFlags {
    flags: HashMap<String, bool>,
}

impl FeatureFlags {
    pub fn new() -> Self {
        let mut flags = HashMap::new();
        flags.insert("rich_embeds".to_string(), true);
        flags.insert("file_uploads".to_string(), true);
        flags.insert("interactive_buttons".to_string(), true);
        flags.insert("external_api_calls".to_string(), true);
        
        Self { flags }
    }

    pub fn is_enabled(&self, feature: &str) -> bool {
        self.flags.get(feature).copied().unwrap_or(false)
    }

    pub fn disable_feature(&mut self, feature: &str) {
        tracing::warn!("Disabling feature: {}", feature);
        self.flags.insert(feature.to_string(), false);
    }

    pub fn enable_feature(&mut self, feature: &str) {
        tracing::info!("Enabling feature: {}", feature);
        self.flags.insert(feature.to_string(), true);
    }
}

struct DegradableBot {
    circuit_breaker: CircuitBreaker,
    feature_flags: Arc<Mutex<FeatureFlags>>,
}

impl DegradableBot {
    pub fn new() -> Self {
        Self {
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(60)),
            feature_flags: Arc::new(Mutex::new(FeatureFlags::new())),
        }
    }

    async fn send_message_with_fallback(&self, ctx: &Context, message: &Message, content: &str) -> Result<(), BotError> {
        let flags = self.feature_flags.lock().await;
        
        // Try rich embed first
        if flags.is_enabled("rich_embeds") {
            drop(flags);
            let result = self.circuit_breaker.call(
                self.send_rich_message(ctx, message, content)
            ).await;
            
            if result.is_ok() {
                return result;
            }
            
            // Disable rich embeds on failure
            let mut flags = self.feature_flags.lock().await;
            flags.disable_feature("rich_embeds");
        }

        // Fallback to simple text message
        tracing::info!("Falling back to simple text message");
        message.reply(&ctx.api, &ctx.token, content).await
    }

    async fn send_rich_message(&self, ctx: &Context, message: &Message, content: &str) -> Result<(), BotError> {
        use botrs::models::message::{Embed, MessageParams};
        
        let embed = Embed {
            title: Some("Response".to_string()),
            description: Some(content.to_string()),
            color: Some(0x3498db),
            ..Default::default()
        };

        let params = MessageParams {
            embed: Some(embed),
            ..Default::default()
        };

        ctx.send_message(&message.channel_id, &params).await?;
        Ok(())
    }
}
```

## Health Monitoring

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct HealthMetrics {
    pub total_messages: AtomicU64,
    pub successful_messages: AtomicU64,
    pub failed_messages: AtomicU64,
    pub last_error: Mutex<Option<(DateTime<Utc>, String)>>,
    pub uptime_start: DateTime<Utc>,
}

impl HealthMetrics {
    pub fn new() -> Self {
        Self {
            total_messages: AtomicU64::new(0),
            successful_messages: AtomicU64::new(0),
            failed_messages: AtomicU64::new(0),
            last_error: Mutex::new(None),
            uptime_start: Utc::now(),
        }
    }

    pub fn record_success(&self) {
        self.total_messages.fetch_add(1, Ordering::Relaxed);
        self.successful_messages.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn record_failure(&self, error: &str) {
        self.total_messages.fetch_add(1, Ordering::Relaxed);
        self.failed_messages.fetch_add(1, Ordering::Relaxed);
        
        let mut last_error = self.last_error.lock().await;
        *last_error = Some((Utc::now(), error.to_string()));
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.total_messages.load(Ordering::Relaxed);
        if total == 0 {
            return 1.0;
        }
        
        let successful = self.successful_messages.load(Ordering::Relaxed);
        successful as f64 / total as f64
    }

    pub fn is_healthy(&self) -> bool {
        let success_rate = self.success_rate();
        let total_messages = self.total_messages.load(Ordering::Relaxed);
        
        // Consider healthy if success rate > 95% and we've processed some messages
        success_rate > 0.95 || total_messages < 10
    }
}

struct MonitoredBot {
    health_metrics: Arc<HealthMetrics>,
    circuit_breaker: CircuitBreaker,
}

impl MonitoredBot {
    pub fn new() -> Self {
        Self {
            health_metrics: Arc::new(HealthMetrics::new()),
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(30)),
        }
    }

    async fn handle_message_safely(&self, ctx: &Context, message: &Message) {
        let result = self.circuit_breaker.call(
            self.process_message(ctx, message)
        ).await;

        match result {
            Ok(_) => {
                self.health_metrics.record_success();
                tracing::debug!("Message processed successfully");
            }
            Err(e) => {
                self.health_metrics.record_failure(&e.to_string()).await;
                tracing::error!("Message processing failed: {}", e);
            }
        }
    }

    async fn process_message(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        if let Some(content) = &message.content {
            match content.trim() {
                "!health" => {
                    self.send_health_report(ctx, message).await?;
                }
                "!status" => {
                    self.send_status_report(ctx, message).await?;
                }
                _ => {
                    // Process other commands
                }
            }
        }
        Ok(())
    }

    async fn send_health_report(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let metrics = &self.health_metrics;
        let total = metrics.total_messages.load(Ordering::Relaxed);
        let successful = metrics.successful_messages.load(Ordering::Relaxed);
        let failed = metrics.failed_messages.load(Ordering::Relaxed);
        let success_rate = metrics.success_rate();
        let uptime = Utc::now() - metrics.uptime_start;
        
        let health_status = if metrics.is_healthy() { "🟢 Healthy" } else { "🔴 Unhealthy" };
        
        let last_error = {
            let last_error_guard = metrics.last_error.lock().await;
            match &*last_error_guard {
                Some((timestamp, error)) => format!("**Last Error:** {} ({})", error, timestamp.format("%Y-%m-%d %H:%M:%S UTC")),
                None => "**Last Error:** None".to_string(),
            }
        };

        let report = format!(
            "🏥 **Bot Health Report**\n\n\
             **Status:** {}\n\
             **Uptime:** {} days, {} hours, {} minutes\n\
             **Messages Processed:** {}\n\
             **Success Rate:** {:.2}%\n\
             **Successful:** {}\n\
             **Failed:** {}\n\n\
             {}",
            health_status,
            uptime.num_days(),
            uptime.num_hours() % 24,
            uptime.num_minutes() % 60,
            total,
            success_rate * 100.0,
            successful,
            failed,
            last_error
        );

        message.reply(&ctx.api, &ctx.token, &report).await?;
        Ok(())
    }

    async fn send_status_report(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let status = if self.health_metrics.is_healthy() {
            "✅ All systems operational"
        } else {
            "⚠️ Some issues detected - check !health for details"
        };

        message.reply(&ctx.api, &ctx.token, status).await?;
        Ok(())
    }
}
```

## Automatic Recovery Strategies

```rust
use tokio::sync::oneshot;

pub struct RecoveryManager {
    recovery_channel: Option<oneshot::Sender<()>>,
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self {
            recovery_channel: None,
        }
    }

    pub async fn start_recovery_monitoring(&mut self) {
        let (tx, mut rx) = oneshot::channel();
        self.recovery_channel = Some(tx);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut rx => {
                        tracing::info!("Recovery monitoring stopped");
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(30)) => {
                        // Perform periodic health checks
                        Self::perform_health_check().await;
                    }
                }
            }
        });
    }

    async fn perform_health_check() {
        tracing::debug!("Performing automated health check");
        
        // Check various system components
        let network_ok = Self::check_network_connectivity().await;
        let memory_ok = Self::check_memory_usage().await;
        let api_ok = Self::check_api_connectivity().await;

        if !network_ok || !memory_ok || !api_ok {
            tracing::warn!("Health check failed, initiating recovery procedures");
            Self::initiate_recovery().await;
        }
    }

    async fn check_network_connectivity() -> bool {
        // Simple network check
        match reqwest::get("https://api.sgroup.qq.com").await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn check_memory_usage() -> bool {
        // Basic memory check (simplified)
        true // In a real implementation, check actual memory usage
    }

    async fn check_api_connectivity() -> bool {
        // Check if we can make basic API calls
        true // In a real implementation, test API endpoints
    }

    async fn initiate_recovery() {
        tracing::info!("Starting automatic recovery procedures");
        
        // Clear caches
        Self::clear_caches().await;
        
        // Reset connections
        Self::reset_connections().await;
        
        // Reduce load
        Self::reduce_load().await;
        
        tracing::info!("Recovery procedures completed");
    }

    async fn clear_caches() {
        tracing::debug!("Clearing internal caches");
        // Implementation depends on your caching strategy
    }

    async fn reset_connections() {
        tracing::debug!("Resetting network connections");
        // Close and reopen connections if needed
    }

    async fn reduce_load() {
        tracing::debug!("Reducing system load");
        // Temporarily disable non-essential features
    }
}
```

## Complete Recovery Bot Example

```rust
struct AdvancedRecoveryBot {
    health_metrics: Arc<HealthMetrics>,
    circuit_breaker: CircuitBreaker,
    feature_flags: Arc<Mutex<FeatureFlags>>,
    recovery_manager: Arc<Mutex<RecoveryManager>>,
}

impl AdvancedRecoveryBot {
    pub fn new() -> Self {
        Self {
            health_metrics: Arc::new(HealthMetrics::new()),
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(60)),
            feature_flags: Arc::new(Mutex::new(FeatureFlags::new())),
            recovery_manager: Arc::new(Mutex::new(RecoveryManager::new())),
        }
    }

    pub async fn start_monitoring(&self) {
        let mut recovery_manager = self.recovery_manager.lock().await;
        recovery_manager.start_recovery_monitoring().await;
    }
}

#[async_trait::async_trait]
impl EventHandler for AdvancedRecoveryBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        tracing::info!("Advanced recovery bot ready: {}", ready.user.username);
        
        // Start health monitoring
        self.start_monitoring().await;
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // Wrap all message processing in error recovery
        let result = self.circuit_breaker.call(async {
            self.process_message_with_recovery(&ctx, &message).await
        }).await;

        // Record metrics
        match result {
            Ok(_) => self.health_metrics.record_success(),
            Err(e) => {
                self.health_metrics.record_failure(&e.to_string()).await;
                tracing::error!("Message processing failed: {}", e);
            }
        }
    }

    async fn error(&self, error: BotError) {
        tracing::error!("Bot error: {}", error);
        
        // Record the error
        self.health_metrics.record_failure(&error.to_string()).await;
        
        // Implement specific recovery strategies based on error type
        match error {
            BotError::RateLimited(_) => {
                tracing::info!("Rate limited - enabling backoff mode");
                let mut flags = self.feature_flags.lock().await;
                flags.disable_feature("external_api_calls");
                
                // Re-enable after delay
                let flags_clone = Arc::clone(&self.feature_flags);
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(300)).await; // 5 minutes
                    let mut flags = flags_clone.lock().await;
                    flags.enable_feature("external_api_calls");
                });
            }
            BotError::Network(_) => {
                tracing::info!("Network error - entering degraded mode");
                let mut flags = self.feature_flags.lock().await;
                flags.disable_feature("file_uploads");
                flags.disable_feature("rich_embeds");
            }
            BotError::Gateway(_) => {
                tracing::info!("Gateway error - preparing for reconnection");
                // Gateway errors are usually handled by the client automatically
            }
            _ => {
                tracing::warn!("Unhandled error type, using default recovery");
            }
        }
    }
}

impl AdvancedRecoveryBot {
    async fn process_message_with_recovery(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        if let Some(content) = &message.content {
            match content.trim() {
                "!health" => self.send_comprehensive_health_report(ctx, message).await?,
                "!recover" => self.manual_recovery(ctx, message).await?,
                "!features" => self.show_feature_status(ctx, message).await?,
                _ => {
                    // Process other commands with fallback
                    self.send_message_with_all_fallbacks(ctx, message, "Command processed with recovery protection").await?;
                }
            }
        }
        Ok(())
    }

    async fn send_comprehensive_health_report(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let flags = self.feature_flags.lock().await;
        let features_status = format!(
            "**Features:**\n\
             • Rich Embeds: {}\n\
             • File Uploads: {}\n\
             • Interactive Buttons: {}\n\
             • External API Calls: {}",
            if flags.is_enabled("rich_embeds") { "✅" } else { "❌" },
            if flags.is_enabled("file_uploads") { "✅" } else { "❌" },
            if flags.is_enabled("interactive_buttons") { "✅" } else { "❌" },
            if flags.is_enabled("external_api_calls") { "✅" } else { "❌" }
        );
        drop(flags);

        // Get basic health metrics
        let success_rate = self.health_metrics.success_rate() * 100.0;
        let total = self.health_metrics.total_messages.load(Ordering::Relaxed);
        let health_emoji = if self.health_metrics.is_healthy() { "🟢" } else { "🔴" };

        let comprehensive_report = format!(
            "{} **Comprehensive Health Report**\n\n\
             **Overall Status:** {}\n\
             **Success Rate:** {:.1}%\n\
             **Total Messages:** {}\n\n\
             {}",
            health_emoji,
            if self.health_metrics.is_healthy() { "Healthy" } else { "Needs Attention" },
            success_rate,
            total,
            features_status
        );

        self.send_message_with_all_fallbacks(ctx, message, &comprehensive_report).await
    }

    async fn manual_recovery(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        tracing::info!("Manual recovery initiated by user");
        
        // Reset all features
        let mut flags = self.feature_flags.lock().await;
        flags.enable_feature("rich_embeds");
        flags.enable_feature("file_uploads");
        flags.enable_feature("interactive_buttons");
        flags.enable_feature("external_api_calls");
        drop(flags);

        message.reply(&ctx.api, &ctx.token, "🔄 Manual recovery completed! All features restored.").await
    }

    async fn show_feature_status(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let flags = self.feature_flags.lock().await;
        let status = format!(
            "🎛️ **Feature Status**\n\n\
             Rich Embeds: {}\n\
             File Uploads: {}\n\
             Interactive Buttons: {}\n\
             External API Calls: {}",
            if flags.is_enabled("rich_embeds") { "✅ Enabled" } else { "❌ Disabled" },
            if flags.is_enabled("file_uploads") { "✅ Enabled" } else { "❌ Disabled" },
            if flags.is_enabled("interactive_buttons") { "✅ Enabled" } else { "❌ Disabled" },
            if flags.is_enabled("external_api_calls") { "✅ Enabled" } else { "❌ Disabled" }
        );
        drop(flags);

        message.reply(&ctx.api, &ctx.token, &status).await
    }

    async fn send_message_with_all_fallbacks(&self, ctx: &Context, message: &Message, content: &str) -> Result<(), BotError> {
        // Try with circuit breaker protection
        let result = self.circuit_breaker.call(async {
            message.reply(&ctx.api, &ctx.token, content).await
        }).await;

        if result.is_ok() {
            return result;
        }

        // Ultimate fallback - simple reply without any enhancements
        tracing::warn!("All enhanced messaging failed, using basic fallback");
        tokio::time::sleep(Duration::from_millis(100)).await; // Brief delay
        message.reply(&ctx.api, &ctx.token, "⚠️ Message sent in recovery mode").await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,error_recovery=info")
        .init();

    tracing::info!("Starting error recovery bot...");

    // Get credentials
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("QQ_BOT_APP_ID environment variable required");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("QQ_BOT_SECRET environment variable required");

    // Create token
    let token = Token::new(app_id, secret);
    token.validate()?;

    // Set up intents
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds();

    // Create resilient bot
    let handler = AdvancedRecoveryBot::new();
    let mut client = Client::new(token, intents, handler, false)?;

    tracing::info!("Error recovery bot starting...");
    client.start().await?;

    Ok(())
}
```

## Usage Examples

### Testing Recovery Features

```
# Check bot health
!health

# View feature status
!features

# Trigger manual recovery
!recover

# Get basic status
!status
```

### Monitoring Commands

```
# Health metrics and uptime
!health

# Circuit breaker status
!status

# Feature availability
!features
```

## Best Practices

1. **Layered Recovery**: Implement multiple fallback levels
2. **Monitoring**: Continuously monitor system health
3. **Graceful Degradation**: Disable non-essential features under stress
4. **Circuit Breakers**: Prevent cascade failures
5. **Automatic Recovery**: Self-healing mechanisms
6. **Observability**: Comprehensive logging and metrics
7. **Testing**: Regularly test recovery scenarios

## Recovery Strategies

- **Immediate**: Retry with exponential backoff
- **Short-term**: Circuit breakers and feature flags
- **Medium-term**: Graceful degradation and load reduction
- **Long-term**: Health monitoring and automatic recovery

## See Also

- [Command Handler](./command-handler.md) - Robust command processing
- [Event Handling](./event-handling.md) - Comprehensive event management
- [Getting Started](./getting-started.md) - Basic bot setup
- [Interactive Messages](./interactive-messages.md) - User interaction patterns
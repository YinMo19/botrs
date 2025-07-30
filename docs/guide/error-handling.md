# Error Handling

Robust error handling is crucial for building reliable QQ Guild bots. BotRS provides a comprehensive error system that helps you gracefully handle failures, implement retry logic, and maintain bot stability in production environments.

## Error Types Overview

BotRS uses a centralized error system built around the `BotError` enum, which covers all possible failure scenarios:

```rust
use botrs::{BotError, Result};

// All BotRS functions return Result<T, BotError>
async fn send_message() -> Result<Message> {
    // Function implementation
}
```

### Core Error Categories

**Network Errors**
- Connection failures
- Timeout errors
- Rate limiting
- HTTP status errors

**API Errors**
- Authentication failures
- Invalid parameters
- Resource not found
- Permission denied

**Gateway Errors**
- WebSocket connection issues
- Invalid session
- Heartbeat failures
- Reconnection problems

**Parsing Errors**
- JSON deserialization failures
- Invalid message formats
- Type conversion errors

## Basic Error Handling

### Simple Error Propagation

The most basic approach uses the `?` operator to propagate errors:

```rust
use botrs::{Context, EventHandler, Message, Result};

impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Err(e) = self.handle_message(&ctx, &msg).await {
            tracing::error!("Failed to handle message: {}", e);
        }
    }
}

impl MyBot {
    async fn handle_message(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let response = self.generate_response(&msg.content).await?;
        
        let params = MessageParams::new_text(&response);
        ctx.api.post_message_with_params(
            &ctx.token, 
            &msg.channel_id, 
            params
        ).await?;
        
        Ok(())
    }
}
```

### Pattern Matching on Errors

Handle specific error types with pattern matching:

```rust
use botrs::{BotError, Context, Message};

async fn send_with_retry(
    ctx: &Context, 
    channel_id: &str, 
    content: &str
) -> Result<Message> {
    let params = MessageParams::new_text(content);
    
    match ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
        Ok(message) => Ok(message),
        Err(BotError::Http(status)) if status.as_u16() == 429 => {
            // Rate limited - wait and retry
            tokio::time::sleep(Duration::from_secs(1)).await;
            let params = MessageParams::new_text(content);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
        }
        Err(BotError::Http(status)) if status.as_u16() == 403 => {
            tracing::warn!("Permission denied for channel {}", channel_id);
            Err(BotError::Http(status))
        }
        Err(e) => {
            tracing::error!("Unexpected error: {}", e);
            Err(e)
        }
    }
}
```

## Advanced Error Handling Patterns

### Retry Logic with Exponential Backoff

Implement sophisticated retry mechanisms for transient failures:

```rust
use tokio::time::{sleep, Duration};
use botrs::{BotError, Result};

struct RetryConfig {
    max_attempts: usize,
    base_delay: Duration,
    max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
        }
    }
}

async fn retry_with_backoff<F, T, Fut>(
    operation: F,
    config: RetryConfig,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_error = None;
    
    for attempt in 0..config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                
                if attempt < config.max_attempts - 1 {
                    let delay = calculate_backoff_delay(attempt, &config);
                    tracing::warn!(
                        "Operation failed, retrying in {:?} (attempt {}/{})",
                        delay,
                        attempt + 1,
                        config.max_attempts
                    );
                    sleep(delay).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}

fn calculate_backoff_delay(attempt: usize, config: &RetryConfig) -> Duration {
    let delay = config.base_delay * 2_u32.pow(attempt as u32);
    std::cmp::min(delay, config.max_delay)
}

// Usage example
async fn send_message_with_retry(
    ctx: &Context,
    channel_id: &str,
    content: &str,
) -> Result<Message> {
    retry_with_backoff(
        || async {
            let params = MessageParams::new_text(content);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
        },
        RetryConfig::default(),
    ).await
}
```

### Circuit Breaker Pattern

Prevent cascading failures with circuit breaker implementation:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::Instant;

#[derive(Clone)]
pub struct CircuitBreaker {
    failure_count: Arc<AtomicUsize>,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
    failure_threshold: usize,
    reset_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            failure_count: Arc::new(AtomicUsize::new(0)),
            last_failure_time: Arc::new(Mutex::new(None)),
            failure_threshold,
            reset_timeout,
        }
    }
    
    pub async fn call<F, T, Fut>(&self, operation: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Check if circuit is open
        if self.is_open().await {
            return Err(BotError::CircuitBreakerOpen);
        }
        
        match operation().await {
            Ok(result) => {
                self.reset().await;
                Ok(result)
            }
            Err(e) => {
                self.record_failure().await;
                Err(e)
            }
        }
    }
    
    async fn is_open(&self) -> bool {
        let failure_count = self.failure_count.load(Ordering::Relaxed);
        if failure_count < self.failure_threshold {
            return false;
        }
        
        let last_failure = self.last_failure_time.lock().await;
        if let Some(last_time) = *last_failure {
            Instant::now().duration_since(last_time) < self.reset_timeout
        } else {
            false
        }
    }
    
    async fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        *self.last_failure_time.lock().await = Some(Instant::now());
    }
    
    async fn reset(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
        *self.last_failure_time.lock().await = None;
    }
}
```

### Error Recovery Strategies

Implement graceful degradation and recovery:

```rust
use botrs::{Context, EventHandler, Message, BotError};

pub struct ResilientBot {
    circuit_breaker: CircuitBreaker,
    fallback_responses: Vec<String>,
}

impl ResilientBot {
    pub fn new() -> Self {
        Self {
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(30)),
            fallback_responses: vec![
                "Sorry, I'm experiencing technical difficulties.".to_string(),
                "I'm temporarily unavailable. Please try again later.".to_string(),
            ],
        }
    }
    
    async fn send_message_with_fallback(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<()> {
        // Try primary operation
        match self.circuit_breaker.call(|| async {
            let params = MessageParams::new_text(content);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
        }).await {
            Ok(_) => {
                tracing::info!("Message sent successfully");
                Ok(())
            }
            Err(BotError::CircuitBreakerOpen) => {
                // Circuit breaker is open, use fallback
                self.send_fallback_message(ctx, channel_id).await
            }
            Err(BotError::Http(status)) if status.as_u16() >= 500 => {
                // Server error, use fallback
                self.send_fallback_message(ctx, channel_id).await
            }
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
                Err(e)
            }
        }
    }
    
    async fn send_fallback_message(
        &self,
        ctx: &Context,
        channel_id: &str,
    ) -> Result<()> {
        let fallback = self.fallback_responses
            .choose(&mut rand::thread_rng())
            .unwrap();
            
        let params = MessageParams::new_text(fallback);
        match ctx.api.post_message_with_params(&ctx.token, channel_id, params).await {
            Ok(_) => {
                tracing::info!("Fallback message sent");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Fallback message also failed: {}", e);
                Err(e)
            }
        }
    }
}

impl EventHandler for ResilientBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content.starts_with("!echo ") {
                let response = &content[6..];
                if let Err(e) = self.send_message_with_fallback(
                    &ctx, 
                    &msg.channel_id, 
                    response
                ).await {
                    tracing::error!("All message sending attempts failed: {}", e);
                }
            }
        }
    }
}
```

## Error Monitoring and Observability

### Structured Logging

Use structured logging to capture error context:

```rust
use tracing::{error, warn, info, instrument};
use serde_json::json;

impl EventHandler for MyBot {
    #[instrument(skip(self, ctx, msg), fields(
        channel_id = %msg.channel_id,
        message_id = %msg.id,
        user_id = msg.author.as_ref().map(|a| a.id.as_str())
    ))]
    async fn message_create(&self, ctx: Context, msg: Message) {
        match self.handle_message(&ctx, &msg).await {
            Ok(_) => info!("Message processed successfully"),
            Err(e) => error!(
                error = %e,
                error_type = ?std::mem::discriminant(&e),
                "Failed to process message"
            ),
        }
    }
    
    async fn handle_message(&self, ctx: &Context, msg: &Message) -> Result<()> {
        // Processing logic with error context
        let response = self.generate_response(msg).await
            .map_err(|e| {
                error!(
                    message_content = msg.content.as_deref().unwrap_or(""),
                    error = %e,
                    "Failed to generate response"
                );
                e
            })?;
            
        self.send_response(ctx, &msg.channel_id, &response).await
            .map_err(|e| {
                error!(
                    channel_id = %msg.channel_id,
                    response_content = %response,
                    error = %e,
                    "Failed to send response"
                );
                e
            })?;
            
        Ok(())
    }
}
```

### Error Metrics Collection

Track error rates and patterns:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct ErrorMetrics {
    error_counts: Arc<Mutex<HashMap<String, usize>>>,
    total_requests: Arc<AtomicUsize>,
}

impl ErrorMetrics {
    pub async fn record_error(&self, error_type: &str) {
        let mut counts = self.error_counts.lock().await;
        *counts.entry(error_type.to_string()).or_insert(0) += 1;
    }
    
    pub fn record_request(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    pub async fn get_error_rate(&self) -> f64 {
        let counts = self.error_counts.lock().await;
        let total_errors: usize = counts.values().sum();
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        
        if total_requests == 0 {
            0.0
        } else {
            total_errors as f64 / total_requests as f64
        }
    }
}

// Usage in event handler
impl EventHandler for MetricsBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        self.metrics.record_request();
        
        if let Err(e) = self.handle_message(&ctx, &msg).await {
            let error_type = match &e {
                BotError::Http(_) => "http_error",
                BotError::Gateway(_) => "gateway_error",
                BotError::Serde(_) => "parsing_error",
                _ => "unknown_error",
            };
            
            self.metrics.record_error(error_type).await;
            tracing::error!("Message handling failed: {}", e);
        }
    }
}
```

## Production Error Handling

### Comprehensive Error Strategy

```rust
use botrs::{Client, EventHandler, Context, Message, BotError, Result};
use tracing::{error, warn, info};

pub struct ProductionBot {
    retry_config: RetryConfig,
    circuit_breaker: CircuitBreaker,
    metrics: ErrorMetrics,
    max_message_length: usize,
}

impl ProductionBot {
    pub fn new() -> Self {
        Self {
            retry_config: RetryConfig {
                max_attempts: 3,
                base_delay: Duration::from_millis(200),
                max_delay: Duration::from_secs(5),
            },
            circuit_breaker: CircuitBreaker::new(10, Duration::from_secs(60)),
            metrics: ErrorMetrics::default(),
            max_message_length: 2000,
        }
    }
    
    async fn safe_send_message(
        &self,
        ctx: &Context,
        channel_id: &str,
        content: &str,
    ) -> Result<()> {
        // Validate input
        if content.is_empty() {
            warn!("Attempted to send empty message");
            return Ok(());
        }
        
        let truncated_content = if content.len() > self.max_message_length {
            warn!("Message too long, truncating");
            &content[..self.max_message_length - 3].to_string() + "..."
        } else {
            content.to_string()
        };
        
        // Attempt to send with retry and circuit breaker
        self.circuit_breaker.call(|| async {
            retry_with_backoff(
                || async {
                    let params = MessageParams::new_text(&truncated_content);
                    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
                },
                self.retry_config.clone(),
            ).await
        }).await?;
        
        Ok(())
    }
}

impl EventHandler for ProductionBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot ready: {} guilds", ready.guilds.len());
    }
    
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Record metrics
        self.metrics.record_request();
        
        // Process message with comprehensive error handling
        let result = self.process_message_safely(&ctx, &msg).await;
        
        if let Err(e) = result {
            self.handle_processing_error(&e, &msg).await;
        }
    }
    
    async fn process_message_safely(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<()> {
        // Skip bot messages
        if msg.author.as_ref().map_or(false, |a| a.bot.unwrap_or(false)) {
            return Ok(());
        }
        
        // Handle commands with error recovery
        if let Some(content) = &msg.content {
            if content.starts_with("!") {
                match self.handle_command(ctx, msg, content).await {
                    Ok(_) => info!("Command processed successfully"),
                    Err(e) => {
                        self.metrics.record_error("command_error").await;
                        
                        // Send user-friendly error message
                        let error_msg = "Sorry, I couldn't process that command. Please try again later.";
                        if let Err(send_err) = self.safe_send_message(
                            ctx, 
                            &msg.channel_id, 
                            error_msg
                        ).await {
                            error!("Failed to send error message: {}", send_err);
                        }
                        
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_processing_error(&self, error: &BotError, msg: &Message) {
        let error_type = match error {
            BotError::Http(status) => {
                if status.as_u16() >= 500 {
                    "server_error"
                } else if status.as_u16() >= 400 {
                    "client_error"
                } else {
                    "http_error"
                }
            }
            BotError::Gateway(_) => "gateway_error",
            BotError::Serde(_) => "parsing_error",
            _ => "unknown_error",
        };
        
        self.metrics.record_error(error_type).await;
        
        error!(
            error = %error,
            error_type = error_type,
            channel_id = %msg.channel_id,
            message_id = %msg.id,
            "Failed to process message"
        );
    }
}
```

## Best Practices

### Error Handling Guidelines

1. **Fail Fast, Recover Gracefully**: Detect errors early but provide fallback behavior
2. **Log with Context**: Include relevant information for debugging
3. **User-Friendly Messages**: Don't expose technical errors to end users
4. **Monitor and Alert**: Track error rates and patterns
5. **Test Error Scenarios**: Include error cases in your test suite

### Common Pitfalls to Avoid

```rust
// ❌ Don't ignore errors silently
async fn bad_error_handling(&self, ctx: &Context, msg: &Message) {
    let _ = self.send_response(ctx, msg).await; // Error ignored!
}

// ❌ Don't expose sensitive information
async fn insecure_error_handling(&self, ctx: &Context, msg: &Message) {
    if let Err(e) = self.send_response(ctx, msg).await {
        // Don't send internal errors to users
        let params = MessageParams::new_text(&format!("Error: {}", e));
        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await;
    }
}

// ✅ Do handle errors appropriately
async fn good_error_handling(&self, ctx: &Context, msg: &Message) {
    if let Err(e) = self.send_response(ctx, msg).await {
        tracing::error!("Failed to send response: {}", e);
        
        let user_message = "I'm having trouble responding right now. Please try again.";
        if let Err(fallback_err) = self.send_fallback(ctx, &msg.channel_id, user_message).await {
            tracing::error!("Fallback message also failed: {}", fallback_err);
        }
    }
}
```

Effective error handling is essential for maintaining a stable, reliable bot. By implementing proper error handling strategies, you can ensure your bot gracefully handles failures and provides a consistent experience for users.
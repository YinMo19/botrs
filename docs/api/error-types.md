# Error Types API Reference

BotRS provides a comprehensive error system through the `BotError` enum, which covers all possible failure scenarios when working with the QQ Guild API. This guide covers the different error types, their meanings, and how to handle them effectively.

## Overview

```rust
use botrs::{BotError, Result};

// All BotRS functions return Result<T, BotError>
pub type Result<T> = std::result::Result<T, BotError>;
```

The `BotError` enum is the primary error type used throughout BotRS. It implements standard traits like `std::error::Error`, `Debug`, and `Display` for easy integration with Rust's error handling ecosystem.

## Error Types

### Network Errors

#### `Http`

HTTP client errors from the underlying reqwest library.

```rust
BotError::Http(reqwest::Error)
```

**Common causes:**
- Network connectivity issues
- DNS resolution failures
- Connection timeouts
- SSL/TLS errors

**Example:**
```rust
match api.get_guild(&token, "guild_id").await {
    Err(BotError::Http(e)) if e.is_timeout() => {
        println!("Request timed out, retrying...");
    }
    Err(BotError::Http(e)) if e.is_connect() => {
        println!("Connection failed: {}", e);
    }
    _ => {}
}
```

#### `WebSocket`

WebSocket connection errors from the gateway.

```rust
BotError::WebSocket(Box<tokio_tungstenite::tungstenite::Error>)
```

**Common causes:**
- Gateway connection failures
- WebSocket protocol errors
- Network interruptions
- Invalid WebSocket frames

**Example:**
```rust
match error {
    BotError::WebSocket(ws_error) => {
        println!("WebSocket error: {}", ws_error);
        // Gateway will automatically attempt reconnection
    }
    _ => {}
}
```

#### `Timeout`

Network timeout errors.

```rust
BotError::Timeout
```

**Handling:**
```rust
match api.get_message(&token, channel_id, message_id).await {
    Err(BotError::Timeout) => {
        println!("Request timed out, implementing retry logic");
        // Implement exponential backoff retry
    }
    _ => {}
}
```

### API Response Errors

#### `Api`

Generic API errors with status code and message.

```rust
BotError::Api { code: u32, message: String }
```

**Example:**
```rust
match error {
    BotError::Api { code, message } => {
        match code {
            400 => println!("Bad request: {}", message),
            500 => println!("Server error: {}", message),
            _ => println!("API error {}: {}", code, message),
        }
    }
    _ => {}
}
```

#### `AuthenticationFailed`

Authentication errors (401 status).

```rust
BotError::AuthenticationFailed(String)
```

**Common causes:**
- Invalid bot token
- Expired credentials
- Incorrect app ID or secret

**Handling:**
```rust
match error {
    BotError::AuthenticationFailed(msg) => {
        eprintln!("Authentication failed: {}", msg);
        // Check and refresh bot credentials
    }
    _ => {}
}
```

#### `Forbidden`

Permission denied errors (403 status).

```rust
BotError::Forbidden(String)
```

**Common causes:**
- Missing bot permissions
- Insufficient role hierarchy
- Channel access restrictions

**Example:**
```rust
match api.delete_message(&token, channel_id, message_id).await {
    Err(BotError::Forbidden(msg)) => {
        println!("Missing permissions to delete message: {}", msg);
    }
    _ => {}
}
```

#### `NotFound`

Resource not found errors (404 status).

```rust
BotError::NotFound(String)
```

**Common causes:**
- Invalid channel/guild/user IDs
- Deleted resources
- Inaccessible content

#### `MethodNotAllowed`

HTTP method not allowed (405 status).

```rust
BotError::MethodNotAllowed(String)
```

#### `SequenceNumber`

Rate limiting errors (429 status).

```rust
BotError::SequenceNumber(String)
```

#### `Server`

Server errors (500, 504 status).

```rust
BotError::Server(String)
```

### Rate Limiting

#### `RateLimit`

Structured rate limiting information.

```rust
BotError::RateLimit { retry_after: u64 }
```

**Handling:**
```rust
match error {
    BotError::RateLimit { retry_after } => {
        println!("Rate limited, retry after {} seconds", retry_after);
        tokio::time::sleep(Duration::from_secs(retry_after)).await;
        // Retry the operation
    }
    _ => {}
}
```

### Data and Parsing Errors

#### `Json`

JSON serialization/deserialization errors.

```rust
BotError::Json(serde_json::Error)
```

**Common causes:**
- Malformed API responses
- Schema mismatches
- Invalid JSON data

#### `Serde`

Alternative name for JSON errors (legacy).

```rust
BotError::Serde(serde_json::Error)
```

#### `InvalidData`

Invalid data format errors.

```rust
BotError::InvalidData(String)
```

### Connection and Gateway Errors

#### `Connection`

General connection errors.

```rust
BotError::Connection(String)
```

#### `Gateway`

Gateway-specific errors.

```rust
BotError::Gateway(String)
```

**Common causes:**
- Invalid session
- Gateway reconnection failures
- Protocol violations

#### `Session`

Session management errors.

```rust
BotError::Session(String)
```

### Configuration Errors

#### `Config`

Configuration-related errors.

```rust
BotError::Config(String)
```

**Common causes:**
- Invalid bot configuration
- Missing required settings
- Conflicting options

#### `Auth`

Authentication configuration errors.

```rust
BotError::Auth(String)
```

### System Errors

#### `Io`

I/O operation errors.

```rust
BotError::Io(std::io::Error)
```

**Common causes:**
- File system operations
- Network I/O failures
- Permission issues

#### `Url`

URL parsing errors.

```rust
BotError::Url(url::ParseError)
```

#### `Internal`

Internal framework errors.

```rust
BotError::Internal(String)
```

#### `NotImplemented`

Feature not yet implemented.

```rust
BotError::NotImplemented(String)
```

## Error Methods

### `is_retryable`

Determines if an error condition is retryable.

```rust
pub fn is_retryable(&self) -> bool
```

**Example:**
```rust
if error.is_retryable() {
    println!("Error is retryable, implementing retry logic");
    // Implement retry with backoff
} else {
    println!("Error is not retryable, giving up");
}
```

**Retryable errors:**
- HTTP timeouts and connection errors
- WebSocket errors
- Connection errors
- Timeout errors
- Gateway errors
- Rate limit errors

### `retry_after`

Gets the recommended retry delay in seconds.

```rust
pub fn retry_after(&self) -> Option<u64>
```

**Example:**
```rust
if let Some(delay) = error.retry_after() {
    println!("Retrying after {} seconds", delay);
    tokio::time::sleep(Duration::from_secs(delay)).await;
}
```

## Constructor Methods

### Error Creation

The `BotError` enum provides several constructor methods for creating specific error types:

```rust
// API error
let error = BotError::api(404, "Channel not found");

// Authentication error
let error = BotError::auth("Invalid token");

// Connection error
let error = BotError::connection("Failed to connect to gateway");

// Configuration error
let error = BotError::config("Missing app ID");

// Invalid data error
let error = BotError::invalid_data("Invalid message format");

// Gateway error
let error = BotError::gateway("Session expired");

// Session error
let error = BotError::session("Invalid session ID");

// Internal error
let error = BotError::internal("Unexpected state");

// Rate limit error
let error = BotError::rate_limit(60);

// Not implemented error
let error = BotError::not_implemented("Feature coming soon");
```

## Error Handling Patterns

### Basic Error Handling

```rust
use botrs::{BotError, Result};

async fn send_message_safely(
    ctx: &Context,
    channel_id: &str,
    content: &str,
) -> Result<()> {
    match ctx.send_message(channel_id, content).await {
        Ok(_) => {
            println!("Message sent successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
            Err(e)
        }
    }
}
```

### Comprehensive Error Handling

```rust
async fn handle_api_operation(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
) -> Result<Guild> {
    match api.get_guild(token, guild_id).await {
        Ok(guild) => Ok(guild),
        Err(BotError::NotFound(msg)) => {
            println!("Guild not found: {}", msg);
            Err(BotError::NotFound(msg))
        }
        Err(BotError::Forbidden(msg)) => {
            println!("Access denied: {}", msg);
            Err(BotError::Forbidden(msg))
        }
        Err(BotError::RateLimit { retry_after }) => {
            println!("Rate limited, waiting {} seconds", retry_after);
            tokio::time::sleep(Duration::from_secs(retry_after)).await;
            // Retry the operation
            api.get_guild(token, guild_id).await
        }
        Err(BotError::Server(msg)) => {
            println!("Server error: {}", msg);
            // Implement retry logic for server errors
            Err(BotError::Server(msg))
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            Err(e)
        }
    }
}
```

### Retry Logic with Error Handling

```rust
use tokio::time::{sleep, Duration};

async fn retry_with_backoff<F, T, Fut>(
    operation: F,
    max_attempts: usize,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_error = None;
    
    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if !e.is_retryable() {
                    return Err(e);
                }
                
                let delay = e.retry_after().unwrap_or_else(|| {
                    // Exponential backoff: 1s, 2s, 4s, 8s, ...
                    std::cmp::min(1 << (attempt - 1), 60)
                });
                
                println!("Attempt {} failed: {}. Retrying in {}s", 
                        attempt, e, delay);
                
                if attempt < max_attempts {
                    sleep(Duration::from_secs(delay)).await;
                }
                
                last_error = Some(e);
            }
        }
    }
    
    Err(last_error.unwrap())
}
```

### Error Categorization

```rust
fn categorize_error(error: &BotError) -> &'static str {
    match error {
        BotError::Http(_) | BotError::WebSocket(_) | BotError::Timeout => "Network",
        BotError::AuthenticationFailed(_) | BotError::Auth(_) => "Authentication",
        BotError::Forbidden(_) | BotError::NotFound(_) => "Permission",
        BotError::RateLimit { .. } | BotError::SequenceNumber(_) => "Rate Limit",
        BotError::Json(_) | BotError::InvalidData(_) => "Data",
        BotError::Gateway(_) | BotError::Session(_) => "Gateway",
        BotError::Config(_) => "Configuration",
        BotError::Server(_) => "Server",
        _ => "Other",
    }
}

async fn handle_categorized_error(error: BotError) {
    let category = categorize_error(&error);
    
    match category {
        "Network" => {
            println!("Network issue detected, checking connectivity");
            // Implement network diagnostic logic
        }
        "Authentication" => {
            println!("Authentication issue, refreshing credentials");
            // Implement credential refresh logic
        }
        "Rate Limit" => {
            println!("Rate limit hit, backing off");
            // Implement rate limit handling
        }
        _ => {
            println!("Error category: {}, Error: {}", category, error);
        }
    }
}
```

## Error Extension Trait

### `IntoBotError`

Extension trait for converting generic errors to `BotError`.

```rust
pub trait IntoBotError<T> {
    fn with_context(self, context: &str) -> Result<T>;
}
```

**Usage:**
```rust
use botrs::IntoBotError;

let result = std::fs::read_to_string("config.json")
    .with_context("Failed to read configuration file")?;
```

## Production Error Handling

### Logging and Monitoring

```rust
use tracing::{error, warn, info};

async fn production_error_handler(error: BotError) {
    match &error {
        BotError::AuthenticationFailed(_) => {
            error!("Authentication failed: {}", error);
            // Alert operations team
        }
        BotError::RateLimit { retry_after } => {
            warn!("Rate limited for {} seconds", retry_after);
            // Update metrics
        }
        BotError::Server(_) => {
            error!("Server error: {}", error);
            // Check service health
        }
        _ => {
            info!("Handled error: {}", error);
        }
    }
}
```

### Error Metrics

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ErrorMetrics {
    counts: Arc<Mutex<HashMap<String, u64>>>,
}

impl ErrorMetrics {
    pub async fn record_error(&self, error: &BotError) {
        let error_type = match error {
            BotError::Http(_) => "http",
            BotError::WebSocket(_) => "websocket",
            BotError::RateLimit { .. } => "rate_limit",
            BotError::AuthenticationFailed(_) => "auth_failed",
            _ => "other",
        };
        
        let mut counts = self.counts.lock().await;
        *counts.entry(error_type.to_string()).or_insert(0) += 1;
    }
    
    pub async fn get_error_counts(&self) -> HashMap<String, u64> {
        self.counts.lock().await.clone()
    }
}
```

## Best Practices

### Error Handling Guidelines

1. **Always handle errors explicitly**: Don't ignore or unwrap errors in production code
2. **Use appropriate error types**: Match error handling to the specific error type
3. **Implement proper logging**: Log errors with sufficient context for debugging
4. **Respect rate limits**: Always handle rate limiting errors appropriately
5. **Provide user feedback**: Convert technical errors to user-friendly messages

### Error Prevention

1. **Validate inputs**: Check parameters before making API calls
2. **Handle edge cases**: Consider scenarios like missing permissions or invalid IDs
3. **Implement timeouts**: Set reasonable timeouts for all operations
4. **Use structured error handling**: Implement consistent error handling patterns

### Recovery Strategies

1. **Exponential backoff**: Use increasing delays for retries
2. **Circuit breakers**: Stop calling failing services temporarily
3. **Graceful degradation**: Provide fallback functionality when possible
4. **Health checks**: Monitor service health and adjust behavior accordingly

## See Also

- [`Client`](./client.md) - Main bot client
- [`Context`](./context.md) - API access in event handlers
- [`BotApi`](./bot-api.md) - Direct API access
- [Error Handling Guide](/guide/error-handling) - Comprehensive error handling strategies
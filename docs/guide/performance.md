# Performance Guide

This guide covers best practices and techniques for optimizing the performance of your BotRS applications. Learn how to build high-performance bots that can handle large-scale deployments efficiently.

## Overview

BotRS is built with performance in mind, leveraging Rust's zero-cost abstractions and async runtime capabilities. However, proper application design and configuration are crucial for achieving optimal performance.

## Core Performance Principles

### Async-First Design

BotRS uses Tokio's async runtime, which allows handling thousands of concurrent operations:

```rust
use botrs::{Client, Context, EventHandler, Message, Token, Intents};
use tokio::time::{sleep, Duration};

struct HighPerformanceBot;

#[async_trait::async_trait]
impl EventHandler for HighPerformanceBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Non-blocking operations
        let api_call = self.process_message(&ctx, &msg);
        let database_write = self.log_message(&msg);
        
        // Execute concurrently
        let (api_result, db_result) = tokio::join!(api_call, database_write);
        
        // Handle results without blocking
        if let Err(e) = api_result {
            tracing::warn!("API call failed: {}", e);
        }
        if let Err(e) = db_result {
            tracing::warn!("Database write failed: {}", e);
        }
    }
}

impl HighPerformanceBot {
    async fn process_message(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        // Perform API operations asynchronously
        if let Some(content) = &msg.content {
            if content.starts_with("!slow_command") {
                // Don't block other messages while processing
                tokio::spawn(async move {
                    sleep(Duration::from_secs(5)).await;
                    // Heavy processing here
                });
            }
        }
        Ok(())
    }
    
    async fn log_message(&self, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        // Non-blocking database write
        Ok(())
    }
}
```

### Memory Management

Rust's ownership system eliminates garbage collection overhead:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

// Shared state with minimal allocations
pub struct BotState {
    user_cache: Arc<RwLock<HashMap<String, Arc<UserData>>>>,
    message_cache: Arc<RwLock<LruCache<String, Arc<Message>>>>,
}

#[derive(Clone)]
pub struct UserData {
    id: String,
    username: String,
    // Use Arc to avoid cloning large data
    preferences: Arc<UserPreferences>,
}

impl BotState {
    pub async fn get_user(&self, user_id: &str) -> Option<Arc<UserData>> {
        let cache = self.user_cache.read().await;
        cache.get(user_id).cloned() // Cheap Arc clone
    }
    
    pub async fn cache_user(&self, user: UserData) {
        let mut cache = self.user_cache.write().await;
        cache.insert(user.id.clone(), Arc::new(user));
    }
}
```

## Connection Management

### WebSocket Optimization

Configure WebSocket settings for optimal performance:

```rust
use botrs::{Client, Intents, Token};

async fn create_optimized_client() -> Result<Client<MyHandler>, botrs::BotError> {
    let token = Token::new("app_id", "secret");
    
    // Optimize intents - only subscribe to needed events
    let intents = Intents::default()
        .with_public_guild_messages()  // Only if needed
        .with_guilds();               // Essential for most bots
        // Avoid .with_guild_members() unless necessary (privileged)
    
    let client = Client::new(token, intents, MyHandler, false)?;
    Ok(client)
}
```

### HTTP Client Optimization

Reuse HTTP connections and configure timeouts:

```rust
use reqwest::Client;
use std::time::Duration;

pub struct OptimizedApiClient {
    client: Client,
}

impl OptimizedApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(60))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .user_agent("MyBot/1.0")
            .build()
            .expect("Failed to create HTTP client");
            
        Self { client }
    }
}
```

## Event Processing Optimization

### Batching Operations

Process multiple events together when possible:

```rust
use tokio::sync::mpsc;
use std::collections::VecDeque;

struct BatchProcessor {
    message_queue: mpsc::UnboundedSender<Message>,
}

impl BatchProcessor {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Background batch processor
        tokio::spawn(async move {
            let mut batch = VecDeque::new();
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            
            loop {
                tokio::select! {
                    msg = rx.recv() => {
                        if let Some(msg) = msg {
                            batch.push_back(msg);
                            
                            // Process when batch is full
                            if batch.len() >= 10 {
                                Self::process_batch(&mut batch).await;
                            }
                        }
                    }
                    _ = interval.tick() => {
                        // Process remaining messages periodically
                        if !batch.is_empty() {
                            Self::process_batch(&mut batch).await;
                        }
                    }
                }
            }
        });
        
        Self { message_queue: tx }
    }
    
    async fn process_batch(batch: &mut VecDeque<Message>) {
        // Process all messages in the batch
        while let Some(message) = batch.pop_front() {
            // Batch database writes, API calls, etc.
        }
    }
}
```

### Selective Event Handling

Only handle events you need:

```rust
#[async_trait::async_trait]
impl EventHandler for OptimizedBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Quick early returns for unwanted messages
        if msg.is_from_bot() {
            return;
        }
        
        let content = match &msg.content {
            Some(content) if !content.is_empty() => content,
            _ => return,
        };
        
        // Fast path for simple commands
        if content == "!ping" {
            let _ = msg.reply(&ctx.api, &ctx.token, "Pong!").await;
            return;
        }
        
        // Complex processing only when needed
        if content.starts_with("!complex") {
            self.handle_complex_command(&ctx, &msg, content).await;
        }
    }
    
    // Don't implement unused event handlers
    // async fn guild_create(&self, ctx: Context, guild: Guild) {} // Skip if not needed
}
```

## Caching Strategies

### Multi-Level Caching

Implement efficient caching for frequently accessed data:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;

pub struct CacheManager {
    // Hot cache for immediate access
    hot_cache: Arc<RwLock<LruCache<String, Arc<CachedData>>>>,
    // Warm cache for recent data
    warm_cache: Arc<RwLock<LruCache<String, Arc<CachedData>>>>,
    // Cold storage (database, file system)
}

#[derive(Clone)]
pub struct CachedData {
    pub value: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub hit_count: Arc<std::sync::atomic::AtomicU64>,
}

impl CacheManager {
    pub async fn get(&self, key: &str) -> Option<Arc<CachedData>> {
        // Try hot cache first
        {
            let mut hot = self.hot_cache.write().await;
            if let Some(data) = hot.get(key) {
                data.hit_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Some(data.clone());
            }
        }
        
        // Try warm cache
        {
            let mut warm = self.warm_cache.write().await;
            if let Some(data) = warm.get(key) {
                // Promote to hot cache
                let mut hot = self.hot_cache.write().await;
                hot.put(key.to_string(), data.clone());
                return Some(data.clone());
            }
        }
        
        // Load from cold storage
        self.load_from_storage(key).await
    }
    
    async fn load_from_storage(&self, key: &str) -> Option<Arc<CachedData>> {
        // Load from database/file system
        None // Placeholder
    }
}
```

### Cache Invalidation

Implement smart cache invalidation:

```rust
impl CacheManager {
    pub async fn invalidate(&self, pattern: &str) {
        // Invalidate entries matching pattern
        let mut hot = self.hot_cache.write().await;
        let mut warm = self.warm_cache.write().await;
        
        // Remove matching keys
        hot.retain(|k, _| !k.contains(pattern));
        warm.retain(|k, _| !k.contains(pattern));
    }
    
    pub async fn refresh_background(&self) {
        // Background refresh of expiring entries
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                // Refresh entries that are about to expire
            }
        });
    }
}
```

## Database Optimization

### Connection Pooling

Use connection pools for database operations:

```rust
use sqlx::{Pool, Postgres, PgPool};
use std::time::Duration;

pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(database_url)
            .await?;
            
        Ok(Self { pool })
    }
    
    pub async fn batch_insert_messages(&self, messages: &[Message]) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        
        for message in messages {
            sqlx::query!(
                "INSERT INTO messages (id, content, author_id, channel_id) VALUES ($1, $2, $3, $4)",
                message.id,
                message.content,
                message.author.as_ref().and_then(|a| a.id.as_ref()),
                message.channel_id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}
```

### Query Optimization

Optimize database queries:

```rust
impl DatabaseManager {
    // Use prepared statements
    pub async fn get_user_messages(&self, user_id: &str, limit: i32) -> Result<Vec<StoredMessage>, sqlx::Error> {
        sqlx::query_as!(
            StoredMessage,
            r#"
            SELECT id, content, created_at
            FROM messages 
            WHERE author_id = $1 
            ORDER BY created_at DESC 
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }
    
    // Use indexes effectively
    pub async fn get_recent_channel_activity(&self, channel_id: &str) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM messages 
            WHERE channel_id = $1 
            AND created_at > NOW() - INTERVAL '1 hour'
            "#,
            channel_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.count.unwrap_or(0))
    }
}
```

## Rate Limiting

### Intelligent Rate Limiting

Implement smart rate limiting to maximize throughput:

```rust
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
}

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn try_acquire(&self, key: &str, tokens: f64) -> bool {
        let mut buckets = self.buckets.lock().await;
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            TokenBucket {
                tokens: 5.0, // Initial tokens
                last_refill: Instant::now(),
                capacity: 5.0,
                refill_rate: 1.0, // 1 token per second
            }
        });
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;
        
        // Try to consume tokens
        if bucket.tokens >= tokens {
            bucket.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

// Usage in bot
impl OptimizedBot {
    async fn send_message_with_rate_limit(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), botrs::BotError> {
        let rate_limiter = &self.rate_limiter;
        
        // Wait for rate limit if necessary
        while !rate_limiter.try_acquire(channel_id, 1.0).await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        // Send message
        let params = botrs::MessageParams::new_text(content);
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await
    }
}
```

## Memory Optimization

### String Interning

Reduce memory usage with string interning:

```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct StringInterner {
    strings: HashMap<String, Arc<str>>,
}

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> Arc<str> {
        if let Some(interned) = self.strings.get(s) {
            interned.clone()
        } else {
            let arc_str: Arc<str> = Arc::from(s);
            self.strings.insert(s.to_string(), arc_str.clone());
            arc_str
        }
    }
}

// Use interned strings for commonly repeated data
struct OptimizedMessage {
    id: String,
    content: Option<String>,
    channel_id: Arc<str>, // Interned - channels are reused frequently
    guild_id: Arc<str>,   // Interned - guilds are reused frequently
}
```

### Object Pooling

Reuse expensive objects:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> ObjectPool<T> {
    pub fn new<F>(factory: F) -> Self 
    where 
        F: Fn() -> T + Send + Sync + 'static 
    {
        Self {
            objects: Arc::new(Mutex::new(Vec::new())),
            factory: Box::new(factory),
        }
    }
    
    pub async fn acquire(&self) -> PooledObject<T> {
        let mut objects = self.objects.lock().await;
        let object = objects.pop().unwrap_or_else(|| (self.factory)());
        PooledObject::new(object, self.objects.clone())
    }
}

pub struct PooledObject<T> {
    object: Option<T>,
    pool: Arc<Mutex<Vec<T>>>,
}

impl<T> PooledObject<T> {
    fn new(object: T, pool: Arc<Mutex<Vec<T>>>) -> Self {
        Self {
            object: Some(object),
            pool,
        }
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(object) = self.object.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                let mut objects = pool.lock().await;
                if objects.len() < 10 { // Max pool size
                    objects.push(object);
                }
            });
        }
    }
}

impl<T> std::ops::Deref for PooledObject<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.object.as_ref().unwrap()
    }
}
```

## Monitoring and Profiling

### Performance Metrics

Track performance metrics:

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Default)]
pub struct Metrics {
    pub messages_processed: AtomicU64,
    pub api_calls: AtomicU64,
    pub errors: AtomicU64,
    pub response_times: Arc<Mutex<Vec<Duration>>>,
}

impl Metrics {
    pub fn record_message(&self) {
        self.messages_processed.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_api_call(&self, duration: Duration) {
        self.api_calls.fetch_add(1, Ordering::Relaxed);
        
        // Record response time (keep last 1000 measurements)
        tokio::spawn({
            let response_times = self.response_times.clone();
            async move {
                let mut times = response_times.lock().await;
                if times.len() >= 1000 {
                    times.remove(0);
                }
                times.push(duration);
            }
        });
    }
    
    pub async fn get_stats(&self) -> PerformanceStats {
        let times = self.response_times.lock().await;
        let avg_response_time = if times.is_empty() {
            Duration::from_millis(0)
        } else {
            times.iter().sum::<Duration>() / times.len() as u32
        };
        
        PerformanceStats {
            messages_processed: self.messages_processed.load(Ordering::Relaxed),
            api_calls: self.api_calls.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
            avg_response_time,
        }
    }
}

pub struct PerformanceStats {
    pub messages_processed: u64,
    pub api_calls: u64,
    pub errors: u64,
    pub avg_response_time: Duration,
}
```

### CPU Profiling

Use Rust's built-in profiling tools:

```toml
[profile.release]
debug = 1  # Enable debug symbols for profiling

[dependencies]
pprof = { version = "0.12", features = ["flamegraph"] }
```

```rust
// Enable profiling in production (gated behind feature flag)
#[cfg(feature = "profiling")]
use pprof::ProfilerGuard;

async fn run_with_profiling() {
    #[cfg(feature = "profiling")]
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    
    // Run your bot
    
    #[cfg(feature = "profiling")]
    {
        if let Ok(report) = guard.report().build() {
            let file = std::fs::File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
        }
    }
}
```

## Deployment Optimization

### Container Optimization

Optimize Docker containers:

```dockerfile
# Multi-stage build for smaller images
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

# Build with optimizations
RUN cargo build --release

FROM debian:bookworm-slim

# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/bot /usr/local/bin/bot

# Run as non-root user
RUN useradd -r -s /bin/false botuser
USER botuser

CMD ["bot"]
```

### Environment Configuration

Configure for production environments:

```rust
pub struct ProductionConfig {
    pub worker_threads: usize,
    pub blocking_threads: usize,
    pub stack_size: usize,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            worker_threads: cpu_count,
            blocking_threads: 512,
            stack_size: 2 * 1024 * 1024, // 2MB
        }
    }
}

#[tokio::main(worker_threads = 8, blocking_threads = 512)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Production-optimized runtime
    let config = ProductionConfig::default();
    
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .max_blocking_threads(config.blocking_threads)
        .thread_stack_size(config.stack_size)
        .enable_all()
        .build()?;
    
    runtime.block_on(async {
        // Your bot logic here
    });
    
    Ok(())
}
```

## Best Practices Summary

### Do's

1. **Use async/await extensively** - Don't block the runtime
2. **Implement proper caching** - Cache frequently accessed data
3. **Optimize database queries** - Use indexes and prepared statements
4. **Monitor performance** - Track metrics and profile regularly
5. **Use connection pooling** - Reuse HTTP and database connections
6. **Implement rate limiting** - Respect API limits proactively
7. **Minimize allocations** - Reuse objects and use string interning

### Don'ts

1. **Don't use blocking operations** in async contexts
2. **Don't ignore rate limits** - This leads to degraded performance
3. **Don't cache everything** - Be selective about what to cache
4. **Don't neglect error handling** - Errors affect performance
5. **Don't use unwrap()** in production - Handle errors gracefully
6. **Don't create unnecessary threads** - Tokio handles concurrency
7. **Don't skip monitoring** - Performance issues are hard to debug without metrics

## Performance Testing

### Load Testing

Test your bot under realistic loads:

```rust
use tokio::time::{interval, Duration};

async fn load_test() {
    let client = create_test_client().await;
    let mut interval = interval(Duration::from_millis(100));
    
    for i in 0..1000 {
        interval.tick().await;
        
        // Simulate message processing
        let message = create_test_message(i);
        let start = Instant::now();
        
        // Process message
        client.handle_message(message).await;
        
        let duration = start.elapsed();
        if duration > Duration::from_millis(100) {
            println!("Slow message processing: {:?}", duration);
        }
    }
}
```

### Benchmarking

Use criterion for micro-benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_message_parsing(c: &mut Criterion) {
    c.bench_function("parse_message", |b| {
        b.iter(|| {
            let message = create_test_message();
            black_box(parse_message_content(message))
        })
    });
}

criterion_group!(benches, benchmark_message_parsing);
criterion_main!(benches);
```

By following these performance optimization strategies, your BotRS application will be able to handle high loads efficiently while maintaining responsiveness and reliability.
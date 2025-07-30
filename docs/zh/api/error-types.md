# 错误类型 API 参考

BotRS 通过 `BotError` 枚举提供了一个全面的错误系统，涵盖了使用 QQ 频道 API 时可能遇到的所有故障场景。本指南介绍了不同的错误类型、其含义以及如何有效处理它们。

## 概述

```rust
use botrs::{BotError, Result};

// 所有 BotRS 函数都返回 Result<T, BotError>
pub type Result<T> = std::result::Result<T, BotError>;
```

`BotError` 枚举是 BotRS 中使用的主要错误类型。它实现了标准特征，如 `std::error::Error`、`Debug` 和 `Display`，便于与 Rust 的错误处理生态系统集成。

## 错误类型

### 网络错误

#### `Http`

来自底层 reqwest 库的 HTTP 客户端错误。

```rust
BotError::Http(reqwest::Error)
```

**常见原因：**
- 网络连接问题
- DNS 解析失败
- 连接超时
- SSL/TLS 错误

**示例：**
```rust
match api.get_guild(&token, "guild_id").await {
    Err(BotError::Http(e)) if e.is_timeout() => {
        println!("请求超时，正在重试...");
    }
    Err(BotError::Http(e)) if e.is_connect() => {
        println!("连接失败：{}", e);
    }
    _ => {}
}
```

#### `WebSocket`

来自网关的 WebSocket 连接错误。

```rust
BotError::WebSocket(Box<tokio_tungstenite::tungstenite::Error>)
```

**常见原因：**
- 网关连接失败
- WebSocket 协议错误
- 网络中断
- 无效的 WebSocket 帧

**示例：**
```rust
match error {
    BotError::WebSocket(ws_error) => {
        println!("WebSocket 错误：{}", ws_error);
        // 网关将自动尝试重连
    }
    _ => {}
}
```

#### `Timeout`

网络超时错误。

```rust
BotError::Timeout
```

**处理方式：**
```rust
match api.get_message(&token, channel_id, message_id).await {
    Err(BotError::Timeout) => {
        println!("请求超时，实施重试逻辑");
        // 实施指数退避重试
    }
    _ => {}
}
```

### API 响应错误

#### `Api`

带有状态码和消息的通用 API 错误。

```rust
BotError::Api { code: u32, message: String }
```

**示例：**
```rust
match error {
    BotError::Api { code, message } => {
        match code {
            400 => println!("错误请求：{}", message),
            500 => println!("服务器错误：{}", message),
            _ => println!("API 错误 {}：{}", code, message),
        }
    }
    _ => {}
}
```

#### `AuthenticationFailed`

身份验证错误（401 状态）。

```rust
BotError::AuthenticationFailed(String)
```

**常见原因：**
- 无效的机器人令牌
- 凭据过期
- 错误的应用 ID 或密钥

**处理方式：**
```rust
match error {
    BotError::AuthenticationFailed(msg) => {
        eprintln!("身份验证失败：{}", msg);
        // 检查并刷新机器人凭据
    }
    _ => {}
}
```

#### `Forbidden`

权限拒绝错误（403 状态）。

```rust
BotError::Forbidden(String)
```

**常见原因：**
- 缺少机器人权限
- 角色层级不足
- 频道访问限制

**示例：**
```rust
match api.delete_message(&token, channel_id, message_id).await {
    Err(BotError::Forbidden(msg)) => {
        println!("缺少删除消息的权限：{}", msg);
    }
    _ => {}
}
```

#### `NotFound`

资源未找到错误（404 状态）。

```rust
BotError::NotFound(String)
```

**常见原因：**
- 无效的频道/频道/用户 ID
- 已删除的资源
- 无法访问的内容

#### `MethodNotAllowed`

不允许的 HTTP 方法（405 状态）。

```rust
BotError::MethodNotAllowed(String)
```

#### `SequenceNumber`

速率限制错误（429 状态）。

```rust
BotError::SequenceNumber(String)
```

#### `Server`

服务器错误（500、504 状态）。

```rust
BotError::Server(String)
```

### 速率限制

#### `RateLimit`

结构化的速率限制信息。

```rust
BotError::RateLimit { retry_after: u64 }
```

**处理方式：**
```rust
match error {
    BotError::RateLimit { retry_after } => {
        println!("被限速，{} 秒后重试", retry_after);
        tokio::time::sleep(Duration::from_secs(retry_after)).await;
        // 重试操作
    }
    _ => {}
}
```

### 数据和解析错误

#### `Json`

JSON 序列化/反序列化错误。

```rust
BotError::Json(serde_json::Error)
```

**常见原因：**
- 格式错误的 API 响应
- 模式不匹配
- 无效的 JSON 数据

#### `InvalidData`

无效数据格式错误。

```rust
BotError::InvalidData(String)
```

### 连接和网关错误

#### `Connection`

通用连接错误。

```rust
BotError::Connection(String)
```

#### `Gateway`

网关特定错误。

```rust
BotError::Gateway(String)
```

**常见原因：**
- 无效会话
- 网关重连失败
- 协议违规

#### `Session`

会话管理错误。

```rust
BotError::Session(String)
```

### 配置错误

#### `Config`

配置相关错误。

```rust
BotError::Config(String)
```

**常见原因：**
- 无效的机器人配置
- 缺少必需设置
- 冲突选项

#### `Auth`

身份验证配置错误。

```rust
BotError::Auth(String)
```

### 系统错误

#### `Io`

I/O 操作错误。

```rust
BotError::Io(std::io::Error)
```

**常见原因：**
- 文件系统操作
- 网络 I/O 失败
- 权限问题

#### `Url`

URL 解析错误。

```rust
BotError::Url(url::ParseError)
```

#### `Internal`

内部框架错误。

```rust
BotError::Internal(String)
```

#### `NotImplemented`

功能尚未实现。

```rust
BotError::NotImplemented(String)
```

## 错误方法

### `is_retryable`

判断错误条件是否可重试。

```rust
pub fn is_retryable(&self) -> bool
```

**示例：**
```rust
if error.is_retryable() {
    println!("错误可重试，实施重试逻辑");
    // 实施带退避的重试
} else {
    println!("错误不可重试，放弃操作");
}
```

**可重试的错误：**
- HTTP 超时和连接错误
- WebSocket 错误
- 连接错误
- 超时错误
- 网关错误
- 速率限制错误

### `retry_after`

获取推荐的重试延迟（秒）。

```rust
pub fn retry_after(&self) -> Option<u64>
```

**示例：**
```rust
if let Some(delay) = error.retry_after() {
    println!("将在 {} 秒后重试", delay);
    tokio::time::sleep(Duration::from_secs(delay)).await;
}
```

## 构造方法

### 错误创建

`BotError` 枚举提供了几种用于创建特定错误类型的构造方法：

```rust
// API 错误
let error = BotError::api(404, "频道未找到");

// 身份验证错误
let error = BotError::auth("无效令牌");

// 连接错误
let error = BotError::connection("无法连接到网关");

// 配置错误
let error = BotError::config("缺少应用 ID");

// 无效数据错误
let error = BotError::invalid_data("无效消息格式");

// 网关错误
let error = BotError::gateway("会话过期");

// 会话错误
let error = BotError::session("无效会话 ID");

// 内部错误
let error = BotError::internal("意外状态");

// 速率限制错误
let error = BotError::rate_limit(60);

// 未实现错误
let error = BotError::not_implemented("功能即将推出");
```

## 错误处理模式

### 基本错误处理

```rust
use botrs::{BotError, Result};

async fn send_message_safely(
    ctx: &Context,
    channel_id: &str,
    content: &str,
) -> Result<()> {
    match ctx.send_message(channel_id, content).await {
        Ok(_) => {
            println!("消息发送成功");
            Ok(())
        }
        Err(e) => {
            eprintln!("发送消息失败：{}", e);
            Err(e)
        }
    }
}
```

### 全面错误处理

```rust
async fn handle_api_operation(
    api: &BotApi,
    token: &Token,
    guild_id: &str,
) -> Result<Guild> {
    match api.get_guild(token, guild_id).await {
        Ok(guild) => Ok(guild),
        Err(BotError::NotFound(msg)) => {
            println!("频道未找到：{}", msg);
            Err(BotError::NotFound(msg))
        }
        Err(BotError::Forbidden(msg)) => {
            println!("访问被拒绝：{}", msg);
            Err(BotError::Forbidden(msg))
        }
        Err(BotError::RateLimit { retry_after }) => {
            println!("被限速，等待 {} 秒", retry_after);
            tokio::time::sleep(Duration::from_secs(retry_after)).await;
            // 重试操作
            api.get_guild(token, guild_id).await
        }
        Err(BotError::Server(msg)) => {
            println!("服务器错误：{}", msg);
            // 为服务器错误实施重试逻辑
            Err(BotError::Server(msg))
        }
        Err(e) => {
            eprintln!("意外错误：{}", e);
            Err(e)
        }
    }
}
```

### 带错误处理的重试逻辑

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
                    // 指数退避：1s、2s、4s、8s...
                    std::cmp::min(1 << (attempt - 1), 60)
                });
                
                println!("尝试 {} 失败：{}。将在 {}s 后重试", 
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

### 错误分类

```rust
fn categorize_error(error: &BotError) -> &'static str {
    match error {
        BotError::Http(_) | BotError::WebSocket(_) | BotError::Timeout => "网络",
        BotError::AuthenticationFailed(_) | BotError::Auth(_) => "身份验证",
        BotError::Forbidden(_) | BotError::NotFound(_) => "权限",
        BotError::RateLimit { .. } | BotError::SequenceNumber(_) => "速率限制",
        BotError::Json(_) | BotError::InvalidData(_) => "数据",
        BotError::Gateway(_) | BotError::Session(_) => "网关",
        BotError::Config(_) => "配置",
        BotError::Server(_) => "服务器",
        _ => "其他",
    }
}

async fn handle_categorized_error(error: BotError) {
    let category = categorize_error(&error);
    
    match category {
        "网络" => {
            println!("检测到网络问题，检查连接");
            // 实施网络诊断逻辑
        }
        "身份验证" => {
            println!("身份验证问题，刷新凭据");
            // 实施凭据刷新逻辑
        }
        "速率限制" => {
            println!("达到速率限制，退避");
            // 实施速率限制处理
        }
        _ => {
            println!("错误类别：{}，错误：{}", category, error);
        }
    }
}
```

## 错误扩展特征

### `IntoBotError`

用于将通用错误转换为 `BotError` 的扩展特征。

```rust
pub trait IntoBotError<T> {
    fn with_context(self, context: &str) -> Result<T>;
}
```

**使用方式：**
```rust
use botrs::IntoBotError;

let result = std::fs::read_to_string("config.json")
    .with_context("读取配置文件失败")?;
```

## 生产环境错误处理

### 日志记录和监控

```rust
use tracing::{error, warn, info};

async fn production_error_handler(error: BotError) {
    match &error {
        BotError::AuthenticationFailed(_) => {
            error!("身份验证失败：{}", error);
            // 警告运维团队
        }
        BotError::RateLimit { retry_after } => {
            warn!("被限速 {} 秒", retry_after);
            // 更新指标
        }
        BotError::Server(_) => {
            error!("服务器错误：{}", error);
            // 检查服务健康状态
        }
        _ => {
            info!("处理错误：{}", error);
        }
    }
}
```

### 错误指标

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

## 最佳实践

### 错误处理指南

1. **始终明确处理错误** - 不要在生产代码中忽略或展开错误
2. **使用适当的错误类型** - 匹配错误处理与特定错误类型
3. **实施适当的日志记录** - 记录具有足够上下文的错误以便调试
4. **遵守速率限制** - 始终适当处理速率限制错误
5. **提供用户反馈** - 将技术错误转换为用户友好的消息

### 错误预防

1. **验证输入** - 在进行 API 调用之前检查参数
2. **处理边缘情况** - 考虑如缺少权限或无效 ID 等场景
3. **实施超时** - 为所有操作设置合理的超时
4. **使用结构化错误处理** - 实施一致的错误处理模式

### 恢复策略

1. **指数退避** - 对重试使用递增延迟
2. **断路器** - 暂时停止调用失败的服务
3. **优雅降级** - 在可能的情况下提供备用功能
4. **健康检查** - 监控服务健康状态并相应调整行为

## 相关链接

- [`Client`](./client.md) - 主要机器人客户端
- [`Context`](./context.md) - 事件处理器中的 API 访问
- [`BotApi`](./bot-api.md) - 直接 API 访问
- [错误处理指南](/zh/guide/error-handling) - 全面的错误处理策略
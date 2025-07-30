# 令牌 API 参考

`Token` 结构体管理 QQ 频道机器人的身份验证凭据，包括应用 ID 和密钥，以及访问令牌的管理。

## 概述

```rust
pub struct Token {
    app_id: String,
    secret: String,
    access_token: Option<String>,
    expires_at: Option<u64>,
    refresh_mutex: Arc<Mutex<()>>,
}
```

`Token` 包含：
- 应用 ID 和密钥（由 QQ 提供）
- 访问令牌（从 QQ API 获取）
- 令牌过期时间和刷新互斥锁

## 构造函数

### `new`

使用给定的应用 ID 和密钥创建新令牌。

```rust
pub fn new<S1: Into<String>, S2: Into<String>>(app_id: S1, secret: S2) -> Self
```

#### 参数

- `app_id`: 机器人的应用 ID
- `secret`: 机器人的应用密钥

#### 返回值

返回新的 `Token` 实例。

#### 示例

```rust
use botrs::Token;

let token = Token::new("你的应用ID", "你的密钥");
```

### `from_env`

从环境变量创建令牌。

```rust
pub fn from_env() -> Result<Self, BotError>
```

期望的环境变量：
- `QQ_BOT_APP_ID`: 应用 ID
- `QQ_BOT_SECRET`: 应用密钥

#### 返回值

返回 `Result<Token, BotError>` - 成功时返回令牌，环境变量缺失时返回错误。

#### 示例

```rust
// 设置环境变量
std::env::set_var("QQ_BOT_APP_ID", "你的应用ID");
std::env::set_var("QQ_BOT_SECRET", "你的密钥");

let token = Token::from_env()?;
```

## 方法

### `validate`

验证令牌的格式和有效性。

```rust
pub fn validate(&self) -> Result<(), BotError>
```

检查：
- 应用 ID 不为空且格式正确
- 密钥不为空且长度足够

#### 返回值

验证成功时返回 `Ok(())`，否则返回描述错误的 `BotError`。

#### 示例

```rust
let token = Token::new("应用ID", "密钥");

match token.validate() {
    Ok(_) => println!("令牌有效"),
    Err(e) => eprintln!("令牌无效: {}", e),
}
```

### `authorization_header`

生成用于 API 请求的授权头。

```rust
pub fn authorization_header(&self) -> String
```

#### 返回值

返回格式为 `"Bot {app_id}.{secret}"` 的授权字符串。

#### 示例

```rust
let token = Token::new("123456", "secret123");
let auth_header = token.authorization_header();
println!("Authorization: {}", auth_header);
// 输出: Authorization: Bot 123456.secret123
```

### `get_access_token`

获取当前的访问令牌，如果过期则自动刷新。

```rust
pub async fn get_access_token(&self, http_client: &HttpClient) -> Result<String, BotError>
```

#### 参数

- `http_client`: HTTP 客户端用于刷新令牌

#### 返回值

返回有效的访问令牌或错误。

#### 示例

```rust
use botrs::{Token, HttpClient};

let token = Token::new("应用ID", "密钥");
let http_client = HttpClient::new();

match token.get_access_token(&http_client).await {
    Ok(access_token) => println!("访问令牌: {}", access_token),
    Err(e) => eprintln!("获取访问令牌失败: {}", e),
}
```

### `refresh_access_token`

强制刷新访问令牌。

```rust
pub async fn refresh_access_token(&self, http_client: &HttpClient) -> Result<(), BotError>
```

#### 参数

- `http_client`: HTTP 客户端用于 API 请求

#### 返回值

成功时返回 `Ok(())`，失败时返回 `BotError`。

### `is_access_token_expired`

检查当前访问令牌是否已过期。

```rust
pub fn is_access_token_expired(&self) -> bool
```

#### 返回值

如果令牌已过期或不存在返回 `true`，否则返回 `false`。

## 访问器

### `app_id`

获取应用 ID。

```rust
pub fn app_id(&self) -> &str
```

#### 示例

```rust
let token = Token::new("123456", "secret");
println!("应用 ID: {}", token.app_id());
```

### `secret`

获取应用密钥（出于安全考虑，实际实现可能会限制访问）。

```rust
pub fn secret(&self) -> &str
```

## 序列化支持

`Token` 支持 serde 序列化和反序列化，但访问令牌等敏感信息会被跳过。

```rust
use serde_json;

let token = Token::new("应用ID", "密钥");
let json = serde_json::to_string(&token)?;
let deserialized: Token = serde_json::from_str(&json)?;
```

## 使用示例

### 基础用法

```rust
use botrs::{Token, Client, Intents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建令牌
    let token = Token::new("你的应用ID", "你的密钥");
    
    // 验证令牌
    token.validate()?;
    
    // 使用令牌创建客户端
    let intents = Intents::default();
    let client = Client::new(token, intents, handler, false)?;
    
    Ok(())
}
```

### 从环境变量加载

```rust
use botrs::Token;

fn load_token() -> Result<Token, Box<dyn std::error::Error>> {
    // 方法1: 直接从环境变量
    let token = Token::from_env()?;
    
    // 方法2: 手动读取环境变量
    let app_id = std::env::var("QQ_BOT_APP_ID")?;
    let secret = std::env::var("QQ_BOT_SECRET")?;
    let token = Token::new(app_id, secret);
    
    token.validate()?;
    Ok(token)
}
```

### 配置文件加载

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    app_id: String,
    secret: String,
}

fn load_token_from_config(path: &str) -> Result<Token, Box<dyn std::error::Error>> {
    let config_data = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_data)?;
    
    let token = Token::new(config.app_id, config.secret);
    token.validate()?;
    
    Ok(token)
}
```

### 访问令牌管理

```rust
use botrs::{Token, HttpClient};

async fn token_management_example() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("应用ID", "密钥");
    let http_client = HttpClient::new();
    
    // 检查令牌是否过期
    if token.is_access_token_expired() {
        println!("访问令牌已过期，正在刷新...");
        token.refresh_access_token(&http_client).await?;
    }
    
    // 获取有效的访问令牌
    let access_token = token.get_access_token(&http_client).await?;
    println!("当前访问令牌: {}", access_token);
    
    Ok(())
}
```

### 安全处理

```rust
use botrs::Token;
use std::fmt;

// 创建一个包装器来安全显示令牌信息
struct SafeTokenDisplay<'a>(&'a Token);

impl<'a> fmt::Display for SafeTokenDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token {{ app_id: \"{}\", secret: \"***\" }}", 
               self.0.app_id())
    }
}

fn safe_token_logging() {
    let token = Token::new("123456", "secret123");
    
    // 安全地显示令牌信息
    println!("令牌信息: {}", SafeTokenDisplay(&token));
    
    // 避免直接打印令牌
    // println!("令牌: {:?}", token); // 不推荐
}
```

## 错误处理

### 常见错误类型

```rust
use botrs::{Token, BotError};

async fn handle_token_errors() {
    let token = Token::new("", ""); // 无效令牌
    
    match token.validate() {
        Ok(_) => println!("令牌有效"),
        Err(BotError::Authentication(msg)) => {
            eprintln!("身份验证错误: {}", msg);
        }
        Err(BotError::InvalidInput(msg)) => {
            eprintln!("输入无效: {}", msg);
        }
        Err(e) => {
            eprintln!("其他错误: {}", e);
        }
    }
}
```

### 访问令牌刷新错误

```rust
use botrs::{Token, HttpClient, BotError};

async fn handle_refresh_errors() {
    let token = Token::new("应用ID", "密钥");
    let http_client = HttpClient::new();
    
    match token.refresh_access_token(&http_client).await {
        Ok(_) => println!("令牌刷新成功"),
        Err(BotError::Authentication(_)) => {
            eprintln!("身份验证失败，检查应用ID和密钥");
        }
        Err(BotError::Network(_)) => {
            eprintln!("网络错误，稍后重试");
        }
        Err(BotError::RateLimited(retry_after)) => {
            eprintln!("速率限制，{}秒后重试", retry_after);
        }
        Err(e) => {
            eprintln!("刷新失败: {}", e);
        }
    }
}
```

## 最佳实践

### 安全存储

```rust
// 推荐：使用环境变量
std::env::set_var("QQ_BOT_APP_ID", "应用ID");
std::env::set_var("QQ_BOT_SECRET", "密钥");

// 推荐：使用配置文件（不提交到版本控制）
// config.toml (添加到 .gitignore)
// app_id = "应用ID"
// secret = "密钥"

// 不推荐：硬编码在源代码中
// let token = Token::new("hardcoded_id", "hardcoded_secret");
```

### 令牌验证

```rust
fn create_validated_token(app_id: &str, secret: &str) -> Result<Token, BotError> {
    let token = Token::new(app_id, secret);
    token.validate()?;
    Ok(token)
}
```

### 生产环境配置

```rust
use botrs::Token;

fn production_token_setup() -> Result<Token, Box<dyn std::error::Error>> {
    // 从环境变量或安全的配置服务加载
    let token = Token::from_env()
        .or_else(|_| load_from_secure_config())?;
    
    // 验证令牌
    token.validate()?;
    
    println!("令牌配置完成，应用ID: {}", token.app_id());
    Ok(token)
}

fn load_from_secure_config() -> Result<Token, BotError> {
    // 从安全配置服务、加密文件等加载
    todo!("实现安全配置加载")
}
```

### 开发环境助手

```rust
#[cfg(debug_assertions)]
fn development_token() -> Token {
    Token::new(
        std::env::var("DEV_APP_ID").unwrap_or_else(|_| "dev_app_id".to_string()),
        std::env::var("DEV_SECRET").unwrap_or_else(|_| "dev_secret".to_string())
    )
}

#[cfg(not(debug_assertions))]
fn development_token() -> Token {
    panic!("开发令牌仅在调试模式下可用");
}
```

## 线程安全

`Token` 是线程安全的，可以在多个线程间共享：

```rust
use std::sync::Arc;
use tokio::task;

async fn shared_token_usage() {
    let token = Arc::new(Token::new("应用ID", "密钥"));
    
    let handles: Vec<_> = (0..5).map(|i| {
        let token = token.clone();
        task::spawn(async move {
            println!("任务 {} 使用令牌: {}", i, token.app_id());
        })
    }).collect();
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

## 另请参阅

- [`Client`](./client.md) - 使用令牌创建客户端
- [`BotApi`](./bot-api.md) - API 请求中的令牌使用
- [配置指南](/zh/guide/configuration.md) - 令牌配置最佳实践
- [安全指南](/zh/guide/security.md) - 令牌安全存储和使用
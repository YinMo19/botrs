# 配置

BotRS 提供灵活的配置选项来自定义您的机器人行为、身份验证和运行时设置。本指南涵盖所有可用的配置方法和最佳实践。

## 身份验证配置

### 令牌设置

最重要的配置是您机器人的身份验证凭据：

```rust
use botrs::Token;

// 基本令牌创建
let token = Token::new("你的应用ID", "你的密钥");

// 使用前验证令牌
if let Err(e) = token.validate() {
    eprintln!("无效令牌: {}", e);
    std::process::exit(1);
}
```

### 环境变量

管理凭据的推荐方法是使用环境变量：

```bash
# 必需的凭据
export QQ_BOT_APP_ID="你的应用ID"
export QQ_BOT_SECRET="你的密钥"

# 可选设置
export QQ_BOT_SANDBOX="false"
export RUST_LOG="botrs=info,my_bot=debug"
```

在应用程序中加载它们：

```rust
use std::env;

let app_id = env::var("QQ_BOT_APP_ID")
    .expect("未设置 QQ_BOT_APP_ID 环境变量");
let secret = env::var("QQ_BOT_SECRET")
    .expect("未设置 QQ_BOT_SECRET 环境变量");

let token = Token::new(app_id, secret);
```

## Intent 配置

Intent 控制机器人接收哪些事件。根据机器人的功能配置它们：

### 基本 Intent

```rust
use botrs::Intents;

// 最小设置 - 仅频道消息
let intents = Intents::default()
    .with_public_guild_messages();

// 常见设置 - 消息和频道事件
let intents = Intents::default()
    .with_public_guild_messages()
    .with_guilds();

// 全功能机器人
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds()
    .with_guild_members()
    .with_guild_messages()
    .with_guild_message_reactions();
```

### 特权 Intent

某些 Intent 需要特殊权限：

```rust
// 这些可能需要 QQ 的批准
let privileged_intents = Intents::default()
    .with_guild_members()      // 成员信息
    .with_guild_presences()    // 在线状态更新
    .with_message_content();   // 完整消息内容访问
```

## 客户端配置

### 基本客户端设置

```rust
use botrs::{Client, Intents, Token};

let token = Token::new("应用ID", "密钥");
let intents = Intents::default().with_public_guild_messages();

// 使用沙盒模式创建客户端
let client = Client::new(token, intents, handler, true)?;  // true = 沙盒

// 为生产环境创建客户端
let client = Client::new(token, intents, handler, false)?; // false = 生产
```

### 高级配置

```rust
use botrs::{Client, ClientConfig, HttpConfig};

let http_config = HttpConfig::new()
    .timeout(std::time::Duration::from_secs(60))
    .user_agent("MyBot/1.0")
    .max_retries(3);

let client_config = ClientConfig::new()
    .http_config(http_config)
    .reconnect_attempts(5)
    .heartbeat_interval(std::time::Duration::from_secs(30));

// 注意：这是概念性的 - 实际实现可能有所不同
```

## 环境配置

### 开发环境

为开发创建 `.env` 文件：

```bash
# .env 文件
QQ_BOT_APP_ID=你的开发应用ID
QQ_BOT_SECRET=你的开发密钥
QQ_BOT_SANDBOX=true
RUST_LOG=botrs=debug,my_bot=trace
BOT_PREFIX=!
WELCOME_CHANNEL_ID=channel_123456
```

使用 `dotenvy` crate 加载：

```rust
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 如果存在则加载 .env 文件
    dotenv().ok();
    
    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID")?,
        std::env::var("QQ_BOT_SECRET")?,
    );
    
    // 从环境变量使用沙盒模式
    let use_sandbox = std::env::var("QQ_BOT_SANDBOX")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    
    // 机器人设置的其余部分...
    Ok(())
}
```

### 生产环境

对于生产环境，使用安全方法注入环境变量：

```bash
# Docker
docker run -e QQ_BOT_APP_ID=xxx -e QQ_BOT_SECRET=yyy my-bot

# Kubernetes
apiVersion: v1
kind: Secret
metadata:
  name: bot-secrets
data:
  app-id: <base64编码的应用ID>
  secret: <base64编码的密钥>
```

## 配置文件

### TOML 配置

创建结构化配置系统：

```toml
# config.toml
[bot]
app_id = "你的应用ID"
secret = "你的密钥"
sandbox = false
command_prefix = "!"

[features]
auto_reconnect = true
max_reconnect_attempts = 5
heartbeat_interval = 30

[channels]
welcome_channel = "channel_123456"
log_channel = "channel_789012"
admin_channel = "channel_345678"

[commands]
enabled = ["ping", "help", "info"]
admin_only = ["reload", "shutdown"]

[logging]
level = "info"
file_output = true
console_output = true
```

加载和使用配置：

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    bot: BotConfig,
    features: FeatureConfig,
    channels: ChannelConfig,
    commands: CommandConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct BotConfig {
    app_id: String,
    secret: String,
    sandbox: bool,
    command_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FeatureConfig {
    auto_reconnect: bool,
    max_reconnect_attempts: u32,
    heartbeat_interval: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChannelConfig {
    welcome_channel: Option<String>,
    log_channel: Option<String>,
    admin_channel: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CommandConfig {
    enabled: Vec<String>,
    admin_only: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingConfig {
    level: String,
    file_output: bool,
    console_output: bool,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("config.toml")?;
    let mut config: Config = toml::from_str(&config_content)?;
    
    // 用环境变量覆盖
    if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
        config.bot.app_id = app_id;
    }
    if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
        config.bot.secret = secret;
    }
    
    Ok(config)
}
```

### JSON 配置

或者，使用 JSON 进行配置：

```json
{
  "bot": {
    "app_id": "你的应用ID",
    "secret": "你的密钥",
    "sandbox": false,
    "command_prefix": "!"
  },
  "intents": {
    "public_guild_messages": true,
    "direct_message": true,
    "guilds": true,
    "guild_members": false
  },
  "features": {
    "auto_reconnect": true,
    "max_reconnect_attempts": 5
  }
}
```

## 日志配置

### 基本日志设置

```rust
use tracing_subscriber::{fmt, EnvFilter};

// 简单控制台日志
tracing_subscriber::fmt()
    .with_env_filter("botrs=info,my_bot=debug")
    .init();

// 更详细的配置
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_target(false)
    .with_thread_ids(true)
    .with_level(true)
    .with_file(true)
    .with_line_number(true)
    .init();
```

### 文件日志

```rust
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

let file_appender = rolling::daily("logs", "bot.log");
let (non_blocking, _guard) = non_blocking(file_appender);

tracing_subscriber::registry()
    .with(fmt::layer().with_writer(std::io::stdout))
    .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
    .with(EnvFilter::from_default_env())
    .init();
```

## 网络配置

### HTTP 客户端设置

```rust
use std::time::Duration;

// 自定义超时和用户代理
let http_client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .user_agent("MyBot/1.0.0")
    .build()?;

// 如果支持，与 BotApi 一起使用
// 注意：实际实现可能有所不同
```

### 代理配置

```rust
// 对于需要代理的环境
let proxy = reqwest::Proxy::http("http://proxy.example.com:8080")?;
let client = reqwest::Client::builder()
    .proxy(proxy)
    .build()?;
```

## 运行时配置

### 优雅关闭

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client().await?;
    
    // 处理关闭信号
    tokio::select! {
        result = client.start() => {
            if let Err(e) = result {
                eprintln!("客户端错误: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            println!("收到关闭信号");
            client.stop().await?;
        }
    }
    
    Ok(())
}
```

### 资源限制

```rust
// 为线程设置栈大小
std::thread::Builder::new()
    .stack_size(8 * 1024 * 1024)  // 8MB 栈
    .spawn(|| {
        // 重计算
    })?;

// 内存限制（概念性）
tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .max_blocking_threads(8)
    .enable_all()
    .build()?
    .block_on(async {
        // 您的机器人逻辑
    });
```

## 配置验证

### 输入验证

```rust
impl Config {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.bot.app_id.is_empty() {
            return Err(ConfigError::MissingAppId);
        }
        
        if self.bot.secret.is_empty() {
            return Err(ConfigError::MissingSecret);
        }
        
        if self.features.max_reconnect_attempts > 10 {
            return Err(ConfigError::InvalidReconnectAttempts);
        }
        
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error("需要应用 ID")]
    MissingAppId,
    #[error("需要密钥")]
    MissingSecret,
    #[error("最大重连尝试次数必须 <= 10")]
    InvalidReconnectAttempts,
}
```

### 环境检测

```rust
fn detect_environment() -> Environment {
    if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        Environment::Kubernetes
    } else if std::env::var("DYNO").is_ok() {
        Environment::Heroku
    } else if std::env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok() {
        Environment::Lambda
    } else {
        Environment::Local
    }
}

enum Environment {
    Local,
    Kubernetes,
    Heroku,
    Lambda,
}
```

## 最佳实践

### 安全性

1. **永远不要在源代码中硬编码凭据**
2. **使用环境变量** 处理敏感数据
3. **定期轮换凭据**
4. **使用最小权限** Intent
5. **验证配置文件的所有输入**

### 性能

1. **为您的用例配置适当的超时**
2. **为 HTTP 客户端使用连接池**
3. **设置合理的重试限制** 以避免无限循环
4. **在生产中监控资源使用情况**

### 可维护性

1. **使用结构化配置** 文件
2. **记录所有配置选项**
3. **提供合理的默认值**
4. **支持特定环境的覆盖**
5. **在启动时验证配置**

## 示例：完整配置设置

```rust
use botrs::{Client, EventHandler, Intents, Token};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    bot: BotSettings,
    intents: IntentSettings,
    logging: LoggingSettings,
}

#[derive(Debug, Deserialize, Serialize)]
struct BotSettings {
    app_id: String,
    secret: String,
    sandbox: bool,
    command_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct IntentSettings {
    public_guild_messages: bool,
    direct_message: bool,
    guilds: bool,
    guild_members: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingSettings {
    level: String,
    file_output: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bot: BotSettings {
                app_id: String::new(),
                secret: String::new(),
                sandbox: true,
                command_prefix: "!".to_string(),
            },
            intents: IntentSettings {
                public_guild_messages: true,
                direct_message: false,
                guilds: true,
                guild_members: false,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file_output: false,
            },
        }
    }
}

fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut config = if std::path::Path::new("config.toml").exists() {
        let content = fs::read_to_string("config.toml")?;
        toml::from_str(&content)?
    } else {
        AppConfig::default()
    };

    // 用环境变量覆盖
    if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
        config.bot.app_id = app_id;
    }
    if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
        config.bot.secret = secret;
    }

    // 验证
    if config.bot.app_id.is_empty() {
        return Err("需要应用 ID".into());
    }
    if config.bot.secret.is_empty() {
        return Err("需要密钥".into());
    }

    Ok(config)
}

fn setup_logging(config: &LoggingSettings) -> Result<(), Box<dyn std::error::Error>> {
    let filter = format!("botrs={},my_bot={}", config.level, config.level);
    
    if config.file_output {
        // 设置文件日志
        let file_appender = tracing_appender::rolling::daily("logs", "bot.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(non_blocking)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .init();
    }
    
    Ok(())
}

fn build_intents(config: &IntentSettings) -> Intents {
    let mut intents = Intents::default();
    
    if config.public_guild_messages {
        intents = intents.with_public_guild_messages();
    }
    if config.direct_message {
        intents = intents.with_direct_message();
    }
    if config.guilds {
        intents = intents.with_guilds();
    }
    if config.guild_members {
        intents = intents.with_guild_members();
    }
    
    intents
}

struct MyBot {
    config: AppConfig,
}

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: botrs::Context, ready: botrs::Ready) {
        info!("机器人就绪，配置: sandbox={}", self.config.bot.sandbox);
    }
    
    // 其他事件处理器...
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    setup_logging(&config.logging)?;
    
    info!("使用配置启动机器人");
    
    let token = Token::new(&config.bot.app_id, &config.bot.secret);
    let intents = build_intents(&config.intents);
    let handler = MyBot { config: config.clone() };
    
    let mut client = Client::new(token, intents, handler, config.bot.sandbox)?;
    client.start().await?;
    
    Ok(())
}
```

这个全面的配置设置为您的 BotRS 应用程序提供了灵活性、安全性和可维护性。
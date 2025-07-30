# Configuration

BotRS provides flexible configuration options to customize your bot's behavior, authentication, and runtime settings. This guide covers all available configuration methods and best practices.

## Authentication Configuration

### Token Setup

The most important configuration is your bot's authentication credentials:

```rust
use botrs::Token;

// Basic token creation
let token = Token::new("your_app_id", "your_secret");

// Validate token before use
if let Err(e) = token.validate() {
    eprintln!("Invalid token: {}", e);
    std::process::exit(1);
}
```

### Environment Variables

The recommended approach for managing credentials is using environment variables:

```bash
# Required credentials
export QQ_BOT_APP_ID="your_app_id_here"
export QQ_BOT_SECRET="your_secret_here"

# Optional settings
export QQ_BOT_SANDBOX="false"
export RUST_LOG="botrs=info,my_bot=debug"
```

Load them in your application:

```rust
use std::env;

let app_id = env::var("QQ_BOT_APP_ID")
    .expect("QQ_BOT_APP_ID environment variable not set");
let secret = env::var("QQ_BOT_SECRET")
    .expect("QQ_BOT_SECRET environment variable not set");

let token = Token::new(app_id, secret);
```

## Intent Configuration

Intents control which events your bot receives. Configure them based on your bot's functionality:

### Basic Intents

```rust
use botrs::Intents;

// Minimal setup - only guild messages
let intents = Intents::default()
    .with_public_guild_messages();

// Common setup - messages and guild events
let intents = Intents::default()
    .with_public_guild_messages()
    .with_guilds();

// Full feature bot
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds()
    .with_guild_members()
    .with_guild_messages()
    .with_guild_message_reactions();
```

### Privileged Intents

Some intents require special permissions:

```rust
// These may require approval from QQ
let privileged_intents = Intents::default()
    .with_guild_members()      // Member information
    .with_guild_presences()    // Presence updates
    .with_message_content();   // Full message content access
```

## Client Configuration

### Basic Client Setup

```rust
use botrs::{Client, Intents, Token};

let token = Token::new("app_id", "secret");
let intents = Intents::default().with_public_guild_messages();

// Create client with sandbox mode
let client = Client::new(token, intents, handler, true)?;  // true = sandbox

// Create client for production
let client = Client::new(token, intents, handler, false)?; // false = production
```

### Advanced Configuration

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

// Note: This is conceptual - actual implementation may vary
```

## Environment Configuration

### Development Environment

Create a `.env` file for development:

```bash
# .env file
QQ_BOT_APP_ID=your_development_app_id
QQ_BOT_SECRET=your_development_secret
QQ_BOT_SANDBOX=true
RUST_LOG=botrs=debug,my_bot=trace
BOT_PREFIX=!
WELCOME_CHANNEL_ID=channel_123456
```

Load using the `dotenvy` crate:

```rust
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenv().ok();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID")?,
        std::env::var("QQ_BOT_SECRET")?,
    );

    // Use sandbox mode from environment
    let use_sandbox = std::env::var("QQ_BOT_SANDBOX")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Rest of your bot setup...
    Ok(())
}
```

### Production Environment

For production, use secure methods to inject environment variables:

```bash
# Docker
docker run -e QQ_BOT_APP_ID=xxx -e QQ_BOT_SECRET=yyy my-bot

# Kubernetes
apiVersion: v1
kind: Secret
metadata:
  name: bot-secrets
data:
  app-id: <base64-encoded-app-id>
  secret: <base64-encoded-secret>
```

## Configuration Files

### TOML Configuration

Create a structured configuration system:

```toml
# config.toml
[bot]
app_id = "your_app_id"
secret = "your_secret"
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

Load and use the configuration:

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

    // Override with environment variables
    if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
        config.bot.app_id = app_id;
    }
    if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
        config.bot.secret = secret;
    }

    Ok(config)
}
```

### JSON Configuration

Alternatively, use JSON for configuration:

```json
{
  "bot": {
    "app_id": "your_app_id",
    "secret": "your_secret",
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

## Logging Configuration

### Basic Logging Setup

```rust
use tracing_subscriber::{fmt, EnvFilter};

// Simple console logging
tracing_subscriber::fmt()
    .with_env_filter("botrs=info,my_bot=debug")
    .init();

// More detailed configuration
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_target(false)
    .with_thread_ids(true)
    .with_level(true)
    .with_file(true)
    .with_line_number(true)
    .init();
```

### File Logging

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

## Network Configuration

### HTTP Client Settings

```rust
use std::time::Duration;

// Custom timeout and user agent
let http_client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .user_agent("MyBot/1.0.0")
    .build()?;

// Use with BotApi if supported
// Note: Actual implementation may vary
```

### Proxy Configuration

```rust
// For environments requiring proxy
let proxy = reqwest::Proxy::http("http://proxy.example.com:8080")?;
let client = reqwest::Client::builder()
    .proxy(proxy)
    .build()?;
```

## Runtime Configuration

### Graceful Shutdown

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client().await?;

    // Handle shutdown signals
    tokio::select! {
        result = client.start() => {
            if let Err(e) = result {
                eprintln!("Client error: {}", e);
            }
        }
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal");
            client.stop().await?;
        }
    }

    Ok(())
}
```

### Resource Limits

```rust
// Set stack size for threads
std::thread::Builder::new()
    .stack_size(8 * 1024 * 1024)  // 8MB stack
    .spawn(|| {
        // Heavy computation
    })?;

// Memory limits (conceptual)
tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .max_blocking_threads(8)
    .enable_all()
    .build()?
    .block_on(async {
        // Your bot logic
    });
```

## Configuration Validation

### Input Validation

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
    #[error("App ID is required")]
    MissingAppId,
    #[error("Secret is required")]
    MissingSecret,
    #[error("Max reconnect attempts must be <= 10")]
    InvalidReconnectAttempts,
}
```

### Environment Detection

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

## Best Practices

### Security

1. **Never hardcode credentials** in source code
2. **Use environment variables** for sensitive data
3. **Rotate credentials** regularly
4. **Use least privilege** intents
5. **Validate all input** from configuration files

### Performance

1. **Configure appropriate timeouts** for your use case
2. **Use connection pooling** for HTTP clients
3. **Set reasonable retry limits** to avoid infinite loops
4. **Monitor resource usage** in production

### Maintainability

1. **Use structured configuration** files
2. **Document all configuration options**
3. **Provide sensible defaults**
4. **Support environment-specific overrides**
5. **Validate configuration** at startup

## Example: Complete Configuration Setup

```rust
use botrs::{Client, EventHandler, Intents, Token};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AppConfig {
    bot: BotSettings,
    intents: IntentSettings,
    logging: LoggingSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct BotSettings {
    app_id: String,
    secret: String,
    sandbox: bool,
    command_prefix: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct IntentSettings {
    public_guild_messages: bool,
    direct_message: bool,
    guilds: bool,
    guild_members: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    // Override with environment variables
    if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
        config.bot.app_id = app_id;
    }
    if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
        config.bot.secret = secret;
    }

    // Validate
    if config.bot.app_id.is_empty() {
        return Err("App ID is required".into());
    }
    if config.bot.secret.is_empty() {
        return Err("Secret is required".into());
    }

    Ok(config)
}

fn setup_logging(config: &LoggingSettings) -> Result<(), Box<dyn std::error::Error>> {
    let filter = format!("botrs={},my_bot={}", config.level, config.level);

    if config.file_output {
        // Setup file logging
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
        info!("Bot ready with config: sandbox={}", self.config.bot.sandbox);
    }

    // Other event handlers...
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    setup_logging(&config.logging)?;

    info!("Starting bot with configuration");

    let token = Token::new(&config.bot.app_id, &config.bot.secret);
    let intents = build_intents(&config.intents);
    let handler = MyBot { config: config.clone() };

    let mut client = Client::new(token, intents, handler, config.bot.sandbox)?;
    client.start().await?;

    Ok(())
}
```

This comprehensive configuration setup provides flexibility, security, and maintainability for your BotRS applications.

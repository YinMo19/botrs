# 入门示例

本页面提供实用示例，帮助您开始使用 BotRS。每个示例都建立在前一个示例的基础上，演示核心概念和常见模式。

在源代码仓库中已经有 [大量的 demo](https://github.com/YinMo19/botrs/tree/main/examples)，大概二十多个 demo，覆盖了所有常用场景。下面的文档只是一些补充说明，可能含有错误，只是提供一些 hint，请不要直接复制到地方运行，可能有编译错误。

## 基础回声机器人

一个简单的机器人，当被提及时会回显消息。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

struct EchoBot;

#[async_trait::async_trait]
impl EventHandler for EchoBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("回声机器人已就绪！登录为：{}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // 带前缀回显消息
            let echo_response = format!("回声：{}", content);

            match message.reply(&ctx.api, &ctx.token, &echo_response).await {
                Ok(_) => info!("回显消息：{}", content),
                Err(e) => warn!("回显消息失败：{}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,echo_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("未设置 QQ_BOT_APP_ID"),
        std::env::var("QQ_BOT_SECRET").expect("未设置 QQ_BOT_SECRET"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, EchoBot, false)?;

    client.start().await?;
    Ok(())
}
```

## 命令处理器机器人

一个更复杂的机器人，处理多个命令并给出不同的响应。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

struct CommandBot;

impl CommandBot {
    fn handle_command(&self, command: &str, args: &[&str]) -> Option<String> {
        match command {
            "ping" => Some("Pong! 🏓".to_string()),
            "hello" => Some("你好！👋".to_string()),
            "time" => {
                let now = chrono::Utc::now();
                Some(format!("当前时间：{}", now.format("%Y-%m-%d %H:%M:%S UTC")))
            }
            "echo" => {
                if args.is_empty() {
                    Some("用法：!echo <消息>".to_string())
                } else {
                    Some(args.join(" "))
                }
            }
            "help" => Some(
                "可用命令：\n\
                • !ping - 测试机器人响应\n\
                • !hello - 获取问候\n\
                • !time - 获取当前时间\n\
                • !echo <消息> - 回显消息\n\
                • !help - 显示此帮助消息"
                    .to_string(),
            ),
            _ => Some(format!("未知命令：{}。输入 !help 查看可用命令。", command)),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("命令机器人已就绪！登录为：{}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            let content = content.trim();

            // 检查消息是否以命令前缀开始
            if let Some(command_text) = content.strip_prefix('!') {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if parts.is_empty() {
                    return;
                }

                let command = parts[0];
                let args = &parts[1..];

                info!("处理命令：{} 参数：{:?}", command, args);

                if let Some(response) = self.handle_command(command, args) {
                    match message.reply(&ctx.api, &ctx.token, &response).await {
                        Ok(_) => info!("命令 {} 执行成功", command),
                        Err(e) => warn!("响应命令 {} 失败：{}", command, e),
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,command_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("未设置 QQ_BOT_APP_ID"),
        std::env::var("QQ_BOT_SECRET").expect("未设置 QQ_BOT_SECRET"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, CommandBot, false)?;

    client.start().await?;
    Ok(())
}
```

## 多事件机器人

处理多种类型事件（包括频道和成员事件）的机器人。

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token,
    Guild, Channel, Member, GroupMessage, DirectMessage
};
use tracing::{info, warn};

struct MultiEventBot;

#[async_trait::async_trait]
impl EventHandler for MultiEventBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("多事件机器人已就绪！");
        info!("机器人用户：{}", ready.user.username);
        info!("连接到 {} 个频道", ready.guilds.len());
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if content == "!serverinfo" {
                let response = format!(
                    "在频道中收到消息：{}",
                    message.channel_id.as_deref().unwrap_or("未知")
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
        }
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            if content == "!groupinfo" {
                let response = format!(
                    "群消息来自：{}",
                    message.group_openid.as_deref().unwrap_or("未知群")
                );
                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
            }
        }
    }

    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            let response = format!("您说：{}", content);
            let _ = message.reply(&ctx.api, &ctx.token, &response).await;
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        info!(
            "加入频道：{}（ID：{}）",
            guild.name.as_deref().unwrap_or("未知"),
            guild.id.as_deref().unwrap_or("未知")
        );
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        if let Some(user) = &member.user {
            info!(
                "新成员加入：{}",
                user.username.as_deref().unwrap_or("未知")
            );

            // 您可以在此发送欢迎消息
            // 注意：您需要知道欢迎频道 ID
            // let welcome_msg = format!("欢迎来到服务器，{}！",
            //                          user.username.as_deref().unwrap_or("朋友"));
        }
    }

    async fn channel_create(&self, _ctx: Context, channel: Channel) {
        info!(
            "创建新频道：{}（类型：{:?}）",
            channel.name.as_deref().unwrap_or("未命名"),
            channel.type_
        );
    }

    async fn error(&self, error: botrs::BotError) {
        warn!("机器人发生错误：{}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,multi_event_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("未设置 QQ_BOT_APP_ID"),
        std::env::var("QQ_BOT_SECRET").expect("未设置 QQ_BOT_SECRET"),
    );

    // 订阅多种事件类型
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds()
        .with_guild_members();

    let mut client = Client::new(token, intents, MultiEventBot, false)?;

    info!("启动多事件机器人...");
    client.start().await?;
    Ok(())
}
```

## 带数据存储的状态机器人

维护状态并跟踪用户交互的机器人。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct UserStats {
    message_count: u64,
    last_message: chrono::DateTime<chrono::Utc>,
    first_seen: chrono::DateTime<chrono::Utc>,
}

struct StatefulBot {
    user_stats: Arc<RwLock<HashMap<String, UserStats>>>,
}

impl StatefulBot {
    fn new() -> Self {
        Self {
            user_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn update_user_stats(&self, user_id: &str) {
        let mut stats = self.user_stats.write().await;
        let now = chrono::Utc::now();

        match stats.get_mut(user_id) {
            Some(user_stat) => {
                user_stat.message_count += 1;
                user_stat.last_message = now;
            }
            None => {
                stats.insert(
                    user_id.to_string(),
                    UserStats {
                        message_count: 1,
                        last_message: now,
                        first_seen: now,
                    },
                );
            }
        }
    }

    async fn get_user_stats(&self, user_id: &str) -> Option<UserStats> {
        let stats = self.user_stats.read().await;
        stats.get(user_id).cloned()
    }
}

#[async_trait::async_trait]
impl EventHandler for StatefulBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("状态机器人已就绪！登录为：{}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // 更新用户统计
        if let Some(author) = &message.author {
            if let Some(user_id) = &author.id {
                self.update_user_stats(user_id).await;

                if let Some(content) = &message.content {
                    match content.trim() {
                        "!stats" => {
                            if let Some(stats) = self.get_user_stats(user_id).await {
                                let response = format!(
                                    "您的统计信息：\n\
                                    • 发送消息数：{}\n\
                                    • 首次见面：{}\n\
                                    • 最后消息：{}",
                                    stats.message_count,
                                    stats.first_seen.format("%Y-%m-%d %H:%M:%S UTC"),
                                    stats.last_message.format("%Y-%m-%d %H:%M:%S UTC")
                                );
                                let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                            }
                        }
                        "!leaderboard" => {
                            let stats = self.user_stats.read().await;
                            let mut sorted_users: Vec<_> = stats.iter().collect();
                            sorted_users.sort_by(|a, b| b.1.message_count.cmp(&a.1.message_count));

                            let mut response = "消息排行榜：\n".to_string();
                            for (i, (user_id, user_stats)) in sorted_users.iter().take(5).enumerate() {
                                response.push_str(&format!(
                                    "{}. 用户 {}：{} 条消息\n",
                                    i + 1,
                                    &user_id[..8], // 显示用户 ID 的前 8 个字符
                                    user_stats.message_count
                                ));
                            }

                            let _ = message.reply(&ctx.api, &ctx.token, &response).await;
                        }
                        "!reset" => {
                            self.user_stats.write().await.remove(user_id);
                            let _ = message.reply(&ctx.api, &ctx.token, "您的统计信息已重置！").await;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,stateful_bot=info")
        .init();

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID").expect("未设置 QQ_BOT_APP_ID"),
        std::env::var("QQ_BOT_SECRET").expect("未设置 QQ_BOT_SECRET"),
    );

    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, StatefulBot::new(), false)?;

    info!("启动状态机器人...");
    client.start().await?;
    Ok(())
}
```

## 基于配置的机器人

从文件和环境变量加载配置的机器人。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct BotConfig {
    bot: BotSettings,
    commands: CommandSettings,
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
struct CommandSettings {
    enabled: Vec<String>,
    admin_only: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingSettings {
    level: String,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            bot: BotSettings {
                app_id: "你的应用ID".to_string(),
                secret: "你的密钥".to_string(),
                sandbox: false,
                command_prefix: "!".to_string(),
            },
            commands: CommandSettings {
                enabled: vec!["ping".to_string(), "help".to_string()],
                admin_only: vec!["reload".to_string()],
            },
            logging: LoggingSettings {
                level: "info".to_string(),
            },
        }
    }
}

struct ConfigurableBot {
    config: BotConfig,
}

impl ConfigurableBot {
    fn new(config: BotConfig) -> Self {
        Self { config }
    }

    fn is_command_enabled(&self, command: &str) -> bool {
        self.config.commands.enabled.contains(&command.to_string())
    }

    fn handle_command(&self, command: &str, _args: &[&str]) -> Option<String> {
        if !self.is_command_enabled(command) {
            return Some("此命令已禁用。".to_string());
        }

        match command {
            "ping" => Some("Pong!".to_string()),
            "help" => {
                let enabled_commands = self.config.commands.enabled.join(", ");
                Some(format!("可用命令：{}", enabled_commands))
            }
            "config" => Some(format!(
                "机器人配置：\n\
                • 命令前缀：{}\n\
                • 沙盒模式：{}\n\
                • 启用的命令：{}",
                self.config.bot.command_prefix,
                self.config.bot.sandbox,
                self.config.commands.enabled.join(", ")
            )),
            _ => Some("未知命令。".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for ConfigurableBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("可配置机器人已就绪！登录为：{}", ready.user.username);
        info!("使用命令前缀：{}", self.config.bot.command_prefix);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            let content = content.trim();

            if let Some(command_text) = content.strip_prefix(&self.config.bot.command_prefix) {
                let parts: Vec<&str> = command_text.split_whitespace().collect();
                if parts.is_empty() {
                    return;
                }

                let command = parts[0];
                let args = &parts[1..];

                if let Some(response) = self.handle_command(command, args) {
                    match message.reply(&ctx.api, &ctx.token, &response).await {
                        Ok(_) => info!("响应命令：{}", command),
                        Err(e) => warn!("响应命令 {} 失败：{}", command, e),
                    }
                }
            }
        }
    }
}

fn load_config() -> Result<BotConfig, Box<dyn std::error::Error>> {
    // 首先尝试从文件加载
    if let Ok(config_content) = fs::read_to_string("config.toml") {
        let mut config: BotConfig = toml::from_str(&config_content)?;

        // 如果存在，用环境变量覆盖
        if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
            config.bot.app_id = app_id;
        }
        if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
            config.bot.secret = secret;
        }

        Ok(config)
    } else {
        // 创建默认配置并保存
        let default_config = BotConfig::default();
        let config_content = toml::to_string_pretty(&default_config)?;
        fs::write("config.toml", config_content)?;

        info!("已创建默认 config.toml - 请使用您的机器人凭据更新它");

        // 仍然尝试使用环境变量
        let mut config = default_config;
        if let Ok(app_id) = std::env::var("QQ_BOT_APP_ID") {
            config.bot.app_id = app_id;
        }
        if let Ok(secret) = std::env::var("QQ_BOT_SECRET") {
            config.bot.secret = secret;
        }

        Ok(config)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    tracing_subscriber::fmt()
        .with_env_filter(format!("botrs={},configurable_bot={}", config.logging.level, config.logging.level))
        .init();

    let token = Token::new(config.bot.app_id.clone(), config.bot.secret.clone());
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, ConfigurableBot::new(config), false)?;

    info!("启动可配置机器人...");
    client.start().await?;
    Ok(())
}
```

## 环境设置

对于上述所有示例，您需要设置环境：

### 环境变量

```bash
export QQ_BOT_APP_ID="你的应用ID"
export QQ_BOT_SECRET="你的密钥"
export RUST_LOG="botrs=info"
```

### 依赖项

将这些添加到您的 `Cargo.toml`：

```toml
[dependencies]
botrs = "0.2.5"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"

# 用于配置示例
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# 用于状态示例
chrono = { version = "0.4", features = ["serde"] }
```

### 运行示例

1. 设置环境变量
2. 将示例代码复制到 `src/main.rs`
3. 使用 `cargo run` 运行

## 下一步

这些示例演示了使用 BotRS 构建 QQ 频道机器人的核心模式。要了解更多：

- [富文本消息](./rich-messages.md) - 发送嵌入内容、文件和交互式内容
- [错误处理](./error-recovery.md) - 构建健壮的生产就绪机器人
- [API 集成](./api-integration.md) - 使用完整的 QQ 频道 API
- [事件处理](./event-handling.md) - 处理所有类型的频道事件

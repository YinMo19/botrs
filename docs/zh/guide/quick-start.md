# 快速开始

本指南将帮助您在几分钟内使用 BotRS 创建您的第一个 QQ 频道机器人。在本教程结束时，您将拥有一个可以响应消息的工作机器人。

## 步骤 1：设置项目

首先，创建一个新的 Rust 项目并添加必要的依赖项：

```bash
cargo new my-first-bot
cd my-first-bot
```

编辑您的 `Cargo.toml` 以包含 BotRS 及其依赖项：

```toml
[package]
name = "my-first-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
botrs = "0.6.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"
```

## 步骤 2：获取机器人凭据

在编写代码之前，您需要从 QQ 频道开发者门户获取凭据：

1. 访问 [QQ 频道开发者门户](https://bot.q.qq.com/)
2. 创建新应用程序或选择现有应用程序
3. 复制您的**应用 ID** 和**密钥**

对于本教程，您可以将它们设置为环境变量：

```bash
export QQ_BOT_APP_ID="你的应用ID"
export QQ_BOT_SECRET="你的密钥"
```

## 步骤 3：编写您的第一个机器人

将 `src/main.rs` 的内容替换为以下代码：

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tracing::{info, warn};

// 定义机器人的事件处理器
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 当机器人成功连接时调用
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("🤖 机器人已就绪！登录为：{}", ready.user.username);
    }

    // 当有人在消息中提及您的机器人时调用
    async fn message_create(&self, ctx: Context, message: Message) {
        // 忽略来自其他机器人的消息
        if message.is_from_bot() {
            return;
        }

        // 获取消息内容
        let content = match &message.content {
            Some(content) => content,
            None => return,
        };

        info!("📨 收到消息：{}", content);

        // 响应不同的命令
        let response = match content.trim() {
            "!ping" => "🏓 Pong!",
            "!hello" => "👋 你好！",
            "!help" => "🤖 可用命令：!ping, !hello, !help, !about",
            "!about" => "🦀 我是用 BotRS 构建的 QQ 机器人 - 一个用于 QQ 频道机器人的 Rust 框架！",
            _ => return, // 不回应其他消息
        };

        // 发送回复
        match message.reply(&ctx.api, &ctx.token, response).await {
            Ok(_) => info!("✅ 回复发送成功"),
            Err(e) => warn!("❌ 发送回复失败：{}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志记录
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,my_first_bot=info")
        .init();

    info!("🚀 启动机器人...");

    // 从环境变量获取凭据
    let app_id = std::env::var("QQ_BOT_APP_ID")
        .expect("未设置 QQ_BOT_APP_ID 环境变量");
    let secret = std::env::var("QQ_BOT_SECRET")
        .expect("未设置 QQ_BOT_SECRET 环境变量");

    // 创建身份验证令牌
    let token = Token::new(app_id, secret);

    // 配置机器人想要接收的事件
    let intents = Intents::default()
        .with_public_guild_messages()  // 接收 @ 提及
        .with_guilds();                // 接收频道事件

    // 创建机器人客户端
    let mut client = Client::new(token, intents, MyBot, true)?;

    info!("🔌 连接到 QQ 频道...");

    // 启动机器人（这将运行直到程序停止）
    client.start().await?;

    Ok(())
}
```

## 步骤 4：运行机器人

现在运行您的机器人：

```bash
cargo run
```

您应该看到类似以下的输出：

```
2024-01-01T12:00:00.000Z  INFO my_first_bot: 🚀 启动机器人...
2024-01-01T12:00:00.100Z  INFO my_first_bot: 🔌 连接到 QQ 频道...
2024-01-01T12:00:01.200Z  INFO my_first_bot: 🤖 机器人已就绪！登录为：MyBot
```

## 步骤 5：测试机器人

1. 将机器人添加到 QQ 频道（服务器）
2. 在机器人有权限的频道中，尝试这些命令：
   - `@您的机器人 !ping` - 机器人应该回复 "🏓 Pong!"
   - `@您的机器人 !hello` - 机器人应该回复 "👋 你好！"
   - `@您的机器人 !help` - 机器人应该显示可用命令

## 理解代码

让我们分解机器人中发生的事情：

### 事件处理器
```rust
struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    // 您的事件处理方法放在这里
}
```

`EventHandler` trait 定义了机器人如何响应不同事件。您只需要实现您关心的事件。

### Ready 事件
```rust
async fn ready(&self, _ctx: Context, ready: Ready) {
    info!("🤖 机器人已就绪！登录为：{}", ready.user.username);
}
```

当机器人成功连接并准备接收事件时调用一次。

### 消息事件
```rust
async fn message_create(&self, ctx: Context, message: Message) {
    // 处理传入的消息
}
```

当有人在消息中提及您的机器人时调用。`ctx` 参数提供对 API 客户端和身份验证令牌的访问。

### Intent
```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_guilds();
```

Intent 控制机器人接收哪些事件。这有助于通过只订阅您需要的事件来优化性能。

## 下一步

恭喜！您已经使用 BotRS 创建了您的第一个 QQ 频道机器人。以下是扩展机器人的一些想法：

### 添加更多命令
```rust
let response = match content.trim() {
    "!ping" => "🏓 Pong!",
    "!time" => &format!("⏰ 当前时间：{}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")),
    "!random" => &format!("🎲 随机数：{}", rand::random::<u32>() % 100),
    // ... 更多命令
};
```

### 处理不同消息类型
```rust
// 处理群消息
async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
    // 处理群聊消息
}

// 处理私信
async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
    // 处理私人消息
}
```

### 发送富文本消息
```rust
use botrs::models::message::{MessageParams, MessageEmbed};

let embed = MessageEmbed {
    title: Some("来自 BotRS 的问候！".to_string()),
    description: Some("这是一个富文本嵌入消息".to_string()),
    color: Some(0x00ff00),
    ..Default::default()
};

let params = MessageParams::new_embed(embed);
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
```

## 故障排除

### 常见问题

**机器人不响应消息：**
- 确保机器人在频道中有适当的权限
- 验证您是否在提及机器人（@机器人名称 命令）
- 检查 `public_guild_messages` intent 是否已启用

**身份验证错误：**
- 仔细检查您的应用 ID 和密钥
- 确保环境变量设置正确
- 验证机器人在 QQ 频道开发者门户中正确配置

**连接问题：**
- 检查您的网络连接
- 验证 QQ 频道服务是否正常运行
- 查找防火墙或代理问题

### 获取帮助

如果遇到问题：

1. 查看[示例](/zh/examples/getting-started)获取更多代码示例
2. 阅读 [API 参考](/zh/api/client)获取详细文档
3. 访问 [GitHub 仓库](https://github.com/YinMo19/botrs)获取问题和讨论

## 接下来做什么？

现在您有了一个基本的机器人运行，探索这些指南以了解更多：

- **[配置](/zh/guide/configuration)** - 了解高级配置选项
- **[消息与回复](/zh/guide/messages)** - 发现发送消息的所有方式
- **[错误处理](/zh/guide/error-handling)** - 构建健壮的生产就绪机器人
- **[示例](/zh/examples/getting-started)** - 查看更复杂的机器人实现

愉快的机器人构建！🤖✨
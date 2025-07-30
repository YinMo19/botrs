# 文本消息示例

本指南演示如何使用 BotRS 处理和发送文本消息。文本消息是 QQ 频道机器人中最常见的交互类型，支持从简单回复到复杂命令系统的各种场景。

## 基础文本处理

### 简单回声机器人

最基本的文本消息处理涉及将用户输入回显给他们。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct EchoHandler;

#[async_trait::async_trait]
impl EventHandler for EchoHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        // 忽略机器人消息以防止循环
        if message.is_from_bot() {
            return;
        }

        // 检查消息是否有内容
        if let Some(content) = &message.content {
            // 简单回声 - 用相同内容回复
            if let Err(e) = message.reply(&ctx.api, &ctx.token, content).await {
                eprintln!("发送回复失败: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = EchoHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### @ 提及回复

专门回复提及机器人的消息。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MentionHandler;

#[async_trait::async_trait]
impl EventHandler for MentionHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人 {} 已准备好接收提及！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // 获取机器人名称以提供个性化响应
            let bot_name = ctx
                .bot_info
                .as_ref()
                .map(|info| info.username.as_str())
                .unwrap_or("机器人");

            let reply_content = format!(
                "你好！机器人 {} 收到了你的提及：{}",
                bot_name, content
            );

            if let Err(e) = message.reply(&ctx.api, &ctx.token, &reply_content).await {
                eprintln!("回复提及失败: {}", e);
            }
        }
    }
}
```

## 命令系统

### 简单命令处理器

构建一个响应带前缀消息的基本命令系统。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;

type CommandHandler = fn(&str) -> String;

struct CommandBot {
    commands: HashMap<String, CommandHandler>,
}

impl CommandBot {
    fn new() -> Self {
        let mut commands = HashMap::new();
        
        // 注册命令
        commands.insert("ping".to_string(), ping_command as CommandHandler);
        commands.insert("hello".to_string(), hello_command as CommandHandler);
        commands.insert("echo".to_string(), echo_command as CommandHandler);
        commands.insert("help".to_string(), help_command as CommandHandler);
        
        Self { commands }
    }

    fn handle_command(&self, content: &str) -> Option<String> {
        // 命令以 ! 开头
        if !content.starts_with('!') {
            return None;
        }

        let content = &content[1..]; // 移除 !
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).unwrap_or(&"");

        self.commands.get(command).map(|handler| handler(args))
    }
}

// 命令实现
fn ping_command(_args: &str) -> String {
    "Pong!".to_string()
}

fn hello_command(args: &str) -> String {
    if args.is_empty() {
        "你好！".to_string()
    } else {
        format!("你好，{}！", args)
    }
}

fn echo_command(args: &str) -> String {
    if args.is_empty() {
        "没有内容可回显！".to_string()
    } else {
        format!("回显：{}", args)
    }
}

fn help_command(_args: &str) -> String {
    "可用命令：\n!ping - 回复 Pong!\n!hello [名字] - 向你问好\n!echo <文本> - 回显你的文本\n!help - 显示此帮助".to_string()
}

#[async_trait::async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("命令机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if let Some(response) = self.handle_command(content) {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("发送命令响应失败: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = CommandBot::new();
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### 带别名的高级命令系统

更复杂的命令处理，支持命令别名和参数解析。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct CommandRegistry {
    commands: Vec<(Vec<String>, fn(&str) -> Option<String>)>,
}

impl CommandRegistry {
    fn new() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
        };

        // 注册带别名的命令
        registry.register(vec!["hello", "hi", "hey", "你好"], hello_handler);
        registry.register(vec!["goodbye", "bye", "cya", "再见"], goodbye_handler);
        registry.register(vec!["time", "clock", "时间"], time_handler);
        registry.register(vec!["random", "rand", "随机"], random_handler);

        registry
    }

    fn register(&mut self, aliases: Vec<&str>, handler: fn(&str) -> Option<String>) {
        let aliases: Vec<String> = aliases.iter().map(|s| s.to_string()).collect();
        self.commands.push((aliases, handler));
    }

    fn execute(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();
        
        for (aliases, handler) in &self.commands {
            for alias in aliases {
                if trimmed.starts_with(alias) {
                    let params = if trimmed.len() > alias.len() {
                        trimmed[alias.len()..].trim()
                    } else {
                        ""
                    };
                    return handler(params);
                }
            }
        }
        
        None
    }
}

// 命令处理器
fn hello_handler(params: &str) -> Option<String> {
    Some(if params.is_empty() {
        "你好！今天过得怎么样？".to_string()
    } else {
        format!("你好，{}！很高兴认识你！", params)
    })
}

fn goodbye_handler(params: &str) -> Option<String> {
    Some(if params.is_empty() {
        "再见！祝你有美好的一天！".to_string()
    } else {
        format!("再见，{}！回头见！", params)
    })
}

fn time_handler(_params: &str) -> Option<String> {
    use chrono::Utc;
    let now = Utc::now();
    Some(format!("当前 UTC 时间：{}", now.format("%Y-%m-%d %H:%M:%S")))
}

fn random_handler(params: &str) -> Option<String> {
    use rand::Rng;
    
    if params.is_empty() {
        let num = rand::thread_rng().gen_range(1..=100);
        Some(format!("随机数字：{}", num))
    } else if let Ok(max) = params.parse::<u32>() {
        let num = rand::thread_rng().gen_range(1..=max);
        Some(format!("随机数字 (1-{})：{}", max, num))
    } else {
        Some("无效的数字格式。用法：random [最大数字]".to_string())
    }
}

struct AdvancedCommandHandler {
    registry: CommandRegistry,
}

impl AdvancedCommandHandler {
    fn new() -> Self {
        Self {
            registry: CommandRegistry::new(),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for AdvancedCommandHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("高级命令机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            if let Some(response) = self.registry.execute(content) {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                    eprintln!("发送响应失败: {}", e);
                }
            }
        }
    }
}
```

## 不同消息类型

### 群消息

处理 QQ 群中的文本消息。

```rust
use botrs::{Client, Context, EventHandler, GroupMessage, Intents, Ready, Token};

struct GroupTextHandler;

#[async_trait::async_trait]
impl EventHandler for GroupTextHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("群机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            println!("收到群消息: {}", content);

            // 处理特定的群命令
            let response = if content.contains("你好") || content.contains("hello") {
                Some("大家好！")
            } else if content.contains("帮助") || content.contains("help") {
                Some("群命令：你好、帮助、信息")
            } else if content.contains("信息") || content.contains("info") {
                Some("这是一个使用 BotRS 构建的 QQ 群机器人")
            } else {
                None
            };

            if let Some(reply_text) = response {
                // 使用便捷的回复方法
                if let Err(e) = message.reply(&ctx.api, &ctx.token, reply_text).await {
                    eprintln!("回复群消息失败: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_messages();
    let handler = GroupTextHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### C2C 消息

处理客户端到客户端的文本消息。

```rust
use botrs::{C2CMessage, Client, Context, EventHandler, Intents, Ready, Token};

struct C2CTextHandler;

#[async_trait::async_trait]
impl EventHandler for C2CTextHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("C2C 机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn c2c_message_create(&self, ctx: Context, message: C2CMessage) {
        if let Some(content) = &message.content {
            println!("收到 C2C 消息: {}", content);

            // 创建个性化响应
            let reply_content = format!("我收到了你的私人消息：{}", content);

            // 回复 C2C 消息
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &reply_content).await {
                eprintln!("回复 C2C 消息失败: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_messages();
    let handler = C2CTextHandler;
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

### 私信

处理频道环境中的私信。

```rust
use botrs::{Client, Context, DirectMessage, EventHandler, Intents, Ready, Token};

struct DirectMessageHandler;

#[async_trait::async_trait]
impl EventHandler for DirectMessageHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("私信机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            println!("收到私信: {}", content);

            // 处理私信特定命令
            let response = match content.to_lowercase().as_str() {
                "帮助" | "help" => "私信命令：帮助、状态、信息",
                "状态" | "status" => "机器人运行正常",
                "信息" | "info" => "这是与机器人的私人对话",
                _ => "感谢你的消息！输入'帮助'查看可用命令。",
            };

            if let Err(e) = message.reply(&ctx.api, &ctx.token, response).await {
                eprintln!("回复私信失败: {}", e);
            }
        }
    }
}
```

## 高级文本处理

### 文本分析和响应

分析消息内容以提供智能响应。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct AnalysisHandler;

impl AnalysisHandler {
    fn analyze_sentiment(&self, text: &str) -> &'static str {
        let positive_words = ["好", "棒", "太棒了", "喜欢", "开心", "优秀", "不错"];
        let negative_words = ["坏", "糟糕", "讨厌", "难过", "可怕", "恐怖"];
        
        let positive_count = positive_words.iter()
            .map(|word| text.matches(word).count())
            .sum::<usize>();
        let negative_count = negative_words.iter()
            .map(|word| text.matches(word).count())
            .sum::<usize>();
        
        if positive_count > negative_count {
            "积极"
        } else if negative_count > positive_count {
            "消极"
        } else {
            "中性"
        }
    }

    fn generate_response(&self, content: &str) -> String {
        let sentiment = self.analyze_sentiment(content);
        let word_count = content.chars().count();
        
        match sentiment {
            "积极" => format!("很高兴听到积极的话！你的消息（{} 个字符）听起来很棒！", word_count),
            "消极" => format!("希望一切都会好起来！感谢你与我分享这 {} 个字符。", word_count),
            _ => format!("感谢你的消息！我收到了你的 {} 个字符。", word_count),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for AnalysisHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("分析机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // 跳过短消息
            if content.len() < 10 {
                return;
            }

            let response = self.generate_response(content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("发送分析响应失败: {}", e);
            }
        }
    }
}
```

### 频率限制和垃圾邮件保护

为文本响应实现基本的频率限制。

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct RateLimitedHandler {
    last_response: HashMap<String, Instant>,
    cooldown: Duration,
}

impl RateLimitedHandler {
    fn new(cooldown_seconds: u64) -> Self {
        Self {
            last_response: HashMap::new(),
            cooldown: Duration::from_secs(cooldown_seconds),
        }
    }

    fn can_respond(&mut self, user_id: &str) -> bool {
        let now = Instant::now();
        
        if let Some(&last_time) = self.last_response.get(user_id) {
            if now.duration_since(last_time) < self.cooldown {
                return false;
            }
        }
        
        self.last_response.insert(user_id.to_string(), now);
        true
    }
}

#[async_trait::async_trait]
impl EventHandler for RateLimitedHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("频率限制机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&mut self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let (Some(content), Some(author)) = (&message.content, &message.author) {
            // 检查频率限制
            if !self.can_respond(&author.id) {
                return; // 如果用户被频率限制，则静默忽略
            }

            // 带频率限制的简单回声
            let response = format!("回显（频率限制）：{}", content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("发送频率限制响应失败: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let handler = RateLimitedHandler::new(5); // 5 秒冷却时间
    
    let mut client = Client::new(token, intents, handler, true)?;
    client.start().await?;
    
    Ok(())
}
```

## 错误处理

### 健壮的错误处理

为文本消息操作实现全面的错误处理。

```rust
use botrs::{BotError, Client, Context, EventHandler, Intents, Message, Ready, Token};

struct RobustHandler;

impl RobustHandler {
    async fn safe_reply(&self, ctx: &Context, message: &Message, content: &str) -> Result<(), BotError> {
        match message.reply(&ctx.api, &ctx.token, content).await {
            Ok(_) => {
                println!("成功发送回复");
                Ok(())
            }
            Err(BotError::Http(status)) => {
                eprintln!("HTTP 错误 {}：发送消息失败", status);
                Err(BotError::Http(status))
            }
            Err(BotError::RateLimit(retry_after)) => {
                eprintln!("频率限制，{}秒后重试", retry_after);
                // 可以在这里实现重试逻辑
                Err(BotError::RateLimit(retry_after))
            }
            Err(e) => {
                eprintln!("其他错误: {}", e);
                Err(e)
            }
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for RobustHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("健壮机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            // 验证消息内容
            if content.len() > 2000 {
                let _ = self.safe_reply(&ctx, &message, "消息太长！请保持在2000个字符以内。").await;
                return;
            }

            if content.trim().is_empty() {
                return; // 忽略空消息
            }

            // 处理消息
            let response = format!("已处理：{}", content);
            let _ = self.safe_reply(&ctx, &message, &response).await;
        }
    }

    async fn error(&self, error: BotError) {
        eprintln!("处理器错误: {}", error);
    }
}
```

## 最佳实践

### 性能考虑

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};
use tokio::sync::Semaphore;
use std::sync::Arc;

struct PerformantHandler {
    // 限制并发消息处理
    semaphore: Arc<Semaphore>,
}

impl PerformantHandler {
    fn new(max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn process_message(&self, ctx: Context, message: Message) {
        // 获取处理许可
        let _permit = self.semaphore.acquire().await.unwrap();

        if let Some(content) = &message.content {
            // 模拟处理时间
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let response = format!("已处理：{}", content);
            if let Err(e) = message.reply(&ctx.api, &ctx.token, &response).await {
                eprintln!("回复失败: {}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for PerformantHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("高性能机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // 为并发处理生成任务
        let handler = Arc::new(self.clone());
        tokio::spawn(async move {
            handler.process_message(ctx, message).await;
        });
    }
}
```

这份全面的指南涵盖了在 BotRS 中处理文本消息的基本模式，从基本的回声机器人到具有错误处理和性能优化的复杂命令系统。

## 相关文档

- [富文本消息](./rich-messages.md) - 使用嵌入内容、附件和交互内容
- [消息模型](../api/models/messages.md) - 消息类型的详细 API 参考
- [事件处理](./event-handling.md) - 事件处理模式的完整指南
- [错误恢复](./error-recovery.md) - 高级错误处理策略
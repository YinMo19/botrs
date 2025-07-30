# 从 Python botpy 迁移

本指南帮助开发者从官方 Python `botpy` 库迁移到 BotRS。虽然两个库都实现了 QQ 频道机器人 API，但 BotRS 提供了 Rust 的类型安全、性能优势和内存安全保证。

## 概述

BotRS 保持与 Python botpy 设计模式的 API 兼容性，同时添加了 Rust 特有的改进。这使得熟悉 Python 生态系统的开发者能够直接迁移。

### 主要差异

- **类型安全**：Rust 的编译时类型检查可防止许多运行时错误
- **性能**：Rust 的零成本抽象和高效内存管理
- **内存安全**：无垃圾回收开销，可预测的内存使用
- **异步运行时**：使用 Tokio 进行高性能异步操作
- **错误处理**：使用 `Result<T, E>` 类型进行显式错误处理

## 项目结构迁移

### Python botpy 项目
```
my_bot/
├── main.py
├── config.yaml
├── bot/
│   ├── __init__.py
│   ├── handlers.py
│   └── utils.py
└── requirements.txt
```

### BotRS 项目
```
my_bot/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   └── utils.rs
├── config.toml
└── examples/
```

### Cargo.toml 设置
```toml
[package]
name = "my_bot"
version = "0.1.0"
edition = "2021"

[dependencies]
botrs = "0.2"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
```

## 配置迁移

### Python botpy 配置
```python
# config.py
import yaml

class Config:
    def __init__(self):
        with open('config.yaml') as f:
            data = yaml.safe_load(f)
        
        self.app_id = data['bot']['app_id']
        self.secret = data['bot']['secret']
        self.sandbox = data.get('sandbox', False)
```

### BotRS 配置
```rust
// src/config.rs
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub sandbox: Option<bool>,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub app_id: String,
    pub secret: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
```

## 客户端设置迁移

### Python botpy 客户端
```python
import botpy
from botpy import logging
from botpy.ext.cog_yaml import read

class MyClient(botpy.Client):
    async def on_ready(self):
        _log.info(f"robot 「{self.robot.name}」 on_ready!")

    async def on_at_message_create(self, message):
        await message.reply(content=f"机器人{self.robot.name}收到你的@消息了: {message.content}")

if __name__ == "__main__":
    intents = botpy.Intents(public_guild_messages=True)
    client = MyClient(intents=intents)
    client.run(appid="APP_ID", secret="SECRET")
```

### BotRS 客户端
```rust
// src/main.rs
use botrs::{Client, EventHandler, Context, Message, Intents, Token};
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, ctx: Context) {
        info!("机器人已准备就绪!");
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            let reply = format!("机器人收到你的@消息了: {}", content);
            let params = botrs::MessageParams::new_text(&reply);
            
            if let Some(channel_id) = &message.channel_id {
                ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();
    
    let config = Config::load()?;
    let token = Token::new(config.bot.app_id, config.bot.secret);
    let intents = Intents::default().with_public_guild_messages();
    
    let mut client = Client::new(token, intents, MyBot, false)?;
    client.start().await?;
    
    Ok(())
}
```

## 事件处理器迁移

### Python botpy 事件处理器
```python
class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        """处理 @ 提及"""
        await self.handle_at_message(message)
    
    async def on_guild_member_add(self, member):
        """处理新成员加入"""
        await self.welcome_member(member)
    
    async def on_message_reaction_add(self, reaction):
        """处理表情回应添加"""
        await self.handle_reaction(reaction)
    
    async def handle_at_message(self, message):
        if message.content.strip() == "hello":
            await message.reply(content="你好！有什么可以帮助你的吗？")
        elif message.content.strip() == "ping":
            await message.reply(content="Pong!")
```

### BotRS 事件处理器
```rust
use botrs::{EventHandler, Context, Message, GuildMember, MessageReaction};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        self.handle_at_message(ctx, message).await;
    }
    
    async fn guild_member_add(&self, ctx: Context, member: GuildMember) {
        self.welcome_member(ctx, member).await;
    }
    
    async fn message_reaction_add(&self, ctx: Context, reaction: MessageReaction) {
        self.handle_reaction(ctx, reaction).await;
    }
}

impl MyBot {
    async fn handle_at_message(&self, ctx: Context, message: Message) {
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };
        
        let response = match content {
            "hello" => "你好！有什么可以帮助你的吗？",
            "ping" => "Pong!",
            _ => return,
        };
        
        if let Some(channel_id) = &message.channel_id {
            let params = botrs::MessageParams::new_text(response);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
}
```

## 消息发送迁移

### Python botpy 消息发送
```python
# 简单文本消息
await message.reply(content="你好，世界！")

# 嵌入消息
embed = botpy.Embed(title="我的嵌入", description="这是一个嵌入消息")
await message.reply(embed=embed)

# 文件上传
with open("image.png", "rb") as f:
    await message.reply(file=botpy.File(f, "image.png"))

# Markdown 消息
markdown = botpy.MessageMarkdown(content="# 你好\n\n这是**粗体**文本")
await message.reply(markdown=markdown)

# 键盘消息
keyboard = botpy.MessageKeyboard(content=buttons_data)
await message.reply(keyboard=keyboard)
```

### BotRS 消息发送
```rust
use botrs::{MessageParams, Embed, MarkdownPayload};

// 简单文本消息
let params = MessageParams::new_text("你好，世界！");
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// 嵌入消息
let embed = Embed {
    title: Some("我的嵌入".to_string()),
    description: Some("这是一个嵌入消息".to_string()),
    ..Default::default()
};
let params = MessageParams {
    content: Some("看看这个：".to_string()),
    embed: Some(embed),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// 文件上传
let image_data = std::fs::read("image.png")?;
let params = MessageParams::new_text("这是一张图片：")
    .with_file_image(&image_data);
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// Markdown 消息
let markdown = MarkdownPayload {
    content: Some("# 你好\n\n这是**粗体**文本".to_string()),
    ..Default::default()
};
let params = MessageParams {
    markdown: Some(markdown),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// 键盘消息（类似结构）
let params = MessageParams {
    keyboard: Some(keyboard_data),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
```

## Intent 系统迁移

### Python botpy Intents
```python
import botpy

# 基本 intents
intents = botpy.Intents.default()
intents.public_guild_messages = True

# 多个 intents
intents = botpy.Intents(
    public_guild_messages=True,
    direct_message=True,
    guild_messages=True
)

# 所有 intents
intents = botpy.Intents.all()
```

### BotRS Intents
```rust
use botrs::Intents;

// 基本 intents
let intents = Intents::default().with_public_guild_messages();

// 多个 intents
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guild_messages();

// 所有 intents
let intents = Intents::all();

// 自定义组合
let intents = Intents::from_bits(0b1010).unwrap_or_default();
```

## 错误处理迁移

### Python botpy 错误处理
```python
import botpy
from botpy.errors import *

try:
    await message.reply(content="你好！")
except ServerError as e:
    print(f"服务器错误: {e}")
except Forbidden as e:
    print(f"权限错误: {e}")
except Exception as e:
    print(f"未知错误: {e}")
```

### BotRS 错误处理
```rust
use botrs::Error;

match ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await {
    Ok(response) => {
        println!("消息发送成功: {:?}", response);
    }
    Err(Error::Http(status)) if status == 403 => {
        println!("权限错误: 机器人缺少必要权限");
    }
    Err(Error::Http(status)) if status >= 500 => {
        println!("服务器错误: {}", status);
    }
    Err(e) => {
        println!("其他错误: {}", e);
    }
}

// 使用 ? 操作符进行早期返回
async fn send_message(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), Error> {
    let params = MessageParams::new_text(content);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

## 异步/等待迁移

### Python botpy 异步
```python
import asyncio
import botpy

class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        # 简单异步操作
        await message.reply(content="你好！")
        
        # 多个异步操作
        tasks = [
            self.send_notification(message.author.id),
            self.log_message(message.content),
            self.update_stats()
        ]
        await asyncio.gather(*tasks)
    
    async def send_notification(self, user_id):
        await asyncio.sleep(1)  # 模拟工作
        print(f"通知已发送给 {user_id}")
```

### BotRS 异步
```rust
use tokio::time::{sleep, Duration};

impl MyBot {
    async fn handle_message(&self, ctx: Context, message: Message) {
        // 简单异步操作
        let params = MessageParams::new_text("你好！");
        if let Some(channel_id) = &message.channel_id {
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
        
        // 多个异步操作
        let user_id = message.author.as_ref().map(|a| &a.id);
        let content = message.content.as_deref();
        
        tokio::join!(
            self.send_notification(user_id),
            self.log_message(content),
            self.update_stats()
        );
    }
    
    async fn send_notification(&self, user_id: Option<&String>) {
        sleep(Duration::from_secs(1)).await; // 模拟工作
        if let Some(id) = user_id {
            println!("通知已发送给 {}", id);
        }
    }
}
```

## 数据模型迁移

### Python botpy 模型
```python
# 访问消息数据
user_id = message.author.id
username = message.author.username
channel_id = message.channel_id
guild_id = message.guild_id
content = message.content

# 访问频道数据
guild_name = guild.name
guild_id = guild.id
member_count = guild.member_count
```

### BotRS 模型
```rust
// 访问消息数据（带 Option 处理）
let user_id = message.author.as_ref().map(|a| &a.id);
let username = message.author.as_ref().and_then(|a| a.username.as_ref());
let channel_id = message.channel_id.as_ref();
let guild_id = message.guild_id.as_ref();
let content = message.content.as_ref();

// 使用模式匹配进行安全访问
if let Some(author) = &message.author {
    if let Some(username) = &author.username {
        println!("消息来自: {}", username);
    }
}

// 使用 unwrap_or 提供默认值
let content = message.content.as_deref().unwrap_or("无内容");
```

## 命令系统迁移

### Python botpy 命令
```python
class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        content = message.content.strip()
        
        if content.startswith("!hello"):
            await self.handle_hello(message)
        elif content.startswith("!help"):
            await self.handle_help(message)
        elif content.startswith("!echo "):
            text = content[6:]  # 移除 "!echo "
            await message.reply(content=f"回声: {text}")
    
    async def handle_hello(self, message):
        await message.reply(content="你好！")
    
    async def handle_help(self, message):
        help_text = """
        可用命令:
        !hello - 打招呼
        !help - 显示此帮助
        !echo <文本> - 回声你的文本
        """
        await message.reply(content=help_text)
```

### BotRS 命令
```rust
impl MyBot {
    async fn handle_message(&self, ctx: Context, message: Message) {
        let content = match message.content.as_deref() {
            Some(content) => content.trim(),
            None => return,
        };
        
        if content.starts_with("!hello") {
            self.handle_hello(&ctx, &message).await;
        } else if content.starts_with("!help") {
            self.handle_help(&ctx, &message).await;
        } else if content.starts_with("!echo ") {
            let text = &content[6..]; // 移除 "!echo "
            self.handle_echo(&ctx, &message, text).await;
        }
    }
    
    async fn handle_hello(&self, ctx: &Context, message: &Message) {
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text("你好！");
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
    
    async fn handle_help(&self, ctx: &Context, message: &Message) {
        let help_text = r#"可用命令:
!hello - 打招呼
!help - 显示此帮助
!echo <文本> - 回声你的文本"#;
        
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text(help_text);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
    
    async fn handle_echo(&self, ctx: &Context, message: &Message, text: &str) {
        let response = format!("回声: {}", text);
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text(&response);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
}
```

## 数据库集成迁移

### Python botpy 与 SQLAlchemy
```python
from sqlalchemy import create_engine, Column, Integer, String
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

Base = declarative_base()

class User(Base):
    __tablename__ = 'users'
    id = Column(Integer, primary_key=True)
    user_id = Column(String, unique=True)
    username = Column(String)

class MyClient(botpy.Client):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.engine = create_engine('sqlite:///bot.db')
        Base.metadata.create_all(self.engine)
        Session = sessionmaker(bind=self.engine)
        self.session = Session()
    
    async def on_at_message_create(self, message):
        # 存储用户信息
        user = self.session.query(User).filter_by(user_id=message.author.id).first()
        if not user:
            user = User(user_id=message.author.id, username=message.author.username)
            self.session.add(user)
            self.session.commit()
```

### BotRS 与 SQLx
```rust
use sqlx::{SqlitePool, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    user_id: String,
    username: Option<String>,
}

struct MyBot {
    db_pool: SqlitePool,
}

impl MyBot {
    async fn new() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect("sqlite:bot.db").await?;
        
        // 创建表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                user_id TEXT UNIQUE NOT NULL,
                username TEXT
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(Self { db_pool: pool })
    }
    
    async fn store_user(&self, user_id: &str, username: Option<&str>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR REPLACE INTO users (user_id, username) VALUES (?, ?)"
        )
        .bind(user_id)
        .bind(username)
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(author) = &message.author {
            let username = author.username.as_deref();
            if let Err(e) = self.store_user(&author.id, username).await {
                println!("数据库错误: {}", e);
            }
        }
    }
}
```

## 测试迁移

### Python botpy 测试
```python
import unittest
from unittest.mock import AsyncMock, patch
import botpy

class TestMyBot(unittest.IsolatedAsyncioTestCase):
    async def test_hello_command(self):
        client = MyClient()
        
        # 模拟消息
        message = AsyncMock()
        message.content = "!hello"
        message.reply = AsyncMock()
        
        await client.on_at_message_create(message)
        
        message.reply.assert_called_once_with(content="你好！")
```

### BotRS 测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use botrs::{Context, Message, Author};
    
    #[tokio::test]
    async fn test_hello_command() {
        let bot = MyBot::new().await.unwrap();
        
        // 创建模拟消息
        let message = Message {
            id: Some("123".to_string()),
            content: Some("!hello".to_string()),
            channel_id: Some("channel_123".to_string()),
            author: Some(Author {
                id: "user_123".to_string(),
                username: Some("TestUser".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        
        // 测试需要模拟 API 调用
        // 实际上，你会使用依赖注入或特征进行测试
    }
    
    #[test]
    fn test_message_parsing() {
        let content = "!echo Hello World";
        assert!(content.starts_with("!echo "));
        let text = &content[6..];
        assert_eq!(text, "Hello World");
    }
}
```

## 性能考虑

### 内存使用
- **Python**：垃圾回收，不可预测的内存使用
- **Rust**：栈分配，可预测的内存模式，零成本抽象

### 并发性
- **Python**：全局解释器锁（GIL）限制真正的并行性
- **Rust**：使用 `tokio` 实现真正的并行性，无 GIL 限制

### 错误处理
- **Python**：运行时异常，可能意外崩溃
- **Rust**：编译时错误检查，显式错误处理

## 迁移检查清单

- [ ] 使用 `Cargo.toml` 设置 Rust 项目
- [ ] 将配置文件从 YAML/JSON 转换为 TOML
- [ ] 迁移事件处理器以使用 `#[async_trait::async_trait]`
- [ ] 更新消息发送以使用 `MessageParams`
- [ ] 转换 intent 设置以使用 BotRS intent 系统
- [ ] 更新错误处理以使用 `Result<T, E>`
- [ ] 迁移数据库代码以使用异步 Rust 库
- [ ] 更新日志以使用 `tracing` 而不是 Python 日志
- [ ] 添加适当的类型注解和 Option 处理
- [ ] 为 Rust 编译和测试设置 CI/CD

## 常见迁移模式

### 可选值
```rust
// Python: value or None
username = message.author.username if message.author else None

// Rust: Option<T>
let username = message.author.as_ref().and_then(|a| a.username.as_ref());
```

### 错误传播
```python
# Python: try/except
try:
    result = await api_call()
    return process(result)
except Exception as e:
    print(f"错误: {e}")
    return None
```

```rust
// Rust: ? 操作符
async fn handle_api_call(&self) -> Result<ProcessedResult, Error> {
    let result = api_call().await?;
    Ok(process(result))
}
```

### 字符串处理
```python
# Python: str
content = message.content.strip().lower()
```

```rust
// Rust: String/&str 与 Option
let content = message.content
    .as_deref()
    .unwrap_or("")
    .trim()
    .to_lowercase();
```

本迁移指南提供了从 Python botpy 到 BotRS 的全面路径，充分利用 Rust 的优势，同时在可能的情况下保持熟悉的模式。
# Migration from Python botpy

This guide helps developers migrate from the official Python `botpy` library to BotRS. While both libraries implement the QQ Guild Bot API, BotRS provides Rust's type safety, performance benefits, and memory safety guarantees.

## Overview

BotRS maintains API compatibility with Python botpy's design patterns while adding Rust-specific improvements. This makes migration straightforward for developers familiar with the Python ecosystem.

### Key Differences

- **Type Safety**: Rust's compile-time type checking prevents many runtime errors
- **Performance**: Rust's zero-cost abstractions and efficient memory management
- **Memory Safety**: No garbage collection overhead, predictable memory usage
- **Async Runtime**: Uses Tokio for high-performance async operations
- **Error Handling**: Explicit error handling with `Result<T, E>` types

## Project Structure Migration

### Python botpy Project
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

### BotRS Project
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

### Cargo.toml Setup
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

## Configuration Migration

### Python botpy Configuration
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

### BotRS Configuration
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

## Client Setup Migration

### Python botpy Client
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

### BotRS Client
```rust
// src/main.rs
use botrs::{Client, EventHandler, Context, Message, Intents, Token};
use tracing::info;

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, ctx: Context) {
        info!("robot is ready!");
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

## Event Handler Migration

### Python botpy Event Handlers
```python
class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        """Handle @ mentions"""
        await self.handle_at_message(message)
    
    async def on_guild_member_add(self, member):
        """Handle new member joins"""
        await self.welcome_member(member)
    
    async def on_message_reaction_add(self, reaction):
        """Handle reaction additions"""
        await self.handle_reaction(reaction)
    
    async def handle_at_message(self, message):
        if message.content.strip() == "hello":
            await message.reply(content="Hello! How can I help you?")
        elif message.content.strip() == "ping":
            await message.reply(content="Pong!")
```

### BotRS Event Handlers
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
            "hello" => "Hello! How can I help you?",
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

## Message Sending Migration

### Python botpy Message Sending
```python
# Simple text message
await message.reply(content="Hello, world!")

# Embed message
embed = botpy.Embed(title="My Embed", description="This is an embed")
await message.reply(embed=embed)

# File upload
with open("image.png", "rb") as f:
    await message.reply(file=botpy.File(f, "image.png"))

# Markdown message
markdown = botpy.MessageMarkdown(content="# Hello\n\nThis is **bold**")
await message.reply(markdown=markdown)

# Keyboard message
keyboard = botpy.MessageKeyboard(content=buttons_data)
await message.reply(keyboard=keyboard)
```

### BotRS Message Sending
```rust
use botrs::{MessageParams, Embed, MarkdownPayload};

// Simple text message
let params = MessageParams::new_text("Hello, world!");
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// Embed message
let embed = Embed {
    title: Some("My Embed".to_string()),
    description: Some("This is an embed".to_string()),
    ..Default::default()
};
let params = MessageParams {
    content: Some("Check this out:".to_string()),
    embed: Some(embed),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// File upload
let image_data = std::fs::read("image.png")?;
let params = MessageParams::new_text("Here's an image:")
    .with_file_image(&image_data);
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// Markdown message
let markdown = MarkdownPayload {
    content: Some("# Hello\n\nThis is **bold**".to_string()),
    ..Default::default()
};
let params = MessageParams {
    markdown: Some(markdown),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;

// Keyboard message (similar structure)
let params = MessageParams {
    keyboard: Some(keyboard_data),
    ..Default::default()
};
ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await?;
```

## Intent System Migration

### Python botpy Intents
```python
import botpy

# Basic intents
intents = botpy.Intents.default()
intents.public_guild_messages = True

# Multiple intents
intents = botpy.Intents(
    public_guild_messages=True,
    direct_message=True,
    guild_messages=True
)

# All intents
intents = botpy.Intents.all()
```

### BotRS Intents
```rust
use botrs::Intents;

// Basic intents
let intents = Intents::default().with_public_guild_messages();

// Multiple intents
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guild_messages();

// All intents
let intents = Intents::all();

// Custom combination
let intents = Intents::from_bits(0b1010).unwrap_or_default();
```

## Error Handling Migration

### Python botpy Error Handling
```python
import botpy
from botpy.errors import *

try:
    await message.reply(content="Hello!")
except ServerError as e:
    print(f"Server error: {e}")
except Forbidden as e:
    print(f"Permission error: {e}")
except Exception as e:
    print(f"Unknown error: {e}")
```

### BotRS Error Handling
```rust
use botrs::Error;

match ctx.api.post_message_with_params(&ctx.token, &channel_id, params).await {
    Ok(response) => {
        println!("Message sent successfully: {:?}", response);
    }
    Err(Error::Http(status)) if status == 403 => {
        println!("Permission error: Bot lacks necessary permissions");
    }
    Err(Error::Http(status)) if status >= 500 => {
        println!("Server error: {}", status);
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}

// Using ? operator for early return
async fn send_message(&self, ctx: &Context, channel_id: &str, content: &str) -> Result<(), Error> {
    let params = MessageParams::new_text(content);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    Ok(())
}
```

## Async/Await Migration

### Python botpy Async
```python
import asyncio
import botpy

class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        # Simple async operation
        await message.reply(content="Hello!")
        
        # Multiple async operations
        tasks = [
            self.send_notification(message.author.id),
            self.log_message(message.content),
            self.update_stats()
        ]
        await asyncio.gather(*tasks)
    
    async def send_notification(self, user_id):
        await asyncio.sleep(1)  # Simulate work
        print(f"Notification sent to {user_id}")
```

### BotRS Async
```rust
use tokio::time::{sleep, Duration};

impl MyBot {
    async fn handle_message(&self, ctx: Context, message: Message) {
        // Simple async operation
        let params = MessageParams::new_text("Hello!");
        if let Some(channel_id) = &message.channel_id {
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
        
        // Multiple async operations
        let user_id = message.author.as_ref().map(|a| &a.id);
        let content = message.content.as_deref();
        
        tokio::join!(
            self.send_notification(user_id),
            self.log_message(content),
            self.update_stats()
        );
    }
    
    async fn send_notification(&self, user_id: Option<&String>) {
        sleep(Duration::from_secs(1)).await; // Simulate work
        if let Some(id) = user_id {
            println!("Notification sent to {}", id);
        }
    }
}
```

## Data Models Migration

### Python botpy Models
```python
# Accessing message data
user_id = message.author.id
username = message.author.username
channel_id = message.channel_id
guild_id = message.guild_id
content = message.content

# Accessing guild data
guild_name = guild.name
guild_id = guild.id
member_count = guild.member_count
```

### BotRS Models
```rust
// Accessing message data (with Option handling)
let user_id = message.author.as_ref().map(|a| &a.id);
let username = message.author.as_ref().and_then(|a| a.username.as_ref());
let channel_id = message.channel_id.as_ref();
let guild_id = message.guild_id.as_ref();
let content = message.content.as_ref();

// Safe access with pattern matching
if let Some(author) = &message.author {
    if let Some(username) = &author.username {
        println!("Message from: {}", username);
    }
}

// Using unwrap_or for defaults
let content = message.content.as_deref().unwrap_or("No content");
```

## Command System Migration

### Python botpy Commands
```python
class MyClient(botpy.Client):
    async def on_at_message_create(self, message):
        content = message.content.strip()
        
        if content.startswith("!hello"):
            await self.handle_hello(message)
        elif content.startswith("!help"):
            await self.handle_help(message)
        elif content.startswith("!echo "):
            text = content[6:]  # Remove "!echo "
            await message.reply(content=f"Echo: {text}")
    
    async def handle_hello(self, message):
        await message.reply(content="Hello there!")
    
    async def handle_help(self, message):
        help_text = """
        Available commands:
        !hello - Say hello
        !help - Show this help
        !echo <text> - Echo your text
        """
        await message.reply(content=help_text)
```

### BotRS Commands
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
            let text = &content[6..]; // Remove "!echo "
            self.handle_echo(&ctx, &message, text).await;
        }
    }
    
    async fn handle_hello(&self, ctx: &Context, message: &Message) {
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text("Hello there!");
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
    
    async fn handle_help(&self, ctx: &Context, message: &Message) {
        let help_text = r#"Available commands:
!hello - Say hello
!help - Show this help
!echo <text> - Echo your text"#;
        
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text(help_text);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
    
    async fn handle_echo(&self, ctx: &Context, message: &Message, text: &str) {
        let response = format!("Echo: {}", text);
        if let Some(channel_id) = &message.channel_id {
            let params = MessageParams::new_text(&response);
            ctx.api.post_message_with_params(&ctx.token, channel_id, params).await.ok();
        }
    }
}
```

## Database Integration Migration

### Python botpy with SQLAlchemy
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
        # Store user info
        user = self.session.query(User).filter_by(user_id=message.author.id).first()
        if not user:
            user = User(user_id=message.author.id, username=message.author.username)
            self.session.add(user)
            self.session.commit()
```

### BotRS with SQLx
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
        
        // Create tables
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
                println!("Database error: {}", e);
            }
        }
    }
}
```

## Testing Migration

### Python botpy Testing
```python
import unittest
from unittest.mock import AsyncMock, patch
import botpy

class TestMyBot(unittest.IsolatedAsyncioTestCase):
    async def test_hello_command(self):
        client = MyClient()
        
        # Mock message
        message = AsyncMock()
        message.content = "!hello"
        message.reply = AsyncMock()
        
        await client.on_at_message_create(message)
        
        message.reply.assert_called_once_with(content="Hello there!")
```

### BotRS Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use botrs::{Context, Message, Author};
    
    #[tokio::test]
    async fn test_hello_command() {
        let bot = MyBot::new().await.unwrap();
        
        // Create mock message
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
        
        // Test would require mocking the API calls
        // In practice, you'd use dependency injection or traits for testing
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

## Performance Considerations

### Memory Usage
- **Python**: Garbage collected, unpredictable memory usage
- **Rust**: Stack allocation, predictable memory patterns, zero-cost abstractions

### Concurrency
- **Python**: Global Interpreter Lock (GIL) limits true parallelism
- **Rust**: True parallelism with `tokio`, no GIL limitations

### Error Handling
- **Python**: Runtime exceptions, can crash unexpectedly
- **Rust**: Compile-time error checking, explicit error handling

## Migration Checklist

- [ ] Set up Rust project with `Cargo.toml`
- [ ] Convert configuration files from YAML/JSON to TOML
- [ ] Migrate event handlers to use `#[async_trait::async_trait]`
- [ ] Update message sending to use `MessageParams`
- [ ] Convert intent setup to use BotRS intent system
- [ ] Update error handling to use `Result<T, E>`
- [ ] Migrate database code to use async Rust libraries
- [ ] Update logging to use `tracing` instead of Python logging
- [ ] Add proper type annotations and Option handling
- [ ] Set up CI/CD for Rust compilation and testing

## Common Migration Patterns

### Optional Values
```rust
// Python: value or None
username = message.author.username if message.author else None

// Rust: Option<T>
let username = message.author.as_ref().and_then(|a| a.username.as_ref());
```

### Error Propagation
```python
# Python: try/except
try:
    result = await api_call()
    return process(result)
except Exception as e:
    print(f"Error: {e}")
    return None
```

```rust
// Rust: ? operator
async fn handle_api_call(&self) -> Result<ProcessedResult, Error> {
    let result = api_call().await?;
    Ok(process(result))
}
```

### String Handling
```python
# Python: str
content = message.content.strip().lower()
```

```rust
// Rust: String/&str with Option
let content = message.content
    .as_deref()
    .unwrap_or("")
    .trim()
    .to_lowercase();
```

This migration guide provides a comprehensive path from Python botpy to BotRS, leveraging Rust's strengths while maintaining familiar patterns where possible.
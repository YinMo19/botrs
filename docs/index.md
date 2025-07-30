---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "BotRS"
  text: "Rust QQ Bot Framework"
  tagline: "A type-safe, high-performance, and easy-to-use QQ Guild Bot framework for Rust"
  # image:
  #   src: /logo.svg
  #   alt: BotRS
  actions:
    - theme: brand
      text: Get Started
      link: /guide/introduction
    - theme: alt

      text: View on GitHub
      link: https://github.com/YinMo19/botrs

features:
  - icon: 🛡️
    title: Type Safety
    details: Fully typed API with compile-time error catching. Rust's ownership system ensures memory safety and prevents common programming errors.

  - icon: ⚡
    title: High Performance
    details: Built on Tokio async runtime for high concurrency. Efficient WebSocket handling and HTTP client with connection pooling.

  - icon: 🔧
    title: Easy to Use
    details: Intuitive API design with clear documentation. Get your bot running in minutes with minimal boilerplate code.

  - icon: 🎯
    title: Event-Driven Architecture
    details: Respond to various QQ Guild events with a clean event handler system. Support for messages, channels, members, and more.

  - icon: 📝
    title: Rich Message Support
    details: Send text, embeds, files, markdown, keyboards, and interactive messages. Full support for all QQ Guild message types.

  - icon: 🔄
    title: Intent System
    details: Fine-grained control over event subscriptions. Optimize performance by only receiving events your bot needs.

  - icon: 🌐
    title: WebSocket Gateway
    details: Real-time event processing with automatic reconnection and heartbeat handling. Reliable connection management.

  - icon: 📚
    title: Comprehensive API
    details: Complete coverage of QQ Guild Bot API with structured parameter system. No more confusing multiple None parameters.

  - icon: 🏗️
    title: Structured Parameters
    details: Clean, readable message API with builder patterns. Type-safe parameter construction with default values.
---

## What is BotRS?

BotRS is an asynchronous framework for the Rust programming language designed for building QQ Guild bots. It provides the essential building blocks needed for creating interactive bot applications that can handle messages, manage guilds, and respond to various events in real-time.

At a high level, BotRS provides several major components:

- **Async Runtime Integration**: Built on Tokio for handling thousands of concurrent connections
- **Type-Safe API Bindings**: Complete Rust type definitions for all QQ Guild Bot API endpoints
- **Event-Driven Architecture**: Clean event handler system for responding to guild events
- **Rich Message Support**: Send text, embeds, files, and interactive content
- **WebSocket Gateway**: Real-time event processing with automatic connection management

## BotRS's Role in Your Project

When building a QQ Guild bot, you need a framework that can handle the complexity of real-time messaging, API interactions, and event processing. BotRS serves as the foundation that allows you to focus on your bot's logic rather than the underlying infrastructure.

The framework handles:

- **Connection Management**: Automatic WebSocket reconnection and heartbeat handling
- **Rate Limiting**: Built-in request throttling to respect API limits
- **Type Safety**: Compile-time guarantees that prevent runtime errors
- **Async Processing**: Non-blocking event handling for maximum performance
- **Error Handling**: Comprehensive error types with context and recovery options

## Quick Example

Here's a simple bot that responds to messages:

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot is ready! Logged in as: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("your_app_id", "your_secret");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyBot, true)?;

    client.start().await?;
    Ok(())
}
```

## Getting Started

Ready to build your first QQ Guild bot with BotRS? Follow our comprehensive guide:

1. **[Installation](/guide/installation)** - Add BotRS to your Rust project
2. **[Quick Start](/guide/quick-start)** - Create your first bot in minutes
3. **[Configuration](/guide/configuration)** - Set up your bot credentials and options
4. **[Examples](/examples/getting-started)** - Explore working code examples

## Architecture Highlights

### Compatibility with Python botpy

BotRS maintains API compatibility with the official Python botpy library, making migration straightforward for developers familiar with the Python ecosystem. The structured parameter system mirrors botpy's approach while adding Rust's type safety benefits.

### Performance Characteristics

- **Memory Efficient**: Zero-copy deserialization where possible
- **Concurrent Processing**: Handle multiple events simultaneously
- **Connection Pooling**: Reuse HTTP connections for API calls
- **Minimal Allocations**: Careful memory management for high-throughput scenarios

### Type Safety Guarantees

Rust's type system prevents entire classes of bugs
 common in dynamic languages:

- **Compile-time Validation**: Catch API misuse before deployment
- **No Null Pointer Exceptions**: Option types make null handling explicit
- **Memory Safety**: No use-after-free or buffer overflow vulnerabilities
- **Thread Safety**: Concurrent access patterns validated at compile time

## Community and Support

- **[GitHub Repository](https://github.com/YinMo19/botrs)** - Source code, issues, and discussions
- **[Documentation](https://docs.rs/botrs)** - Complete API reference on docs.rs
- **[Examples](/examples/getting-started)** - Working code samples for common use cases
- **[Changelog](/changelog)** - Version history and breaking changes

---

*BotRS is open source software released under the MIT license. Contributions are welcome!*

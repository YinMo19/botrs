# Introduction

BotRS is an asynchronous runtime framework for building QQ Guild bots in Rust. It provides the essential building blocks needed for creating robust, high-performance bot applications that can handle real-time messaging, guild management, and event processing.

## What is BotRS?

BotRS is designed around the principles of type safety, performance, and ease of use. It serves as a comprehensive wrapper around the QQ Guild Bot API, providing:

- **Type-Safe API Bindings**: Complete Rust type definitions for all API endpoints
- **Async Runtime Integration**: Built on Tokio for handling concurrent operations
- **Event-Driven Architecture**: Clean abstractions for responding to guild events
- **Rich Message Support**: Send text, embeds, files, and interactive content
- **WebSocket Gateway**: Real-time event processing with automatic connection management

## Core Architecture

At its core, BotRS consists of several key components:

### Client
The `Client` is the main entry point for your bot application. It manages the WebSocket connection to QQ's servers, handles authentication, and dispatches events to your event handler.

### Event Handler
The `EventHandler` trait defines how your bot responds to various events such as messages, member joins, channel updates, and more. You implement this trait to define your bot's behavior.

### API Client
The `BotApi` provides direct access to QQ Guild's REST API endpoints, allowing you to send messages, manage channels, handle permissions, and perform other administrative tasks.

### Gateway
The WebSocket gateway manages the real-time connection to QQ's servers, handling heartbeats, reconnection logic, and event dispatching automatically.

## Key Features

### Type Safety
Rust's type system prevents entire classes of runtime errors common in dynamically typed languages:

```rust
// Compile-time validation of message parameters
let params = MessageParams::new_text("Hello, world!")
    .with_reply(message_id)
    .with_markdown(true);

// Type-safe event handling
async fn message_create(&self, ctx: Context, message: Message) {
    // message.content is Option<String> - explicit null handling
    if let Some(content) = &message.content {
        // Handle message content safely
    }
}
```

### High Performance
Built on Tokio's async runtime, BotRS can handle thousands of concurrent operations:

- **Non-blocking I/O**: All network operations are asynchronous
- **Connection Pooling**: HTTP clients reuse connections efficiently
- **Memory Efficiency**: Zero-copy deserialization where possible
- **Concurrent Event Processing**: Handle multiple events simultaneously

### Structured Parameters
BotRS v0.2.0 introduced a new structured parameter system that eliminates the confusion of multiple `None` parameters:

```rust
// Old API (deprecated)
api.post_message(
    token, "channel_id", Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;

// New API (recommended)
let params = MessageParams::new_text("Hello!")
    .with_reply("message_id")
    .with_embed(embed);
api.post_message_with_params(token, "channel_id", params).await?;
```

## Comparison with Other Solutions

### vs Python botpy
BotRS maintains API compatibility with the official Python botpy library while adding:

- **Compile-time Safety**: Catch errors before deployment
- **Better Performance**: Native code execution and efficient memory usage
- **Structured Concurrency**: Built-in async/await support
- **Zero-cost Abstractions**: High-level APIs with minimal runtime overhead

### vs Other Rust Discord Libraries
While there are excellent Discord libraries for Rust, BotRS is specifically designed for QQ Guild's unique API and features:

- **QQ Guild Specific**: Native support for QQ's message types and features
- **Official API Coverage**: Complete implementation of QQ Guild Bot API
- **Chinese Ecosystem**: Built with Chinese developers and use cases in mind
- **Active Maintenance**: Regular updates following QQ's API changes

## Getting Started

Ready to build your first bot? Here's what you'll need:

1. **Rust Installation**: BotRS requires Rust 1.70 or later
2. **QQ Guild Bot Credentials**: App ID and Secret from QQ Guild Developer Portal
3. **Basic Async Knowledge**: Familiarity with Rust's async/await syntax

The fastest way to get started is with our [Quick Start Guide](/guide/quick-start), which will have you running a basic bot in under 5 minutes.

## Community and Ecosystem

BotRS is part of a growing ecosystem of Rust tools for building chat bots and automation:

- **Active Development**: Regular updates and new features
- **Community Driven**: Open source with contributions welcome
- **Production Ready**: Used in production by multiple organizations
- **Comprehensive Documentation**: Detailed guides and API reference

## Design Philosophy

BotRS follows several key design principles:

### Ergonomics First
The API should be intuitive and easy to use, even for developers new to Rust or bot development.

### Safety Without Sacrifice
Type safety and memory safety should not come at the cost of performance or expressiveness.

### Async by Default
All I/O operations are asynchronous to maximize throughput and responsiveness.

### Backward Compatibility
API changes follow semantic versioning, with clear migration paths for breaking changes.

## Next Steps

- **[Installation](/guide/installation)** - Add BotRS to your project
- **[Quick Start](/guide/quick-start)** - Build your first bot
- **[Configuration](/guide/configuration)** - Set up credentials and options
- **[Examples](/examples/getting-started)** - Explore working code samples

The journey to building powerful QQ Guild bots starts here. Let's get building!
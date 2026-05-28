---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "BotRS"
  text: "Rust QQ Bot Framework"
  tagline: "A focused async framework for QQ Guild gateway events and core bot REST actions"
  actions:
    - theme: brand
      text: Get Started
      link: /guide/introduction
    - theme: alt
      text: View on GitHub
      link: https://github.com/YinMo19/botrs

features:
  - icon: 🛡️
    title: Typed Event Payloads
    details: Gateway dispatches are decoded into Rust models for messages, guilds, channels, members, reactions, interactions, audio, manage, and forum events.

  - icon: ⚡
    title: Async Runtime
    details: Built on Tokio with WebSocket sessions, heartbeats, reconnect handling, and typed event dispatch.

  - icon: 🔧
    title: Small Core API
    details: Client, EventHandler, session types, BotApi, Token, and Intents form the main surface used by applications.

  - icon: 🎯
    title: Event-Driven Design
    details: Implement only the callbacks your bot needs; every EventHandler method has a default empty body.

  - icon: 📝
    title: Structured Messages
    details: Send guild, group, C2C, and direct messages with MessageParams, GroupMessageParams, C2CMessageParams, and DirectMessageParams.

  - icon: 🔄
    title: Intent System
    details: Choose the gateway event categories your bot receives with chainable Intents helpers.

  - icon: 🌐
    title: Gateway Managed by Client
    details: Client discovers the gateway, starts shard sessions, sends heartbeats, and forwards dispatches to your handler.

  - icon: 📚
    title: Core REST Actions
    details: BotApi covers bot info, gateway discovery, messages, recalls, media uploads, announcements, schedules, permissions, reactions, and pins.
---

## What Is BotRS?

BotRS is an asynchronous Rust framework for building QQ Guild bots around the QQ gateway and a focused REST surface. It provides the pieces most bots need in the live event path: gateway connection management, typed event payloads, a shared REST client, token handling, and intent selection.

The central types are:

- `Client` owns startup, gateway sessions, and event dispatch.
- `EventHandler` is the trait you implement for gateway events.
- Session types give each callback access to event data, bot info, and the shared `BotApi`.
- `BotApi` sends messages and performs the core REST actions used by the examples.
- `Token` stores credentials and supports environment loading.
- `Intents` controls which gateway categories QQ sends.

## Current Capabilities

BotRS handles the core event-to-action loop:

- Receive typed gateway events for guild/channel/member changes, guild messages, direct messages, group and C2C messages, reactions, interactions, audits, manage events, audio events, and forum events.
- Reply with `session.reply("text")` for plain text, or build a matching params struct for richer payloads.
- Send guild, group, C2C, and direct messages through `BotApi`.
- Upload group/C2C media, then send it as a media message.
- Work with announcements, schedules, API permission requests, reactions, and pinned messages.

## Quick Example

```rust
use botrs::{ChannelReplySession, Client, EventHandler, Intents, ReadySession, Token};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, session: ReadySession) {
        println!("Bot is ready as {}", session.event().user.username);
    }

    async fn message_create(&self, mut session: ChannelReplySession) {
        let message = session.message().clone();
        if message.content.as_deref() == Some("!ping") {
            let _ = session.reply("pong").await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::from_env()?;
    let intents = Intents::new().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyBot, true)?;

    client.start().await?;
    Ok(())
}
```

## Getting Started

1. **[Installation](/guide/installation)** - Add BotRS to your Rust project.
2. **[Quick Start](/guide/quick-start)** - Run a minimal bot.
3. **[Client and Event Handler](/guide/client-handler)** - Learn the event loop.
4. **[Messages](/guide/messages)** - Send text, media, markdown, ark, embed, and keyboard payloads.
5. **[API Client](/guide/api-client)** - Use `BotApi` through sessions or standalone.

## Links

- **[GitHub Repository](https://github.com/YinMo19/botrs)** - Source code and issues.
- **[Documentation](https://docs.rs/botrs)** - docs.rs API reference.
- **[Examples](/examples/getting-started)** - End-to-end usage examples.
- **[Changelog](/changelog)** - Current release summary.

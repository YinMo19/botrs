---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "BotRS"
  text: "Rust QQ Bot Framework"
  tagline: "A focused async framework for QQ gateway events and core bot REST actions"
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
    details: Reply through session helpers or send guild, group, C2C, and direct messages with typed params constructors.

  - icon: 🔄
    title: Intent System
    details: Choose the gateway event categories your bot receives with chainable Intents helpers.

  - icon: 🌐
    title: Gateway Managed by Client
    details: Client discovers the gateway, starts shard sessions, sends heartbeats, and forwards dispatches to your handler.

  - icon: 📚
    title: Core REST Actions
    details: BotApi covers bot info, gateway discovery, messages, media uploads, guild/channel resources, roles, permissions, announcements, schedules, reactions, pins, and audio controls.
---

## What Is BotRS?

BotRS is an asynchronous Rust framework for building QQ bots around the QQ gateway and a focused bot OpenAPI surface. It provides the pieces most bots need in the live event path: gateway connection management, typed event payloads, a shared REST client, token handling, and intent selection.

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
- Reply with `session.reply("text")`, `send_markdown_message`, `send_embed_message`, `send_ark_message`, `send_keyboard_message`, or a matching params struct when you need direct field control.
- Send guild, group, C2C, and direct messages through `BotApi`.
- Upload group/C2C media, then send it as a media message.
- Work with guild/channel resources, roles, mute state, channel permissions, announcements, schedules, API permission requests, reactions, pinned messages, and audio controls.

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
        if message.author.bot {
            return;
        }

        if message.content.trim() == "!ping" {
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

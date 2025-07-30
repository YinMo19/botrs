# Client API Reference

The `Client` is the main entry point for creating and managing QQ Guild bots. It handles the WebSocket connection, authentication, and event dispatching to your event handler.

## Overview

```rust
use botrs::{Client, EventHandler, Intents, Token};

pub struct Client<H: EventHandler> {
    // Internal fields...
}
```

The `Client` manages:
- WebSocket connection to QQ Guild gateway
- Authentication with QQ servers
- Event dispatching to your `EventHandler`
- Automatic reconnection and heartbeat handling
- Rate limiting and request management

## Constructor

### `new`

Creates a new client instance.

```rust
pub fn new(
    token: Token,
    intents: Intents,
    handler: H,
    use_sandbox: bool,
) -> Result<Self>
```

#### Parameters

- `token`: Authentication token containing your app ID and secret
- `intents`: Event subscription configuration
- `handler`: Your event handler implementing the `EventHandler` trait
- `use_sandbox`: Whether to use sandbox environment for testing

#### Returns

Returns `Result<Client<H>, BotError>` - the client instance or an error if initialization fails.

#### Example

```rust
use botrs::{Client, EventHandler, Intents, Token};

struct MyHandler;

#[async_trait::async_trait]
impl EventHandler for MyHandler {
    // Event handling methods...
}

let token = Token::new("your_app_id", "your_secret");
let intents = Intents::default().with_public_guild_messages();
let handler = MyHandler;

let client = Client::new(token, intents, handler, false)?;
```

## Methods

### `start`

Starts the bot and begins listening for events. This method blocks until the connection is closed.

```rust
pub async fn start(&mut self) -> Result<()>
```

#### Returns

Returns `Result<(), BotError>` - `Ok(())` when the bot stops gracefully, or an error if connection fails.

#### Example

```rust
let mut client = Client::new(token, intents, handler, false)?;
client.start().await?;
```

### `stop`

Gracefully stops the bot and closes the WebSocket connection.

```rust
pub async fn stop(&mut self) -> Result<()>
```

#### Returns

Returns `Result<(), BotError>` - `Ok(())` when successfully stopped, or an error if stopping fails.

#### Example

```rust
// In another task or signal handler
client.stop().await?;
```

### `is_connected`

Checks if the client is currently connected to the gateway.

```rust
pub fn is_connected(&self) -> bool
```

#### Returns

Returns `true` if connected, `false` otherwise.

#### Example

```rust
if client.is_connected() {
    println!("Bot is online");
} else {
    println!("Bot is offline");
}
```

### `get_session_info`

Gets information about the current session.

```rust
pub fn get_session_info(&self) -> Option<&ConnectionSession>
```

#### Returns

Returns `Some(&ConnectionSession)` if connected, `None` if disconnected.

#### Example

```rust
if let Some(session) = client.get_session_info() {
    println!("Session ID: {}", session.session_id);
    println!("Shard: {}/{}", session.shard_id, session.shard_count);
}
```

## Configuration

### Environment URLs

The client automatically selects the appropriate API endpoints:

- **Production**: `https://api.sgroup.qq.com`
- **Sandbox**: `https://sandbox.api.sgroup.qq.com`

### Connection Settings

Default connection settings:

- **WebSocket URL**: `wss://api.sgroup.qq.com/websocket`
- **Timeout**: 30 seconds for HTTP requests
- **Heartbeat**: Automatic based on server requirements
- **Reconnection**: Automatic with exponential backoff

## Error Handling

The client can return various errors:

```rust
use botrs::BotError;

match client.start().await {
    Ok(_) => println!("Bot stopped gracefully"),
    Err(BotError::Authentication(e)) => eprintln!("Auth error: {}", e),
    Err(BotError::Network(e)) => eprintln!("Network error: {}", e),
    Err(BotError::Gateway(e)) => eprintln!("Gateway error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Event Flow

1. **Connection**: Client connects to WebSocket gateway
2. **Authentication**: Sends identify payload with token and intents
3. **Ready**: Receives ready event, bot is now online
4. **Event Loop**: Continuously receives and dispatches events
5. **Reconnection**: Automatically reconnects if connection drops

## Thread Safety

The `Client` is designed to be used from a single async task. For multi-threaded applications, wrap it in appropriate synchronization primitives:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let client = Arc::new(Mutex::new(client));
```

## Examples

### Basic Bot

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct BasicBot;

#[async_trait::async_trait]
impl EventHandler for BasicBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot ready: {}", ready.user.username);
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
    let token = Token::new("app_id", "secret");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, BasicBot, false)?;
    
    client.start().await?;
    Ok(())
}
```

### Bot with Graceful Shutdown

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("app_id", "secret");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyHandler, false)?;
    
    // Start bot in background task
    let client_handle = tokio::spawn(async move {
        client.start().await
    });
    
    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    println!("Shutdown signal received");
    
    // Stop the bot
    client_handle.abort();
    
    Ok(())
}
```

### Multiple Intents

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds()
    .with_guild_members();

let mut client = Client::new(token, intents, handler, false)?;
```

## See Also

- [`EventHandler`](./event-handler.md) - Define how your bot responds to events
- [`Context`](./context.md) - Access API client and token in event handlers
- [`Intents`](./intents.md) - Configure which events to receive
- [`Token`](./token.md) - Authentication and credentials management
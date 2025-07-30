# WebSocket Gateway

The WebSocket Gateway is the core component that maintains a persistent connection to QQ Guild's real-time event system. It handles authentication, heartbeat management, automatic reconnection, and event dispatching, providing the foundation for your bot's real-time functionality.

## Overview

The Gateway manages a WebSocket connection that receives events from QQ's servers in real-time. Unlike REST API calls which are request-response based, the Gateway provides a continuous stream of events such as new messages, member joins, guild updates, and more.

```rust
use botrs::{Client, Gateway, Token, Intents};

// Gateway is automatically managed by the Client
let mut client = Client::new("your_app_id", handler)
    .intents(Intents::GUILD_MESSAGES | Intents::GUILDS)
    .build()
    .await?;

// Gateway connection is established when starting the client
client.start().await?;
```

## Gateway Architecture

### Connection Lifecycle

The Gateway follows a specific connection lifecycle:

1. **Initial Connection**: Establishes WebSocket connection to the gateway URL
2. **Authentication**: Sends IDENTIFY or RESUME payload based on session state
3. **Ready State**: Receives READY event and begins heartbeat mechanism
4. **Event Processing**: Continuously receives and processes events
5. **Reconnection**: Automatically handles disconnections and reconnects

### Event Flow

```rust
// Events flow from Gateway -> Client -> EventHandler
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // This event came through the Gateway
        println!("Received message: {}", msg.content.unwrap_or_default());
    }
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Gateway is ready and authenticated
        println!("Bot {} is ready with {} guilds", 
                ready.user.username, ready.guilds.len());
    }
}
```

## Heartbeat System

The Gateway implements a robust heartbeat mechanism to maintain connection health:

### Automatic Heartbeats

```rust
// Heartbeats are sent automatically every 30 seconds
// This is handled internally by the Gateway
```

The heartbeat system:
- Sends heartbeat packets every 30 seconds (fixed interval)
- Tracks heartbeat acknowledgments from the server
- Monitors connection health and detects timeouts
- Automatically terminates unhealthy connections

### Heartbeat Monitoring

```rust
// Internal heartbeat tracking (read-only access)
impl Gateway {
    pub fn is_ready(&self) -> bool {
        // Returns true if connection is authenticated and ready
    }
    
    pub fn last_sequence(&self) -> u64 {
        // Returns the last sequence number received
    }
    
    pub fn session_id(&self) -> Option<&str> {
        // Returns the current session ID if available
    }
}
```

## Connection Management

### Automatic Reconnection

The Gateway handles disconnections gracefully with exponential backoff:

```rust
// Reconnection is automatic with intelligent backoff
// - Initial attempts: 5 second delay
// - Subsequent attempts: exponential backoff (5, 10, 20, 40 seconds max)
// - Maximum attempts: unlimited until explicitly stopped
```

### Session Recovery

The Gateway maintains session state for seamless reconnection:

```rust
// Session state is automatically preserved
// - Session ID for session resumption
// - Last sequence number for event continuity
// - Connection state for proper recovery
```

### Connection States

```rust
use botrs::Gateway;

// Check connection status
if gateway.is_ready() {
    println!("Gateway is connected and ready");
}

if gateway.can_reconnect() {
    println!("Gateway can attempt reconnection");
}

// Access session information
if let Some(session_id) = gateway.session_id() {
    println!("Current session: {}", session_id);
}

let last_seq = gateway.last_sequence();
println!("Last sequence number: {}", last_seq);
```

## Event Handling

### System Events

The Gateway handles several system-level events automatically:

**HELLO Event**
- Receives server heartbeat interval
- Triggers authentication process
- Initializes heartbeat mechanism

**READY Event**
- Confirms successful authentication
- Provides bot information and guild list
- Starts regular heartbeat transmission

**HEARTBEAT_ACK Event**
- Acknowledges heartbeat packets
- Used for connection health monitoring
- Tracks round-trip latency

**RECONNECT Event**
- Server requests reconnection
- Triggers graceful connection reset
- Preserves session state

**INVALID_SESSION Event**
- Indicates session has expired
- Forces full re-authentication
- Clears session state

### Dispatch Events

All other events are dispatched to your EventHandler:

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Regular message events
    }
    
    async fn guild_member_add(&self, ctx: Context, member: Member) {
        // Member join events
    }
    
    async fn guild_create(&self, ctx: Context, guild: Guild) {
        // Guild events
    }
    
    // Handle unknown events
    async fn unknown_event(&self, event_type: String, data: serde_json::Value) {
        println!("Unknown event: {}", event_type);
    }
}
```

## Advanced Gateway Usage

### Direct Gateway Access

While the Client manages the Gateway automatically, you can access it for advanced use cases:

```rust
use botrs::{Gateway, Token, Intents};
use tokio::sync::mpsc;

// Create gateway manually
let token = Token::new("app_id", "secret");
let intents = Intents::GUILD_MESSAGES | Intents::GUILDS;
let mut gateway = Gateway::new(
    "wss://api.sgroup.qq.com/websocket",
    token,
    intents,
    None, // No sharding
);

// Set up event channel
let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

// Connect and handle events manually
tokio::spawn(async move {
    if let Err(e) = gateway.connect(event_sender).await {
        eprintln!("Gateway error: {}", e);
    }
});

// Process events manually
while let Some(event) = event_receiver.recv().await {
    println!("Received event: {:?}", event.event_type);
}
```

### Gateway Configuration

```rust
use botrs::{Gateway, Token, Intents};

// Basic gateway setup
let gateway = Gateway::new(
    "wss://api.sgroup.qq.com/websocket", // Gateway URL
    Token::new("app_id", "secret"),      // Authentication
    Intents::non_privileged(),           // Event subscriptions
    None,                                // Sharding info
);

// With sharding (for large bots)
let shard_info = [0, 4]; // Shard 0 of 4 total shards
let gateway = Gateway::new(
    gateway_url,
    token,
    intents,
    Some(shard_info),
);
```

## Error Handling

### Connection Errors

The Gateway handles various connection scenarios:

```rust
// Connection errors are logged and trigger reconnection
// - Network timeouts
// - WebSocket protocol errors
// - Server-side disconnections
// - Authentication failures
```

### Error Recovery

```rust
impl EventHandler for MyBot {
    async fn error(&self, error: BotError) {
        match error {
            BotError::WebSocket(_) => {
                // WebSocket connection issues
                // Gateway will automatically reconnect
                eprintln!("WebSocket error, reconnecting...");
            }
            BotError::Gateway(msg) => {
                // Gateway-specific errors
                eprintln!("Gateway error: {}", msg);
            }
            BotError::AuthenticationFailed(_) => {
                // Authentication problems
                // May require token refresh
                eprintln!("Authentication failed");
            }
            _ => {
                eprintln!("Other error: {}", error);
            }
        }
    }
}
```

### Close Code Handling

The Gateway responds appropriately to different close codes:

```rust
// Close code handling (internal)
// - 4004: Authentication failed - reset session
// - 9001, 9005: Invalid session - create new connection
// - Others: Attempt reconnection with session resume
```

## Performance Considerations

### Memory Usage

The Gateway is designed for efficient memory usage:

- Minimal state tracking (session ID, sequence number, heartbeat status)
- Efficient JSON parsing with serde
- Connection pooling for HTTP requests
- Automatic cleanup of resources

### Network Efficiency

- Compression support for WebSocket messages
- Intelligent reconnection with exponential backoff
- Heartbeat optimization (30-second fixed interval)
- Event filtering based on intents

### Concurrency

```rust
// Gateway operations are async and non-blocking
// - Event processing doesn't block heartbeats
// - Reconnection doesn't block event handling
// - Multiple Gateway instances can run concurrently (sharding)
```

## Monitoring and Debugging

### Connection Monitoring

```rust
use tracing::{info, debug, warn};

// Gateway provides extensive logging
// Set log level to DEBUG for detailed information
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

// Logs include:
// - Connection attempts and duration
// - Heartbeat timing and acknowledgments
// - Event counts and types
// - Error conditions and recovery
```

### Health Metrics

```rust
// Internal metrics tracked by Gateway:
// - Total connection time
// - Heartbeat count and timing
// - Last heartbeat acknowledgment
// - Sequence number tracking
// - Connection state changes
```

### Debugging Tips

1. **Enable Debug Logging**: Set log level to DEBUG for detailed Gateway activity
2. **Monitor Heartbeats**: Watch for heartbeat timing issues
3. **Check Session State**: Verify session ID and sequence numbers
4. **Network Connectivity**: Ensure stable internet connection
5. **Token Validity**: Verify authentication credentials

## Best Practices

### Production Deployment

1. **Robust Error Handling**: Implement comprehensive error handling in your EventHandler
2. **Graceful Shutdown**: Use proper signal handling for clean disconnection
3. **Health Monitoring**: Monitor Gateway health and connection metrics
4. **Log Management**: Configure appropriate log levels for production

### Development Tips

1. **Use Sandbox**: Test with sandbox environment before production
2. **Intent Optimization**: Only subscribe to events you actually need
3. **Rate Limiting**: Be aware of Gateway rate limits and connection frequency
4. **Session Management**: Understand session lifecycle for debugging

### Scaling Considerations

```rust
// For large bots (2500+ guilds), implement sharding:
let total_shards = 4;
for shard_id in 0..total_shards {
    let shard_info = [shard_id, total_shards];
    let gateway = Gateway::new(gateway_url, token.clone(), intents, Some(shard_info));
    // Start each shard in separate task
}
```

The Gateway provides the real-time foundation for your QQ Guild bot, handling all the complexities of WebSocket connection management, authentication, and event delivery. By understanding its capabilities and following best practices, you can build reliable bots that maintain stable connections and process events efficiently.
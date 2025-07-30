# Token API Reference

The `Token` struct manages authentication credentials for QQ Guild Bot API. It handles app ID and secret storage, access token generation, and automatic token refresh for secure API communication.

## Overview

```rust
use botrs::{Token, Result};

// Create token with credentials
let token = Token::new("your_app_id", "your_secret");

// Create from environment variables
let token = Token::from_env()?;

// Get authorization header for API requests
let auth_header = token.authorization_header().await?;
```

The `Token` handles the OAuth 2.0 flow with QQ's API, automatically fetching and refreshing access tokens as needed.

## Constructor Methods

### `new`

Creates a new token with the provided app ID and secret.

```rust
pub fn new(app_id: impl Into<String>, secret: impl Into<String>) -> Self
```

#### Parameters

- `app_id`: The bot's application ID from QQ Developer Portal
- `secret`: The bot's secret key from QQ Developer Portal

#### Returns

A new `Token` instance.

#### Example

```rust
let token = Token::new("123456789", "your_secret_key");
```

### `from_env`

Creates a token from environment variables.

```rust
pub fn from_env() -> Result<Self>
```

Looks for the following environment variables:
- `QQ_BOT_APP_ID`: The bot's application ID
- `QQ_BOT_SECRET`: The bot's secret key

#### Returns

A `Result` containing the token if both environment variables are found.

#### Example

```rust
// Set environment variables first:
// export QQ_BOT_APP_ID=123456789
// export QQ_BOT_SECRET=your_secret_key

let token = Token::from_env()?;
```

## Access Methods

### `app_id`

Gets the application ID.

```rust
pub fn app_id(&self) -> &str
```

#### Returns

A string slice containing the app ID.

#### Example

```rust
let token = Token::new("123456789", "secret");
assert_eq!(token.app_id(), "123456789");
```

### `secret`

Gets the secret key.

```rust
pub fn secret(&self) -> &str
```

#### Returns

A string slice containing the secret.

**Warning**: Be careful when using this method. Avoid logging or exposing the secret.

#### Example

```rust
let token = Token::new("app_id", "secret123");
assert_eq!(token.secret(), "secret123");
```

## Authentication Methods

### `authorization_header`

Generates the authorization header value for API requests.

```rust
pub async fn authorization_header(&self) -> Result<String>
```

This method automatically handles:
- Fetching access tokens from QQ's API
- Token caching and refresh
- Error handling for authentication failures

#### Returns

A `Result` containing the authorization header value in the format "QQBot {access_token}".

#### Example

```rust
let token = Token::new("valid_app_id", "valid_secret");
let auth_header = token.authorization_header().await?;
assert!(auth_header.starts_with("QQBot "));

// Use with HTTP client
let response = client
    .get("https://api.sgroup.qq.com/users/@me")
    .header("Authorization", auth_header)
    .send()
    .await?;
```

### `bot_token`

Generates the bot token for WebSocket authentication.

```rust
pub async fn bot_token(&self) -> Result<String>
```

This is an alias for `authorization_header()` that provides the same token format required for gateway connections.

#### Returns

A `Result` containing the bot token string.

#### Example

```rust
let token = Token::new("app_id", "secret");
let bot_token = token.bot_token().await?;

// Use for WebSocket authentication
let identify = Identify {
    token: bot_token,
    intents: intents.bits(),
    // ... other fields
};
```

## Validation Methods

### `validate`

Validates that the token has non-empty app ID and secret.

```rust
pub fn validate(&self) -> Result<()>
```

#### Returns

`Ok(())` if the token is valid, otherwise returns a `BotError::Auth`.

#### Example

```rust
let token = Token::new("123", "secret");
assert!(token.validate().is_ok());

let invalid_token = Token::new("", "secret");
assert!(invalid_token.validate().is_err());
```

## Utility Methods

### `safe_display`

Safely formats the token for logging purposes.

```rust
pub fn safe_display(&self) -> String
```

This method masks the secret to prevent accidental exposure in logs while keeping the app ID visible.

#### Returns

A string representation safe for logging.

#### Example

```rust
let token = Token::new("123456", "verylongsecret123");
let display = token.safe_display();
println!("{}", display); // "Token { app_id: 123456, secret: very****123 }"
```

## Error Handling

Token operations can fail for several reasons:

### Authentication Errors

```rust
match token.authorization_header().await {
    Ok(header) => {
        // Use the header for API calls
    }
    Err(BotError::Auth(msg)) => {
        eprintln!("Authentication failed: {}", msg);
        // Check app ID and secret
    }
    Err(BotError::Connection(msg)) => {
        eprintln!("Connection error: {}", msg);
        // Check network connectivity
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

### Common Error Scenarios

**Invalid Credentials**
```rust
// Wrong app ID or secret
let token = Token::new("invalid_id", "wrong_secret");
match token.authorization_header().await {
    Err(BotError::Api { code: 401, .. }) => {
        println!("Invalid credentials");
    }
    _ => {}
}
```

**Network Issues**
```rust
// Connection timeout or network error
match token.authorization_header().await {
    Err(BotError::Connection(msg)) => {
        println!("Network error: {}", msg);
        // Implement retry logic
    }
    _ => {}
}
```

**Configuration Errors**
```rust
// Missing environment variables
match Token::from_env() {
    Err(BotError::Config(msg)) => {
        println!("Configuration error: {}", msg);
        // Check environment variables
    }
    _ => {}
}
```

## Security Considerations

### Secret Protection

The `Token` struct implements several security measures:

1. **Debug Safety**: The `Debug` implementation redacts the secret
2. **Safe Display**: The `safe_display()` method masks sensitive parts
3. **No Secret Exposure**: Avoid calling `secret()` unless absolutely necessary

```rust
let token = Token::new("app_id", "secret123");

// Safe for logging
println!("{:?}", token); // Shows [REDACTED] for secret
println!("{}", token.safe_display()); // Shows masked secret

// Avoid this in production logs
println!("Secret: {}", token.secret()); // Exposes full secret
```

### Environment Variables

When using `from_env()`, ensure environment variables are secure:

```bash
# Good: Set in secure environment
export QQ_BOT_APP_ID=123456789
export QQ_BOT_SECRET=your_secret_key

# Bad: Don't put in shell history or scripts
echo "QQ_BOT_SECRET=secret123" >> ~/.bashrc
```

### Token Storage

```rust
// Good: Create token when needed
async fn create_bot() -> Result<()> {
    let token = Token::from_env()?;
    let client = Client::new(app_id, handler)
        .token(token)
        .build()
        .await?;
    Ok(())
}

// Avoid: Storing tokens in plain text files
// Don't serialize tokens to JSON/YAML configuration files
```

## Advanced Usage

### Custom Token Refresh

The token automatically refreshes access tokens, but you can observe the process:

```rust
use tracing::{info, warn};

let token = Token::new("app_id", "secret");

// This will trigger token fetch on first call
match token.authorization_header().await {
    Ok(header) => {
        info!("Successfully obtained access token");
        // Token is cached for subsequent calls
    }
    Err(e) => {
        warn!("Token fetch failed: {}", e);
    }
}

// Subsequent calls use cached token (if not expired)
let header2 = token.authorization_header().await?;
```

### Token Validation in Production

```rust
async fn validate_bot_config() -> Result<()> {
    let token = Token::from_env()?;
    
    // Validate credentials format
    token.validate()?;
    
    // Test actual authentication
    match token.authorization_header().await {
        Ok(_) => {
            println!("Bot credentials verified");
            Ok(())
        }
        Err(e) => {
            eprintln!("Credential verification failed: {}", e);
            Err(e)
        }
    }
}
```

### Multiple Bot Support

```rust
struct MultiBotManager {
    bots: HashMap<String, Token>,
}

impl MultiBotManager {
    fn new() -> Self {
        Self {
            bots: HashMap::new(),
        }
    }
    
    fn add_bot(&mut self, name: String, app_id: String, secret: String) {
        let token = Token::new(app_id, secret);
        self.bots.insert(name, token);
    }
    
    async fn get_auth_header(&self, bot_name: &str) -> Result<String> {
        let token = self.bots.get(bot_name)
            .ok_or_else(|| BotError::config("Bot not found"))?;
        token.authorization_header().await
    }
}
```

## Integration Examples

### With HTTP Client

```rust
use reqwest::Client;

async fn make_api_call(token: &Token) -> Result<serde_json::Value> {
    let client = Client::new();
    let auth_header = token.authorization_header().await?;
    
    let response = client
        .get("https://api.sgroup.qq.com/users/@me")
        .header("Authorization", auth_header)
        .send()
        .await?;
    
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        Err(BotError::api(
            response.status().as_u16() as u32,
            "API call failed".to_string()
        ))
    }
}
```

### With BotRS Client

```rust
use botrs::{Client, EventHandler};

struct MyBot;

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot {} is ready!", ready.user.username);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::from_env()?;
    let bot = MyBot;
    
    let mut client = Client::new(token.app_id(), bot)
        .token(token)
        .build()
        .await?;
    
    client.start().await?;
    Ok(())
}
```

## Best Practices

### Development Environment

```rust
// Use environment variables for development
let token = match Token::from_env() {
    Ok(token) => token,
    Err(_) => {
        eprintln!("Please set QQ_BOT_APP_ID and QQ_BOT_SECRET");
        std::process::exit(1);
    }
};
```

### Production Deployment

```rust
// Validate configuration at startup
async fn initialize_bot() -> Result<()> {
    let token = Token::from_env()
        .map_err(|e| {
            eprintln!("Configuration error: {}", e);
            e
        })?;
    
    // Test credentials before starting
    token.authorization_header().await
        .map_err(|e| {
            eprintln!("Authentication test failed: {}", e);
            e
        })?;
    
    println!("Bot configuration validated");
    Ok(())
}
```

### Error Recovery

```rust
async fn robust_api_call(token: &Token) -> Result<serde_json::Value> {
    const MAX_RETRIES: u32 = 3;
    
    for attempt in 1..=MAX_RETRIES {
        match token.authorization_header().await {
            Ok(auth_header) => {
                // Make API call with header
                return make_request_with_auth(auth_header).await;
            }
            Err(BotError::Auth(_)) => {
                // Authentication failed, don't retry
                return Err(BotError::auth("Authentication failed permanently"));
            }
            Err(e) if attempt < MAX_RETRIES => {
                eprintln!("Attempt {} failed: {}, retrying...", attempt, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    
    unreachable!()
}
```

## See Also

- [`Client`](./client.md) - Bot client that uses tokens
- [`Context`](./context.md) - Context provides token access in handlers
- [`BotApi`](./bot-api.md) - API client that requires authentication
- [Error Types](./error-types.md) - Authentication error handling
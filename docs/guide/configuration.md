# Configuration

`botrs` deliberately keeps its configuration surface tiny: a `Token`, an `Intents` set, a sandbox flag, and an HTTP timeout. Everything else is library-agnostic and stays in your own application.

## Token

`Token::new(app_id, secret)` is the basic constructor. `Token::from_env()` reads `QQ_BOT_APP_ID` and `QQ_BOT_SECRET` from the process environment, validates them, and returns `BotError::Config` if either is missing.

```rust
let token = Token::from_env()?;
token.validate()?; // optional; from_env already calls this
```

`Token::validate()` returns `BotError::Auth` when `app_id` or `secret` is empty. There are no other format checks — the gateway and REST endpoints reject bad credentials at runtime.

For logging, prefer `token.safe_display()` over `Debug`/`Display` of the raw token: it masks the secret with `first4****last4`.

```rust
tracing::info!("loaded {}", token.safe_display());
```

## Intents

`Intents` is described in detail in [intents](/guide/intents). The relevant configuration shape is:

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message();
```

Choose the smallest set you need; the gateway only delivers events for enabled flags.

## Sandbox vs production

`Client::new(token, intents, handler, is_sandbox: bool)` — pass `true` to point the HTTP client and gateway at QQ's sandbox endpoints. The two endpoint constants are exported for reference:

```rust
botrs::DEFAULT_API_URL  // "https://api.sgroup.qq.com"
botrs::SANDBOX_API_URL  // "https://sandbox.api.sgroup.qq.com"
botrs::DEFAULT_WS_URL   // "wss://api.sgroup.qq.com/websocket"
```

Internally, the client builds an `HttpClient` with `HttpClient::new(timeout_secs, is_sandbox)`. If you construct a `BotApi` outside of `Client` (for one-off REST calls), use the same constructor to pick the environment.

## HTTP timeout

`Client::new` uses the constant `botrs::DEFAULT_TIMEOUT` (30 seconds). Override it with `Client::with_config(token, intents, handler, timeout_secs, is_sandbox)`. The timeout applies to each HTTP request individually; long-polling style operations are not used by the framework.

```rust
let client = Client::with_config(token, intents, MyBot, 60, false)?;
```

If you build your own `HttpClient`, the same timeout argument applies:

```rust
let http = botrs::http::HttpClient::new(60, false)?;
let api = botrs::BotApi::new(http);
```

## Logging

The framework uses `tracing` and emits no logs unless you initialize a subscriber:

```rust
tracing_subscriber::fmt()
    .with_env_filter("botrs=info")
    .init();
```

Bumping the filter to `botrs=debug` enables gateway-frame and HTTP-debug traces.

## What `botrs` does not configure

The framework reads no config files of its own. It does not require, set, or interpret variables beyond the two `Token::from_env()` looks for. Application-level concerns — config file format, secrets management, runtime tuning, observability stacks — stay in your code, not in the framework.

# Token

`Token` carries the bot's `app_id` and `secret`, signs API requests, and caches an access token shared across `Token` clones. The cache is refreshed on demand and protected by an internal mutex, so cloning `Token` is cheap and safe across tasks.

```rust
pub struct Token { /* private fields */ }
```

`Token` implements `Clone`, `Debug`, `Serialize`, `Deserialize`. The serialized form drops the cache so credentials embedded in config files round-trip cleanly.

## Construction

- `Token::new(app_id, secret)` — explicit credentials.
- `Token::from_env()` — read `QQ_BOT_APP_ID` / `QQ_BOT_SECRET`. Returns `BotError::Config` if either variable is missing.

```rust
let token = Token::new("123456789", "abcdef...");
let token = Token::from_env()?;       // env-driven
```

## Accessors

- `app_id()` / `secret()` — borrow the credential strings (returns `&str`).
- `safe_display()` — returns a redacted human-readable form for logs (`Token { app_id: "1234***", secret: "***" }`). Use this anywhere you would otherwise be tempted to print the raw token.

## Authentication

- `authorization_header().await` — `Authorization: QQBot <access-token>` string suitable for HTTP requests; refreshes the cached token if it has expired.
- `bot_token().await` — just the bot token portion (without the `QQBot ` prefix), used by the gateway `Identify` payload.

Both methods share the same access-token cache. Concurrent callers will block briefly on the refresh mutex when a renewal is needed and then race-free read the freshly minted token.

## Validation

`validate()` runs lightweight syntactic checks (non-empty app id, sane lengths) and returns `BotError::Config` on failure. Use it after `Token::new(...)` to fail fast before issuing any request.

## Refresh behaviour

The framework tracks the access token's expiry and automatically renews it shortly before it lapses. If a renewal fails, the next call surfaces a `BotError::Auth` error with the upstream message. There is no exposed `refresh()` method — refresh is an implementation detail driven by `authorization_header()` / `bot_token()`.

When clones share the same internal `Arc<Mutex<TokenState>>`, a renewal triggered on one clone is immediately visible to all others, so the framework never refreshes more than once per expiry window even under heavy concurrency.

## Examples

**Build a token from the environment and validate it once at startup:**

```rust
let token = Token::from_env()?;
token.validate()?;
```

**Construct an HTTP client manually:**

```rust
let auth = token.authorization_header().await?;
let request = client
    .get("https://api.example.com/whatever")
    .header("Authorization", auth)
    .send()
    .await?;
```

**Log a token without leaking secrets:**

```rust
info!("starting bot {}", token.safe_display());
```

## Security notes

- Never `Debug`-print a `Token`; always go through `safe_display()`.
- Prefer reading credentials from env vars or a secret manager. Keep them out of source-controlled config files.
- Rotate the secret in the QQ Developer Portal if you suspect a leak; the next `authorization_header()` call after rotation will fail with `BotError::Auth`, and you can rebuild the `Token` with fresh values.

## See also

- [Bot API](./bot-api.md) — every route ultimately consumes a `Token`.
- [Context](./context.md) — embeds a `Token` so handlers don't pass it around manually.
- [Error types](./error-types.md) — `BotError::Auth` and `BotError::Config` behaviour referenced above.

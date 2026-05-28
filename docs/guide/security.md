# Security

The security boundary is mostly token handling, environment selection, and logging. Bot business code usually runs inside gateway event callbacks and then calls REST through the event session.

## Token

`Token` holds the `app_id` and `secret`. Do not write the secret into the repository, logs, or panic output. Use `safe_display()` when you need a log-friendly representation:

```rust
tracing::info!("starting with {}", token.safe_display());
```

Prefer `Token::from_env()` for loading credentials:

- `QQ_BOT_APP_ID`
- `QQ_BOT_SECRET`

`Token::validate()` only checks that fields are non-empty. Real authentication results come from REST calls or gateway identify.

## Sandbox and Production

The last argument to `Client::new(token, intents, handler, is_sandbox)` chooses sandbox or production. Use sandbox during development and validation, then switch to production after event, permission, and message flows are confirmed.

When constructing `HttpClient` independently, pass the same sandbox flag:

```rust
let http = HttpClient::new(30, true)?;
```

## Logging

The gateway and HTTP layers emit diagnostics through `tracing`. In production, avoid broad debug logging that collects raw request bodies into external systems, especially logs containing user message content, openids, or token-related errors.

## See Also

- [Token](../api/token.md)
- [Configuration](./configuration.md)
- [Gateway](./gateway.md)

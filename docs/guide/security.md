# Security

The framework's security surface is small: keep the bot's `Token` out of logs and source control, verify webhook callbacks before trusting them, and pick the right environment (sandbox vs production) for what you're doing. The rest is application-level concerns the framework doesn't try to manage.

## Token handling

A `Token` is `app_id` + `secret`. Don't print or serialize it raw — `Token` deliberately uses an opaque `Debug` impl, but format strings, JSON serializers, and panic backtraces can still leak it. Use `safe_display()` whenever you need to surface a token in a log line:

```rust
tracing::info!("starting with {}", token.safe_display());
// "Token { app_id: 1234, secret: 1234****abcd }"
```

`Token::from_env()` is the recommended loader; it reads `QQ_BOT_APP_ID` and `QQ_BOT_SECRET` and validates that neither is empty (returns `BotError::Config` otherwise). Never check the actual values into the repository — keep them in env vars, a secrets manager, or a file outside the working tree.

`Token::validate()` only enforces non-emptiness. It is not an authentication round-trip; the gateway / REST APIs decide whether the credentials are accepted. A successful `client.start()` is the real signal.

## Webhook signature verification

If you accept QQ's HTTP callbacks (instead of running a gateway connection), every dispatch carries an Ed25519 signature in `X-Signature-Ed25519`, with the timestamp in `X-Signature-Timestamp`. The framework exposes verification helpers in `botrs::signature`:

```rust
use botrs::signature::{HEADER_SIGNATURE, HEADER_TIMESTAMP, verify};

fn handle(headers: &reqwest::header::HeaderMap, body: &[u8], bot_secret: &str) -> botrs::Result<()> {
    if !verify(bot_secret, headers, body)? {
        return Err(botrs::BotError::auth("bad signature"));
    }
    // signature is valid; deserialize body and dispatch
    Ok(())
}
```

`verify` reads `X-Signature-Ed25519` (hex-encoded signature) and `X-Signature-Timestamp` from the header map, derives a signing key from your bot secret using the scheme QQ documents for interaction callbacks, and returns `Ok(true)` only when the signature checks out. `generate(secret, headers, body)` is the inverse, useful for tests or when you mirror QQ's signing scheme yourself. The header-name constants `HEADER_SIGNATURE` and `HEADER_TIMESTAMP` are exported so you don't hard-code them.

Reject any request that fails `verify` immediately. Accepting an unsigned or wrongly signed body lets an attacker forge dispatches in your handler.

## Sandbox vs production

`Client::new(token, intents, handler, is_sandbox)` chooses the base URLs. Use the sandbox while developing; the gateway and REST endpoints accept the same `Token` but operate against an isolated environment with rate limits and quotas tuned for testing. Only flip to `is_sandbox = false` once you're ready for real users.

The sandbox URL is exposed as `botrs::SANDBOX_API_URL` (`https://sandbox.api.sgroup.qq.com`), production as `botrs::DEFAULT_API_URL` (`https://api.sgroup.qq.com`).

## What this guide doesn't cover

Generic operational hardening (TLS termination, secret rotation cadence, intrusion detection, rate-limiting middleware, audit logging, vulnerability scanning) is out of scope: those concerns live in your deployment, not in the bot framework. Use whatever your team already uses.

# Installation

Add `botrs` to your `Cargo.toml` together with an async runtime:

```toml
[dependencies]
botrs = "0.14.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

BotRS requires Rust 1.88 or newer. Use `async-trait` to implement individual `EventHandler` callbacks. Closure handlers only need `botrs` and Tokio.

## Feature flags

The crate ships with no default features. The only feature you may want to opt into is `examples`, which pulls in `toml` and `tracing-subscriber` for the binaries under `examples/`. Library users do not need it.

```toml
botrs = { version = "0.14.0", features = ["examples"] }
```

## Sanity check

```rust
use botrs::{Token, Intents};

fn main() {
    let _ = Token::new("app_id", "secret");
    let _ = Intents::default();
    println!("botrs {}", botrs::VERSION);
}
```

If this compiles you have a working install. From here:

- [Quick start](/guide/quick-start) for a minimal end-to-end bot.
- [Configuration](/guide/configuration) for `Token::from_env` and sandbox vs production.

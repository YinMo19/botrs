# Installation

Getting started with BotRS is straightforward. This guide will walk you through adding BotRS to your Rust project and setting up the necessary dependencies.

## Prerequisites

Before installing BotRS, ensure you have the following:

- **Rust 1.70 or later**: BotRS uses modern Rust features and requires a recent compiler version
- **Cargo**: Rust's package manager (included with Rust installations)
- **QQ Guild Bot Credentials**: App ID and Secret from the QQ Guild Developer Portal

### Installing Rust

If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) and follow the installation instructions for your platform:

```bash
# Install Rust (Unix-like systems)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Obtaining Bot Credentials

1. Visit the [QQ Guild Developer Portal](https://bot.q.qq.com/)
2. Create a new application or select an existing one
3. Note your **App ID** and **Secret** - you'll need these for authentication

## Adding BotRS to Your Project

### Creating a New Project

Start by creating a new Rust project:

```bash
cargo new my-qq-bot
cd my-qq-bot
```

### Adding Dependencies

Add BotRS and required dependencies to your `Cargo.toml`:

```toml
[dependencies]
# BotRS framework
botrs = "0.2.5"

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Logging (recommended)
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Async trait support
async-trait = "0.1"
```

### Optional Dependencies

Depending on your bot's requirements, you might want to add:

```toml
[dependencies]
# For configuration files
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# For command line argument parsing
clap = { version = "4.0", features = ["derive"] }

# For environment variable handling
dotenvy = "0.15"

# For HTTP client customization
reqwest = { version = "0.11", features = ["json"] }

# For date/time handling
chrono = { version = "0.4", features = ["serde"] }
```

## Feature Flags

BotRS provides several feature flags to customize your installation:

```toml
[dependencies]
botrs = { version = "0.2.5", features = ["examples"] }
```

### Available Features

- **`examples`**: Enables example-specific dependencies (clap, toml)
- **Default features**: Core functionality is always included

## Verifying Installation

Create a simple test to verify BotRS is properly installed:

```rust
// src/main.rs
use botrs::{Client, EventHandler, Intents, Token};

struct TestBot;

#[async_trait::async_trait]
impl EventHandler for TestBot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BotRS installed successfully!");
    println!("Version: {}", botrs::VERSION);
    Ok(())
}
```

Run the test:

```bash
cargo run
```

You should see output indicating BotRS is properly installed and the version number.

## Development Dependencies

For development and testing, consider adding these dependencies:

```toml
[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
```

## IDE Setup

### VS Code

For the best development experience with VS Code:

1. Install the **rust-analyzer** extension
2. Configure your settings:

```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### Other IDEs

- **IntelliJ IDEA**: Use the Rust plugin
- **Vim/Neovim**: Configure with rust-analyzer LSP
- **Emacs**: Use rustic-mode with rust-analyzer

## Common Installation Issues

### Compilation Errors

**Issue**: Compilation fails with missing features
```
error: failed to resolve: use of undeclared crate or module `tokio`
```

**Solution**: Ensure all required features are enabled:
```toml
tokio = { version = "1.0", features = ["full"] }
```

**Issue**: Async trait compilation errors
```
error: async trait not supported
```

**Solution**: Add the async-trait dependency:
```toml
async-trait = "0.1"
```

### Version Conflicts

**Issue**: Dependency version conflicts
```
error: failed to select a version for `serde`
```

**Solution**: Use `cargo tree` to identify conflicts and specify compatible versions:
```bash
cargo tree
```

### Network Issues

**Issue**: Downloads fail due to network restrictions

**Solution**: Configure cargo to use a proxy or mirror:
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://crates.io/api/v1/crates"
```

## Environment Setup

### Environment Variables

Create a `.env` file for development (optional):

```bash
# .env
QQ_BOT_APP_ID=your_app_id_here
QQ_BOT_SECRET=your_secret_here
RUST_LOG=botrs=debug,my_qq_bot=info
```

Add to `.gitignore`:
```gitignore
.env
target/
Cargo.lock  # for libraries only
```

### Configuration File

Create a configuration template:

```toml
# config.toml
[bot]
app_id = "your_app_id"
secret = "your_secret"
sandbox = false

[logging]
level = "info"
```

## Docker Setup (Optional)

For containerized deployment:

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-qq-bot /usr/local/bin/my-qq-bot

CMD ["my-qq-bot"]
```

## Next Steps

Now that BotRS is installed, you're ready to:

1. **[Quick Start](/guide/quick-start)** - Create your first bot
2. **[Configuration](/guide/configuration)** - Set up bot credentials
3. **[Client & Event Handler](/guide/client-handler)** - Learn the core concepts

## Troubleshooting

If you encounter issues during installation:

1. Check the [GitHub Issues](https://github.com/YinMo19/botrs/issues) page
2. Verify your Rust version: `rustc --version`
3. Update your toolchain: `rustup update`
4. Clean and rebuild: `cargo clean && cargo build`

For additional help, feel free to open an issue on the GitHub repository with:
- Your Rust version
- Complete error messages
- Your `Cargo.toml` configuration
# BotRS Examples

This directory contains complete examples for developing QQ Guild bots using the BotRS framework. These examples are Rust implementations equivalent to the Python botpy examples.

## Directory Structure

```
examples/
├── README.md                    # This file
├── config.example.toml          # Example configuration file
├── common/                      # Common utilities for examples
│   ├── mod.rs                   # Common module exports
│   └── config.rs                # Configuration loading utilities
├── simple_bot.rs                # Basic bot with simple message handling
├── demo_at_reply.rs             # Bot @ reply example
├── demo_at_reply_command.rs     # Bot @ reply with command system
├── demo_at_reply_embed.rs       # Bot @ reply with embed messages
├── demo_at_reply_markdown.rs    # Bot @ reply with markdown messages
├── demo_at_reply_keyboard.rs    # Bot @ reply with keyboard messages
├── demo_at_reply_file_data.rs   # Bot @ reply with file uploads
├── demo_at_reply_reference.rs   # Bot @ reply with message references
├── demo_group_reply_text.rs     # Bot group message reply example
├── demo_c2c_reply_text.rs       # Bot C2C (private) message reply example
├── demo_dms_reply.rs            # Bot direct message reply example
└── demo_recall.rs               # Bot message recall (delete) example
```

## Setup

### 1. Install Dependencies

Make sure you have Rust installed, then build the project:

```bash
cargo build --features examples
```

### 2. Configuration

Copy the example configuration file and fill in your bot credentials:

```bash
cp config.example.toml config.toml
```

Edit `config.toml` and provide your bot's App ID and Secret:

```toml
[bot]
app_id = "your_bot_app_id"
secret = "your_bot_secret"
```

### 3. Alternative Configuration Methods

You can also configure the bot using:

#### Environment Variables:
```bash
export QQ_BOT_APP_ID="your_bot_app_id"
export QQ_BOT_SECRET="your_bot_secret"
```

#### Command Line Arguments:
```bash
cargo run --example demo_at_reply --features examples your_bot_app_id your_bot_secret
```

## Running Examples

### Basic AT Reply Bot
```bash
cargo run --example demo_at_reply --features examples
```

### Command-based AT Reply Bot
```bash
cargo run --example demo_at_reply_command --features examples
```

### Embed Message Bot
```bash
cargo run --example demo_at_reply_embed --features examples
```

### Markdown Message Bot
```bash
cargo run --example demo_at_reply_markdown --features examples
```

### Keyboard Message Bot
```bash
cargo run --example demo_at_reply_keyboard --features examples
```

### Group Message Bot
```bash
cargo run --example demo_group_reply_text --features examples
```

### C2C (Private) Message Bot
```bash
cargo run --example demo_c2c_reply_text --features examples
```

### File Upload Bot
```bash
cargo run --example demo_at_reply_file_data --features examples
```

### Message Recall Bot
```bash
cargo run --example demo_recall --features examples
```

### Direct Message Bot
```bash
cargo run --example demo_dms_reply --features examples
```

### Message Reference Bot
```bash
cargo run --example demo_at_reply_reference --features examples
```

### Simple Bot (Original)
```bash
cargo run --example simple_bot --features examples
```

## Example Descriptions

### demo_at_reply.rs
- **Python equivalent**: `demo_at_reply.py`
- **Features**: Basic @ mention handling, async message processing, sleep command
- **Intents**: `public_guild_messages`

### demo_at_reply_command.rs
- **Python equivalent**: `demo_at_reply_command.py`
- **Features**: Command system with aliases, parameter parsing, dual sending methods
- **Commands**: `你好/hello`, `晚安`
- **Intents**: `public_guild_messages`

### demo_at_reply_embed.rs
- **Python equivalent**: `demo_at_reply_embed.py`
- **Features**: Rich embed messages with fields, colors, and formatting
- **Intents**: `public_guild_messages`

### demo_at_reply_markdown.rs
- **Python equivalent**: `demo_at_reply_markdown.py`
- **Features**: Markdown messages with templates and custom content
- **Methods**: Template-based and content-based markdown
- **Intents**: `public_guild_messages`

### demo_at_reply_keyboard.rs
- **Python equivalent**: `demo_at_reply_keyboard.py`
- **Features**: Interactive keyboard messages with buttons and actions
- **Methods**: Template keyboards and custom-defined keyboards
- **Intents**: `public_guild_messages`

### demo_group_reply_text.rs
- **Python equivalent**: `demo_group_reply_text.py`
- **Features**: Group message handling and replies
- **Intents**: `public_messages`

### demo_c2c_reply_text.rs
- **Python equivalent**: `demo_c2c_reply_text.py`
- **Features**: C2C (private/friend) message handling and replies
- **Intents**: `public_messages`

### demo_at_reply_file_data.rs
- **Python equivalent**: `demo_at_reply_file_data.py`
- **Features**: File upload functionality with multiple methods (bytes, direct, path-based)
- **File types**: Images, documents, media files
- **Intents**: `public_guild_messages`

### demo_recall.rs
- **Python equivalent**: `demo_recall.py`
- **Features**: Message sending and immediate recall (deletion)
- **Methods**: Reply and recall with hide tip option
- **Intents**: `public_guild_messages`

### demo_dms_reply.rs
- **Python equivalent**: `demo_dms_reply.py`
- **Features**: Direct message handling, DM session creation, private message replies
- **Commands**: `/私信` to trigger DM session creation
- **Intents**: `direct_message`, `public_guild_messages`

### demo_at_reply_reference.rs
- **Python equivalent**: `demo_at_reply_reference.py`
- **Features**: Message references (replies to specific messages), emoji support
- **Methods**: Reference creation and reply with message context
- **Intents**: `public_guild_messages`

## Intent Types

Different examples use different intent types to receive specific event types:

- **`public_guild_messages`**: For receiving @ mentions in guild channels
- **`public_messages`**: For receiving group and C2C messages
- **`direct_message`**: For receiving direct messages (DMs)
- **`guilds`**: For receiving guild-related events

## Message Types

The examples demonstrate various message types:

- **Text messages**: Plain text content
- **Embed messages**: Rich embeds with titles, descriptions, fields, and colors
- **Markdown messages**: Formatted messages using markdown syntax
- **Keyboard messages**: Interactive messages with clickable buttons
- **Media messages**: Messages with attachments (files, images, etc.)

## Error Handling

All examples include proper error handling:
- Token validation
- API call error handling
- Event processing error handling
- Graceful degradation when optional data is missing

## Logging

Examples use the `tracing` crate for structured logging:
- Debug level for BotRS internal operations
- Info level for bot operations and successful actions
- Warn level for recoverable errors
- Error level for serious issues

You can control logging with the `RUST_LOG` environment variable:
```bash
RUST_LOG=debug cargo run --example demo_at_reply --features examples
```

## Development Tips

1. **Testing**: Use the sandbox environment by setting `sandbox = true` in your config
2. **Debugging**: Enable debug logging to see detailed API interactions
3. **Hot Reloading**: The bot will automatically reconnect on network issues
4. **Rate Limiting**: The framework handles rate limiting automatically

## Common Issues

### Invalid Token
- Ensure your App ID and Secret are correct
- Check that your bot has the necessary permissions
- Verify you're using the correct environment (production vs sandbox)

### No Events Received
- Check your intent configuration
- Ensure your bot is added to the target guild/group
- Verify network connectivity

### Message Send Failures
- Check bot permissions in the target channel/group
- Ensure the message content meets platform requirements
- Verify the target ID (channel_id, group_openid, user_openid) is correct

## Further Reading

- [BotRS Documentation](https://docs.rs/botrs)
- [QQ Guild Bot API Documentation](https://bot.q.qq.com/wiki/)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)
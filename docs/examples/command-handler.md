# Command Handler

BotRS does not ship a command framework. The reference pattern is in [`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs): a small `CommandRegistry` mapping aliases to a `fn(&str) -> Option<String>`, dispatched from `EventHandler::message_create`.

## The pattern

Build the registry once in your handler's constructor, then in each event call `registry.try_execute(content)` with the trimmed message text. The handler returns `Some(reply)` if it matched, `None` otherwise.

```rust
struct CommandRegistry { commands: Vec<(Vec<String>, fn(&str) -> Option<String>)> }

impl CommandRegistry {
    fn try_execute(&self, content: &str) -> Option<String> {
        let trimmed = content.trim();
        for (aliases, handler) in &self.commands {
            for alias in aliases {
                if trimmed.starts_with(alias) {
                    let params = trimmed[alias.len()..].trim();
                    return handler(params);
                }
            }
        }
        None
    }
}
```

The example registers two commands (`你好` / `hello` and `晚安`) and replies with `session.reply(&response)`. Use `send_markdown_message`, `send_embed_message`, `send_ark_message`, or `send_keyboard_message` when a command returns a richer payload.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Related: [Interactive Messages](./interactive-messages.md) for keyboard-driven UI
- Example: [`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs)

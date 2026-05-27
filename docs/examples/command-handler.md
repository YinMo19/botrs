# Command Handler

BotRS does not ship a command framework. The reference pattern is in [`examples/demo_at_reply_command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_command.rs): a small `CommandRegistry` mapping `Vec<&str>` aliases to a `fn(&str) -> Option<String>`, dispatched from `EventHandler::message_create`.

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

The demo registers two commands (`你好` / `hello` and `晚安`) and demonstrates two ways to send the response: the convenience `Message::reply(&ctx, &response)` and the explicit `MessageParams { content: Some(response), msg_id: message.id.clone(), ..Default::default() }` followed by `ctx.send_message(...)`. Use whichever fits — they hit the same endpoint.

## See also

- Guide: [`docs/guide/messages.md`](../guide/messages.md)
- Related: [Interactive Messages](./interactive-messages.md) for keyboard-driven UI
- Demo: [`examples/demo_at_reply_command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/demo_at_reply_command.rs)

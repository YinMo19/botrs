# 命令处理器

BotRS 自身不带命令框架。参考实现在 [`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs)：一个极小的 `CommandRegistry`，把别名映射到 `fn(&str) -> Option<String>`，由 `EventHandler::message_create` 分发。

## 套路

在 handler 构造时建好 registry，事件中传入 trim 后的消息内容调用 `registry.try_execute(content)`。handler 命中返回 `Some(reply)`，否则返回 `None`。

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

示例注册了两个命令（`你好` / `hello` 与 `晚安`），并演示了两种回复写法：方便的 `session.reply(&response)` 与显式的 `MessageParams { content: Some(response), ..Default::default() }` + `session.send_message(...)`。两种写法落到同一个接口，按需选择。

## 参见

- 指南：[`docs/zh/guide/messages.md`](../guide/messages.md)
- 相关：[交互式消息](./interactive-messages.md)（按钮驱动的 UI）
- 示例：[`examples/guild/command.rs`](https://github.com/YinMo19/botrs/blob/main/examples/guild/command.rs)

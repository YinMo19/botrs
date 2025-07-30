# v0.2.0 Message API Migration Guide

This guide helps you migrate from the old message API (v0.1.x) to the new structured parameter API introduced in v0.2.0. The new API eliminates the confusing multiple `None` parameters and provides a cleaner, more maintainable interface.

## Overview of Changes

### What Changed

- **Structured Parameters**: Introduction of `MessageParams`, `GroupMessageParams`, `C2CMessageParams`, and `DirectMessageParams`
- **Builder Pattern**: Fluent API with methods like `.with_reply()`, `.with_file_image()`, etc.
- **Type Safety**: Better compile-time checks and reduced runtime errors
- **Cleaner Code**: No more long parameter lists with multiple `None` values

### What's Deprecated

The old multi-parameter message methods are deprecated but still functional:

```rust
// DEPRECATED - Don't use in new code
api.post_message(
    token, 
    "channel_id", 
    Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

These methods will be removed in v0.3.0.

## Migration Examples

### Basic Text Messages

**Old API (v0.1.x):**
```rust
// Confusing with many None parameters
api.post_message(
    &token,
    "channel_123",
    Some("Hello, world!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

**New API (v0.2.0+):**
```rust
// Clean and readable
let params = MessageParams::new_text("Hello, world!");
api.post_message_with_params(&token, "channel_123", params).await?;
```

### Reply Messages

**Old API (v0.1.x):**
```rust
// Hard to understand which parameter is the reply
api.post_message(
    &token,
    "channel_123",
    Some("Thanks for your message!"),
    None,
    Some(MessageReference {
        message_id: Some("msg_456".to_string()),
        ..Default::default()
    }),
    None, None, None, None, None, None, None
).await?;
```

**New API (v0.2.0+):**
```rust
// Clear intent with builder pattern
let params = MessageParams::new_text("Thanks for your message!")
    .with_reply("msg_456");
api.post_message_with_params(&token, "channel_123", params).await?;
```

### Embed Messages

**Old API (v0.1.x):**
```rust
// Which parameter is the embed?
let embed = Embed {
    title: Some("News Update".to_string()),
    description: Some("Latest updates from our team".to_string()),
    ..Default::default()
};

api.post_message(
    &token,
    "channel_123",
    Some("Check out this update:"),
    None, None,
    Some(embed),
    None, None, None, None, None, None
).await?;
```

**New API (v0.2.0+):**
```rust
// Self-documenting structure
let embed = Embed {
    title: Some("News Update".to_string()),
    description: Some("Latest updates from our team".to_string()),
    ..Default::default()
};

let params = MessageParams {
    content: Some("Check out this update:".to_string()),
    embed: Some(embed),
    ..Default::default()
};
api.post_message_with_params(&token, "channel_123", params).await?;
```

### File Attachments

**Old API (v0.1.x):**
```rust
// Unclear parameter ordering
api.post_message(
    &token,
    "channel_123",
    Some("Here's your file:"),
    None, None, None,
    Some(image_data),
    None, None, None, None, None
).await?;
```

**New API (v0.2.0+):**
```rust
// Explicit and chainable
let params = MessageParams::new_text("Here's your file:")
    .with_file_image(&image_data);
api.post_message_with_params(&token, "channel_123", params).await?;
```

### Markdown Messages

**Old API (v0.1.x):**
```rust
let markdown = MarkdownPayload {
    content: Some("# Hello\n\nThis is **bold** text".to_string()),
    ..Default::default()
};

api.post_message(
    &token,
    "channel_123",
    None, None, None, None, None,
    Some(markdown),
    None, None, None, None
).await?;
```

**New API (v0.2.0+):**
```rust
let markdown = MarkdownPayload {
    content: Some("# Hello\n\nThis is **bold** text".to_string()),
    ..Default::default()
};

let params = MessageParams {
    markdown: Some(markdown),
    ..Default::default()
};
api.post_message_with_params(&token, "channel_123", params).await?;
```

## Message Type Specific APIs

### Group Messages

**Old API:**
```rust
api.post_group_message(
    &token,
    "group_123",
    Some("Hello group!"),
    None, None, None, None, None, None
).await?;
```

**New API:**
```rust
let params = GroupMessageParams::new_text("Hello group!");
api.post_group_message_with_params(&token, "group_123", params).await?;
```

### C2C (Client-to-Client) Messages

**Old API:**
```rust
api.post_c2c_message(
    &token,
    "user_123",
    Some("Hello there!"),
    None, None, None, None, None, None
).await?;
```

**New API:**
```rust
let params = C2CMessageParams::new_text("Hello there!");
api.post_c2c_message_with_params(&token, "user_123", params).await?;
```

### Direct Messages

**Old API:**
```rust
api.post_dms(
    &token,
    "guild_123",
    Some("Direct message!"),
    None, None, None, None, None, None
).await?;
```

**New API:**
```rust
let params = DirectMessageParams::new_text("Direct message!");
api.post_dms_with_params(&token, "guild_123", params).await?;
```

## Migration Strategies

### Gradual Migration

You can migrate gradually since the old API still works:

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Migrate one command at a time
        match msg.content.as_deref() {
            Some("!new") => {
                // Use new API for new features
                let params = MessageParams::new_text("Using new API!");
                ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await.ok();
            }
            Some("!old") => {
                // Keep old API for now (but plan to migrate)
                ctx.api.post_message(
                    &ctx.token, &msg.channel_id, 
                    Some("Still using old API"), 
                    None, None, None, None, None, None, None, None, None
                ).await.ok();
            }
            _ => {}
        }
    }
}
```

### Helper Functions

Create helper functions to ease migration:

```rust
// Migration helpers
fn simple_text(content: &str) -> MessageParams {
    MessageParams::new_text(content)
}

fn reply_text(content: &str, reply_to: &str) -> MessageParams {
    MessageParams::new_text(content).with_reply(reply_to)
}

fn embed_message(content: &str, embed: Embed) -> MessageParams {
    MessageParams {
        content: Some(content.to_string()),
        embed: Some(embed),
        ..Default::default()
    }
}

// Usage
let params = reply_text("Thanks!", &message_id);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### Bulk Migration Script

For large codebases, consider creating a script to help with migration:

```rust
// Example migration pattern matching
fn migrate_message_call(old_call: &str) -> String {
    // This is a simplified example - real implementation would need
    // proper parsing and AST manipulation
    if old_call.contains("post_message(") && old_call.contains("Some(") {
        // Extract content and transform to new API
        // This would be much more complex in practice
        "MessageParams::new_text(content)".to_string()
    } else {
        old_call.to_string()
    }
}
```

## Event Handler Migration

### Old Event Handling

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // Old response pattern
        ctx.api.post_message(
            &ctx.token,
            &msg.channel_id,
            Some("Hello!"),
            None, None, None, None, None, None, None, None, None
        ).await.ok();
    }
}
```

### New Event Handling

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // New response pattern
        let params = MessageParams::new_text("Hello!");
        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await.ok();
    }
}
```

## Builder Pattern Benefits

The new API supports a fluent builder pattern:

```rust
let params = MessageParams::new_text("Check this out!")
    .with_reply(&original_message_id)
    .with_file_image(&image_data)
    .with_embed(my_embed);

api.post_message_with_params(&token, &channel_id, params).await?;
```

This is equivalent to the old API but much more readable:

```rust
// Old equivalent (don't use)
api.post_message(
    &token,
    &channel_id,
    Some("Check this out!"),
    None,
    Some(MessageReference {
        message_id: Some(original_message_id.clone()),
        ..Default::default()
    }),
    Some(my_embed),
    Some(image_data),
    None, None, None, None, None
).await?;
```

## Common Migration Issues

### Issue 1: Parameter Order Confusion

**Problem:**
```rust
// Which parameter is which?
api.post_message(token, channel, content, embed, None, None, None, None, None, None, None, None).await?;
```

**Solution:**
```rust
// Self-documenting
let params = MessageParams {
    content: Some(content),
    embed: Some(embed),
    ..Default::default()
};
api.post_message_with_params(token, channel, params).await?;
```

### Issue 2: Forgetting to Update Method Names

**Problem:**
```rust
// Still using old method name
api.post_message(&token, &channel_id, params).await?; // Wrong!
```

**Solution:**
```rust
// Use the new method name
api.post_message_with_params(&token, &channel_id, params).await?; // Correct!
```

### Issue 3: Mixing Old and New Patterns

**Problem:**
```rust
// Don't mix old None parameters with new struct
api.post_message(
    &token, &channel_id, 
    None, None, None, 
    Some(MessageParams::new_text("Hello")), // Wrong approach
    None, None, None, None, None
).await?;
```

**Solution:**
```rust
// Use the new API consistently
let params = MessageParams::new_text("Hello");
api.post_message_with_params(&token, &channel_id, params).await?;
```

## Testing Your Migration

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_params_creation() {
        let params = MessageParams::new_text("Hello");
        assert_eq!(params.content, Some("Hello".to_string()));
        assert!(params.embed.is_none());
    }

    #[test]
    fn test_reply_message() {
        let params = MessageParams::new_text("Reply")
            .with_reply("msg_123");
        
        assert_eq!(params.content, Some("Reply".to_string()));
        assert!(params.message_reference.is_some());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_new_api_integration() {
    let params = MessageParams::new_text("Test message");
    
    // Test with mock API
    let result = mock_api.post_message_with_params(&token, &channel_id, params).await;
    assert!(result.is_ok());
}
```

## Performance Considerations

The new API has several performance benefits:

1. **Reduced Allocations**: Fewer `Option<T>` allocations for unused parameters
2. **Better Caching**: Structured parameters are easier to cache and reuse
3. **Compile-time Optimization**: Better dead code elimination for unused fields

## Backwards Compatibility

- Old API methods are marked as `#[deprecated]` but still functional
- No breaking changes in v0.2.x series
- Old methods will be removed in v0.3.0
- Compile-time warnings guide migration

## Timeline and Support

- **v0.2.0**: New API introduced, old API deprecated
- **v0.2.x**: Both APIs supported with deprecation warnings
- **v0.3.0**: Old API removed (planned)

## Getting Help

If you encounter issues during migration:

1. Check the [examples directory](/examples/getting-started) for usage patterns
2. Review the [API documentation](/api/client)
3. Open an issue on [GitHub](https://github.com/YinMo19/botrs/issues)
4. Join our community discussions

## Conclusion

The v0.2.0 message API provides a cleaner, more maintainable way to send messages. While migration requires some work, the benefits include:

- **Better Code Readability**: Self-documenting parameter names
- **Reduced Errors**: Type safety and clear intent
- **Easier Maintenance**: Structured parameters are easier to modify
- **Future-Proof**: Foundation for upcoming features

Start migrating incrementally and take advantage of the migration helpers to make the process smooth.
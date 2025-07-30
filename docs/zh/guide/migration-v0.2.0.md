# v0.2.0 消息 API 迁移指南

本指南帮助您从旧的消息 API（v0.1.x）迁移到 v0.2.0 中引入的新结构化参数 API。新 API 消除了令人困惑的多个 `None` 参数，提供了更清洁、更易维护的接口。

## 变更概览

### 主要变更

- **结构化参数**：引入 `MessageParams`、`GroupMessageParams`、`C2CMessageParams` 和 `DirectMessageParams`
- **构建器模式**：流畅的 API，支持 `.with_reply()`、`.with_file_image()` 等方法
- **类型安全**：更好的编译时检查，减少运行时错误
- **代码简洁**：不再有包含多个 `None` 值的长参数列表

### 已弃用内容

旧的多参数消息方法已弃用但仍可使用：

```rust
// 已弃用 - 请勿在新代码中使用
api.post_message(
    token, 
    "channel_id", 
    Some("Hello!"),
    None, None, None, None, None, None, None, None, None
).await?;
```

这些方法将在 v0.3.0 中移除。

## 迁移示例

### 基本文本消息

**旧 API（v0.1.x）：**
```rust
// 包含许多 None 参数，令人困惑
api.post_message(
    &token,
    "channel_123",
    Some("你好，世界！"),
    None, None, None, None, None, None, None, None, None
).await?;
```

**新 API（v0.2.0+）：**
```rust
// 清洁且可读
let params = MessageParams::new_text("你好，世界！");
api.post_message_with_params(&token, "channel_123", params).await?;
```

### 回复消息

**旧 API（v0.1.x）：**
```rust
// 难以理解哪个参数是回复
api.post_message(
    &token,
    "channel_123",
    Some("感谢您的消息！"),
    None,
    Some(MessageReference {
        message_id: Some("msg_456".to_string()),
        ..Default::default()
    }),
    None, None, None, None, None, None, None
).await?;
```

**新 API（v0.2.0+）：**
```rust
// 使用构建器模式，意图清晰
let params = MessageParams::new_text("感谢您的消息！")
    .with_reply("msg_456");
api.post_message_with_params(&token, "channel_123", params).await?;
```

### 嵌入消息

**旧 API（v0.1.x）：**
```rust
// 哪个参数是嵌入？
let embed = Embed {
    title: Some("新闻更新".to_string()),
    description: Some("来自我们团队的最新更新".to_string()),
    ..Default::default()
};

api.post_message(
    &token,
    "channel_123",
    Some("查看这个更新："),
    None, None,
    Some(embed),
    None, None, None, None, None, None
).await?;
```

**新 API（v0.2.0+）：**
```rust
// 自描述结构
let embed = Embed {
    title: Some("新闻更新".to_string()),
    description: Some("来自我们团队的最新更新".to_string()),
    ..Default::default()
};

let params = MessageParams {
    content: Some("查看这个更新：".to_string()),
    embed: Some(embed),
    ..Default::default()
};
api.post_message_with_params(&token, "channel_123", params).await?;
```

### 文件附件

**旧 API（v0.1.x）：**
```rust
// 参数顺序不清楚
api.post_message(
    &token,
    "channel_123",
    Some("这是您的文件："),
    None, None, None,
    Some(image_data),
    None, None, None, None, None
).await?;
```

**新 API（v0.2.0+）：**
```rust
// 明确且可链式调用
let params = MessageParams::new_text("这是您的文件：")
    .with_file_image(&image_data);
api.post_message_with_params(&token, "channel_123", params).await?;
```

### Markdown 消息

**旧 API（v0.1.x）：**
```rust
let markdown = MarkdownPayload {
    content: Some("# 你好\n\n这是**粗体**文本".to_string()),
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

**新 API（v0.2.0+）：**
```rust
let markdown = MarkdownPayload {
    content: Some("# 你好\n\n这是**粗体**文本".to_string()),
    ..Default::default()
};

let params = MessageParams {
    markdown: Some(markdown),
    ..Default::default()
};
api.post_message_with_params(&token, "channel_123", params).await?;
```

## 特定消息类型 API

### 群组消息

**旧 API：**
```rust
api.post_group_message(
    &token,
    "group_123",
    Some("群组你好！"),
    None, None, None, None, None, None
).await?;
```

**新 API：**
```rust
let params = GroupMessageParams::new_text("群组你好！");
api.post_group_message_with_params(&token, "group_123", params).await?;
```

### C2C（客户端到客户端）消息

**旧 API：**
```rust
api.post_c2c_message(
    &token,
    "user_123",
    Some("你好！"),
    None, None, None, None, None, None
).await?;
```

**新 API：**
```rust
let params = C2CMessageParams::new_text("你好！");
api.post_c2c_message_with_params(&token, "user_123", params).await?;
```

### 私信

**旧 API：**
```rust
api.post_dms(
    &token,
    "guild_123",
    Some("私信！"),
    None, None, None, None, None, None
).await?;
```

**新 API：**
```rust
let params = DirectMessageParams::new_text("私信！");
api.post_dms_with_params(&token, "guild_123", params).await?;
```

## 迁移策略

### 渐进式迁移

由于旧 API 仍然可用，您可以逐步迁移：

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // 一次迁移一个命令
        match msg.content.as_deref() {
            Some("!new") => {
                // 对新功能使用新 API
                let params = MessageParams::new_text("使用新 API！");
                ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await.ok();
            }
            Some("!old") => {
                // 暂时保持旧 API（但计划迁移）
                ctx.api.post_message(
                    &ctx.token, &msg.channel_id, 
                    Some("仍在使用旧 API"), 
                    None, None, None, None, None, None, None, None, None
                ).await.ok();
            }
            _ => {}
        }
    }
}
```

### 辅助函数

创建辅助函数以简化迁移：

```rust
// 迁移辅助函数
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

// 使用
let params = reply_text("谢谢！", &message_id);
api.post_message_with_params(&token, &channel_id, params).await?;
```

### 批量迁移脚本

对于大型代码库，考虑创建脚本来帮助迁移：

```rust
// 迁移模式匹配示例
fn migrate_message_call(old_call: &str) -> String {
    // 这是一个简化示例 - 实际实现需要
    // 适当的解析和 AST 操作
    if old_call.contains("post_message(") && old_call.contains("Some(") {
        // 提取内容并转换为新 API
        // 在实践中这会更复杂
        "MessageParams::new_text(content)".to_string()
    } else {
        old_call.to_string()
    }
}
```

## 事件处理器迁移

### 旧事件处理

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // 旧响应模式
        ctx.api.post_message(
            &ctx.token,
            &msg.channel_id,
            Some("你好！"),
            None, None, None, None, None, None, None, None, None
        ).await.ok();
    }
}
```

### 新事件处理

```rust
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // 新响应模式
        let params = MessageParams::new_text("你好！");
        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await.ok();
    }
}
```

## 构建器模式的优势

新 API 支持流畅的构建器模式：

```rust
let params = MessageParams::new_text("看看这个！")
    .with_reply(&original_message_id)
    .with_file_image(&image_data)
    .with_embed(my_embed);

api.post_message_with_params(&token, &channel_id, params).await?;
```

这等同于旧 API，但可读性更强：

```rust
// 旧的等效方式（请勿使用）
api.post_message(
    &token,
    &channel_id,
    Some("看看这个！"),
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

## 常见迁移问题

### 问题 1：参数顺序混乱

**问题：**
```rust
// 哪个参数是什么？
api.post_message(token, channel, content, embed, None, None, None, None, None, None, None, None).await?;
```

**解决方案：**
```rust
// 自描述
let params = MessageParams {
    content: Some(content),
    embed: Some(embed),
    ..Default::default()
};
api.post_message_with_params(token, channel, params).await?;
```

### 问题 2：忘记更新方法名

**问题：**
```rust
// 仍在使用旧方法名
api.post_message(&token, &channel_id, params).await?; // 错误！
```

**解决方案：**
```rust
// 使用新方法名
api.post_message_with_params(&token, &channel_id, params).await?; // 正确！
```

### 问题 3：混合新旧模式

**问题：**
```rust
// 不要将旧的 None 参数与新结构混合
api.post_message(
    &token, &channel_id, 
    None, None, None, 
    Some(MessageParams::new_text("Hello")), // 错误方法
    None, None, None, None, None
).await?;
```

**解决方案：**
```rust
// 一致使用新 API
let params = MessageParams::new_text("Hello");
api.post_message_with_params(&token, &channel_id, params).await?;
```

## 测试您的迁移

### 单元测试

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

### 集成测试

```rust
#[tokio::test]
async fn test_new_api_integration() {
    let params = MessageParams::new_text("测试消息");
    
    // 使用模拟 API 测试
    let result = mock_api.post_message_with_params(&token, &channel_id, params).await;
    assert!(result.is_ok());
}
```

## 性能考虑

新 API 具有多项性能优势：

1. **减少分配**：为未使用的参数分配更少的 `Option<T>`
2. **更好的缓存**：结构化参数更易于缓存和重用
3. **编译时优化**：对未使用字段的死代码消除更好

## 向后兼容性

- 旧 API 方法标记为 `#[deprecated]` 但仍可使用
- v0.2.x 系列中没有破坏性变更
- 旧方法将在 v0.3.0 中移除
- 编译时警告指导迁移

## 时间线和支持

- **v0.2.0**：引入新 API，弃用旧 API
- **v0.2.x**：支持两种 API，带有弃用警告
- **v0.3.0**：移除旧 API（计划中）

## 获取帮助

如果在迁移过程中遇到问题：

1. 查看 [示例目录](/zh/examples/getting-started) 了解使用模式
2. 查看 [API 文档](/zh/api/client)
3. 在 [GitHub](https://github.com/YinMo19/botrs/issues) 上提出问题
4. 参加我们的社区讨论

## 结论

v0.2.0 消息 API 提供了更清洁、更易维护的消息发送方式。虽然迁移需要一些工作，但好处包括：

- **更好的代码可读性**：自描述的参数名称
- **减少错误**：类型安全和清晰意图
- **更易维护**：结构化参数更易于修改
- **面向未来**：为即将到来的功能奠定基础

开始增量迁移，利用迁移辅助函数使过程顺利进行。
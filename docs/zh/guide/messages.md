# 消息与回复

BotRS 提供了强大而灵活的消息处理系统，支持多种消息类型和富文本内容。本指南将介绍如何处理不同类型的消息以及如何发送各种格式的回复。

## 消息类型概述

BotRS 支持以下几种主要的消息类型：

- **频道消息 (Message)**: 在 QQ 频道中的消息，通常需要 @ 机器人才能触发
- **私信 (DirectMessage)**: 用户与机器人的私人对话
- **群聊消息 (GroupMessage)**: QQ 群组中的消息
- **C2C 消息 (C2CMessage)**: 客户端到客户端的直接消息

## 基础消息处理

### 处理频道消息

频道消息是最常见的消息类型，通常在用户 @ 机器人时触发：

```rust
use botrs::{Context, EventHandler, Message};

#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn message_create(&self, ctx: Context, message: Message) {
        // 忽略机器人自己的消息
        if message.is_from_bot() {
            return;
        }

        // 获取消息内容
        if let Some(content) = &message.content {
            match content.trim() {
                "!ping" => {
                    // 简单文本回复
                    if let Err(e) = message.reply(&ctx.api, &ctx.token, "Pong!").await {
                        eprintln!("回复失败: {}", e);
                    }
                }
                "!help" => {
                    let help_text = "可用命令:\n• !ping - 测试连接\n• !info - 获取信息\n• !help - 显示帮助";
                    message.reply(&ctx.api, &ctx.token, help_text).await?;
                }
                _ => {
                    // 处理其他消息
                    println!("收到消息: {}", content);
                }
            }
        }
    }
}
```

### 处理群聊消息

群聊消息处理类似，但使用不同的事件：

```rust
#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn group_message_create(&self, ctx: Context, message: GroupMessage) {
        if let Some(content) = &message.content {
            // 检查是否包含关键词
            if content.contains("机器人") || content.contains("bot") {
                let response = format!("我听到有人在叫我！你说的是: {}", content);
                message.reply(&ctx.api, &ctx.token, &response).await?;
            }
        }
    }
}
```

### 处理私信

私信提供了更私密的交互方式：

```rust
#[async_trait::async_trait]
impl EventHandler for MyBot {
    async fn direct_message_create(&self, ctx: Context, message: DirectMessage) {
        if let Some(content) = &message.content {
            // 私信通常用于个人化的交互
            let personalized_response = format!("你好！我收到了你的私信: {}", content);
            message.reply(&ctx.api, &ctx.token, &personalized_response).await?;
        }
    }
}
```

## 富文本消息

### 嵌入消息 (Embed)

嵌入消息允许发送结构化的富文本内容：

```rust
use botrs::models::message::{Embed, EmbedField, EmbedFooter};

async fn send_embed_message(ctx: &Context, channel_id: &str) -> Result<()> {
    let embed = Embed {
        title: Some("机器人信息".to_string()),
        description: Some("这是一个基于 BotRS 构建的 QQ 机器人".to_string()),
        color: Some(0x00ff00), // 绿色
        fields: vec![
            EmbedField {
                name: "版本".to_string(),
                value: "0.2.5".to_string(),
                inline: Some(true),
            },
            EmbedField {
                name: "语言".to_string(),
                value: "Rust".to_string(),
                inline: Some(true),
            },
            EmbedField {
                name: "功能".to_string(),
                value: "高性能异步处理".to_string(),
                inline: Some(false),
            },
        ],
        footer: Some(EmbedFooter {
            text: "BotRS 框架".to_string(),
            icon_url: None,
        }),
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
        ..Default::default()
    };

    let params = MessageParams {
        embed: Some(embed),
        ..Default::default()
    };

    ctx.send_message(channel_id, &params).await?;
    Ok(())
}
```

### Markdown 消息

支持发送 Markdown 格式的消息：

```rust
use botrs::models::message::{MarkdownPayload, MarkdownParam};

async fn send_markdown_message(ctx: &Context, channel_id: &str) -> Result<()> {
    let markdown = MarkdownPayload {
        template_id: Some(37), // 使用系统模板
        custom_template_id: None,
        params: vec![
            MarkdownParam {
                key: "title".to_string(),
                values: vec!["欢迎使用 BotRS".to_string()],
            },
            MarkdownParam {
                key: "content".to_string(),
                values: vec![
                    "这是一个 **Markdown** 消息示例。\n\n支持:\n- 粗体和斜体\n- 列表\n- 链接等".to_string()
                ],
            },
        ],
        content: None,
    };

    let params = MessageParams {
        markdown: Some(markdown),
        ..Default::default()
    };

    ctx.send_message(channel_id, &params).await?;
    Ok(())
}
```

### 交互式键盘

创建带有按钮的交互式消息：

```rust
use botrs::models::message::{
    Keyboard, KeyboardContent, KeyboardRow, KeyboardButton,
    KeyboardButtonRenderData, KeyboardButtonAction
};

async fn send_interactive_message(ctx: &Context, channel_id: &str) -> Result<()> {
    let keyboard = Keyboard {
        content: Some(KeyboardContent {
            rows: vec![
                KeyboardRow {
                    buttons: vec![
                        KeyboardButton {
                            id: Some("btn_yes".to_string()),
                            render_data: Some(KeyboardButtonRenderData {
                                label: "是 ✅".to_string(),
                                visited_label: "已选择: 是".to_string(),
                                style: Some(1), // 绿色样式
                            }),
                            action: Some(KeyboardButtonAction {
                                action_type: Some(2), // 回调
                                permission: None,
                                click_limit: None,
                                data: Some("yes".to_string()),
                                reply: None,
                                enter: Some(true),
                            }),
                        },
                        KeyboardButton {
                            id: Some("btn_no".to_string()),
                            render_data: Some(KeyboardButtonRenderData {
                                label: "否 ❌".to_string(),
                                visited_label: "已选择: 否".to_string(),
                                style: Some(2), // 红色样式
                            }),
                            action: Some(KeyboardButtonAction {
                                action_type: Some(2),
                                permission: None,
                                click_limit: None,
                                data: Some("no".to_string()),
                                reply: None,
                                enter: Some(true),
                            }),
                        },
                    ],
                },
            ],
        }),
    };

    let params = MessageParams {
        content: Some("请选择你的答案:".to_string()),
        keyboard: Some(keyboard),
        ..Default::default()
    };

    ctx.send_message(channel_id, &params).await?;
    Ok(())
}
```

## 文件和媒体

### 发送图片

```rust
async fn send_image(ctx: &Context, channel_id: &str, image_data: &str) -> Result<()> {
    let params = MessageParams::new_text("这里是一张图片:")
        .with_file_image(image_data); // Base64 编码的文件信息

    ctx.send_message(channel_id, &params).await?;
    Ok(())
}
```

### 发送文件

```rust
use botrs::models::message::Media;

async fn send_file(ctx: &Context, group_openid: &str, file_info: &str) -> Result<()> {
    let media = Media {
        file_info: Some(file_info.to_string()),
        ttl: Some(3600), // 文件有效期 1 小时
    };

    let params = GroupMessageParams {
        content: Some("文件已上传".to_string()),
        media: Some(media),
        ..Default::default()
    };

    ctx.send_group_message(group_openid, &params).await?;
    Ok(())
}
```

## 消息引用和回复

### 引用消息

创建对特定消息的引用回复：

```rust
use botrs::models::message::Reference;

async fn reply_to_message(
    ctx: &Context,
    channel_id: &str,
    original_message_id: &str,
    reply_content: &str,
) -> Result<()> {
    let params = MessageParams {
        content: Some(reply_content.to_string()),
        message_reference: Some(Reference {
            message_id: original_message_id.to_string(),
            ignore_get_message_error: Some(true),
        }),
        ..Default::default()
    };

    ctx.send_message(channel_id, &params).await?;
    Ok(())
}
```

### 便捷回复方法

所有消息类型都提供了便捷的回复方法：

```rust
// 对频道消息回复
message.reply(&ctx.api, &ctx.token, "回复内容").await?;

// 对群聊消息回复
group_message.reply(&ctx.api, &ctx.token, "群聊回复").await?;

// 对私信回复
direct_message.reply(&ctx.api, &ctx.token, "私信回复").await?;
```

## 高级消息处理

### 消息过滤和验证

```rust
fn should_process_message(message: &Message) -> bool {
    // 忽略机器人消息
    if message.is_from_bot() {
        return false;
    }

    // 只处理有内容的消息
    if !message.has_content() {
        return false;
    }

    // 检查内容长度
    if let Some(content) = &message.content {
        if content.len() > 2000 {
            return false; // 内容过长
        }
    }

    true
}

async fn message_create(&self, ctx: Context, message: Message) {
    if !should_process_message(&message) {
        return;
    }

    // 处理消息...
}
```

### 命令解析

```rust
struct Command {
    name: String,
    args: Vec<String>,
}

fn parse_command(content: &str) -> Option<Command> {
    if !content.starts_with('!') {
        return None;
    }

    let parts: Vec<&str> = content[1..].split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    Some(Command {
        name: parts[0].to_string(),
        args: parts[1..].iter().map(|s| s.to_string()).collect(),
    })
}

async fn handle_command(ctx: &Context, message: &Message, cmd: Command) -> Result<()> {
    match cmd.name.as_str() {
        "echo" => {
            let text = cmd.args.join(" ");
            message.reply(&ctx.api, &ctx.token, &text).await?;
        }
        "info" => {
            let info = format!("服务器时间: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
            message.reply(&ctx.api, &ctx.token, &info).await?;
        }
        "user" => {
            if let Some(author) = &message.author {
                let user_info = format!(
                    "用户信息:\n用户名: {}\n用户ID: {}",
                    author.username.as_deref().unwrap_or("未知"),
                    author.id
                );
                message.reply(&ctx.api, &ctx.token, &user_info).await?;
            }
        }
        _ => {
            message.reply(&ctx.api, &ctx.token, "未知命令，输入 !help 查看可用命令").await?;
        }
    }
    Ok(())
}
```

### 消息队列和限流

```rust
use std::collections::VecDeque;
use tokio::sync::Mutex;

struct MessageQueue {
    queue: Mutex<VecDeque<PendingMessage>>,
    rate_limit: tokio::time::Interval,
}

struct PendingMessage {
    channel_id: String,
    content: String,
    timestamp: std::time::Instant,
}

impl MessageQueue {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            rate_limit: tokio::time::interval(std::time::Duration::from_millis(500)),
        }
    }

    async fn enqueue(&self, channel_id: String, content: String) {
        let message = PendingMessage {
            channel_id,
            content,
            timestamp: std::time::Instant::now(),
        };

        self.queue.lock().await.push_back(message);
    }

    async fn process_queue(&mut self, ctx: &Context) -> Result<()> {
        loop {
            self.rate_limit.tick().await;

            let message = {
                let mut queue = self.queue.lock().await;
                queue.pop_front()
            };

            if let Some(msg) = message {
                let params = MessageParams::new_text(&msg.content);
                if let Err(e) = ctx.send_message(&msg.channel_id, &params).await {
                    eprintln!("发送消息失败: {}", e);
                    // 可以选择重新入队或记录错误
                }
            }
        }
    }
}
```

## 最佳实践

### 错误处理

```rust
async fn safe_reply(message: &Message, ctx: &Context, content: &str) -> bool {
    match message.reply(&ctx.api, &ctx.token, content).await {
        Ok(_) => {
            println!("消息发送成功");
            true
        }
        Err(e) => {
            eprintln!("消息发送失败: {}", e);
            false
        }
    }
}
```

### 内容验证

```rust
fn validate_message_content(content: &str) -> Result<(), String> {
    if content.is_empty() {
        return Err("消息内容不能为空".to_string());
    }

    if content.len() > 4000 {
        return Err("消息内容过长".to_string());
    }

    if content.contains("@everyone") || content.contains("@here") {
        return Err("不允许使用全体提及".to_string());
    }

    Ok(())
}
```

### 用户权限检查

```rust
async fn has_permission(ctx: &Context, channel_id: &str, user_id: &str) -> bool {
    match ctx.get_channel_user_permissions(channel_id, user_id).await {
        Ok(permissions) => {
            // 解析权限字符串并检查特定权限
            let perm_value: u64 = permissions.permissions.parse().unwrap_or(0);
            const SEND_MESSAGES: u64 = 1 << 11;
            (perm_value & SEND_MESSAGES) != 0
        }
        Err(_) => false,
    }
}
```

## 总结

BotRS 的消息系统提供了丰富的功能来处理各种类型的消息和发送多样化的回复。通过合理使用这些功能，你可以创建出功能强大且用户友好的 QQ 机器人。

主要要点：

1. **消息类型**: 理解不同消息类型的用途和特点
2. **富文本**: 利用 Embed、Markdown 和键盘创建丰富的交互体验
3. **文件处理**: 支持图片和文件的发送
4. **消息引用**: 实现上下文相关的回复
5. **错误处理**: 始终处理可能的错误情况
6. **性能优化**: 使用消息队列和限流避免触发速率限制

## 下一步

- 查看 [交互式消息示例](../../examples/interactive-messages.md) 了解更多高级用法
- 阅读 [错误处理指南](./error-handling.md) 学习如何优雅地处理错误
- 探索 [API 客户端使用](./api-client.md) 了解更多 API 功能
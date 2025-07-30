# 富文本消息示例

本示例展示如何使用 BotRS 发送各种类型的富文本消息，包括嵌入内容（Embed）、Ark 消息、Markdown 格式和交互式组件。

## 概述

QQ 频道支持多种富文本消息格式，让机器人能够发送更加丰富和交互性的内容：

- **Embed 消息**: 结构化的富文本卡片
- **Ark 消息**: 基于模板的结构化消息
- **Markdown 消息**: 支持 Markdown 语法的文本
- **交互式消息**: 包含按钮和选择菜单的消息

## 基础嵌入消息

### 简单嵌入消息

```rust
use botrs::{Context, EventHandler, Message, MessageParams, Embed};

async fn send_simple_embed(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let embed = Embed::new()
        .title("欢迎使用机器人")
        .description("这是一个简单的嵌入消息示例")
        .color(0x3498db); // 蓝色
    
    let params = MessageParams::new_embed(embed);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

### 带字段的嵌入消息

```rust
async fn send_detailed_embed(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let embed = Embed::new()
        .title("服务器状态")
        .description("当前服务器运行状态信息")
        .color(0x00ff00) // 绿色
        .field("CPU 使用率", "25%", true)
        .field("内存使用率", "60%", true)
        .field("磁盘使用率", "45%", true)
        .field("网络延迟", "12ms", true)
        .field("运行时间", "7天 3小时 25分钟", false)
        .field("活跃用户", "1,234 人在线", false)
        .timestamp(chrono::Utc::now())
        .footer("系统监控", Some("https://example.com/icon.png"));
    
    let params = MessageParams::new_embed(embed);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

### 带图片的嵌入消息

```rust
async fn send_embed_with_image(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let embed = Embed::new()
        .title("每日图片")
        .description("今日推荐的精美图片")
        .color(0xff6b6b) // 红色
        .image("https://example.com/daily-image.jpg")
        .thumbnail("https://example.com/thumbnail.jpg")
        .author("图片机器人", Some("https://example.com/bot-avatar.png"))
        .url("https://example.com/full-gallery");
    
    let params = MessageParams::new_embed(embed);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

## Ark 消息

### 链接卡片 Ark

```rust
use serde_json::json;

async fn send_link_ark(
    ctx: &Context,
    channel_id: &str,
    url: &str,
    title: &str,
    description: &str
) -> Result<(), botrs::BotError> {
    let ark_data = json!({
        "template_id": 23, // 链接模板 ID
        "kv": [
            {
                "key": "#DESC#",
                "value": description
            },
            {
                "key": "#PROMPT#",
                "value": title
            },
            {
                "key": "#URL#",
                "value": url
            }
        ]
    });
    
    let params = MessageParams::new_ark(ark_data);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

### 自定义 Ark 消息

```rust
async fn send_custom_ark(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let ark_data = json!({
        "template_id": 37, // 自定义模板 ID
        "kv": [
            {
                "key": "#TITLE#",
                "value": "重要通知"
            },
            {
                "key": "#CONTENT#",
                "value": "系统将在今晚进行维护，预计持续2小时"
            },
            {
                "key": "#TIME#",
                "value": "2024-01-15 22:00 - 24:00"
            },
            {
                "key": "#LEVEL#",
                "value": "高"
            }
        ]
    });
    
    let params = MessageParams::new_ark(ark_data);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

## Markdown 消息

### 基础 Markdown

```rust
async fn send_markdown_message(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let markdown_content = r#"
# 机器人帮助文档

欢迎使用我们的多功能机器人！

## 主要功能

### 消息功能
- **文本消息**: 发送普通文本
- **富文本**: 支持 Markdown 格式
- **图片分享**: 上传和分享图片
- **文件传输**: 支持多种文件格式

### 管理功能
- **成员管理**: 查看和管理频道成员
- **权限控制**: 角色和权限分配
- **频道设置**: 自定义频道配置

### 娱乐功能
- **小游戏**: 内置多种小游戏
- **音乐播放**: 语音频道音乐播放
- **表情包**: 丰富的表情包资源

## 使用方法

1. 使用 `!help` 查看命令列表
2. 使用 `!设置` 进行个性化配置
3. 使用 `@机器人` 直接对话

---

**技术支持**: support@example.com  
**更新日志**: [点击查看](https://example.com/changelog)
"#;
    
    let params = MessageParams::new_markdown(markdown_content);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

### 动态 Markdown

```rust
async fn send_dynamic_markdown(
    ctx: &Context,
    channel_id: &str,
    user_name: &str,
    stats: &UserStats
) -> Result<(), botrs::BotError> {
    let markdown_content = format!(r#"
# 用户统计报告

## 📊 {user_name} 的数据概览

### 基础信息
- **用户名**: {user_name}
- **注册时间**: {register_date}
- **最后活跃**: {last_active}

### 活动统计
| 项目 | 数值 | 排名 |
|------|------|------|
| 发送消息 | **{message_count}** 条 | 🥇 #{message_rank} |
| 在线时长 | **{online_hours}** 小时 | 🥈 #{online_rank} |
| 获得点赞 | **{likes_count}** 个 | 🥉 #{likes_rank} |

### 成就徽章
{achievements}

### 本月目标
- [ ] 发送 1000 条消息 ({current_messages}/1000)
- [ ] 在线 100 小时 ({current_hours}/100)
- [x] ~~获得 50 个点赞~~ ✅

> 💡 **提示**: 继续保持活跃，下个月可能获得"活跃之星"称号！
"#,
        user_name = user_name,
        register_date = stats.register_date,
        last_active = stats.last_active,
        message_count = stats.message_count,
        message_rank = stats.message_rank,
        online_hours = stats.online_hours,
        online_rank = stats.online_rank,
        likes_count = stats.likes_count,
        likes_rank = stats.likes_rank,
        achievements = stats.achievements.join(" "),
        current_messages = stats.current_month_messages,
        current_hours = stats.current_month_hours,
    );
    
    let params = MessageParams::new_markdown(&markdown_content);
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}

struct UserStats {
    register_date: String,
    last_active: String,
    message_count: u32,
    message_rank: u32,
    online_hours: u32,
    online_rank: u32,
    likes_count: u32,
    likes_rank: u32,
    achievements: Vec<String>,
    current_month_messages: u32,
    current_month_hours: u32,
}
```

## 交互式消息

### 按钮消息

```rust
use botrs::{MessageKeyboard, KeyboardButton, KeyboardRow};

async fn send_button_message(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    let keyboard = MessageKeyboard::new()
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("👍 点赞", "like_button"))
            .add_button(KeyboardButton::new("👎 踩", "dislike_button"))
            .add_button(KeyboardButton::new("❤️ 收藏", "favorite_button"))
        )
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("📊 查看统计", "stats_button"))
            .add_button(KeyboardButton::new("⚙️ 设置", "settings_button"))
        )
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("🔗 访问官网", "website_button").with_url("https://example.com"))
        );
    
    let params = MessageParams::new_text("请选择操作:")
        .with_keyboard(keyboard);
    
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

### 复杂交互式卡片

```rust
async fn send_interactive_card(
    ctx: &Context,
    channel_id: &str
) -> Result<(), botrs::BotError> {
    // 创建嵌入内容
    let embed = Embed::new()
        .title("📋 任务管理系统")
        .description("选择要执行的操作")
        .color(0x4a90e2)
        .field("待办任务", "5 个", true)
        .field("进行中", "3 个", true)
        .field("已完成", "12 个", true)
        .thumbnail("https://example.com/task-icon.png");
    
    // 创建键盘
    let keyboard = MessageKeyboard::new()
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("📝 新建任务", "create_task"))
            .add_button(KeyboardButton::new("📋 查看任务", "view_tasks"))
        )
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("✅ 完成任务", "complete_task"))
            .add_button(KeyboardButton::new("🗑️ 删除任务", "delete_task"))
        )
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("📈 统计报告", "task_stats"))
            .add_button(KeyboardButton::new("⚙️ 设置提醒", "set_reminder"))
        );
    
    let params = MessageParams::new_embed(embed)
        .with_keyboard(keyboard);
    
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

## 实际应用示例

### 天气信息卡片

```rust
async fn send_weather_card(
    ctx: &Context,
    channel_id: &str,
    city: &str,
    weather_data: &WeatherData
) -> Result<(), botrs::BotError> {
    let weather_emoji = match weather_data.condition.as_str() {
        "sunny" => "☀️",
        "cloudy" => "☁️",
        "rainy" => "🌧️",
        "snowy" => "❄️",
        _ => "🌤️",
    };
    
    let embed = Embed::new()
        .title(&format!("{} {} 天气", weather_emoji, city))
        .description(&format!("当前天气: {}", weather_data.description))
        .color(match weather_data.condition.as_str() {
            "sunny" => 0xffd700,
            "cloudy" => 0x808080,
            "rainy" => 0x4169e1,
            "snowy" => 0xe6e6fa,
            _ => 0x87ceeb,
        })
        .field("🌡️ 温度", &format!("{}°C", weather_data.temperature), true)
        .field("💧 湿度", &format!("{}%", weather_data.humidity), true)
        .field("💨 风速", &format!("{} km/h", weather_data.wind_speed), true)
        .field("👁️ 能见度", &format!("{} km", weather_data.visibility), true)
        .field("🌅 日出", &weather_data.sunrise, true)
        .field("🌇 日落", &weather_data.sunset, true)
        .thumbnail(&weather_data.icon_url)
        .footer("数据更新时间", None)
        .timestamp(chrono::Utc::now());
    
    let keyboard = MessageKeyboard::new()
        .add_row(KeyboardRow::new()
            .add_button(KeyboardButton::new("🔄 刷新", "refresh_weather"))
            .add_button(KeyboardButton::new("📅 7天预报", "week_forecast"))
            .add_button(KeyboardButton::new("🏙️ 切换城市", "change_city"))
        );
    
    let params = MessageParams::new_embed(embed)
        .with_keyboard(keyboard);
    
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}

struct WeatherData {
    condition: String,
    description: String,
    temperature: i32,
    humidity: u32,
    wind_speed: u32,
    visibility: u32,
    sunrise: String,
    sunset: String,
    icon_url: String,
}
```

### 投票系统

```rust
async fn send_poll_message(
    ctx: &Context,
    channel_id: &str,
    question: &str,
    options: &[String]
) -> Result<(), botrs::BotError> {
    let embed = Embed::new()
        .title("📊 投票")
        .description(question)
        .color(0x9b59b6)
        .field("参与方式", "点击下方按钮进行投票", false)
        .footer("投票将在24小时后截止", None);
    
    let mut keyboard = MessageKeyboard::new();
    let mut current_row = KeyboardRow::new();
    
    for (index, option) in options.iter().enumerate() {
        let emoji = match index {
            0 => "🅰️",
            1 => "🅱️",
            2 => "🅲️",
            3 => "🅳️",
            _ => "▫️",
        };
        
        current_row = current_row.add_button(
            KeyboardButton::new(
                &format!("{} {}", emoji, option),
                &format!("vote_{}", index)
            )
        );
        
        // 每行最多2个按钮
        if current_row.buttons.len() >= 2 || index == options.len() - 1 {
            keyboard = keyboard.add_row(current_row);
            current_row = KeyboardRow::new();
        }
    }
    
    // 添加结果查看按钮
    keyboard = keyboard.add_row(KeyboardRow::new()
        .add_button(KeyboardButton::new("📈 查看结果", "poll_results"))
    );
    
    let params = MessageParams::new_embed(embed)
        .with_keyboard(keyboard);
    
    ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
    
    Ok(())
}
```

## 完整事件处理器示例

```rust
use botrs::{Context, EventHandler, Message, Ready, Interaction};
use tracing::{info, warn};

struct RichMessageHandler;

#[async_trait::async_trait]
impl EventHandler for RichMessageHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("富文本消息机器人已就绪: {}", ready.user.username);
    }
    
    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }
        
        let content = match &message.content {
            Some(content) => content.trim(),
            None => return,
        };
        
        match content {
            "!embed" => {
                if let Err(e) = send_simple_embed(&ctx, &message.channel_id).await {
                    warn!("发送嵌入消息失败: {}", e);
                }
            }
            "!weather" => {
                let weather_data = WeatherData {
                    condition: "sunny".to_string(),
                    description: "晴朗".to_string(),
                    temperature: 25,
                    humidity: 60,
                    wind_speed: 10,
                    visibility: 15,
                    sunrise: "06:30".to_string(),
                    sunset: "18:45".to_string(),
                    icon_url: "https://example.com/sunny.png".to_string(),
                };
                
                if let Err(e) = send_weather_card(&ctx, &message.channel_id, "北京", &weather_data).await {
                    warn!("发送天气卡片失败: {}", e);
                }
            }
            "!poll" => {
                let options = vec![
                    "选项 A".to_string(),
                    "选项 B".to_string(),
                    "选项 C".to_string(),
                ];
                
                if let Err(e) = send_poll_message(
                    &ctx,
                    &message.channel_id,
                    "你最喜欢哪种编程语言？",
                    &options
                ).await {
                    warn!("发送投票消息失败: {}", e);
                }
            }
            "!markdown" => {
                if let Err(e) = send_markdown_message(&ctx, &message.channel_id).await {
                    warn!("发送 Markdown 消息失败: {}", e);
                }
            }
            "!buttons" => {
                if let Err(e) = send_button_message(&ctx, &message.channel_id).await {
                    warn!("发送按钮消息失败: {}", e);
                }
            }
            _ => {}
        }
    }
    
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // 处理按钮点击等交互事件
        if let Some(data) = &interaction.data {
            match data.custom_id.as_str() {
                "like_button" => {
                    // 处理点赞
                    info!("用户点击了点赞按钮");
                }
                "vote_0" | "vote_1" | "vote_2" | "vote_3" => {
                    // 处理投票
                    info!("用户进行了投票: {}", data.custom_id);
                }
                "refresh_weather" => {
                    // 刷新天气信息
                    info!("用户请求刷新天气");
                }
                _ => {}
            }
        }
    }
}
```

## 最佳实践

### 设计原则

1. **清晰简洁**: 信息层次分明，避免过度装饰
2. **用户友好**: 按钮文字明确，操作逻辑清晰
3. **响应及时**: 交互操作要有即时反馈
4. **适配主题**: 颜色搭配符合频道主题

### 性能考虑

1. **图片优化**: 使用适当大小的图片，避免过大文件
2. **内容长度**: 控制嵌入内容的字段数量和长度
3. **交互限制**: 合理设置按钮数量，避免界面拥挤
4. **缓存利用**: 对静态内容进行适当缓存

### 错误处理

```rust
async fn safe_send_rich_message<F, Fut>(
    operation_name: &str,
    operation: F
) -> Result<(), botrs::BotError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), botrs::BotError>>,
{
    match operation().await {
        Ok(_) => {
            info!("{} 发送成功", operation_name);
            Ok(())
        }
        Err(e) => {
            warn!("{} 发送失败: {}", operation_name, e);
            Err(e)
        }
    }
}
```

富文本消息让机器人能够提供更加丰富和交互性的用户体验。通过合理使用不同的消息类型，您可以创建出功能强大且用户友好的机器人应用程序。

## 另请参阅

- [交互式消息示例](/zh/examples/interactive-messages.md) - 深入了解交互功能
- [文件上传示例](/zh/examples/file-uploads.md) - 在富文本中集成文件
- [消息处理指南](/zh/guide/messages.md) - 消息系统详细说明
- [API 客户端使用](/zh/guide/api-client.md) - API 使用最佳实践
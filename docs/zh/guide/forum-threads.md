# 论坛与话题

BotRS 为 QQ 频道的论坛功能提供了全面支持，允许机器人与论坛话题、帖子和回复进行交互。本指南涵盖了如何处理论坛事件、创建内容以及管理基于论坛的交互。

## 概述

BotRS 中的论坛系统支持：

- **论坛话题**：具有丰富内容的主要讨论主题
- **话题帖子**：对话题的回应和贡献
- **话题回复**：对帖子的嵌套回应
- **富媒体内容**：论坛内容中的文本、图片、视频和链接
- **内容格式化**：支持包括 Markdown 在内的各种内容格式

## 论坛内容结构

### 内容元素

论坛内容使用结构化元素系统构建：

```rust
use botrs::{Content, Paragraph, Elem, Text, Image, Video, Url, Format};

// 文本元素
let text_elem = Elem {
    element_type: Some(1), // 文本类型
    text: Some(Text {
        text: Some("你好，论坛！".to_string()),
    }),
    image: None,
    video: None,
    url: None,
};

// 图片元素
let image_elem = Elem {
    element_type: Some(2), // 图片类型
    text: None,
    image: Some(Image {
        plat_image: PlatImage {
            url: Some("https://example.com/image.png".to_string()),
            width: Some(800),
            height: Some(600),
            image_id: Some("img_123".to_string()),
        },
    }),
    video: None,
    url: None,
};

// 视频元素
let video_elem = Elem {
    element_type: Some(3), // 视频类型
    text: None,
    image: None,
    video: Some(Video {
        url: Some("https://example.com/video.mp4".to_string()),
        duration: Some(120), // 秒
        cover: Some("https://example.com/cover.jpg".to_string()),
    }),
    url: None,
};
```

### 内容格式化

构建完整的论坛内容：

```rust
impl ForumBot {
    fn create_forum_content(&self, title: &str, elements: Vec<Elem>) -> Content {
        Content {
            paragraph: Some(Paragraph {
                elems: elements,
                props: None,
            }),
        }
    }
    
    async fn post_forum_content(&self, ctx: Context, channel_id: &str, title: &str, content: Content) {
        match ctx.create_forum_thread(channel_id, title, content).await {
            Ok(thread) => {
                println!("论坛话题创建成功: {:?}", thread.thread_id);
            }
            Err(e) => {
                eprintln!("创建论坛话题失败: {}", e);
            }
        }
    }
}
```

## 论坛事件

### 话题事件

处理论坛话题的生命周期事件：

```rust
impl EventHandler for ForumBot {
    async fn open_forum_thread_create(&self, ctx: Context, thread: ForumThread) {
        println!("新话题创建: {}", thread.thread_info.thread_id);
        
        // 欢迎新话题
        let welcome_msg = format!("欢迎来到新话题：{}", thread.thread_info.title);
        if let Err(e) = ctx.send_message(&thread.thread_info.channel_id, &welcome_msg).await {
            eprintln!("发送欢迎消息失败: {}", e);
        }
        
        // 记录话题创建
        self.log_thread_activity(&thread.thread_info.thread_id, "created").await;
    }
    
    async fn open_forum_thread_update(&self, ctx: Context, thread: ForumThread) {
        println!("话题更新: {}", thread.thread_info.thread_id);
        self.log_thread_activity(&thread.thread_info.thread_id, "updated").await;
    }
    
    async fn open_forum_thread_delete(&self, ctx: Context, thread: ForumThread) {
        println!("话题删除: {}", thread.thread_info.thread_id);
        self.cleanup_thread_data(&thread.thread_info.thread_id).await;
    }
}
```

### 帖子和回复事件

处理帖子和回复的创建与删除：

```rust
impl EventHandler for ForumBot {
    async fn open_forum_post_create(&self, ctx: Context, post: ForumPost) {
        println!("新帖子创建在话题: {}", post.thread_id);
        
        // 分析帖子内容
        if let Some(content) = &post.content {
            self.analyze_post_content(content).await;
        }
        
        // 自动回复机制
        if self.should_auto_reply(&post).await {
            self.create_auto_reply(&ctx, &post).await;
        }
    }
    
    async fn open_forum_post_delete(&self, ctx: Context, post: ForumPost) {
        println!("帖子删除: {}", post.post_id);
        self.handle_post_deletion(&post).await;
    }
    
    async fn open_forum_reply_create(&self, ctx: Context, reply: ForumReply) {
        println!("新回复创建: {}", reply.reply_id);
        
        // 通知原帖作者
        if let Some(original_author) = &reply.original_author_id {
            self.notify_author(&ctx, original_author, &reply).await;
        }
    }
    
    async fn open_forum_reply_delete(&self, ctx: Context, reply: ForumReply) {
        println!("回复删除: {}", reply.reply_id);
        self.handle_reply_deletion(&reply).await;
    }
}
```

## 处理论坛内容

### 创建富媒体内容

构建包含多种媒体类型的论坛内容：

```rust
impl ForumBot {
    fn create_rich_content(&self, title: &str, description: &str, image_url: Option<&str>) -> Content {
        let mut elements = Vec::new();
        
        // 添加标题文本
        elements.push(Elem {
            element_type: Some(1),
            text: Some(Text {
                text: Some(format!("## {}\n", title)),
            }),
            image: None,
            video: None,
            url: None,
        });
        
        // 添加描述文本
        elements.push(Elem {
            element_type: Some(1),
            text: Some(Text {
                text: Some(description.to_string()),
            }),
            image: None,
            video: None,
            url: None,
        });
        
        // 添加图片（如果提供）
        if let Some(url) = image_url {
            elements.push(Elem {
                element_type: Some(2),
                text: None,
                image: Some(Image {
                    plat_image: PlatImage {
                        url: Some(url.to_string()),
                        width: Some(800),
                        height: Some(600),
                        image_id: None,
                    },
                }),
                video: None,
                url: None,
            });
        }
        
        Content {
            paragraph: Some(Paragraph {
                elems: elements,
                props: None,
            }),
        }
    }
}
```

### 解析论坛内容

解析和处理接收到的论坛内容：

```rust
impl ForumBot {
    async fn parse_thread_content(&self, content: &Content) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(paragraph) = &content.paragraph {
            for elem in &paragraph.elems {
                match elem.element_type {
                    Some(1) => {
                        // 处理文本元素
                        if let Some(text) = &elem.text {
                            if let Some(content) = &text.text {
                                self.process_text_content(content).await;
                            }
                        }
                    }
                    Some(2) => {
                        // 处理图片元素
                        if let Some(image) = &elem.image {
                            self.process_image_content(image).await;
                        }
                    }
                    Some(3) => {
                        // 处理视频元素
                        if let Some(video) = &elem.video {
                            self.process_video_content(video).await;
                        }
                    }
                    Some(4) => {
                        // 处理URL元素
                        if let Some(url) = &elem.url {
                            self.process_url_content(url).await;
                        }
                    }
                    _ => {
                        println!("未知的内容元素类型");
                    }
                }
            }
        }
        Ok(())
    }
    
    async fn process_content_element(&self, elem: &Elem) {
        match elem.element_type {
            Some(1) if elem.text.is_some() => {
                // 文本处理逻辑
                let text = elem.text.as_ref().unwrap();
                if let Some(content) = &text.text {
                    println!("处理文本内容: {}", content);
                    self.analyze_text_content(content).await;
                }
            }
            Some(2) if elem.image.is_some() => {
                // 图片处理逻辑
                let image = elem.image.as_ref().unwrap();
                if let Some(url) = &image.plat_image.url {
                    println!("处理图片: {}", url);
                    self.process_image_content(image).await;
                }
            }
            Some(3) if elem.video.is_some() => {
                // 视频处理逻辑
                let video = elem.video.as_ref().unwrap();
                if let Some(url) = &video.url {
                    println!("处理视频: {}", url);
                    self.process_video_content(video).await;
                }
            }
            Some(4) if elem.url.is_some() => {
                // URL处理逻辑
                let url = elem.url.as_ref().unwrap();
                println!("处理URL链接");
                self.process_url_content(url).await;
            }
            _ => {
                println!("未处理的内容元素类型");
            }
        }
    }
}
```

## 论坛机器人实现

### 完整的论坛机器人示例

```rust
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct ForumBot {
    thread_stats: HashMap<String, ThreadStats>,
}

struct ThreadStats {
    post_count: u32,
    reply_count: u32,
    last_activity: DateTime<Utc>,
}

impl ForumBot {
    pub fn new() -> Self {
        Self {
            thread_stats: HashMap::new(),
        }
    }
    
    async fn log_thread_activity(&mut self, thread_id: &str, activity: &str) {
        let stats = self.thread_stats.entry(thread_id.to_string()).or_insert(ThreadStats {
            post_count: 0,
            reply_count: 0,
            last_activity: Utc::now(),
        });
        
        stats.last_activity = Utc::now();
        println!("话题 {} 活动记录: {}", thread_id, activity);
    }
    
    async fn cleanup_thread_data(&mut self, thread_id: &str) {
        self.thread_stats.remove(thread_id);
        println!("清理话题数据: {}", thread_id);
    }
    
    async fn handle_new_post(&mut self, post: &ForumPost) {
        if let Some(stats) = self.thread_stats.get_mut(&post.thread_id) {
            stats.post_count += 1;
            stats.last_activity = Utc::now();
        }
    }
    
    async fn handle_new_reply(&mut self, reply: &ForumReply) {
        if let Some(stats) = self.thread_stats.get_mut(&reply.thread_id) {
            stats.reply_count += 1;
            stats.last_activity = Utc::now();
        }
        
        // 自动标记活跃话题
        if stats.reply_count > 10 {
            println!("话题 {} 非常活跃，有 {} 个回复", reply.thread_id, stats.reply_count);
        }
    }
    
    async fn handle_post_deletion(&self, post: &ForumPost) {
        println!("处理帖子删除: {}", post.post_id);
    }
    
    async fn handle_reply_deletion(&self, reply: &ForumReply) {
        println!("处理回复删除: {}", reply.reply_id);
    }
    
    async fn analyze_text_content(&self, content: &str) {
        // 实现内容分析逻辑
        if content.len() > 1000 {
            println!("检测到长文本内容");
        }
        
        if content.contains("@") {
            println!("内容包含提及");
        }
    }
    
    async fn process_image_content(&self, image: &Image) {
        println!("处理图片内容");
    }
    
    async fn process_video_content(&self, video: &Video) {
        println!("处理视频内容");
    }
    
    async fn process_url_content(&self, url: &Url) {
        println!("处理URL内容");
    }
    
    async fn get_thread_statistics(&self, thread_id: &str) -> Option<&ThreadStats> {
        self.thread_stats.get(thread_id)
    }
}

impl EventHandler for ForumBot {
    async fn ready(&self, ctx: Context) {
        println!("论坛机器人已准备就绪");
    }
    
    async fn open_forum_thread_create(&self, ctx: Context, thread: ForumThread) {
        println!("新话题创建: {}", thread.thread_info.thread_id);
        
        // 发送欢迎消息
        let welcome_msg = format!("欢迎来到新话题：{}", thread.thread_info.title);
        if let Err(e) = ctx.send_message(&thread.thread_info.channel_id, &welcome_msg).await {
            eprintln!("发送欢迎消息失败: {}", e);
        }
        
        // 初始化话题统计
        self.thread_stats.insert(thread.thread_info.thread_id.clone(), ThreadStats {
            post_count: 0,
            reply_count: 0,
            last_activity: Utc::now(),
        });
    }
    
    async fn open_forum_thread_update(&self, ctx: Context, thread: ForumThread) {
        println!("话题更新: {}", thread.thread_info.thread_id);
    }
    
    async fn open_forum_thread_delete(&self, ctx: Context, thread: ForumThread) {
        println!("话题删除: {}", thread.thread_info.thread_id);
    }
    
    async fn open_forum_post_create(&self, ctx: Context, post: ForumPost) {
        println!("新帖子创建在话题: {}", post.thread_id);
        
        self.handle_new_post(&post).await;
        
        // 分析帖子内容
        if let Some(content) = &post.content {
            if let Err(e) = self.parse_thread_content(content).await {
                eprintln!("解析帖子内容失败: {}", e);
            }
        }
    }
    
    async fn open_forum_post_delete(&self, ctx: Context, post: ForumPost) {
        self.handle_post_deletion(&post).await;
    }
    
    async fn open_forum_reply_create(&self, ctx: Context, reply: ForumReply) {
        self.handle_new_reply(&reply).await;
    }
    
    async fn open_forum_reply_delete(&self, ctx: Context, reply: ForumReply) {
        self.handle_reply_deletion(&reply).await;
    }
    
    async fn message_create(&self, ctx: Context, msg: botrs::Message) {
        if msg.content.starts_with("!forum") {
            self.handle_forum_command(ctx, msg).await;
        }
    }
}

impl ForumBot {
    async fn handle_forum_command(&self, ctx: Context, msg: botrs::Message) {
        let parts: Vec<&str> = msg.content.split_whitespace().collect();
        
        match parts.get(1) {
            Some(&"stats") => {
                if let Some(thread_id) = parts.get(2) {
                    if let Some(stats) = self.get_thread_statistics(thread_id).await {
                        let stats_msg = format!(
                            "话题统计:\n帖子数: {}\n回复数: {}\n最后活动: {}",
                            stats.post_count,
                            stats.reply_count,
                            stats.last_activity.format("%Y-%m-%d %H:%M:%S")
                        );
                        ctx.send_message(&msg.channel_id, &stats_msg).await.ok();
                    } else {
                        ctx.send_message(&msg.channel_id, "未找到话题统计信息").await.ok();
                    }
                }
            }
            Some(&"help") => {
                let help_msg = "论坛命令:\n!forum stats <thread_id> - 查看话题统计\n!forum help - 显示帮助";
                ctx.send_message(&msg.channel_id, help_msg).await.ok();
            }
            _ => {
                ctx.send_message(&msg.channel_id, "未知的论坛命令，使用 !forum help 查看帮助").await.ok();
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = ForumBot::new();
    let mut client = botrs::Client::new("your_bot_token", botrs::Intents::all());
    
    client.set_event_handler(bot).await;
    client.start().await?;
    
    Ok(())
}
```

## 最佳实践

### 论坛内容管理

- 验证内容格式和大小限制
- 实施适当的内容过滤
- 提供清晰的错误消息
- 支持内容的批量操作

### 性能优化

- 缓存经常访问的话题数据
- 使用分页处理大量内容
- 异步处理内容分析
- 优化数据库查询

### 用户体验

- 提供实时的操作反馈
- 支持富文本预览
- 实现智能内容推荐
- 优化移动端显示

### 错误处理

- 处理网络超时和重试
- 验证用户权限
- 优雅地处理API限制
- 记录详细的错误日志

### 内容安全

- 实施内容审核机制
- 防止垃圾内容和滥用
- 支持用户举报功能
- 遵守平台内容政策
# Forum & Threads

BotRS provides comprehensive support for QQ Guild's forum functionality, allowing bots to interact with forum threads, posts, and replies. This guide covers how to handle forum events, create content, and manage forum-based interactions.

## Overview

The forum system in BotRS supports:

- **Forum Threads**: Main discussion topics with rich content
- **Thread Posts**: Responses and contributions to threads
- **Thread Replies**: Nested responses to posts
- **Rich Content**: Text, images, videos, and URLs in forum content
- **Content Formatting**: Support for various content formats including Markdown

## Forum Content Structure

### Content Elements

Forum content is built using a structured element system:

```rust
use botrs::{Content, Paragraph, Elem, Text, Image, Video, Url, Format};

// Text element
let text_elem = Elem {
    element_type: Some(1), // Text type
    text: Some(Text {
        text: Some("Hello, forum!".to_string()),
    }),
    image: None,
    video: None,
    url: None,
};

// Image element
let image_elem = Elem {
    element_type: Some(2), // Image type
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
```

### Content Formatting

Different content formats are supported:

```rust
use botrs::Format;

match format {
    Format::PlainText => {
        // Simple text content
        println!("Plain text format");
    }
    Format::Html => {
        // HTML formatted content
        println!("HTML format");
    }
    Format::Markdown => {
        // Markdown formatted content
        println!("Markdown format");
    }
    Format::Json => {
        // Structured JSON content
        println!("JSON format");
    }
}
```

## Forum Events

### Thread Events

Handle forum thread creation, updates, and deletion:

```rust
use botrs::{EventHandler, Context, Thread, OpenThread};

impl EventHandler for ForumBot {
    async fn open_forum_thread_create(&self, ctx: Context, thread: OpenThread) {
        println!("New forum thread created!");
        
        if let Some(channel_id) = &thread.channel_id {
            if let Some(author_id) = &thread.author_id {
                let welcome_msg = format!(
                    "Welcome to the forum, <@{}>! Your thread has been created in <#{}>",
                    author_id, channel_id
                );
                
                if let Err(e) = ctx.send_message(channel_id, &welcome_msg).await {
                    eprintln!("Failed to send welcome message: {}", e);
                }
            }
        }
    }
    
    async fn open_forum_thread_update(&self, ctx: Context, thread: OpenThread) {
        println!("Forum thread updated: {:?}", thread.channel_id);
        
        // Handle thread updates
        self.log_thread_activity(&thread, "updated").await;
    }
    
    async fn open_forum_thread_delete(&self, ctx: Context, thread: OpenThread) {
        println!("Forum thread deleted: {:?}", thread.channel_id);
        
        // Clean up any thread-related data
        self.cleanup_thread_data(&thread).await;
    }
}
```

### Post and Reply Events

Handle forum posts and replies:

```rust
impl EventHandler for ForumBot {
    async fn open_forum_post_create(&self, ctx: Context, post_data: serde_json::Value) {
        println!("New forum post created");
        
        // Parse post data
        if let Some(thread_id) = post_data.get("thread_id").and_then(|v| v.as_str()) {
            if let Some(author_id) = post_data.get("author_id").and_then(|v| v.as_str()) {
                // Process new post
                self.handle_new_post(thread_id, author_id, &post_data).await;
            }
        }
    }
    
    async fn open_forum_post_delete(&self, ctx: Context, post_data: serde_json::Value) {
        println!("Forum post deleted");
        
        // Handle post deletion
        if let Some(post_id) = post_data.get("post_id").and_then(|v| v.as_str()) {
            self.handle_post_deletion(post_id).await;
        }
    }
    
    async fn open_forum_reply_create(&self, ctx: Context, reply_data: serde_json::Value) {
        println!("New forum reply created");
        
        // Process reply
        if let Some(post_id) = reply_data.get("post_id").and_then(|v| v.as_str()) {
            if let Some(author_id) = reply_data.get("author_id").and_then(|v| v.as_str()) {
                self.handle_new_reply(post_id, author_id, &reply_data).await;
            }
        }
    }
    
    async fn open_forum_reply_delete(&self, ctx: Context, reply_data: serde_json::Value) {
        println!("Forum reply deleted");
        
        // Handle reply deletion
        if let Some(reply_id) = reply_data.get("reply_id").and_then(|v| v.as_str()) {
            self.handle_reply_deletion(reply_id).await;
        }
    }
}
```

## Working with Forum Content

### Creating Rich Content

Build complex forum content with multiple elements:

```rust
use botrs::{Content, Paragraph, Elem, Text, Image, Video, Url};

impl ForumBot {
    fn create_rich_content(&self) -> Content {
        // Create text element
        let text_elem = Elem {
            element_type: Some(1),
            text: Some(Text {
                text: Some("Welcome to our forum discussion!".to_string()),
            }),
            image: None,
            video: None,
            url: None,
        };
        
        // Create image element
        let image_elem = Elem {
            element_type: Some(2),
            text: None,
            image: Some(Image {
                plat_image: PlatImage {
                    url: Some("https://example.com/welcome.png".to_string()),
                    width: Some(800),
                    height: Some(400),
                    image_id: Some("welcome_img".to_string()),
                },
            }),
            video: None,
            url: None,
        };
        
        // Create URL element
        let url_elem = Elem {
            element_type: Some(4),
            text: None,
            image: None,
            video: None,
            url: Some(Url {
                url: Some("https://example.com/rules".to_string()),
                desc: Some("Forum Rules and Guidelines".to_string()),
            }),
        };
        
        // Combine elements into paragraphs
        let paragraph = Paragraph {
            elems: vec![text_elem, image_elem, url_elem],
            props: None,
        };
        
        Content {
            paragraphs: vec![paragraph],
        }
    }
}
```

### Parsing Forum Content

Extract information from forum content:

```rust
impl ForumBot {
    async fn parse_thread_content(&self, thread: &Thread) {
        // Access thread information
        println!("Thread ID: {:?}", thread.thread_info.thread_id);
        println!("Created: {:?}", thread.thread_info.date_time);
        
        // Parse title content
        for paragraph in &thread.thread_info.title.paragraphs {
            for elem in &paragraph.elems {
                match elem.element_type {
                    Some(1) => {
                        // Text element
                        if let Some(text) = &elem.text {
                            if let Some(content) = &text.text {
                                println!("Title text: {}", content);
                            }
                        }
                    }
                    Some(2) => {
                        // Image element
                        if let Some(image) = &elem.image {
                            if let Some(url) = &image.plat_image.url {
                                println!("Title image: {}", url);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Parse main content
        for paragraph in &thread.thread_info.content.paragraphs {
            for elem in &paragraph.elems {
                self.process_content_element(elem).await;
            }
        }
    }
    
    async fn process_content_element(&self, elem: &Elem) {
        match elem.element_type {
            Some(1) => {
                // Text content
                if let Some(text) = &elem.text {
                    if let Some(content) = &text.text {
                        println!("Text: {}", content);
                        self.analyze_text_content(content).await;
                    }
                }
            }
            Some(2) => {
                // Image content
                if let Some(image) = &elem.image {
                    if let Some(url) = &image.plat_image.url {
                        println!("Image: {} ({}x{})", 
                                url,
                                image.plat_image.width.unwrap_or(0),
                                image.plat_image.height.unwrap_or(0));
                        self.process_image_content(url).await;
                    }
                }
            }
            Some(3) => {
                // Video content
                if let Some(video) = &elem.video {
                    if let Some(url) = &video.plat_video.url {
                        println!("Video: {}", url);
                        if let Some(cover_url) = &video.plat_video.cover.url {
                            println!("Video cover: {}", cover_url);
                        }
                        self.process_video_content(url).await;
                    }
                }
            }
            Some(4) => {
                // URL content
                if let Some(url_elem) = &elem.url {
                    if let Some(url) = &url_elem.url {
                        println!("URL: {} - {}", 
                                url, 
                                url_elem.desc.as_deref().unwrap_or("No description"));
                        self.process_url_content(url).await;
                    }
                }
            }
            _ => {
                println!("Unknown element type: {:?}", elem.element_type);
            }
        }
    }
}
```

## Forum Bot Implementation

### Complete Forum Bot Example

```rust
use botrs::{Client, EventHandler, Context, Thread, OpenThread, Intents};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct ForumBot {
    thread_stats: Mutex<HashMap<String, ThreadStats>>,
}

#[derive(Debug, Clone)]
struct ThreadStats {
    post_count: u32,
    reply_count: u32,
    last_activity: std::time::SystemTime,
}

impl ForumBot {
    pub fn new() -> Self {
        Self {
            thread_stats: Mutex::new(HashMap::new()),
        }
    }
    
    async fn log_thread_activity(&self, thread: &OpenThread, action: &str) {
        if let Some(thread_id) = &thread.channel_id {
            println!("Thread {} {}", thread_id, action);
            
            let mut stats = self.thread_stats.lock().await;
            let thread_stats = stats.entry(thread_id.clone()).or_insert(ThreadStats {
                post_count: 0,
                reply_count: 0,
                last_activity: std::time::SystemTime::now(),
            });
            
            thread_stats.last_activity = std::time::SystemTime::now();
        }
    }
    
    async fn cleanup_thread_data(&self, thread: &OpenThread) {
        if let Some(thread_id) = &thread.channel_id {
            let mut stats = self.thread_stats.lock().await;
            stats.remove(thread_id);
            println!("Cleaned up data for thread {}", thread_id);
        }
    }
    
    async fn handle_new_post(&self, thread_id: &str, author_id: &str, post_data: &serde_json::Value) {
        let mut stats = self.thread_stats.lock().await;
        if let Some(thread_stats) = stats.get_mut(thread_id) {
            thread_stats.post_count += 1;
            thread_stats.last_activity = std::time::SystemTime::now();
        }
        
        println!("New post by {} in thread {}", author_id, thread_id);
    }
    
    async fn handle_new_reply(&self, post_id: &str, author_id: &str, reply_data: &serde_json::Value) {
        // Extract thread ID from reply data if available
        if let Some(thread_id) = reply_data.get("thread_id").and_then(|v| v.as_str()) {
            let mut stats = self.thread_stats.lock().await;
            if let Some(thread_stats) = stats.get_mut(thread_id) {
                thread_stats.reply_count += 1;
                thread_stats.last_activity = std::time::SystemTime::now();
            }
        }
        
        println!("New reply by {} to post {}", author_id, post_id);
    }
    
    async fn handle_post_deletion(&self, post_id: &str) {
        println!("Post {} was deleted", post_id);
        // Implement any cleanup logic for deleted posts
    }
    
    async fn handle_reply_deletion(&self, reply_id: &str) {
        println!("Reply {} was deleted", reply_id);
        // Implement any cleanup logic for deleted replies
    }
    
    async fn analyze_text_content(&self, content: &str) {
        // Implement content analysis (e.g., keyword detection, sentiment analysis)
        if content.contains("help") || content.contains("question") {
            println!("Detected help request in content");
        }
        
        if content.len() > 1000 {
            println!("Long-form content detected ({} characters)", content.len());
        }
    }
    
    async fn process_image_content(&self, url: &str) {
        println!("Processing image: {}", url);
        // Implement image processing logic (e.g., content moderation, analysis)
    }
    
    async fn process_video_content(&self, url: &str) {
        println!("Processing video: {}", url);
        // Implement video processing logic
    }
    
    async fn process_url_content(&self, url: &str) {
        println!("Processing URL: {}", url);
        // Implement URL validation, link analysis, etc.
    }
    
    async fn get_thread_statistics(&self, thread_id: &str) -> Option<ThreadStats> {
        let stats = self.thread_stats.lock().await;
        stats.get(thread_id).cloned()
    }
}

impl EventHandler for ForumBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Forum bot {} is ready!", ready.user.username);
    }
    
    async fn open_forum_thread_create(&self, ctx: Context, thread: OpenThread) {
        println!("New forum thread created in channel {:?}", thread.channel_id);
        self.log_thread_activity(&thread, "created").await;
        
        // Send welcome message to the thread
        if let Some(channel_id) = &thread.channel_id {
            if let Some(author_id) = &thread.author_id {
                let welcome_msg = format!(
                    "Welcome <@{}>! Thank you for starting a new discussion. Please follow our community guidelines.",
                    author_id
                );
                
                let _ = ctx.send_message(channel_id, &welcome_msg).await;
            }
        }
    }
    
    async fn open_forum_thread_update(&self, ctx: Context, thread: OpenThread) {
        println!("Forum thread updated in channel {:?}", thread.channel_id);
        self.log_thread_activity(&thread, "updated").await;
    }
    
    async fn open_forum_thread_delete(&self, ctx: Context, thread: OpenThread) {
        println!("Forum thread deleted in channel {:?}", thread.channel_id);
        self.cleanup_thread_data(&thread).await;
    }
    
    async fn open_forum_post_create(&self, ctx: Context, post_data: serde_json::Value) {
        if let Some(thread_id) = post_data.get("thread_id").and_then(|v| v.as_str()) {
            if let Some(author_id) = post_data.get("author_id").and_then(|v| v.as_str()) {
                self.handle_new_post(thread_id, author_id, &post_data).await;
                
                // Check if this is a milestone post
                if let Some(stats) = self.get_thread_statistics(thread_id).await {
                    if stats.post_count % 10 == 0 {
                        let milestone_msg = format!(
                            "This thread has reached {} posts! 🎉", 
                            stats.post_count
                        );
                        let _ = ctx.send_message(thread_id, &milestone_msg).await;
                    }
                }
            }
        }
    }
    
    async fn open_forum_post_delete(&self, ctx: Context, post_data: serde_json::Value) {
        if let Some(post_id) = post_data.get("post_id").and_then(|v| v.as_str()) {
            self.handle_post_deletion(post_id).await;
        }
    }
    
    async fn open_forum_reply_create(&self, ctx: Context, reply_data: serde_json::Value) {
        if let Some(post_id) = reply_data.get("post_id").and_then(|v| v.as_str()) {
            if let Some(author_id) = reply_data.get("author_id").and_then(|v| v.as_str()) {
                self.handle_new_reply(post_id, author_id, &reply_data).await;
            }
        }
    }
    
    async fn open_forum_reply_delete(&self, ctx: Context, reply_data: serde_json::Value) {
        if let Some(reply_id) = reply_data.get("reply_id").and_then(|v| v.as_str()) {
            self.handle_reply_deletion(reply_id).await;
        }
    }
    
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content.starts_with("!forum ") {
                let command = &content[7..];
                self.handle_forum_command(&ctx, &msg.channel_id, command).await;
            }
        }
    }
}

impl ForumBot {
    async fn handle_forum_command(&self, ctx: &Context, channel_id: &str, command: &str) {
        match command {
            "stats" => {
                if let Some(stats) = self.get_thread_statistics(channel_id).await {
                    let response = format!(
                        "Forum Statistics:\n• Posts: {}\n• Replies: {}\n• Last Activity: {:?}",
                        stats.post_count,
                        stats.reply_count,
                        stats.last_activity
                    );
                    let _ = ctx.send_message(channel_id, &response).await;
                } else {
                    let _ = ctx.send_message(channel_id, "No statistics available for this thread").await;
                }
            }
            "help" => {
                let help_text = "Forum Commands:\n• !forum stats - Show thread statistics\n• !forum help - Show this help message";
                let _ = ctx.send_message(channel_id, help_text).await;
            }
            _ => {
                let _ = ctx.send_message(channel_id, "Unknown forum command. Use !forum help for available commands").await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = ForumBot::new();
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::GUILD_MESSAGES | Intents::FORUMS_EVENT)
        .build()
        .await?;
    
    client.start().await?;
    Ok(())
}
```

## Best Practices

### Forum Content Management

1. **Content Validation**: Validate rich content elements before processing
2. **Image Optimization**: Handle image sizes and formats appropriately
3. **URL Safety**: Validate and sanitize URLs in forum content
4. **Content Moderation**: Implement automated content screening

### Performance Optimization

1. **Lazy Loading**: Process large content elements asynchronously
2. **Caching**: Cache frequently accessed thread data
3. **Batch Processing**: Group related forum operations
4. **Memory Management**: Clean up thread data for deleted threads

### User Experience

1. **Rich Responses**: Use structured content in bot responses
2. **Activity Tracking**: Monitor and respond to forum engagement
3. **Milestone Celebrations**: Acknowledge thread milestones
4. **Help Integration**: Provide contextual help and guidance

### Error Handling

1. **Content Parsing**: Handle malformed content gracefully
2. **Missing Data**: Provide fallbacks for missing thread information
3. **API Failures**: Implement retry logic for forum operations
4. **Cleanup**: Ensure proper cleanup of deleted content

The forum and threads system in BotRS enables rich, interactive discussions with support for multimedia content and structured data. By leveraging these capabilities, you can create engaging forum experiences that enhance community interaction and knowledge sharing.
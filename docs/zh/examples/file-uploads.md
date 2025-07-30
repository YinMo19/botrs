# 文件上传示例

此示例演示如何使用 BotRS 处理文件上传、下载和管理，包括图像处理、文件验证和安全检查。

## 概述

文件上传功能允许机器人接收、处理和管理用户上传的文件。这包括图像、文档和其他媒体文件的处理。

## 基本文件上传

### 上传图像

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token, MessageParams};
use std::path::Path;
use tracing::{info, warn, error};

struct FileBot;

#[async_trait::async_trait]
impl EventHandler for FileBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("文件机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.is_from_bot() {
            return;
        }

        if let Some(content) = &msg.content {
            match content.trim() {
                "!upload_image" => {
                    if let Err(e) = self.upload_sample_image(&ctx, &msg).await {
                        warn!("上传示例图像失败：{}", e);
                    }
                }
                "!upload_doc" => {
                    if let Err(e) = self.upload_sample_document(&ctx, &msg).await {
                        warn!("上传示例文档失败：{}", e);
                    }
                }
                _ if content.starts_with("!upload_url ") => {
                    let url = &content[12..];
                    if let Err(e) = self.upload_from_url(&ctx, &msg, url).await {
                        warn!("从 URL 上传失败：{}", e);
                    }
                }
                _ => {}
            }
        }

        // 处理用户上传的附件
        if let Some(attachments) = &msg.attachments {
            if !attachments.is_empty() {
                if let Err(e) = self.process_user_attachments(&ctx, &msg, attachments).await {
                    warn!("处理用户附件失败：{}", e);
                }
            }
        }
    }
}

impl FileBot {
    async fn upload_sample_image(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        // 创建示例图像数据
        let image_data = self.create_sample_image().await?;
        
        let params = MessageParams::new_text("这里是一个示例图像：")
            .with_file_data("sample.png", image_data);

        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await?;
        info!("成功上传示例图像");
        Ok(())
    }

    async fn upload_sample_document(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        // 创建示例文档
        let document_content = "# 示例文档\n\n这是一个示例 Markdown 文档。\n\n## 功能\n- 文件上传\n- 文档处理\n- 内容分析";
        
        let params = MessageParams::new_text("这里是一个示例文档：")
            .with_file_data("example.md", document_content.as_bytes().to_vec());

        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await?;
        info!("成功上传示例文档");
        Ok(())
    }

    async fn upload_from_url(&self, ctx: &Context, msg: &Message, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 从 URL 下载文件
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        
        // 检查文件大小
        const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if let Some(content_length) = response.content_length() {
            if content_length > MAX_FILE_SIZE as u64 {
                msg.reply(&ctx.api, &ctx.token, "文件太大，无法上传（最大 10MB）。").await?;
                return Ok(());
            }
        }

        let file_data = response.bytes().await?;
        
        // 从 URL 提取文件名
        let filename = url.split('/').last().unwrap_or("downloaded_file");
        
        let params = MessageParams::new_text(&format!("从 URL 下载的文件：{}", url))
            .with_file_data(filename, file_data.to_vec());

        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await?;
        info!("成功从 URL 上传文件：{}", url);
        Ok(())
    }

    async fn process_user_attachments(&self, ctx: &Context, msg: &Message, attachments: &[botrs::MessageAttachment]) -> Result<(), Box<dyn std::error::Error>> {
        for attachment in attachments {
            info!("处理附件：{}", attachment.filename.as_deref().unwrap_or("未知文件"));
            
            // 验证文件
            if let Err(error_msg) = self.validate_file_upload(attachment) {
                msg.reply(&ctx.api, &ctx.token, &error_msg).await?;
                continue;
            }

            // 根据文件类型处理
            if let Some(content_type) = &attachment.content_type {
                if content_type.starts_with("image/") {
                    self.process_image_attachment(ctx, msg, attachment).await?;
                } else {
                    let response = format!("收到文件：{} ({})", 
                        attachment.filename.as_deref().unwrap_or("未知"),
                        content_type);
                    msg.reply(&ctx.api, &ctx.token, &response).await?;
                }
            }
        }
        Ok(())
    }

    async fn process_image_attachment(&self, ctx: &Context, msg: &Message, attachment: &botrs::MessageAttachment) -> Result<(), Box<dyn std::error::Error>> {
        let filename = attachment.filename.as_deref().unwrap_or("image");
        let size = attachment.size.unwrap_or(0);
        
        // 分析图像信息
        let analysis = self.analyze_image(attachment);
        
        let response = format!(
            "📷 收到图像：{}\n📏 大小：{} bytes\n🔍 分析：{}",
            filename, size, analysis
        );
        
        msg.reply(&ctx.api, &ctx.token, &response).await?;
        Ok(())
    }

    fn analyze_image(&self, attachment: &botrs::MessageAttachment) -> String {
        let filename = attachment.filename.as_deref().unwrap_or("");
        match filename.split('.').last() {
            Some("jpg") | Some("jpeg") => "JPEG 图像，适合照片",
            Some("png") => "PNG 图像，支持透明度",
            Some("gif") => "GIF 图像，可能包含动画",
            Some("webp") => "WebP 图像，现代格式",
            _ => "未知图像格式",
        }.to_string()
    }

    async fn create_sample_image(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 创建简单的 PNG 图像数据（实际应用中可能使用图像处理库）
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG 签名
            // 简化的 PNG 数据...
        ];
        Ok(png_data)
    }
}
```

## 高级文件操作

### 文件验证和安全

```rust
impl FileBot {
    fn validate_file_upload(&self, attachment: &botrs::MessageAttachment) -> Result<(), String> {
        const MAX_SIZE: usize = 50 * 1024 * 1024; // 50MB
        
        // 检查文件大小
        if let Some(size) = attachment.size {
            if size > MAX_SIZE {
                return Err("文件太大，最大允许 50MB。".to_string());
            }
        }

        // 检查文件类型
        if let Some(content_type) = &attachment.content_type {
            let allowed_types = [
                "image/jpeg", "image/png", "image/gif", "image/webp",
                "text/plain", "text/markdown", "application/pdf",
                "application/zip", "video/mp4", "audio/mpeg"
            ];
            
            if !allowed_types.iter().any(|&t| content_type.starts_with(t)) {
                return Err(format!("不支持的文件类型：{}", content_type));
            }
        }

        // 检查文件名
        if let Some(filename) = &attachment.filename {
            let dangerous_extensions = ["exe", "bat", "cmd", "scr", "com"];
            if let Some(ext) = filename.split('.').last() {
                if dangerous_extensions.contains(&ext.to_lowercase().as_str()) {
                    return Err("危险的文件类型，上传被拒绝。".to_string());
                }
            }
        }

        Ok(())
    }

    async fn scan_file_for_safety(&self, file_data: &[u8]) -> bool {
        // 简化的安全扫描 - 实际应用中应使用专业的反病毒引擎
        
        // 检查常见的恶意软件签名
        let malicious_signatures = [
            b"MZ\x90\x00", // PE 可执行文件头
            b"\x7fELF",     // ELF 可执行文件头
        ];

        for signature in &malicious_signatures {
            if file_data.starts_with(signature) {
                return false;
            }
        }

        true
    }
}
```

### 图像处理

```rust
impl FileBot {
    async fn process_image_effects(&self, ctx: &Context, msg: &Message, effect: &str, attachment: &botrs::MessageAttachment) -> Result<(), Box<dyn std::error::Error>> {
        // 下载原始图像
        let client = reqwest::Client::new();
        let image_data = if let Some(url) = &attachment.url {
            client.get(url).send().await?.bytes().await?.to_vec()
        } else {
            return Err("无法获取图像数据".into());
        };

        // 应用效果
        let processed_data = match effect {
            "grayscale" => self.apply_grayscale(&image_data).await?,
            "blur" => self.apply_blur(&image_data).await?,
            "brightness" => self.apply_brightness(&image_data).await?,
            _ => {
                msg.reply(&ctx.api, &ctx.token, "未知的图像效果。").await?;
                return Ok(());
            }
        };

        // 发送处理后的图像
        let filename = format!("{}_{}", effect, attachment.filename.as_deref().unwrap_or("image.png"));
        let params = MessageParams::new_text(&format!("应用 {} 效果后的图像：", effect))
            .with_file_data(&filename, processed_data);

        ctx.api.post_message_with_params(&ctx.token, &msg.channel_id, params).await?;
        Ok(())
    }

    async fn apply_grayscale(&self, _image_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 实际实现中使用图像处理库如 image crate
        Ok(vec![]) // 占位符
    }

    async fn apply_blur(&self, _image_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![]) // 占位符
    }

    async fn apply_brightness(&self, _image_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![]) // 占位符
    }
}
```

### 文件存储管理

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

pub struct FileStorage {
    base_path: PathBuf,
    file_index: HashMap<String, FileMetadata>,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub uploaded_by: String,
    pub uploaded_at: DateTime<Utc>,
    pub path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
            file_index: HashMap::new(),
        }
    }

    pub async fn store_file(&mut self, filename: &str, data: &[u8], user_id: &str, content_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        // 创建唯一的文件 ID
        let file_id = uuid::Uuid::new_v4().to_string();
        
        // 构建文件路径
        let user_dir = self.base_path.join(user_id);
        tokio::fs::create_dir_all(&user_dir).await?;
        
        let file_path = user_dir.join(&file_id);
        
        // 保存文件
        tokio::fs::write(&file_path, data).await?;
        
        // 记录元数据
        let metadata = FileMetadata {
            filename: filename.to_string(),
            size: data.len() as u64,
            content_type: content_type.to_string(),
            uploaded_by: user_id.to_string(),
            uploaded_at: Utc::now(),
            path: file_path,
        };
        
        self.file_index.insert(file_id.clone(), metadata);
        Ok(file_id)
    }

    pub async fn retrieve_file(&self, file_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if let Some(metadata) = self.file_index.get(file_id) {
            let data = tokio::fs::read(&metadata.path).await?;
            Ok(data)
        } else {
            Err("文件未找到".into())
        }
    }

    pub fn list_files_by_user(&self, user_id: &str) -> Vec<&FileMetadata> {
        self.file_index
            .values()
            .filter(|meta| meta.uploaded_by == user_id)
            .collect()
    }

    pub async fn delete_file(&mut self, file_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(metadata) = self.file_index.remove(file_id) {
            tokio::fs::remove_file(&metadata.path).await?;
            Ok(())
        } else {
            Err("文件未找到".into())
        }
    }
}
```

## 文件管理命令

```rust
impl FileBot {
    async fn handle_file_commands(&self, ctx: &Context, msg: &Message, command: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            "store" => {
                if let Some(attachments) = &msg.attachments {
                    if !attachments.is_empty() {
                        self.store_user_files(ctx, msg, attachments).await?;
                    } else {
                        msg.reply(&ctx.api, &ctx.token, "请附加要存储的文件。").await?;
                    }
                }
            }
            "list" => {
                self.list_user_files(ctx, msg).await?;
            }
            "delete" => {
                if let Some(file_id) = args.get(0) {
                    // 实现文件删除逻辑
                    msg.reply(&ctx.api, &ctx.token, &format!("文件 {} 已删除。", file_id)).await?;
                } else {
                    msg.reply(&ctx.api, &ctx.token, "请提供要删除的文件 ID。").await?;
                }
            }
            _ => {
                msg.reply(&ctx.api, &ctx.token, "未知的文件命令。使用 !help files 查看可用命令。").await?;
            }
        }
        Ok(())
    }

    async fn store_user_files(&self, ctx: &Context, msg: &Message, attachments: &[botrs::MessageAttachment]) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = msg.author.as_ref()
            .and_then(|a| a.id.as_ref())
            .unwrap_or("unknown");

        let mut stored_files = Vec::new();

        for attachment in attachments {
            // 验证文件
            if let Err(error) = self.validate_file_upload(attachment) {
                msg.reply(&ctx.api, &ctx.token, &error).await?;
                continue;
            }

            // 下载文件数据
            if let Some(url) = &attachment.url {
                let client = reqwest::Client::new();
                let file_data = client.get(url).send().await?.bytes().await?;

                // 安全扫描
                if !self.scan_file_for_safety(&file_data).await {
                    msg.reply(&ctx.api, &ctx.token, "文件安全扫描失败，存储被拒绝。").await?;
                    continue;
                }

                // 存储文件（这里需要实际的存储实现）
                let filename = attachment.filename.as_deref().unwrap_or("unknown");
                let content_type = attachment.content_type.as_deref().unwrap_or("application/octet-stream");
                
                // 模拟文件存储
                let file_id = uuid::Uuid::new_v4().to_string();
                stored_files.push((filename, file_id));
            }
        }

        if !stored_files.is_empty() {
            let mut response = "文件已成功存储：\n".to_string();
            for (filename, file_id) in stored_files {
                response.push_str(&format!("• {} (ID: {})\n", filename, file_id));
            }
            msg.reply(&ctx.api, &ctx.token, &response).await?;
        }

        Ok(())
    }

    async fn list_user_files(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let user_id = msg.author.as_ref()
            .and_then(|a| a.id.as_ref())
            .unwrap_or("unknown");

        // 模拟获取用户文件列表
        let response = format!("用户 {} 的文件列表：\n• example.png (ID: 123456)\n• document.pdf (ID: 789012)", user_id);
        msg.reply(&ctx.api, &ctx.token, &response).await?;
        Ok(())
    }
}
```

## 使用示例

### 基本文件操作

```bash
!upload_image           # 上传示例图像
!upload_doc            # 上传示例文档
!upload_url https://example.com/file.png  # 从 URL 上传

# 发送附件消息
# 附加文件到消息中，机器人会自动处理
```

### 图像处理

```bash
# 发送图像附件并添加效果命令
!effect grayscale      # 应用灰度效果
!effect blur          # 应用模糊效果
!effect brightness    # 调整亮度
```

### 文件管理

```bash
!file store           # 存储附件文件
!file list           # 列出用户文件
!file delete 123456  # 删除文件
```

## 最佳实践

1. **文件验证**：始终验证文件类型、大小和内容
2. **安全扫描**：实施病毒和恶意软件检查
3. **存储管理**：合理组织文件存储结构
4. **权限控制**：限制文件访问权限
5. **清理策略**：定期清理过期或无用文件
6. **备份机制**：重要文件应有备份
7. **带宽优化**：压缩大文件以节省带宽

## 安全考虑

1. **文件类型过滤**：只允许安全的文件类型
2. **大小限制**：防止大文件攻击
3. **扫描检查**：使用反病毒引擎扫描上传文件
4. **隔离存储**：将用户文件与系统文件隔离
5. **访问控制**：实施严格的文件访问控制
6. **日志记录**：记录所有文件操作以便审计

## 相关链接

- [富消息](./rich-messages.md) - 了解如何发送丰富的消息内容
- [命令处理器](./command-handler.md) - 创建文件管理命令
- [错误恢复](./error-recovery.md) - 处理文件操作错误
- [API 集成](./api-integration.md) - 集成外部文件服务
# File Uploads Example

This example demonstrates how to handle file uploads and downloads in your QQ Guild bot using BotRS.

## Overview

File handling is an essential feature for many bots. This example shows how to upload images, documents, and other media files, as well as how to process files sent by users.

## Basic File Upload

### Uploading Images

```rust
use botrs::{
    Client, Context, EventHandler, Intents, Message, Ready, Token, BotError,
    models::message::{MessageParams, Media}
};
use std::fs;
use base64;

struct FileBot;

#[async_trait::async_trait]
impl EventHandler for FileBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("File bot ready: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        if let Some(content) = &message.content {
            match content.trim() {
                "!upload-image" => {
                    if let Err(e) = self.upload_sample_image(&ctx, &message).await {
                        eprintln!("Failed to upload image: {}", e);
                    }
                }
                "!upload-document" => {
                    if let Err(e) = self.upload_sample_document(&ctx, &message).await {
                        eprintln!("Failed to upload document: {}", e);
                    }
                }
                _ if content.starts_with("!upload-url ") => {
                    let url = content.strip_prefix("!upload-url ").unwrap();
                    if let Err(e) = self.upload_from_url(&ctx, &message, url).await {
                        eprintln!("Failed to upload from URL: {}", e);
                    }
                }
                _ => {}
            }
        }

        // Handle file attachments sent by users
        if message.has_attachments() {
            if let Err(e) = self.process_user_attachments(&ctx, &message).await {
                eprintln!("Failed to process attachments: {}", e);
            }
        }
    }
}

impl FileBot {
    async fn upload_sample_image(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        // Read image file from local storage
        let image_path = "assets/sample.png";
        let image_data = match fs::read(image_path) {
            Ok(data) => data,
            Err(_) => {
                // Fallback: create a simple colored rectangle as PNG
                self.create_sample_image().await?
            }
        };

        // Convert to base64
        let base64_image = base64::encode(&image_data);
        
        // Create file info string (this format may vary based on QQ API requirements)
        let file_info = format!("data:image/png;base64,{}", base64_image);

        let params = MessageParams::new_text("Here's a sample image!")
            .with_file_image(&file_info);

        ctx.send_message(&message.channel_id, &params).await?;
        
        message.reply(&ctx.api, &ctx.token, "Image uploaded successfully! 📸").await?;
        Ok(())
    }

    async fn upload_sample_document(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        // Create a sample text document
        let document_content = r#"
# Sample Document

This is a sample document created by the QQ Guild Bot.

## Features
- Text formatting
- Lists and bullet points
- Code blocks

## Bot Information
- Framework: BotRS
- Language: Rust
- Version: 0.2.5

Thank you for using our bot!
"#;

        let document_bytes = document_content.as_bytes();
        let base64_document = base64::encode(document_bytes);
        
        // For documents, you might need to use Media instead of file_image
        let media = Media {
            file_info: Some(format!("data:text/markdown;base64,{}", base64_document)),
            ttl: Some(3600), // 1 hour TTL
        };

        // Note: This example assumes group message context for media upload
        // For guild channels, you might need different approach
        let group_params = botrs::models::message::GroupMessageParams {
            content: Some("Here's a sample document! 📄".to_string()),
            media: Some(media),
            ..Default::default()
        };

        // This would require group context - adapting for channel instead
        message.reply(&ctx.api, &ctx.token, "Document prepared! (Check group messages for media upload)").await?;
        Ok(())
    }

    async fn upload_from_url(&self, ctx: &Context, message: &Message, url: &str) -> Result<(), BotError> {
        // Download file from URL
        let response = reqwest::get(url).await
            .map_err(|e| BotError::Network(format!("Failed to download file: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(BotError::Network(format!("HTTP error: {}", response.status())));
        }

        let content_type = response.headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("application/octet-stream");

        let file_bytes = response.bytes().await
            .map_err(|e| BotError::Network(format!("Failed to read response: {}", e)))?;

        // Check file size (25MB limit example)
        const MAX_FILE_SIZE: usize = 25 * 1024 * 1024;
        if file_bytes.len() > MAX_FILE_SIZE {
            message.reply(&ctx.api, &ctx.token, "File too large! Maximum size is 25MB.").await?;
            return Ok(());
        }

        // Check if it's an image
        if content_type.starts_with("image/") {
            let base64_data = base64::encode(&file_bytes);
            let file_info = format!("data:{};base64,{}", content_type, base64_data);

            let params = MessageParams::new_text(&format!("Downloaded from: {}", url))
                .with_file_image(&file_info);

            ctx.send_message(&message.channel_id, &params).await?;
        } else {
            message.reply(&ctx.api, &ctx.token, "Non-image files require different upload method").await?;
        }

        Ok(())
    }

    async fn process_user_attachments(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        for attachment in &message.attachments {
            let filename = attachment.filename.as_deref().unwrap_or("unknown");
            let size = attachment.size.unwrap_or(0);
            let content_type = attachment.content_type.as_deref().unwrap_or("unknown");

            let mut response = format!("📎 **File Detected**\n");
            response.push_str(&format!("**Name:** {}\n", filename));
            response.push_str(&format!("**Size:** {} bytes\n", size));
            response.push_str(&format!("**Type:** {}\n", content_type));

            if attachment.is_image() {
                response.push_str("**Category:** Image 🖼️\n");
                
                if let Some(width) = attachment.width {
                    if let Some(height) = attachment.height {
                        response.push_str(&format!("**Dimensions:** {}x{}\n", width, height));
                    }
                }

                // Process image
                if let Err(e) = self.process_image_attachment(ctx, message, attachment).await {
                    eprintln!("Failed to process image: {}", e);
                    response.push_str("*Failed to process image*\n");
                }
            } else if attachment.is_video() {
                response.push_str("**Category:** Video 🎥\n");
            } else if attachment.is_audio() {
                response.push_str("**Category:** Audio 🎵\n");
            } else {
                response.push_str("**Category:** Document 📄\n");
            }

            message.reply(&ctx.api, &ctx.token, &response).await?;
        }

        Ok(())
    }

    async fn process_image_attachment(
        &self,
        ctx: &Context,
        message: &Message,
        attachment: &botrs::models::message::MessageAttachment,
    ) -> Result<(), BotError> {
        if let Some(url) = &attachment.url {
            // Download and analyze the image
            let response = reqwest::get(url).await
                .map_err(|e| BotError::Network(format!("Failed to download attachment: {}", e)))?;

            if response.status().is_success() {
                let image_bytes = response.bytes().await
                    .map_err(|e| BotError::Network(format!("Failed to read image: {}", e)))?;

                // Basic image analysis
                let analysis = self.analyze_image(&image_bytes);
                
                message.reply(&ctx.api, &ctx.token, &format!("🔍 **Image Analysis:**\n{}", analysis)).await?;
            }
        }

        Ok(())
    }

    fn analyze_image(&self, _image_data: &[u8]) -> String {
        // Simple image analysis (in a real bot, you might use image processing libraries)
        // This is a placeholder implementation
        format!(
            "• File format appears to be valid\n\
             • Ready for processing\n\
             • Use `!enhance` to apply filters"
        )
    }

    async fn create_sample_image(&self) -> Result<Vec<u8>, BotError> {
        // Create a simple PNG image programmatically
        // This is a simplified example - in practice you'd use an image library
        
        // For now, return a minimal valid PNG header + data
        // In a real implementation, use libraries like `image` or `png`
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            // Add minimal PNG chunks here...
        ];
        
        Ok(png_data)
    }
}
```

## Advanced File Operations

### File Validation and Security

```rust
use std::path::Path;

impl FileBot {
    fn validate_file_upload(&self, filename: &str, size: u64, content_type: &str) -> Result<(), String> {
        // Check file size
        const MAX_SIZE: u64 = 25 * 1024 * 1024; // 25MB
        if size > MAX_SIZE {
            return Err(format!("File size {} exceeds maximum of {} bytes", size, MAX_SIZE));
        }

        // Check file extension
        let allowed_extensions = [
            "jpg", "jpeg", "png", "gif", "webp", // Images
            "pdf", "txt", "md", "doc", "docx",   // Documents
            "mp3", "wav", "ogg",                 // Audio
            "mp4", "avi", "mov", "webm",         // Video
        ];

        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .unwrap_or_default();

        if !allowed_extensions.contains(&extension.as_str()) {
            return Err(format!("File type '{}' is not allowed", extension));
        }

        // Check MIME type
        let allowed_mime_prefixes = ["image/", "text/", "application/pdf", "audio/", "video/"];
        if !allowed_mime_prefixes.iter().any(|prefix| content_type.starts_with(prefix)) {
            return Err(format!("Content type '{}' is not allowed", content_type));
        }

        // Additional security checks
        if filename.contains("..") || filename.contains("/") || filename.contains("\\") {
            return Err("Invalid filename: path traversal detected".to_string());
        }

        Ok(())
    }

    async fn scan_file_for_safety(&self, file_data: &[u8], filename: &str) -> Result<bool, BotError> {
        // Basic file signature validation
        let is_safe = match filename.split('.').last().unwrap_or("").to_lowercase().as_str() {
            "png" => file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47]),
            "jpg" | "jpeg" => file_data.starts_with(&[0xFF, 0xD8, 0xFF]),
            "gif" => file_data.starts_with(b"GIF8"),
            "pdf" => file_data.starts_with(b"%PDF"),
            _ => true, // Allow other types for now
        };

        if !is_safe {
            return Err(BotError::InvalidInput("File signature doesn't match extension".to_string()));
        }

        // Additional virus scanning could be implemented here
        // using external services or libraries

        Ok(true)
    }
}
```

### Image Processing

```rust
// Note: This requires adding image processing dependencies to Cargo.toml
// [dependencies]
// image = "0.24"

impl FileBot {
    async fn process_image_effects(&self, ctx: &Context, message: &Message, effect: &str) -> Result<(), BotError> {
        // This is a conceptual example - actual implementation would require image processing libraries
        
        if !message.has_attachments() {
            message.reply(&ctx.api, &ctx.token, "Please attach an image to process!").await?;
            return Ok(());
        }

        for attachment in &message.attachments {
            if !attachment.is_image() {
                continue;
            }

            if let Some(url) = &attachment.url {
                // Download image
                let response = reqwest::get(url).await
                    .map_err(|e| BotError::Network(format!("Failed to download image: {}", e)))?;
                
                let image_bytes = response.bytes().await
                    .map_err(|e| BotError::Network(format!("Failed to read image: {}", e)))?;

                // Process image based on effect
                let processed_image = match effect {
                    "grayscale" => self.apply_grayscale(&image_bytes).await?,
                    "blur" => self.apply_blur(&image_bytes).await?,
                    "brighten" => self.apply_brightness(&image_bytes, 1.2).await?,
                    "darken" => self.apply_brightness(&image_bytes, 0.8).await?,
                    _ => {
                        message.reply(&ctx.api, &ctx.token, "Unknown effect! Available: grayscale, blur, brighten, darken").await?;
                        return Ok(());
                    }
                };

                // Upload processed image
                let base64_image = base64::encode(&processed_image);
                let file_info = format!("data:image/png;base64,{}", base64_image);

                let params = MessageParams::new_text(&format!("Applied '{}' effect:", effect))
                    .with_file_image(&file_info);

                ctx.send_message(&message.channel_id, &params).await?;
            }
        }

        Ok(())
    }

    async fn apply_grayscale(&self, image_data: &[u8]) -> Result<Vec<u8>, BotError> {
        // Placeholder for image processing
        // In real implementation, use image processing library
        Ok(image_data.to_vec())
    }

    async fn apply_blur(&self, image_data: &[u8]) -> Result<Vec<u8>, BotError> {
        // Placeholder for blur effect
        Ok(image_data.to_vec())
    }

    async fn apply_brightness(&self, image_data: &[u8], factor: f32) -> Result<Vec<u8>, BotError> {
        // Placeholder for brightness adjustment
        Ok(image_data.to_vec())
    }
}
```

### File Storage Management

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

pub struct FileStorage {
    base_path: PathBuf,
    file_index: HashMap<String, FileMetadata>,
}

#[derive(Clone)]
pub struct FileMetadata {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub uploaded_by: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
            file_index: HashMap::new(),
        }
    }

    pub async fn store_file(
        &mut self,
        file_data: &[u8],
        filename: &str,
        content_type: &str,
        uploaded_by: &str,
    ) -> Result<String, BotError> {
        // Generate unique file ID
        let file_id = uuid::Uuid::new_v4().to_string();
        
        // Create storage path
        let file_path = self.base_path.join(&file_id);
        
        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| BotError::InternalError(format!("Failed to create directory: {}", e)))?;
        }

        // Write file
        fs::write(&file_path, file_data).await
            .map_err(|e| BotError::InternalError(format!("Failed to write file: {}", e)))?;

        // Store metadata
        let metadata = FileMetadata {
            filename: filename.to_string(),
            size: file_data.len() as u64,
            content_type: content_type.to_string(),
            uploaded_by: uploaded_by.to_string(),
            uploaded_at: chrono::Utc::now(),
            path: file_path,
        };

        self.file_index.insert(file_id.clone(), metadata);

        Ok(file_id)
    }

    pub async fn retrieve_file(&self, file_id: &str) -> Result<(Vec<u8>, &FileMetadata), BotError> {
        let metadata = self.file_index.get(file_id)
            .ok_or_else(|| BotError::InvalidInput("File not found".to_string()))?;

        let file_data = fs::read(&metadata.path).await
            .map_err(|e| BotError::InternalError(format!("Failed to read file: {}", e)))?;

        Ok((file_data, metadata))
    }

    pub fn list_files_by_user(&self, user_id: &str) -> Vec<&FileMetadata> {
        self.file_index
            .values()
            .filter(|metadata| metadata.uploaded_by == user_id)
            .collect()
    }

    pub async fn delete_file(&mut self, file_id: &str) -> Result<(), BotError> {
        if let Some(metadata) = self.file_index.remove(file_id) {
            fs::remove_file(&metadata.path).await
                .map_err(|e| BotError::InternalError(format!("Failed to delete file: {}", e)))?;
        }

        Ok(())
    }
}
```

## File Management Commands

```rust
impl FileBot {
    async fn handle_file_commands(&self, ctx: &Context, message: &Message, command: &str, args: &[String]) -> Result<(), BotError> {
        match command {
            "upload" => {
                if message.has_attachments() {
                    self.store_user_files(ctx, message).await?;
                } else {
                    message.reply(&ctx.api, &ctx.token, "Please attach files to upload").await?;
                }
            }
            "list" => {
                self.list_user_files(ctx, message).await?;
            }
            "download" => {
                if let Some(file_id) = args.get(0) {
                    self.download_file(ctx, message, file_id).await?;
                } else {
                    message.reply(&ctx.api, &ctx.token, "Usage: !download <file_id>").await?;
                }
            }
            "delete" => {
                if let Some(file_id) = args.get(0) {
                    self.delete_user_file(ctx, message, file_id).await?;
                } else {
                    message.reply(&ctx.api, &ctx.token, "Usage: !delete <file_id>").await?;
                }
            }
            "info" => {
                if let Some(file_id) = args.get(0) {
                    self.show_file_info(ctx, message, file_id).await?;
                } else {
                    message.reply(&ctx.api, &ctx.token, "Usage: !info <file_id>").await?;
                }
            }
            _ => {
                message.reply(&ctx.api, &ctx.token, "Unknown file command").await?;
            }
        }

        Ok(())
    }

    async fn store_user_files(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let user_id = message.author.as_ref().unwrap().id.clone();
        let mut stored_files = Vec::new();

        for attachment in &message.attachments {
            let filename = attachment.filename.as_deref().unwrap_or("unknown");
            let size = attachment.size.unwrap_or(0);
            let content_type = attachment.content_type.as_deref().unwrap_or("application/octet-stream");

            // Validate file
            if let Err(error) = self.validate_file_upload(filename, size, content_type) {
                message.reply(&ctx.api, &ctx.token, &format!("❌ {}: {}", filename, error)).await?;
                continue;
            }

            if let Some(url) = &attachment.url {
                // Download file
                let response = reqwest::get(url).await
                    .map_err(|e| BotError::Network(format!("Failed to download: {}", e)))?;

                let file_data = response.bytes().await
                    .map_err(|e| BotError::Network(format!("Failed to read: {}", e)))?;

                // Security scan
                if !self.scan_file_for_safety(&file_data, filename).await? {
                    message.reply(&ctx.api, &ctx.token, &format!("❌ {}: Security scan failed", filename)).await?;
                    continue;
                }

                // Store file (this would use the FileStorage implementation)
                // let file_id = storage.store_file(&file_data, filename, content_type, &user_id).await?;
                let file_id = format!("file_{}", uuid::Uuid::new_v4());
                stored_files.push((file_id, filename.to_string()));
            }
        }

        if !stored_files.is_empty() {
            let mut response = "✅ **Files uploaded successfully:**\n\n".to_string();
            for (file_id, filename) in stored_files {
                response.push_str(&format!("📁 `{}` → ID: `{}`\n", filename, file_id));
            }
            response.push_str("\nUse `!list` to see all your files or `!download <file_id>` to retrieve a file.");

            message.reply(&ctx.api, &ctx.token, &response).await?;
        }

        Ok(())
    }

    async fn list_user_files(&self, ctx: &Context, message: &Message) -> Result<(), BotError> {
        let user_id = message.author.as_ref().unwrap().id.clone();
        
        // This would query the actual file storage
        // let files = storage.list_files_by_user(&user_id);
        
        // Mock data for example
        let mock_files = vec![
            ("file_123", "image.png", "2.1 MB", "2024-01-15"),
            ("file_456", "document.pdf", "1.5 MB", "2024-01-14"),
        ];

        if mock_files.is_empty() {
            message.reply(&ctx.api, &ctx.token, "You haven't uploaded any files yet. Use `!upload` to store files.").await?;
            return Ok(());
        }

        let mut response = "📂 **Your uploaded files:**\n\n".to_string();
        for (file_id, filename, size, date) in mock_files {
            response.push_str(&format!("🗂️ **{}**\n", filename));
            response.push_str(&format!("   📋 ID: `{}`\n", file_id));
            response.push_str(&format!("   📏 Size: {}\n", size));
            response.push_str(&format!("   📅 Uploaded: {}\n\n", date));
        }
        
        response.push_str("Use `!download <file_id>` to retrieve a file or `!info <file_id>` for details.");

        message.reply(&ctx.api, &ctx.token, &response).await?;
        Ok(())
    }
}
```

## Usage Examples

### Basic File Operations

```
# Upload an image (attach file to message)
!upload

# List your uploaded files
!list

# Download a specific file
!download file_123

# Get file information
!info file_123

# Delete a file
!delete file_123
```

### Image Processing

```
# Apply effects to attached images
!effect grayscale
!effect blur
!effect brighten
!effect darken

# Upload from URL
!upload-url https://example.com/image.jpg
```

## Best Practices

1. **File Validation**: Always validate file types, sizes, and content
2. **Security Scanning**: Implement virus scanning and malware detection
3. **Storage Management**: Use efficient storage with proper cleanup
4. **Access Control**: Ensure users can only access their own files
5. **Error Handling**: Provide clear feedback for upload failures
6. **Rate Limiting**: Prevent abuse with upload limits and timeouts
7. **Backup Strategy**: Implement regular backups of stored files

## Security Considerations

- Validate file signatures against extensions
- Scan for malicious content
- Implement size limits
- Use secure storage locations
- Log all file operations
- Implement access controls
- Regular security audits

## See Also

- [Interactive Messages](./interactive-messages.md) - Creating engaging user interfaces
- [Rich Messages](./rich-messages.md) - Advanced message formatting
- [Command Handler](./command-handler.md) - Structured command processing
- [Error Recovery](./error-recovery.md) - Handling upload failures gracefully
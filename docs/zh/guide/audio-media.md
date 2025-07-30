# 音频与媒体

BotRS 为 QQ 频道机器人提供了全面的音频和媒体内容支持。这包括语音频道管理、直播功能、音频播放控制以及富媒体消息处理。

## 概述

BotRS 中的音频和媒体系统包含几个关键组件：

- **音频事件**：实时音频频道事件（加入/离开）
- **音频控制**：音频内容的播放管理
- **语音频道**：传统语音聊天功能
- **直播频道**：流媒体和广播功能
- **媒体消息**：富媒体内容传递（图片、视频、文件）

## 音频频道事件

### 语音频道事件

处理用户加入和离开语音频道：

```rust
use botrs::{EventHandler, Context, PublicAudio, PublicAudioType};

impl EventHandler for MyBot {
    async fn audio_or_live_channel_member_enter(
        &self, 
        ctx: Context, 
        audio: PublicAudio
    ) {
        if let Some(channel_type) = audio.channel_type {
            match channel_type {
                PublicAudioType::Voice => {
                    println!("用户加入语音频道: {:?}", audio.channel_id);
                }
                PublicAudioType::Live => {
                    println!("用户加入直播频道: {:?}", audio.channel_id);
                }
            }
        }
        
        // 为语音频道加入发送欢迎消息
        if let (Some(channel_id), Some(user_id)) = (&audio.channel_id, &audio.user_id) {
            let welcome_msg = format!("欢迎来到语音频道，<@{}>！", user_id);
            if let Err(e) = ctx.send_message(channel_id, &welcome_msg).await {
                eprintln!("发送欢迎消息失败: {}", e);
            }
        }
    }
    
    async fn audio_or_live_channel_member_exit(
        &self, 
        ctx: Context, 
        audio: PublicAudio
    ) {
        if let Some(user_id) = &audio.user_id {
            println!("用户离开音频频道: {}", user_id);
            
            // 可选：发送离开通知
            if let Some(channel_id) = &audio.channel_id {
                let goodbye_msg = format!("用户 <@{}> 已离开语音频道", user_id);
                let _ = ctx.send_message(channel_id, &goodbye_msg).await;
            }
        }
    }
}
```

### 音频事件数据

音频事件包含丰富的上下文信息：

```rust
impl EventHandler for AudioBot {
    async fn audio_or_live_channel_member_enter(
        &self, 
        ctx: Context, 
        audio: PublicAudio
    ) {
        // 访问音频事件的详细信息
        println!("频道 ID: {:?}", audio.channel_id);
        println!("用户 ID: {:?}", audio.user_id);
        println!("频道类型: {:?}", audio.channel_type);
        println!("事件时间: {:?}", audio.timestamp);
        
        // 根据频道类型执行不同的逻辑
        if let Some(channel_type) = audio.channel_type {
            match channel_type {
                PublicAudioType::Voice => {
                    self.handle_voice_join(&ctx, &audio).await;
                }
                PublicAudioType::Live => {
                    self.handle_live_join(&ctx, &audio).await;
                }
            }
        }
    }
}
```

## 音频控制

### 音频播放管理

管理语音频道中的音频播放：

```rust
impl MyBot {
    async fn handle_audio_command(&self, ctx: Context, command: &str, channel_id: &str) {
        match command {
            "play" => {
                // 开始播放音频
                match ctx.start_audio_playback(channel_id).await {
                    Ok(_) => {
                        ctx.send_message(channel_id, "开始播放音频").await.ok();
                    }
                    Err(e) => {
                        eprintln!("播放音频失败: {}", e);
                        ctx.send_message(channel_id, "播放失败").await.ok();
                    }
                }
            }
            "pause" => {
                // 暂停音频播放
                match ctx.pause_audio_playback(channel_id).await {
                    Ok(_) => {
                        ctx.send_message(channel_id, "音频已暂停").await.ok();
                    }
                    Err(e) => {
                        eprintln!("暂停音频失败: {}", e);
                    }
                }
            }
            "stop" => {
                // 停止音频播放
                match ctx.stop_audio_playback(channel_id).await {
                    Ok(_) => {
                        ctx.send_message(channel_id, "音频已停止").await.ok();
                    }
                    Err(e) => {
                        eprintln!("停止音频失败: {}", e);
                    }
                }
            }
            "resume" => {
                // 恢复音频播放
                match ctx.resume_audio_playback(channel_id).await {
                    Ok(_) => {
                        ctx.send_message(channel_id, "音频已恢复播放").await.ok();
                    }
                    Err(e) => {
                        eprintln!("恢复播放失败: {}", e);
                    }
                }
            }
            _ => {
                ctx.send_message(channel_id, "未知的音频命令").await.ok();
            }
        }
    }
}
```

### 音频状态管理

跟踪和管理音频播放状态：

```rust
impl MyBot {
    async fn handle_audio_status(&self, ctx: Context, channel_id: &str) {
        match ctx.get_audio_status(channel_id).await {
            Ok(status) => {
                let status_msg = match status {
                    AudioStatus::Playing => "正在播放",
                    AudioStatus::Paused => "已暂停",
                    AudioStatus::Stopped => "已停止",
                    AudioStatus::Loading => "加载中",
                };
                
                ctx.send_message(channel_id, &format!("当前音频状态: {}", status_msg))
                    .await.ok();
            }
            Err(e) => {
                eprintln!("获取音频状态失败: {}", e);
            }
        }
    }

    async fn start_audio_session(&self, ctx: Context, channel_id: &str) {
        // 实现音频会话开始逻辑
    }

    async fn pause_audio_session(&self, ctx: Context, channel_id: &str) {
        // 实现音频会话暂停逻辑
    }

    async fn resume_audio_session(&self, ctx: Context, channel_id: &str) {
        // 实现音频会话恢复逻辑
    }

    async fn stop_audio_session(&self, ctx: Context, channel_id: &str) {
        // 实现音频会话停止逻辑
    }
}
```

## 语音频道管理

### 麦克风控制

管理用户的麦克风权限：

```rust
impl EventHandler for VoiceBot {
    async fn message_create(&self, ctx: Context, msg: botrs::Message) {
        if msg.content.starts_with("!mute") {
            if let Some(user_id) = self.extract_user_id(&msg.content) {
                match ctx.mute_member(&msg.channel_id, &user_id).await {
                    Ok(_) => {
                        ctx.send_message(&msg.channel_id, &format!("用户 <@{}> 已被静音", user_id))
                            .await.ok();
                    }
                    Err(e) => {
                        eprintln!("静音用户失败: {}", e);
                        ctx.send_message(&msg.channel_id, "静音操作失败").await.ok();
                    }
                }
            }
        } else if msg.content.starts_with("!unmute") {
            if let Some(user_id) = self.extract_user_id(&msg.content) {
                match ctx.unmute_member(&msg.channel_id, &user_id).await {
                    Ok(_) => {
                        ctx.send_message(&msg.channel_id, &format!("用户 <@{}> 已解除静音", user_id))
                            .await.ok();
                    }
                    Err(e) => {
                        eprintln!("解除静音失败: {}", e);
                        ctx.send_message(&msg.channel_id, "解除静音操作失败").await.ok();
                    }
                }
            }
        } else if msg.content == "!voice_status" {
            self.show_voice_channel_status(&ctx, &msg.channel_id).await;
        }
    }
}
```

### 个人成员控制

对特定成员进行语音频道管理：

```rust
impl MyBot {
    async fn moderate_voice_channel(&self, ctx: Context, channel_id: &str, user_id: &str, action: &str) {
        match action {
            "mute" => {
                if let Err(e) = ctx.mute_member(channel_id, user_id).await {
                    eprintln!("静音成员失败: {}", e);
                }
            }
            "unmute" => {
                if let Err(e) = ctx.unmute_member(channel_id, user_id).await {
                    eprintln!("解除静音失败: {}", e);
                }
            }
            "kick" => {
                if let Err(e) = ctx.kick_from_voice_channel(channel_id, user_id).await {
                    eprintln!("踢出语音频道失败: {}", e);
                }
            }
            _ => {
                eprintln!("未知的管理操作: {}", action);
            }
        }
    }
}
```

## 富媒体消息

### 图片消息

发送图片内容：

```rust
impl MyBot {
    async fn send_image(&self, ctx: Context, channel_id: &str, image_path: &str) {
        match std::fs::read(image_path) {
            Ok(image_data) => {
                let file_info = botrs::FileInfo {
                    filename: "image.jpg".to_string(),
                    content_type: "image/jpeg".to_string(),
                    data: image_data,
                };
                
                if let Err(e) = ctx.send_file_message(channel_id, file_info).await {
                    eprintln!("发送图片失败: {}", e);
                }
            }
            Err(e) => {
                eprintln!("读取图片文件失败: {}", e);
            }
        }
    }
}
```

### 视频消息

发送视频内容：

```rust
impl MyBot {
    async fn send_video(&self, ctx: Context, channel_id: &str, video_path: &str) {
        let file_info = botrs::FileInfo::from_path(video_path, "video/mp4").await?;
        
        match ctx.send_file_message(channel_id, file_info).await {
            Ok(_) => println!("视频发送成功"),
            Err(e) => eprintln!("发送视频失败: {}", e),
        }
    }
}
```

### 音频消息

发送音频文件：

```rust
impl MyBot {
    async fn send_audio_message(&self, ctx: Context, channel_id: &str, audio_path: &str) {
        let file_info = botrs::FileInfo::from_path(audio_path, "audio/mpeg").await?;
        
        match ctx.send_file_message(channel_id, file_info).await {
            Ok(_) => println!("音频消息发送成功"),
            Err(e) => eprintln!("发送音频消息失败: {}", e),
        }
    }
}
```

### 文件上传

上传任意类型的文件：

```rust
impl MyBot {
    async fn upload_file(&self, ctx: Context, channel_id: &str, file_path: &str) {
        let file_data = std::fs::read(file_path)?;
        let filename = std::path::Path::new(file_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        
        let file_info = botrs::FileInfo {
            filename,
            content_type: "application/octet-stream".to_string(),
            data: file_data,
        };
        
        ctx.send_file_message(channel_id, file_info).await?;
    }
}
```

## 高级音频功能

### 音频会话跟踪

跟踪和管理活跃的音频会话：

```rust
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct AudioSessionManager {
    active_sessions: HashMap<String, AudioSession>,
}

pub struct AudioSession {
    channel_id: String,
    start_time: DateTime<Utc>,
    current_track: Option<String>,
    status: AudioStatus,
}

impl AudioSessionManager {
    pub fn new() -> Self {
        Self {
            active_sessions: HashMap::new(),
        }
    }

    pub async fn start_session(&mut self, channel_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let session = AudioSession {
            channel_id: channel_id.clone(),
            start_time: Utc::now(),
            current_track: None,
            status: AudioStatus::Starting,
        };
        
        self.active_sessions.insert(channel_id, session);
        Ok(())
    }

    pub async fn update_session_status(&mut self, channel_id: &str, status: AudioStatus) {
        if let Some(session) = self.active_sessions.get_mut(channel_id) {
            session.status = status;
        }
    }

    pub async fn end_session(&mut self, channel_id: &str) {
        self.active_sessions.remove(channel_id);
    }

    pub async fn get_session_duration(&self, channel_id: &str) -> Option<chrono::Duration> {
        self.active_sessions.get(channel_id)
            .map(|session| Utc::now() - session.start_time)
    }
}
```

### 音频机器人示例

完整的音频机器人实现：

```rust
pub struct AudioBot {
    session_manager: AudioSessionManager,
}

impl AudioBot {
    pub fn new() -> Self {
        Self {
            session_manager: AudioSessionManager::new(),
        }
    }
}

impl EventHandler for AudioBot {
    async fn ready(&self, ctx: Context) {
        println!("音频机器人已准备就绪");
    }

    async fn message_create(&self, ctx: Context, msg: botrs::Message) {
        if msg.content.starts_with("!audio") {
            self.handle_audio_command(ctx, msg).await;
        }
    }

    async fn audio_or_live_channel_member_enter(&self, ctx: Context, audio: PublicAudio) {
        println!("音频频道成员加入事件");
        
        if let Some(channel_id) = &audio.channel_id {
            // 记录会话开始
            if let Err(e) = self.session_manager.start_session(channel_id.clone()).await {
                eprintln!("启动音频会话失败: {}", e);
            }
        }
    }

    async fn audio_or_live_channel_member_exit(&self, ctx: Context, audio: PublicAudio) {
        println!("音频频道成员离开事件");
    }
}

impl AudioBot {
    async fn handle_audio_command(&self, ctx: Context, msg: botrs::Message) {
        let parts: Vec<&str> = msg.content.split_whitespace().collect();
        
        match parts.get(1) {
            Some(&"play") => {
                ctx.send_message(&msg.channel_id, "开始播放音频").await.ok();
            }
            Some(&"stop") => {
                ctx.send_message(&msg.channel_id, "停止播放音频").await.ok();
            }
            Some(&"status") => {
                if let Some(duration) = self.session_manager.get_session_duration(&msg.channel_id).await {
                    let status_msg = format!("当前会话已持续: {} 分钟", duration.num_minutes());
                    ctx.send_message(&msg.channel_id, &status_msg).await.ok();
                } else {
                    ctx.send_message(&msg.channel_id, "没有活跃的音频会话").await.ok();
                }
            }
            Some(&"help") => {
                let help_msg = "音频命令:\n!audio play - 开始播放\n!audio stop - 停止播放\n!audio status - 查看状态";
                ctx.send_message(&msg.channel_id, help_msg).await.ok();
            }
            _ => {
                ctx.send_message(&msg.channel_id, "未知的音频命令，使用 !audio help 查看帮助").await.ok();
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = AudioBot::new();
    let mut client = botrs::Client::new("your_bot_token", botrs::Intents::all());
    
    client.set_event_handler(bot).await;
    client.start().await?;
    
    Ok(())
}
```

## 最佳实践

### 音频性能

- 使用适当的音频格式和比特率
- 实现音频缓冲和预加载
- 监控音频延迟和质量
- 优化音频数据传输

### 语音频道管理

- 实现适当的权限检查
- 提供清晰的用户反馈
- 记录管理操作以供审计
- 处理并发管理操作

### 媒体消息优化

- 验证文件大小和格式
- 实现文件压缩和优化
- 提供上传进度反馈
- 处理网络中断和重试

### 错误处理

- 处理音频设备不可用的情况
- 优雅地处理网络连接问题
- 提供有意义的错误消息
- 实现适当的重试机制
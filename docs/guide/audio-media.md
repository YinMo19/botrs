# Audio & Media

BotRS provides comprehensive support for audio and media content in QQ Guild bots. This includes voice channel management, live streaming capabilities, audio playback controls, and rich media message handling.

## Overview

The audio and media system in BotRS consists of several key components:

- **Audio Events**: Real-time audio channel events (join/leave)
- **Audio Controls**: Playback management for audio content
- **Voice Channels**: Traditional voice chat functionality
- **Live Channels**: Streaming and broadcasting capabilities
- **Media Messages**: Rich content delivery (images, videos, files)

## Audio Channel Events

### Voice Channel Events

Handle users joining and leaving voice channels:

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
                    println!("User joined voice channel: {:?}", audio.channel_id);
                }
                PublicAudioType::Live => {
                    println!("User joined live channel: {:?}", audio.channel_id);
                }
            }
        }
        
        // Welcome message for voice channel joins
        if let (Some(channel_id), Some(user_id)) = (&audio.channel_id, &audio.user_id) {
            let welcome_msg = format!("Welcome to the voice channel, <@{}>!", user_id);
            if let Err(e) = ctx.send_message(channel_id, &welcome_msg).await {
                eprintln!("Failed to send welcome message: {}", e);
            }
        }
    }
    
    async fn audio_or_live_channel_member_exit(
        &self, 
        ctx: Context, 
        audio: PublicAudio
    ) {
        println!("User left audio channel: {:?}", audio.user_id);
        
        // Optional: Send goodbye message
        if let (Some(channel_id), Some(user_id)) = (&audio.channel_id, &audio.user_id) {
            let goodbye_msg = format!("Goodbye, <@{}>!", user_id);
            let _ = ctx.send_message(channel_id, &goodbye_msg).await;
        }
    }
}
```

### Audio Event Data

Access detailed information about audio events:

```rust
impl EventHandler for AudioBot {
    async fn audio_or_live_channel_member_enter(
        &self, 
        ctx: Context, 
        audio: PublicAudio
    ) {
        // Access audio event properties
        println!("Guild ID: {:?}", audio.guild_id);
        println!("Channel ID: {:?}", audio.channel_id);
        println!("User ID: {:?}", audio.user_id);
        println!("Channel Type: {:?}", audio.channel_type);
        
        // Get API client for additional operations
        let api = audio.api();
        
        // Perform additional operations if needed
        if let Some(guild_id) = &audio.guild_id {
            if let Some(user_id) = &audio.user_id {
                match api.guild_member(&ctx.token, guild_id, user_id).await {
                    Ok(member) => {
                        println!("Member {} joined audio channel", member.nick.unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Failed to get member info: {}", e);
                    }
                }
            }
        }
    }
}
```

## Audio Controls

### Audio Playback Management

Control audio playback in voice channels:

```rust
use botrs::{AudioControl, AudioStatus, Context};

impl MyBot {
    async fn handle_audio_command(&self, ctx: &Context, cmd: &str, channel_id: &str) {
        match cmd {
            "play" => {
                let audio_control = AudioControl {
                    audio_url: "https://example.com/audio.mp3".to_string(),
                    text: "Now playing: Example Song".to_string(),
                    status: AudioStatus::Start,
                };
                
                if let Err(e) = ctx.update_audio(channel_id, audio_control).await {
                    eprintln!("Failed to start audio: {}", e);
                }
            }
            "pause" => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Audio paused".to_string(),
                    status: AudioStatus::Pause,
                };
                
                if let Err(e) = ctx.update_audio(channel_id, audio_control).await {
                    eprintln!("Failed to pause audio: {}", e);
                }
            }
            "resume" => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Audio resumed".to_string(),
                    status: AudioStatus::Resume,
                };
                
                if let Err(e) = ctx.update_audio(channel_id, audio_control).await {
                    eprintln!("Failed to resume audio: {}", e);
                }
            }
            "stop" => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Audio stopped".to_string(),
                    status: AudioStatus::Stop,
                };
                
                if let Err(e) = ctx.update_audio(channel_id, audio_control).await {
                    eprintln!("Failed to stop audio: {}", e);
                }
            }
            _ => {
                println!("Unknown audio command: {}", cmd);
            }
        }
    }
}
```

### Audio Status Management

Track and respond to different audio states:

```rust
use botrs::AudioStatus;

impl MyBot {
    async fn handle_audio_status(&self, status: AudioStatus) {
        match status {
            AudioStatus::Start => {
                println!("Audio playback started");
                // Initialize audio session tracking
                self.start_audio_session().await;
            }
            AudioStatus::Pause => {
                println!("Audio playback paused");
                // Pause session tracking
                self.pause_audio_session().await;
            }
            AudioStatus::Resume => {
                println!("Audio playback resumed");
                // Resume session tracking
                self.resume_audio_session().await;
            }
            AudioStatus::Stop => {
                println!("Audio playback stopped");
                // Clean up audio session
                self.stop_audio_session().await;
            }
        }
    }
    
    async fn start_audio_session(&self) {
        // Implementation for starting audio session tracking
    }
    
    async fn pause_audio_session(&self) {
        // Implementation for pausing audio session
    }
    
    async fn resume_audio_session(&self) {
        // Implementation for resuming audio session
    }
    
    async fn stop_audio_session(&self) {
        // Implementation for stopping audio session
    }
}
```

## Voice Channel Management

### Microphone Controls

Manage microphone permissions and states:

```rust
impl EventHandler for VoiceBot {
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            match content.as_str() {
                "!mute" => {
                    // Mute user's microphone
                    if let Some(author) = &msg.author {
                        if let Err(e) = ctx.off_microphone(&msg.channel_id, &author.id).await {
                            eprintln!("Failed to mute microphone: {}", e);
                        } else {
                            let _ = ctx.send_message(&msg.channel_id, "Microphone muted").await;
                        }
                    }
                }
                "!unmute" => {
                    // Unmute user's microphone
                    if let Some(author) = &msg.author {
                        if let Err(e) = ctx.on_microphone(&msg.channel_id, &author.id).await {
                            eprintln!("Failed to unmute microphone: {}", e);
                        } else {
                            let _ = ctx.send_message(&msg.channel_id, "Microphone unmuted").await;
                        }
                    }
                }
                "!mute_all" => {
                    // Mute all users in channel
                    if let Err(e) = ctx.mute_all(&msg.channel_id).await {
                        eprintln!("Failed to mute all: {}", e);
                    } else {
                        let _ = ctx.send_message(&msg.channel_id, "All users muted").await;
                    }
                }
                "!unmute_all" => {
                    // Unmute all users in channel
                    if let Err(e) = ctx.cancel_mute_all(&msg.channel_id).await {
                        eprintln!("Failed to unmute all: {}", e);
                    } else {
                        let _ = ctx.send_message(&msg.channel_id, "All users unmuted").await;
                    }
                }
                _ => {}
            }
        }
    }
}
```

### Individual Member Control

Fine-grained control over individual members:

```rust
impl MyBot {
    async fn moderate_voice_channel(
        &self,
        ctx: &Context,
        channel_id: &str,
        user_id: &str,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            "mute" => {
                ctx.mute_member(channel_id, user_id, "Moderation action").await?;
                println!("Muted user {} in channel {}", user_id, channel_id);
            }
            "kick" => {
                // Note: kick_member is for guild-level, not voice-specific
                ctx.kick_member(channel_id, user_id, "Removed from voice channel").await?;
                println!("Kicked user {} from channel {}", user_id, channel_id);
            }
            _ => {
                println!("Unknown moderation action: {}", action);
            }
        }
        
        Ok(())
    }
}
```

## Rich Media Messages

### Image Messages

Send and handle image content:

```rust
use botrs::MessageParams;
use std::path::Path;

impl MyBot {
    async fn send_image(&self, ctx: &Context, channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Method 1: Send local image file
        let image_path = Path::new("./images/example.png");
        let params = MessageParams::new_text("Check out this image!")
            .with_image_file(image_path)?;
        
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        
        // Method 2: Send image by URL
        let params = MessageParams::new_text("Image from URL")
            .with_image("https://example.com/image.png");
        
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        
        Ok(())
    }
}
```

### Video Messages

Handle video content:

```rust
impl MyBot {
    async fn send_video(&self, ctx: &Context, channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Send local video file
        let video_path = Path::new("./videos/example.mp4");
        let params = MessageParams::new_text("Here's a video!")
            .with_video_file(video_path)?;
        
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        
        Ok(())
    }
}
```

### Audio Messages

Send audio content in messages:

```rust
impl MyBot {
    async fn send_audio_message(&self, ctx: &Context, channel_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Send audio file
        let audio_path = Path::new("./audio/voice_message.mp3");
        let params = MessageParams::new_text("Voice message")
            .with_audio_file(audio_path)?;
        
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        
        Ok(())
    }
}
```

### File Uploads

Handle general file uploads:

```rust
use botrs::FileType;

impl MyBot {
    async fn upload_file(&self, ctx: &Context, channel_id: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Upload file and get file info
        let file_info = ctx.api.post_file(&ctx.token, file_path, FileType::Document).await?;
        
        // Send message with uploaded file
        let params = MessageParams::new_text("File uploaded successfully!")
            .with_file_url(&file_info.url);
        
        ctx.api.post_message_with_params(&ctx.token, channel_id, params).await?;
        
        Ok(())
    }
}
```

## Advanced Audio Features

### Audio Session Tracking

Track active audio sessions:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AudioSessionManager {
    active_sessions: Arc<Mutex<HashMap<String, AudioSession>>>,
}

pub struct AudioSession {
    channel_id: String,
    start_time: std::time::Instant,
    current_track: Option<String>,
    status: AudioStatus,
}

impl AudioSessionManager {
    pub fn new() -> Self {
        Self {
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn start_session(&self, channel_id: String, track_url: String) {
        let mut sessions = self.active_sessions.lock().await;
        let session = AudioSession {
            channel_id: channel_id.clone(),
            start_time: std::time::Instant::now(),
            current_track: Some(track_url),
            status: AudioStatus::Start,
        };
        sessions.insert(channel_id, session);
    }
    
    pub async fn update_session_status(&self, channel_id: &str, status: AudioStatus) {
        let mut sessions = self.active_sessions.lock().await;
        if let Some(session) = sessions.get_mut(channel_id) {
            session.status = status;
        }
    }
    
    pub async fn end_session(&self, channel_id: &str) {
        let mut sessions = self.active_sessions.lock().await;
        sessions.remove(channel_id);
    }
    
    pub async fn get_session_duration(&self, channel_id: &str) -> Option<std::time::Duration> {
        let sessions = self.active_sessions.lock().await;
        sessions.get(channel_id).map(|session| session.start_time.elapsed())
    }
}
```

### Audio Bot Example

Complete example of an audio-enabled bot:

```rust
use botrs::{Client, EventHandler, Context, Message, PublicAudio, AudioControl, AudioStatus};

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
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Audio bot {} is ready!", ready.user.username);
    }
    
    async fn message_create(&self, ctx: Context, msg: Message) {
        if let Some(content) = &msg.content {
            if content.starts_with("!audio ") {
                let command = &content[7..];
                self.handle_audio_command(&ctx, &msg.channel_id, command).await;
            }
        }
    }
    
    async fn audio_or_live_channel_member_enter(&self, ctx: Context, audio: PublicAudio) {
        if let Some(user_id) = &audio.user_id {
            println!("User {} entered audio channel", user_id);
            
            // Send welcome audio message
            if let Some(channel_id) = &audio.channel_id {
                let welcome_audio = AudioControl {
                    audio_url: "https://example.com/welcome.mp3".to_string(),
                    text: "Welcome to the voice channel!".to_string(),
                    status: AudioStatus::Start,
                };
                
                let _ = ctx.update_audio(channel_id, welcome_audio).await;
            }
        }
    }
    
    async fn audio_or_live_channel_member_exit(&self, _ctx: Context, audio: PublicAudio) {
        if let Some(user_id) = &audio.user_id {
            println!("User {} left audio channel", user_id);
        }
    }
}

impl AudioBot {
    async fn handle_audio_command(&self, ctx: &Context, channel_id: &str, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"play") => {
                if let Some(&url) = parts.get(1) {
                    let audio_control = AudioControl {
                        audio_url: url.to_string(),
                        text: format!("Now playing: {}", url),
                        status: AudioStatus::Start,
                    };
                    
                    if let Ok(_) = ctx.update_audio(channel_id, audio_control).await {
                        self.session_manager.start_session(channel_id.to_string(), url.to_string()).await;
                        let _ = ctx.send_message(channel_id, "Audio playback started").await;
                    }
                }
            }
            Some(&"stop") => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Playback stopped".to_string(),
                    status: AudioStatus::Stop,
                };
                
                if let Ok(_) = ctx.update_audio(channel_id, audio_control).await {
                    if let Some(duration) = self.session_manager.get_session_duration(channel_id).await {
                        let _ = ctx.send_message(
                            channel_id, 
                            &format!("Audio stopped. Duration: {:?}", duration)
                        ).await;
                    }
                    self.session_manager.end_session(channel_id).await;
                }
            }
            Some(&"pause") => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Playback paused".to_string(),
                    status: AudioStatus::Pause,
                };
                
                if let Ok(_) = ctx.update_audio(channel_id, audio_control).await {
                    self.session_manager.update_session_status(channel_id, AudioStatus::Pause).await;
                    let _ = ctx.send_message(channel_id, "Audio paused").await;
                }
            }
            Some(&"resume") => {
                let audio_control = AudioControl {
                    audio_url: String::new(),
                    text: "Playback resumed".to_string(),
                    status: AudioStatus::Resume,
                };
                
                if let Ok(_) = ctx.update_audio(channel_id, audio_control).await {
                    self.session_manager.update_session_status(channel_id, AudioStatus::Resume).await;
                    let _ = ctx.send_message(channel_id, "Audio resumed").await;
                }
            }
            _ => {
                let _ = ctx.send_message(
                    channel_id, 
                    "Available commands: play <url>, stop, pause, resume"
                ).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = AudioBot::new();
    
    let mut client = Client::new("your_app_id", bot)
        .intents(Intents::GUILD_MESSAGES | Intents::AUDIO_ACTION)
        .build()
        .await?;
    
    client.start().await?;
    Ok(())
}
```

## Best Practices

### Audio Performance

1. **Use Appropriate Formats**: Prefer MP3 or AAC for audio content
2. **File Size Management**: Keep audio files under reasonable size limits
3. **URL Validation**: Verify audio URLs are accessible before playback
4. **Error Handling**: Implement robust error handling for audio operations

### Voice Channel Moderation

1. **Permission Checks**: Verify bot permissions before moderating
2. **Audit Logging**: Log moderation actions for accountability
3. **Rate Limiting**: Implement cooldowns for moderation commands
4. **User Feedback**: Provide clear feedback for moderation actions

### Media Message Optimization

1. **File Validation**: Check file types and sizes before upload
2. **Compression**: Optimize images and videos for faster loading
3. **Fallback Content**: Provide text alternatives for media content
4. **Cache Management**: Consider caching frequently used media

The audio and media system in BotRS provides powerful capabilities for creating engaging, interactive voice experiences in QQ Guild communities. By leveraging these features effectively, you can build bots that enhance real-time communication and content sharing.
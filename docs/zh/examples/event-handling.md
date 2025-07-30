# 事件处理示例

本示例展示如何在 BotRS 机器人中处理各种类型的事件，包括消息事件、频道事件、成员事件等，以及如何构建高效的事件处理架构。

## 概述

QQ 频道机器人可以接收多种类型的事件，每种事件都对应不同的用户行为或系统状态变化。本示例展示如何优雅地处理这些事件并构建响应式的机器人应用。

## 基础事件处理

### 简单事件处理器

```rust
use botrs::{Context, EventHandler, Message, Ready, Guild, Channel, Member};
use tracing::{info, warn, debug};

pub struct BasicEventHandler;

#[async_trait::async_trait]
impl EventHandler for BasicEventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("机器人就绪: {}", ready.user.username);
        info!("会话 ID: {}", ready.session_id);
        
        if let Some(guilds) = ready.guilds {
            info!("机器人已加入 {} 个频道", guilds.len());
        }
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        debug!("收到消息: {} 来自用户: {:?}", 
               message.id, 
               message.author.as_ref().map(|a| &a.username));

        if let Some(content) = &message.content {
            if content.trim() == "!ping" {
                if let Err(e) = message.reply(&ctx.api, &ctx.token, "Pong!").await {
                    warn!("回复消息失败: {}", e);
                }
            }
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        info!("加入新频道: {} (ID: {})", 
              guild.name.unwrap_or_default(), 
              guild.id);
    }

    async fn guild_member_add(&self, _ctx: Context, member: Member) {
        info!("新成员加入: {:?}", member.user.username);
    }
}
```

## 高级事件处理架构

### 模块化事件处理器

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

// 命令处理器 trait
#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle(&self, ctx: &Context, message: &Message, args: &[&str]) -> Result<(), Box<dyn std::error::Error>>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

// 事件监听器 trait
#[async_trait::async_trait]
pub trait EventListener: Send + Sync {
    async fn on_message(&self, _ctx: &Context, _message: &Message) {}
    async fn on_guild_join(&self, _ctx: &Context, _guild: &Guild) {}
    async fn on_member_join(&self, _ctx: &Context, _member: &Member) {}
    async fn on_member_leave(&self, _ctx: &Context, _member: &Member) {}
}

pub struct ModularEventHandler {
    commands: Arc<RwLock<HashMap<String, Box<dyn CommandHandler>>>>,
    listeners: Arc<RwLock<Vec<Box<dyn EventListener>>>>,
    command_prefix: String,
    statistics: Arc<RwLock<EventStatistics>>,
}

#[derive(Default)]
pub struct EventStatistics {
    pub messages_processed: u64,
    pub commands_executed: u64,
    pub guild_events: u64,
    pub member_events: u64,
    pub errors_encountered: u64,
}

impl ModularEventHandler {
    pub fn new(command_prefix: String) -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            listeners: Arc::new(RwLock::new(Vec::new())),
            command_prefix,
            statistics: Arc::new(RwLock::new(EventStatistics::default())),
        }
    }

    pub async fn register_command(&self, handler: Box<dyn CommandHandler>) {
        let mut commands = self.commands.write().await;
        commands.insert(handler.name().to_string(), handler);
    }

    pub async fn register_listener(&self, listener: Box<dyn EventListener>) {
        let mut listeners = self.listeners.write().await;
        listeners.push(listener);
    }

    async fn process_command(&self, ctx: &Context, message: &Message, content: &str) {
        if !content.starts_with(&self.command_prefix) {
            return;
        }

        let command_text = &content[self.command_prefix.len()..];
        let args: Vec<&str> = command_text.split_whitespace().collect();
        
        if args.is_empty() {
            return;
        }

        let command_name = args[0];
        let command_args = &args[1..];

        let commands = self.commands.read().await;
        if let Some(handler) = commands.get(command_name) {
            match handler.handle(ctx, message, command_args).await {
                Ok(_) => {
                    info!("命令执行成功: {}", command_name);
                    let mut stats = self.statistics.write().await;
                    stats.commands_executed += 1;
                }
                Err(e) => {
                    warn!("命令执行失败 {}: {}", command_name, e);
                    let mut stats = self.statistics.write().await;
                    stats.errors_encountered += 1;
                    
                    let error_msg = format!("命令执行失败: {}", e);
                    if let Err(e) = message.reply(&ctx.api, &ctx.token, &error_msg).await {
                        warn!("发送错误消息失败: {}", e);
                    }
                }
            }
        } else {
            debug!("未知命令: {}", command_name);
        }
    }

    async fn notify_listeners_message(&self, ctx: &Context, message: &Message) {
        let listeners = self.listeners.read().await;
        for listener in listeners.iter() {
            listener.on_message(ctx, message).await;
        }
    }

    async fn notify_listeners_guild_join(&self, ctx: &Context, guild: &Guild) {
        let listeners = self.listeners.read().await;
        for listener in listeners.iter() {
            listener.on_guild_join(ctx, guild).await;
        }
    }

    pub async fn get_statistics(&self) -> EventStatistics {
        self.statistics.read().await.clone()
    }
}

#[async_trait::async_trait]
impl EventHandler for ModularEventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("模块化事件处理器就绪: {}", ready.user.username);
        
        let commands = self.commands.read().await;
        info!("已注册 {} 个命令", commands.len());
        
        let listeners = self.listeners.read().await;
        info!("已注册 {} 个事件监听器", listeners.len());
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if message.is_from_bot() {
            return;
        }

        // 更新统计
        {
            let mut stats = self.statistics.write().await;
            stats.messages_processed += 1;
        }

        // 通知监听器
        self.notify_listeners_message(&ctx, &message).await;

        // 处理命令
        if let Some(content) = &message.content {
            self.process_command(&ctx, &message, content).await;
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        info!("加入频道: {}", guild.name.unwrap_or_default());
        
        {
            let mut stats = self.statistics.write().await;
            stats.guild_events += 1;
        }

        self.notify_listeners_guild_join(&ctx, &guild).await;
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        info!("新成员加入: {:?}", member.user.username);
        
        {
            let mut stats = self.statistics.write().await;
            stats.member_events += 1;
        }

        let listeners = self.listeners.read().await;
        for listener in listeners.iter() {
            listener.on_member_join(&ctx, &member).await;
        }
    }

    async fn guild_member_remove(&self, ctx: Context, member: Member) {
        info!("成员离开: {:?}", member.user.username);
        
        {
            let mut stats = self.statistics.write().await;
            stats.member_events += 1;
        }

        let listeners = self.listeners.read().await;
        for listener in listeners.iter() {
            listener.on_member_leave(&ctx, &member).await;
        }
    }
}
```

## 具体命令处理器实现

### 帮助命令处理器

```rust
pub struct HelpCommandHandler {
    commands: Arc<RwLock<HashMap<String, Box<dyn CommandHandler>>>>,
}

impl HelpCommandHandler {
    pub fn new(commands: Arc<RwLock<HashMap<String, Box<dyn CommandHandler>>>>) -> Self {
        Self { commands }
    }
}

#[async_trait::async_trait]
impl CommandHandler for HelpCommandHandler {
    async fn handle(&self, ctx: &Context, message: &Message, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            // 显示所有命令
            let commands = self.commands.read().await;
            let mut help_text = "可用命令:\n".to_string();
            
            for (name, handler) in commands.iter() {
                help_text.push_str(&format!("• `{}` - {}\n", name, handler.description()));
            }
            
            message.reply(&ctx.api, &ctx.token, &help_text).await?;
        } else {
            // 显示特定命令的帮助
            let command_name = args[0];
            let commands = self.commands.read().await;
            
            if let Some(handler) = commands.get(command_name) {
                let help_text = format!("命令: `{}`\n描述: {}", command_name, handler.description());
                message.reply(&ctx.api, &ctx.token, &help_text).await?;
            } else {
                message.reply(&ctx.api, &ctx.token, &format!("未找到命令: {}", command_name)).await?;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "显示帮助信息"
    }
}
```

### 统计命令处理器

```rust
pub struct StatsCommandHandler {
    statistics: Arc<RwLock<EventStatistics>>,
}

impl StatsCommandHandler {
    pub fn new(statistics: Arc<RwLock<EventStatistics>>) -> Self {
        Self { statistics }
    }
}

#[async_trait::async_trait]
impl CommandHandler for StatsCommandHandler {
    async fn handle(&self, ctx: &Context, message: &Message, _args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let stats = self.statistics.read().await;
        
        let stats_text = format!(
            "📊 机器人统计信息:\n\
            • 处理消息: {} 条\n\
            • 执行命令: {} 次\n\
            • 频道事件: {} 次\n\
            • 成员事件: {} 次\n\
            • 遇到错误: {} 次",
            stats.messages_processed,
            stats.commands_executed,
            stats.guild_events,
            stats.member_events,
            stats.errors_encountered
        );
        
        message.reply(&ctx.api, &ctx.token, &stats_text).await?;
        Ok(())
    }

    fn name(&self) -> &str {
        "stats"
    }

    fn description(&self) -> &str {
        "显示机器人统计信息"
    }
}
```

### 时间命令处理器

```rust
use chrono::{DateTime, Utc, Local};

pub struct TimeCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for TimeCommandHandler {
    async fn handle(&self, ctx: &Context, message: &Message, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let timezone = if args.is_empty() {
            "UTC"
        } else {
            args[0]
        };

        let time_text = match timezone.to_uppercase().as_str() {
            "UTC" => {
                let utc_time: DateTime<Utc> = Utc::now();
                format!("🕐 UTC 时间: {}", utc_time.format("%Y-%m-%d %H:%M:%S UTC"))
            }
            "LOCAL" | "本地" => {
                let local_time = Local::now();
                format!("🕐 本地时间: {}", local_time.format("%Y-%m-%d %H:%M:%S %Z"))
            }
            _ => {
                return Err("不支持的时区，请使用 UTC 或 LOCAL".into());
            }
        };

        message.reply(&ctx.api, &ctx.token, &time_text).await?;
        Ok(())
    }

    fn name(&self) -> &str {
        "time"
    }

    fn description(&self) -> &str {
        "显示当前时间 (用法: !time [UTC|LOCAL])"
    }
}
```

## 事件监听器实现

### 欢迎消息监听器

```rust
pub struct WelcomeListener {
    welcome_channel_id: Option<String>,
}

impl WelcomeListener {
    pub fn new(welcome_channel_id: Option<String>) -> Self {
        Self { welcome_channel_id }
    }
}

#[async_trait::async_trait]
impl EventListener for WelcomeListener {
    async fn on_member_join(&self, ctx: &Context, member: &Member) {
        if let Some(channel_id) = &self.welcome_channel_id {
            let welcome_msg = format!(
                "🎉 欢迎 {} 加入我们的频道！\n\
                请阅读频道规则，如有问题请随时提问。",
                member.user.username.as_deref().unwrap_or("新成员")
            );

            if let Err(e) = ctx.api.post_message(&ctx.token, channel_id, Some(&welcome_msg), None).await {
                warn!("发送欢迎消息失败: {}", e);
            }
        }
    }

    async fn on_member_leave(&self, ctx: &Context, member: &Member) {
        if let Some(channel_id) = &self.welcome_channel_id {
            let farewell_msg = format!(
                "👋 {} 离开了频道，祝一切顺利！",
                member.user.username.as_deref().unwrap_or("成员")
            );

            if let Err(e) = ctx.api.post_message(&ctx.token, channel_id, Some(&farewell_msg), None).await {
                warn!("发送告别消息失败: {}", e);
            }
        }
    }
}
```

### 日志监听器

```rust
use chrono::Utc;

pub struct LoggingListener {
    log_channel_id: Option<String>,
}

impl LoggingListener {
    pub fn new(log_channel_id: Option<String>) -> Self {
        Self { log_channel_id }
    }

    async fn log_event(&self, ctx: &Context, event_type: &str, details: &str) {
        if let Some(channel_id) = &self.log_channel_id {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            let log_msg = format!("📝 [{}] {}: {}", timestamp, event_type, details);

            if let Err(e) = ctx.api.post_message(&ctx.token, channel_id, Some(&log_msg), None).await {
                warn!("发送日志消息失败: {}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl EventListener for LoggingListener {
    async fn on_guild_join(&self, ctx: &Context, guild: &Guild) {
        let details = format!("机器人加入频道: {}", guild.name.as_deref().unwrap_or("未知"));
        self.log_event(ctx, "GUILD_JOIN", &details).await;
    }

    async fn on_member_join(&self, ctx: &Context, member: &Member) {
        let details = format!("新成员加入: {}", 
                            member.user.username.as_deref().unwrap_or("未知用户"));
        self.log_event(ctx, "MEMBER_JOIN", &details).await;
    }

    async fn on_member_leave(&self, ctx: &Context, member: &Member) {
        let details = format!("成员离开: {}", 
                            member.user.username.as_deref().unwrap_or("未知用户"));
        self.log_event(ctx, "MEMBER_LEAVE", &details).await;
    }
}
```

## 高级事件处理模式

### 事件过滤器

```rust
pub struct EventFilter<T> {
    predicate: Box<dyn Fn(&T) -> bool + Send + Sync>,
    handler: Box<dyn Fn(&Context, &T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
}

impl<T> EventFilter<T> {
    pub fn new<P, H, F>(predicate: P, handler: H) -> Self
    where
        P: Fn(&T) -> bool + Send + Sync + 'static,
        H: Fn(&Context, &T) -> F + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        Self {
            predicate: Box::new(predicate),
            handler: Box::new(move |ctx, event| Box::pin(handler(ctx, event))),
        }
    }

    pub async fn process(&self, ctx: &Context, event: &T) {
        if (self.predicate)(event) {
            (self.handler)(ctx, event).await;
        }
    }
}

pub struct FilteredEventHandler {
    message_filters: Vec<EventFilter<Message>>,
    guild_filters: Vec<EventFilter<Guild>>,
    member_filters: Vec<EventFilter<Member>>,
}

impl FilteredEventHandler {
    pub fn new() -> Self {
        Self {
            message_filters: Vec::new(),
            guild_filters: Vec::new(),
            member_filters: Vec::new(),
        }
    }

    pub fn add_message_filter<P, H, F>(&mut self, predicate: P, handler: H)
    where
        P: Fn(&Message) -> bool + Send + Sync + 'static,
        H: Fn(&Context, &Message) -> F + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.message_filters.push(EventFilter::new(predicate, handler));
    }

    pub fn add_guild_filter<P, H, F>(&mut self, predicate: P, handler: H)
    where
        P: Fn(&Guild) -> bool + Send + Sync + 'static,
        H: Fn(&Context, &Guild) -> F + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.guild_filters.push(EventFilter::new(predicate, handler));
    }
}

#[async_trait::async_trait]
impl EventHandler for FilteredEventHandler {
    async fn message_create(&self, ctx: Context, message: Message) {
        for filter in &self.message_filters {
            filter.process(&ctx, &message).await;
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        for filter in &self.guild_filters {
            filter.process(&ctx, &guild).await;
        }
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        for filter in &self.member_filters {
            filter.process(&ctx, &member).await;
        }
    }
}
```

### 异步事件队列

```rust
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug, Clone)]
pub enum BotEvent {
    MessageReceived {
        message: Message,
        context: Context,
    },
    MemberJoined {
        member: Member,
        context: Context,
    },
    MemberLeft {
        member: Member,
        context: Context,
    },
    GuildJoined {
        guild: Guild,
        context: Context,
    },
}

pub struct AsyncEventProcessor {
    event_sender: mpsc::UnboundedSender<BotEvent>,
    _processor_handle: task::JoinHandle<()>,
}

impl AsyncEventProcessor {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<BotEvent>();

        let processor_handle = task::spawn(async move {
            while let Some(event) = receiver.recv().await {
                Self::process_event(event).await;
            }
        });

        Self {
            event_sender: sender,
            _processor_handle: processor_handle,
        }
    }

    async fn process_event(event: BotEvent) {
        match event {
            BotEvent::MessageReceived { message, context } => {
                info!("处理消息事件: {}", message.id);
                // 在这里执行耗时的消息处理逻辑
                Self::process_message_async(&context, &message).await;
            }
            BotEvent::MemberJoined { member, context } => {
                info!("处理成员加入事件: {:?}", member.user.username);
                Self::process_member_join_async(&context, &member).await;
            }
            BotEvent::MemberLeft { member, context } => {
                info!("处理成员离开事件: {:?}", member.user.username);
                Self::process_member_leave_async(&context, &member).await;
            }
            BotEvent::GuildJoined { guild, context } => {
                info!("处理频道加入事件: {}", guild.name.unwrap_or_default());
                Self::process_guild_join_async(&context, &guild).await;
            }
        }
    }

    async fn process_message_async(_ctx: &Context, _message: &Message) {
        // 模拟耗时处理
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        debug!("消息处理完成");
    }

    async fn process_member_join_async(_ctx: &Context, _member: &Member) {
        // 模拟数据库更新等操作
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        debug!("成员加入处理完成");
    }

    async fn process_member_leave_async(_ctx: &Context, _member: &Member) {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        debug!("成员离开处理完成");
    }

    async fn process_guild_join_async(_ctx: &Context, _guild: &Guild) {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        debug!("频道加入处理完成");
    }

    pub fn queue_event(&self, event: BotEvent) {
        if let Err(e) = self.event_sender.send(event) {
            warn!("事件队列发送失败: {}", e);
        }
    }
}

pub struct QueuedEventHandler {
    processor: AsyncEventProcessor,
}

impl QueuedEventHandler {
    pub fn new() -> Self {
        Self {
            processor: AsyncEventProcessor::new(),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for QueuedEventHandler {
    async fn message_create(&self, ctx: Context, message: Message) {
        let event = BotEvent::MessageReceived { message, context: ctx };
        self.processor.queue_event(event);
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        let event = BotEvent::MemberJoined { member, context: ctx };
        self.processor.queue_event(event);
    }

    async fn guild_member_remove(&self, ctx: Context, member: Member) {
        let event = BotEvent::MemberLeft { member, context: ctx };
        self.processor.queue_event(event);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        let event = BotEvent::GuildJoined { guild, context: ctx };
        self.processor.queue_event(event);
    }
}
```

## 完整示例程序

```rust
use botrs::{Client, Intents, Token};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=debug,event_handling=info")
        .init();

    info!("启动事件处理示例机器人");

    // 加载配置
    let token = Token::from_env()?;
    token.validate()?;

    // 创建模块化事件处理器
    let mut handler = ModularEventHandler::new("!".to_string());

    // 注册命令处理器
    let commands = handler.commands.clone();
    let statistics = handler.statistics.clone();

    handler.register_command(Box::new(HelpCommandHandler::new(commands.clone()))).await;
    handler.register_command(Box::new(StatsCommandHandler::new(statistics.clone()))).await;
    handler.register_command(Box::new(TimeCommandHandler)).await;

    // 注册事件监听器
    handler.register_listener(Box::new(WelcomeListener::new(
        std::env::var("WELCOME_CHANNEL_ID").ok()
    ))).await;
    
    handler.register_listener(Box::new(LoggingListener::new(
        std::env::var("LOG_CHANNEL_ID").ok()
    ))).await;

    // 配置 Intent
    let intents = Intents::default()
        .with_public_guild_messages()
        .with_guilds()
        .with_guild_members();

    // 创建并启动客户端
    let mut client = Client::new(token, intents, handler, false)?;

    info!("事件处理示例机器人启动中...");
    client.start().await?;

    info!("事件处理示例机器人已停止");
    Ok(())
}
```

## 性能优化和最佳实践

### 事件处理性能优化

1. **异步处理**: 使用事件队列避免阻塞主事件循环
2. **批量处理**: 对相似事件进行批量处理以提高效率
3. **缓存策略**: 缓存频繁访问的数据减少重复计算
4. **并发控制**: 合理控制并发事件处理数量

### 错误处理和恢复

```rust
use std::panic;

pub struct ResilientEventHandler<T: EventHandler> {
    inner: T,
    error_count: Arc<std::sync::atomic::AtomicU64>,
}

impl<T: EventHandler> ResilientEventHandler<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    async fn safe_execute<F, Fut>(&self, operation: F, operation_name: &str)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        let result = panic::AssertUnwindSafe(operation()).catch_unwind().await;
        
        match result {
            Ok(_) => {
                debug!("事件处理成功: {}", operation_name);
            }
            Err(_) => {
                error!("事件处理发生 panic: {}", operation_name);
                self.error_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
}

#[async_trait::async_trait]
impl<T: EventHandler> EventHandler for ResilientEventHandler<T> {
    async fn ready(&self, ctx: Context, ready: Ready) {
        self.safe_execute(|| self.inner.ready(ctx, ready), "ready").await;
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        self.safe_execute(|| self.inner.message_create(ctx, message), "message_create").await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild) {
        self.safe_execute(|| self.inner.guild_create(ctx, guild), "guild_create").await;
    }

    async fn guild_member_add(&self, ctx: Context, member: Member) {
        self.safe_execute(|| self.inner.guild_member_add(ctx, member), "guild_member_add").await;
    }
}
```

## 测试和调试

### 事件处理器测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use botrs::{User, Author};

    fn create_test_message() -> Message {
        Message {
            id: "test_msg_123".to_string(),
            channel_id: "test_channel_456".to_string(),
            guild_id: Some("test_guild_789".to_string()),
            content: Some("!test command".to_string()),
            author: Some(Author {
                id: "test_user_001".to_string(),
                username: Some("TestUser".to_string()),
                avatar: None,
                bot: Some(false),
                member: None,
            }),
            timestamp: chrono::Utc::now().to_rfc3339(),
            edited_timestamp: None,
            mention_everyone: false,
            mentions: Vec::new(),
            mention_roles: Vec::new(),
            attachments: Vec::new(),
            embeds: Vec::new(),
            reactions: None,
            nonce: None,
            pinned: false,
            webhook_id: None,
            message_type: 0,
            activity: None,
            application: None,
            message_reference: None,
            flags: None,
            stickers: None,
            referenced_message: None,
            interaction: None,
            thread: None,
            components: None,
            ark: None,
            markdown: None,
            keyboard: None,
            seq: None,
            seq_in_channel: None,
            message_audit: None,
        }
    }

    #[tokio::test]
    async fn test_command_handling() {
        let handler = ModularEventHandler::new("!".to_string());
        
        // 注册测试命令
        struct TestCommand;
        
        #[async_trait::async_trait]
        impl CommandHandler for TestCommand {
            async fn handle(&self, _ctx: &Context, _message: &Message, _args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            
            fn name(&self) -> &str {
                "test"
            }
            
            fn description(&self) -> &str {
                "测试命令"
            }
        }
        
        handler.register_command(Box::new(TestCommand)).await;
        
        let commands = handler.commands.read().await;
        assert!(commands.contains_key("test"));
        assert_eq!(commands["test"].name(), "test");
        assert_eq!(commands["test"].description(), "测试命令");
    }

    #[tokio::test]
    async fn test_event_statistics() {
        let handler = ModularEventHandler::new("!".to_string());
        
        // 模拟处理一些事件
        {
            let mut stats = handler.statistics.write().await;
            stats.messages_processed = 10;
            stats.commands_executed = 5;
            stats.guild_events = 2;
            stats.member_events = 3;
            stats.errors_encountered = 1;
        }
        
        let stats = handler.get_statistics().await;
        assert_eq!(stats.messages_processed, 10);
        assert_eq!(stats.commands_executed, 5);
        assert_eq!(stats.guild_events, 2);
        assert_eq!(stats.member_events, 3);
        assert_eq!(stats.errors_encountered, 1);
    }
}
```

## 监控和指标

### 事件性能监控

```rust
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct EventMetrics {
    pub average_processing_time: Duration,
    pub total_events_processed: u64,
    pub events_per_second: f64,
    pub error_rate: f64,
}

pub struct EventPerformanceMonitor {
    start_time: Instant,
    processing_times: Arc<RwLock<Vec<Duration>>>,
    total_events: Arc<std::sync::atomic::AtomicU64>,
    total_errors: Arc<std::sync::atomic::AtomicU64>,
}

impl EventPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            processing_times: Arc::new(RwLock::new(Vec::new())),
            total_events: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            total_errors: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub async fn record_event_processing(&self, duration: Duration) {
        self.total_events.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let mut times = self.processing_times.write().await;
        times.push(duration);
        
        // 保留最近1000个记录
        if times.len() > 1000 {
            times.remove(0);
        }
    }

    pub fn record_error(&self) {
        self.total_errors.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn get_metrics(&self) -> EventMetrics {
        let times = self.processing_times.read().await;
        let total_events = self.total_events.load(std::sync::atomic::Ordering::Relaxed);
        let total_errors = self.total_errors.load(std::sync::atomic::Ordering::Relaxed);
        let uptime = self.start_time.elapsed();

        let average_processing_time = if !times.is_empty() {
            let total: Duration = times.iter().sum();
            total / times.len() as u32
        } else {
            Duration::from_millis(0)
        };

        let events_per_second = if uptime.as_secs() > 0 {
            total_events as f64 / uptime.as_secs() as f64
        } else {
            0.0
        };

        let error_rate = if total_events > 0 {
            total_errors as f64 / total_events as f64 * 100.0
        } else {
            0.0
        };

        EventMetrics {
            average_processing_time,
            total_events_processed: total_events,
            events_per_second,
            error_rate,
        }
    }
}
```

通过合理的事件处理架构设计，您可以构建出高性能、可扩展且易于维护的机器人应用程序。本示例展示了从基础事件处理到高级架构模式的完整实现方案。

## 另请参阅

- [错误恢复示例](/zh/examples/error-recovery.md) - 事件处理中的错误恢复
- [API 集成示例](/zh/examples/api-integration.md) - 在事件处理中集成外部 API
- [富文本消息示例](/zh/examples/rich-messages.md) - 在事件响应中发送富文本
- [`EventHandler` API 参考](/zh/api/event-handler.md) - 事件处理器详细文档
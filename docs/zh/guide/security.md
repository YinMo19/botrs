# 安全指南

本综合指南涵盖了使用 BotRS 开发和部署 QQ 频道机器人的安全最佳实践。安全性应该是整个开发生命周期中的首要考虑因素。

## 概述

机器人安全性涵盖几个关键领域：
- **凭据管理** - 保护身份验证令牌和密钥
- **输入验证** - 防止注入攻击和恶意数据
- **权限管理** - 实施适当的访问控制
- **数据保护** - 保护用户数据和通信
- **基础设施安全** - 加固部署环境

## 凭据安全

### 令牌管理

永远不要在源代码中硬编码凭据：

```rust
// ❌ 错误：硬编码凭据
let token = Token::new("123456789", "my_secret_key");

// ✅ 正确：环境变量
let app_id = std::env::var("QQ_BOT_APP_ID")
    .expect("未设置 QQ_BOT_APP_ID 环境变量");
let secret = std::env::var("QQ_BOT_SECRET")
    .expect("未设置 QQ_BOT_SECRET 环境变量");
let token = Token::new(app_id, secret);
```

### 安全令牌存储

在生产环境中使用安全存储机制：

```rust
use std::fs;
use std::path::Path;

// 安全配置加载
pub struct SecureConfig {
    app_id: String,
    secret: String,
}

impl SecureConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // 确保文件具有限制性权限（600）
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions();
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if permissions.mode() & 0o777 != 0o600 {
                return Err("配置文件必须具有 600 权限".into());
            }
        }
        
        let content = fs::read_to_string(path)?;
        let config: SecureConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    // 从加密文件加载
    pub fn load_encrypted(path: &Path, key: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let encrypted_data = fs::read(path)?;
        let decrypted = decrypt_data(&encrypted_data, key)?;
        let config: SecureConfig = toml::from_slice(&decrypted)?;
        Ok(config)
    }
}

// 加密函数占位符
fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 使用适当的加密库，如 `ring` 或 `age`
    unimplemented!("使用适当的加密实现")
}
```

### 令牌轮换

实现令牌轮换以增强安全性：

```rust
use chrono::{DateTime, Utc, Duration};

pub struct RotatingToken {
    current_token: Token,
    expires_at: DateTime<Utc>,
    refresh_callback: Box<dyn Fn() -> Result<Token, Box<dyn std::error::Error>> + Send + Sync>,
}

impl RotatingToken {
    pub fn new<F>(token: Token, expires_at: DateTime<Utc>, refresh_callback: F) -> Self 
    where
        F: Fn() -> Result<Token, Box<dyn std::error::Error>> + Send + Sync + 'static
    {
        Self {
            current_token: token,
            expires_at,
            refresh_callback: Box::new(refresh_callback),
        }
    }
    
    pub async fn get_valid_token(&mut self) -> Result<&Token, Box<dyn std::error::Error>> {
        if Utc::now() > self.expires_at - Duration::minutes(5) {
            // 在过期前刷新令牌
            self.current_token = (self.refresh_callback)()?;
            self.expires_at = Utc::now() + Duration::hours(24);
        }
        Ok(&self.current_token)
    }
}
```

## 输入验证

### 消息内容验证

始终验证和清理用户输入：

```rust
use regex::Regex;
use std::collections::HashSet;

pub struct InputValidator {
    url_regex: Regex,
    sql_injection_patterns: Vec<Regex>,
    max_message_length: usize,
    blocked_words: HashSet<String>,
}

impl InputValidator {
    pub fn new() -> Self {
        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        let sql_injection_patterns = vec![
            Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter)\s").unwrap(),
            Regex::new(r"(?i)(or|and)\s+\d+\s*=\s*\d+").unwrap(),
            Regex::new(r"(?i)'\s*(or|and)\s+'").unwrap(),
        ];
        
        Self {
            url_regex,
            sql_injection_patterns,
            max_message_length: 2000,
            blocked_words: HashSet::new(),
        }
    }
    
    pub fn validate_message_content(&self, content: &str) -> Result<String, ValidationError> {
        // 长度检查
        if content.len() > self.max_message_length {
            return Err(ValidationError::TooLong);
        }
        
        // 检查 SQL 注入模式
        for pattern in &self.sql_injection_patterns {
            if pattern.is_match(content) {
                return Err(ValidationError::SqlInjection);
            }
        }
        
        // 检查被阻止的词语
        let words: Vec<&str> = content.split_whitespace().collect();
        for word in words {
            if self.blocked_words.contains(&word.to_lowercase()) {
                return Err(ValidationError::BlockedContent);
            }
        }
        
        // 清理 HTML/markdown
        let sanitized = self.sanitize_content(content);
        Ok(sanitized)
    }
    
    fn sanitize_content(&self, content: &str) -> String {
        // 移除或转义潜在危险内容
        content
            .replace("<script", "&lt;script")
            .replace("javascript:", "")
            .replace("data:", "")
    }
    
    pub fn extract_safe_urls(&self, content: &str) -> Vec<String> {
        let mut safe_urls = Vec::new();
        
        for url_match in self.url_regex.find_iter(content) {
            let url = url_match.as_str();
            if self.is_safe_url(url) {
                safe_urls.push(url.to_string());
            }
        }
        
        safe_urls
    }
    
    fn is_safe_url(&self, url: &str) -> bool {
        // 检查安全域名白名单
        let safe_domains = ["github.com", "docs.rs", "crates.io"];
        
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                return safe_domains.iter().any(|&safe| domain.ends_with(safe));
            }
        }
        false
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("消息过长")]
    TooLong,
    #[error("检测到潜在的 SQL 注入")]
    SqlInjection,
    #[error("检测到被阻止的内容")]
    BlockedContent,
    #[error("无效的 URL")]
    InvalidUrl,
}
```

### 命令参数验证

验证命令参数以防止滥用：

```rust
pub struct CommandValidator;

impl CommandValidator {
    pub fn validate_user_id(user_id: &str) -> Result<String, ValidationError> {
        // QQ 用户 ID 是数字字符串
        if user_id.chars().all(|c| c.is_ascii_digit()) && user_id.len() <= 20 {
            Ok(user_id.to_string())
        } else {
            Err(ValidationError::InvalidUserId)
        }
    }
    
    pub fn validate_channel_id(channel_id: &str) -> Result<String, ValidationError> {
        // 频道 ID 的类似验证
        if channel_id.chars().all(|c| c.is_ascii_digit()) && channel_id.len() <= 20 {
            Ok(channel_id.to_string())
        } else {
            Err(ValidationError::InvalidChannelId)
        }
    }
    
    pub fn validate_duration(duration_str: &str) -> Result<std::time::Duration, ValidationError> {
        use std::time::Duration;
        
        let duration_regex = Regex::new(r"^(\d+)([smhd])$").unwrap();
        
        if let Some(captures) = duration_regex.captures(duration_str) {
            let value: u64 = captures[1].parse().map_err(|_| ValidationError::InvalidDuration)?;
            let unit = &captures[2];
            
            let duration = match unit {
                "s" => Duration::from_secs(value),
                "m" => Duration::from_secs(value * 60),
                "h" => Duration::from_secs(value * 3600),
                "d" => Duration::from_secs(value * 86400),
                _ => return Err(ValidationError::InvalidDuration),
            };
            
            // 强制执行合理限制
            if duration > Duration::from_secs(86400 * 30) { // 最多 30 天
                return Err(ValidationError::DurationTooLong);
            }
            
            Ok(duration)
        } else {
            Err(ValidationError::InvalidDuration)
        }
    }
}
```

## 权限管理

### 基于角色的访问控制

实施适当的权限检查：

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Permission {
    SendMessages,
    DeleteMessages,
    KickMembers,
    BanMembers,
    ManageChannels,
    ManageGuild,
    Administrator,
}

pub struct PermissionManager {
    role_permissions: HashMap<String, Vec<Permission>>,
    user_roles: HashMap<String, Vec<String>>,
}

impl PermissionManager {
    pub fn new() -> Self {
        let mut role_permissions = HashMap::new();
        
        // 定义默认角色和权限
        role_permissions.insert("moderator".to_string(), vec![
            Permission::SendMessages,
            Permission::DeleteMessages,
            Permission::KickMembers,
        ]);
        
        role_permissions.insert("admin".to_string(), vec![
            Permission::SendMessages,
            Permission::DeleteMessages,
            Permission::KickMembers,
            Permission::BanMembers,
            Permission::ManageChannels,
            Permission::ManageGuild,
        ]);
        
        role_permissions.insert("owner".to_string(), vec![
            Permission::Administrator,
        ]);
        
        Self {
            role_permissions,
            user_roles: HashMap::new(),
        }
    }
    
    pub fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        if let Some(roles) = self.user_roles.get(user_id) {
            for role in roles {
                if let Some(perms) = self.role_permissions.get(role) {
                    if perms.contains(&Permission::Administrator) || perms.contains(permission) {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    pub async fn check_command_permission(
        &self,
        user_id: &str,
        command: &str,
    ) -> Result<(), PermissionError> {
        let required_permission = match command {
            "kick" => Permission::KickMembers,
            "ban" => Permission::BanMembers,
            "delete" => Permission::DeleteMessages,
            "mute" => Permission::KickMembers,
            _ => Permission::SendMessages,
        };
        
        if self.has_permission(user_id, &required_permission) {
            Ok(())
        } else {
            Err(PermissionError::InsufficientPermissions)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("权限不足")]
    InsufficientPermissions,
    #[error("未找到用户")]
    UserNotFound,
    #[error("未找到角色")]
    RoleNotFound,
}
```

### 安全性速率限制

实施以安全为重点的速率限制：

```rust
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

pub struct SecurityRateLimiter {
    user_attempts: HashMap<String, Vec<Instant>>,
    command_cooldowns: HashMap<String, Duration>,
}

impl SecurityRateLimiter {
    pub fn new() -> Self {
        let mut command_cooldowns = HashMap::new();
        command_cooldowns.insert("kick".to_string(), Duration::from_secs(10));
        command_cooldowns.insert("ban".to_string(), Duration::from_secs(30));
        command_cooldowns.insert("mute".to_string(), Duration::from_secs(5));
        
        Self {
            user_attempts: HashMap::new(),
            command_cooldowns,
        }
    }
    
    pub fn check_rate_limit(&mut self, user_id: &str, command: &str) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let attempts = self.user_attempts.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // 移除旧尝试（超过 1 分钟）
        attempts.retain(|&attempt| now.duration_since(attempt) < Duration::from_secs(60));
        
        // 检查滥用（每分钟超过 10 个命令）
        if attempts.len() >= 10 {
            return Err(RateLimitError::TooManyAttempts);
        }
        
        // 检查命令特定冷却时间
        if let Some(cooldown) = self.command_cooldowns.get(command) {
            if let Some(&last_attempt) = attempts.last() {
                if now.duration_since(last_attempt) < *cooldown {
                    return Err(RateLimitError::CommandCooldown);
                }
            }
        }
        
        attempts.push(now);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("尝试次数过多")]
    TooManyAttempts,
    #[error("命令冷却中")]
    CommandCooldown,
}
```

## 数据保护

### 敏感数据处理

安全处理用户数据：

```rust
use sha2::{Sha256, Digest};
use std::fmt;

// 永远不要以明文存储密码
pub struct UserCredentials {
    user_id: String,
    password_hash: String,
    salt: String,
}

impl UserCredentials {
    pub fn new(user_id: String, password: &str) -> Self {
        let salt = generate_salt();
        let password_hash = hash_password(password, &salt);
        
        Self {
            user_id,
            password_hash,
            salt,
        }
    }
    
    pub fn verify_password(&self, password: &str) -> bool {
        let computed_hash = hash_password(password, &self.salt);
        computed_hash == self.password_hash
    }
}

fn generate_salt() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32).map(|_| rng.gen::<u8>()).map(|b| format!("{:02x}", b)).collect()
}

fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    format!("{:x}", hasher.finalize())
}

// 安全日志记录 - 避免记录敏感数据
pub struct SecureLogger;

impl SecureLogger {
    pub fn log_user_action(user_id: &str, action: &str, details: Option<&str>) {
        // 对用户 ID 进行哈希以保护隐私
        let user_hash = Self::hash_user_id(user_id);
        
        match details {
            Some(details) => {
                tracing::info!("用户 {} 执行了操作：{}（{}）", user_hash, action, details);
            }
            None => {
                tracing::info!("用户 {} 执行了操作：{}", user_hash, action);
            }
        }
    }
    
    fn hash_user_id(user_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        format!("{:x}", hasher.finalize())[0..8].to_string()
    }
}
```

### 数据加密

对静态敏感数据进行加密：

```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::RngCore;

pub struct DataEncryption {
    cipher: Aes256Gcm,
}

impl DataEncryption {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher.encrypt(nonce, data)?;
        
        // 在密文前添加随机数
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
    
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if encrypted_data.len() < 12 {
            return Err("无效的加密数据".into());
        }
        
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }
}
```

## 安全事件处理

### 事件验证

验证所有传入事件：

```rust
use botrs::{EventHandler, Context, Message, BotError};

pub struct SecureEventHandler {
    validator: InputValidator,
    permission_manager: PermissionManager,
    rate_limiter: SecurityRateLimiter,
}

#[async_trait::async_trait]
impl EventHandler for SecureEventHandler {
    async fn message_create(&self, ctx: Context, msg: Message) {
        // 验证消息来源
        if let Err(e) = self.validate_message_source(&msg) {
            tracing::warn!("无效的消息来源：{}", e);
            return;
        }
        
        // 验证消息内容
        let content = match &msg.content {
            Some(content) => content,
            None => return,
        };
        
        let validated_content = match self.validator.validate_message_content(content) {
            Ok(content) => content,
            Err(e) => {
                tracing::warn!("消息验证失败：{}", e);
                self.send_security_warning(&ctx, &msg, "无效的消息内容").await;
                return;
            }
        };
        
        // 处理验证过的消息
        self.process_secure_message(&ctx, &msg, &validated_content).await;
    }
    
    async fn error(&self, error: BotError) {
        // 记录安全相关错误
        match &error {
            BotError::AuthenticationFailed(_) => {
                tracing::error!("安全警报：身份验证失败 - {}", error);
                // 警报安全团队
            }
            BotError::Forbidden(_) => {
                tracing::warn!("权限被拒绝：{}", error);
            }
            _ => {
                tracing::debug!("机器人错误：{}", error);
            }
        }
    }
}

impl SecureEventHandler {
    async fn validate_message_source(&self, msg: &Message) -> Result<(), SecurityError> {
        // 验证消息来自预期来源
        if let Some(guild_id) = &msg.guild_id {
            if !self.is_trusted_guild(guild_id) {
                return Err(SecurityError::UntrustedSource);
            }
        }
        
        // 检查机器人消息（潜在的冒充）
        if msg.is_from_bot() {
            return Err(SecurityError::BotMessage);
        }
        
        Ok(())
    }
    
    fn is_trusted_guild(&self, guild_id: &str) -> bool {
        // 实施频道白名单检查
        true // 占位符
    }
    
    async fn send_security_warning(&self, ctx: &Context, msg: &Message, reason: &str) {
        let warning = format!("⚠️ 安全警告：{}", reason);
        if let Err(e) = msg.reply(&ctx.api, &ctx.token, &warning).await {
            tracing::error!("发送安全警告失败：{}", e);
        }
    }
    
    async fn process_secure_message(&self, ctx: &Context, msg: &Message, content: &str) {
        // 实施安全消息处理
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("不受信任的消息来源")]
    UntrustedSource,
    #[error("检测到来自机器人的消息")]
    BotMessage,
    #[error("检测到可疑活动")]
    SuspiciousActivity,
}
```

## 基础设施安全

### TLS 配置

确保安全连接：

```rust
use reqwest::ClientBuilder;
use std::time::Duration;

pub fn create_secure_http_client() -> Result<reqwest::Client, reqwest::Error> {
    ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .tcp_keepalive(Duration::from_secs(60))
        .use_rustls_tls() // 使用 rustls 以获得更好的安全性
        .min_tls_version(reqwest::tls::Version::TLS_1_2)
        .https_only(true)
        .build()
}
```

### 安全配置

实施安全配置管理：

```rust
use std::fs;
use std::path::Path;

pub struct SecureConfiguration {
    config_path: String,
    file_permissions: u32,
}

impl SecureConfiguration {
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
            file_permissions: 0o600, // 仅所有者读/写
        }
    }
    
    pub fn load_config<T>(&self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = Path::new(&self.config_path);
        
        // 验证文件权限
        self.verify_file_permissions(path)?;
        
        // 加载和解析配置
        let content = fs::read_to_string(path)?;
        let config: T = toml::from_str(&content)?;
        Ok(config)
    }
    
    fn verify_file_permissions(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = fs::metadata(path)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode() & 0o777;
            if mode != self.file_permissions {
                return Err(format!(
                    "不安全的文件权限：{:o}，预期：{:o}",
                    mode, self.file_permissions
                ).into());
            }
        }
        
        Ok(())
    }
}
```

## 安全监控

### 审计日志

实施全面的审计日志：

```rust
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuditEvent {
    timestamp: String,
    event_type: String,
    user_id: Option<String>,
    action: String,
    details: serde_json::Value,
    ip_address: Option<String>,
    success: bool,
}

pub struct AuditLogger {
    log_file: std::fs::File,
}

impl AuditLogger {
    pub fn new(log_path: &str) -> Result<Self, std::io::Error> {
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
            
        Ok(Self { log_file })
    }
    
    pub fn log_security_event(
        &mut self,
        event_type: &str,
        user_id: Option<&str>,
        action: &str,
        details: serde_json::Value,
        success: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = AuditEvent {
            timestamp: Utc::now().to_rfc3339(),
            event_type: event_type.to_string(),
            user_id: user_id.map(|s| s.to_string()),
            action: action.to_string(),
            details,
            ip_address: None, // 将从请求上下文填充
            success,
        };
        
        writeln!(self.log_file, "{}", serde_json::to_string(&event)?)?;
        Ok(())
    }
}
```

### 入侵检测

实施基本入侵检测：

```rust
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

pub struct IntrusionDetector {
    failed_attempts: HashMap<String, Vec<Instant>>,
    blocked_users: HashMap<String, Instant>,
}

impl IntrusionDetector {
    pub fn new() -> Self {
        Self {
            failed_attempts: HashMap::new(),
            blocked_users: HashMap::new(),
        }
    }
    
    pub fn record_failed_attempt(&mut self, user_id: &str) -> bool {
        let now = Instant::now();
        let attempts = self.failed_attempts.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // 移除 1 小时前的尝试
        attempts.retain(|&attempt| now.duration_since(attempt) < Duration::from_secs(3600));
        
        attempts.push(now);
        
        // 如果 1 小时内失败尝试超过 5 次则阻止用户
        if attempts.len() > 5 {
            self.blocked_users.insert(user_id.to_string(), now);
            tracing::warn!("由于重复失败尝试，用户 {} 被阻止", user_id);
            true
        } else {
            false
        }
    }
    
    pub fn is_blocked(&self, user_id: &str) -> bool {
        if let Some(&blocked_at) = self.blocked_users.get(user_id) {
            // 阻止 24 小时
            Instant::now().duration_since(blocked_at) < Duration::from_secs(86400)
        } else {
            false
        }
    }
    
    pub fn clear_user_attempts(&mut self, user_id: &str) {
        self.failed_attempts.remove(user_id);
        self.blocked_users.remove(user_id);
    }
}
```

## 安全最佳实践

### 开发指南

1. **最小权限原则** - 授予必要的最小权限
2. **深度防御** - 实施多个安全层
3. **安全失败** - 确保失败不会暴露敏感信息
4. **输入验证** - 验证来自不受信任来源的所有输入
5. **输出编码** - 正确编码输出以防止注入

### 部署安全

1. **仅使用 HTTPS/WSS** - 永远不要通过未加密连接传输凭据
2. **安全环境变量** - 使用适当的密钥管理
3. **定期更新** - 保持依赖项和运行时更新
4. **网络隔离** - 使用防火墙和网络分段
5. **监控和警报** - 实施安全监控

### 代码审查清单

- [ ] 没有硬编码的凭据或密钥
- [ ] 对所有用户输入实施输入验证
- [ ] 适当的错误处理而不泄露信息
- [ ] 身份验证和授权检查到位
- [ ] 敏感数据正确加密/哈希
- [ ] 安全相关事件的审计日志
- [ ] 对
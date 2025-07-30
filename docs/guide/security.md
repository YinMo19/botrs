# Security Guide

This comprehensive guide covers security best practices for developing and deploying QQ Guild bots with BotRS. Security should be a primary consideration throughout the development lifecycle.

## Overview

Bot security encompasses several critical areas:
- **Credential Management** - Protecting authentication tokens and secrets
- **Input Validation** - Preventing injection attacks and malformed data
- **Permission Management** - Implementing proper access controls
- **Data Protection** - Securing user data and communications
- **Infrastructure Security** - Hardening deployment environments

## Credential Security

### Token Management

Never hardcode credentials in source code:

```rust
// ❌ BAD: Hardcoded credentials
let token = Token::new("123456789", "my_secret_key");

// ✅ GOOD: Environment variables
let app_id = std::env::var("QQ_BOT_APP_ID")
    .expect("QQ_BOT_APP_ID environment variable not set");
let secret = std::env::var("QQ_BOT_SECRET")
    .expect("QQ_BOT_SECRET environment variable not set");
let token = Token::new(app_id, secret);
```

### Secure Token Storage

Use secure storage mechanisms for production:

```rust
use std::fs;
use std::path::Path;

// Secure configuration loading
pub struct SecureConfig {
    app_id: String,
    secret: String,
}

impl SecureConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // Ensure file has restrictive permissions (600)
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions();
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if permissions.mode() & 0o777 != 0o600 {
                return Err("Config file must have 600 permissions".into());
            }
        }
        
        let content = fs::read_to_string(path)?;
        let config: SecureConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    // Load from encrypted file
    pub fn load_encrypted(path: &Path, key: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let encrypted_data = fs::read(path)?;
        let decrypted = decrypt_data(&encrypted_data, key)?;
        let config: SecureConfig = toml::from_slice(&decrypted)?;
        Ok(config)
    }
}

// Placeholder for encryption function
fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Use a proper encryption library like `ring` or `age`
    unimplemented!("Implement with proper encryption")
}
```

### Token Rotation

Implement token rotation for enhanced security:

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
            // Refresh token before expiration
            self.current_token = (self.refresh_callback)()?;
            self.expires_at = Utc::now() + Duration::hours(24);
        }
        Ok(&self.current_token)
    }
}
```

## Input Validation

### Message Content Validation

Always validate and sanitize user input:

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
        // Length check
        if content.len() > self.max_message_length {
            return Err(ValidationError::TooLong);
        }
        
        // Check for SQL injection patterns
        for pattern in &self.sql_injection_patterns {
            if pattern.is_match(content) {
                return Err(ValidationError::SqlInjection);
            }
        }
        
        // Check for blocked words
        let words: Vec<&str> = content.split_whitespace().collect();
        for word in words {
            if self.blocked_words.contains(&word.to_lowercase()) {
                return Err(ValidationError::BlockedContent);
            }
        }
        
        // Sanitize HTML/markdown
        let sanitized = self.sanitize_content(content);
        Ok(sanitized)
    }
    
    fn sanitize_content(&self, content: &str) -> String {
        // Remove or escape potentially dangerous content
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
        // Check against whitelist of safe domains
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
    #[error("Message too long")]
    TooLong,
    #[error("Potential SQL injection detected")]
    SqlInjection,
    #[error("Blocked content detected")]
    BlockedContent,
    #[error("Invalid URL")]
    InvalidUrl,
}
```

### Command Parameter Validation

Validate command parameters to prevent abuse:

```rust
pub struct CommandValidator;

impl CommandValidator {
    pub fn validate_user_id(user_id: &str) -> Result<String, ValidationError> {
        // QQ user IDs are numeric strings
        if user_id.chars().all(|c| c.is_ascii_digit()) && user_id.len() <= 20 {
            Ok(user_id.to_string())
        } else {
            Err(ValidationError::InvalidUserId)
        }
    }
    
    pub fn validate_channel_id(channel_id: &str) -> Result<String, ValidationError> {
        // Similar validation for channel IDs
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
            
            // Enforce reasonable limits
            if duration > Duration::from_secs(86400 * 30) { // 30 days max
                return Err(ValidationError::DurationTooLong);
            }
            
            Ok(duration)
        } else {
            Err(ValidationError::InvalidDuration)
        }
    }
}
```

## Permission Management

### Role-Based Access Control

Implement proper permission checking:

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
        
        // Define default roles and permissions
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
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    #[error("User not found")]
    UserNotFound,
    #[error("Role not found")]
    RoleNotFound,
}
```

### Rate Limiting for Security

Implement security-focused rate limiting:

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
        
        // Remove old attempts (older than 1 minute)
        attempts.retain(|&attempt| now.duration_since(attempt) < Duration::from_secs(60));
        
        // Check for abuse (more than 10 commands per minute)
        if attempts.len() >= 10 {
            return Err(RateLimitError::TooManyAttempts);
        }
        
        // Check command-specific cooldown
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
    #[error("Too many attempts")]
    TooManyAttempts,
    #[error("Command on cooldown")]
    CommandCooldown,
}
```

## Data Protection

### Sensitive Data Handling

Handle user data securely:

```rust
use sha2::{Sha256, Digest};
use std::fmt;

// Never store passwords in plain text
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

// Secure logging - avoid logging sensitive data
pub struct SecureLogger;

impl SecureLogger {
    pub fn log_user_action(user_id: &str, action: &str, details: Option<&str>) {
        // Hash user ID for privacy
        let user_hash = Self::hash_user_id(user_id);
        
        match details {
            Some(details) => {
                tracing::info!("User {} performed action: {} ({})", user_hash, action, details);
            }
            None => {
                tracing::info!("User {} performed action: {}", user_hash, action);
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

### Data Encryption

Encrypt sensitive data at rest:

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
        
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
    
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if encrypted_data.len() < 12 {
            return Err("Invalid encrypted data".into());
        }
        
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }
}
```

## Secure Event Handling

### Event Validation

Validate all incoming events:

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
        // Validate message source
        if let Err(e) = self.validate_message_source(&msg) {
            tracing::warn!("Invalid message source: {}", e);
            return;
        }
        
        // Validate message content
        let content = match &msg.content {
            Some(content) => content,
            None => return,
        };
        
        let validated_content = match self.validator.validate_message_content(content) {
            Ok(content) => content,
            Err(e) => {
                tracing::warn!("Message validation failed: {}", e);
                self.send_security_warning(&ctx, &msg, "Invalid message content").await;
                return;
            }
        };
        
        // Process validated message
        self.process_secure_message(&ctx, &msg, &validated_content).await;
    }
    
    async fn error(&self, error: BotError) {
        // Log security-relevant errors
        match &error {
            BotError::AuthenticationFailed(_) => {
                tracing::error!("Security alert: Authentication failed - {}", error);
                // Alert security team
            }
            BotError::Forbidden(_) => {
                tracing::warn!("Permission denied: {}", error);
            }
            _ => {
                tracing::debug!("Bot error: {}", error);
            }
        }
    }
}

impl SecureEventHandler {
    async fn validate_message_source(&self, msg: &Message) -> Result<(), SecurityError> {
        // Verify message comes from expected sources
        if let Some(guild_id) = &msg.guild_id {
            if !self.is_trusted_guild(guild_id) {
                return Err(SecurityError::UntrustedSource);
            }
        }
        
        // Check for bot messages (potential impersonation)
        if msg.is_from_bot() {
            return Err(SecurityError::BotMessage);
        }
        
        Ok(())
    }
    
    fn is_trusted_guild(&self, guild_id: &str) -> bool {
        // Implement guild whitelist check
        true // Placeholder
    }
    
    async fn send_security_warning(&self, ctx: &Context, msg: &Message, reason: &str) {
        let warning = format!("⚠️ Security warning: {}", reason);
        if let Err(e) = msg.reply(&ctx.api, &ctx.token, &warning).await {
            tracing::error!("Failed to send security warning: {}", e);
        }
    }
    
    async fn process_secure_message(&self, ctx: &Context, msg: &Message, content: &str) {
        // Implement secure message processing
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Untrusted message source")]
    UntrustedSource,
    #[error("Message from bot detected")]
    BotMessage,
    #[error("Suspicious activity detected")]
    SuspiciousActivity,
}
```

## Infrastructure Security

### TLS Configuration

Ensure secure connections:

```rust
use reqwest::ClientBuilder;
use std::time::Duration;

pub fn create_secure_http_client() -> Result<reqwest::Client, reqwest::Error> {
    ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .tcp_keepalive(Duration::from_secs(60))
        .use_rustls_tls() // Use rustls for better security
        .min_tls_version(reqwest::tls::Version::TLS_1_2)
        .https_only(true)
        .build()
}
```

### Secure Configuration

Implement secure configuration management:

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
            file_permissions: 0o600, // Owner read/write only
        }
    }
    
    pub fn load_config<T>(&self) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = Path::new(&self.config_path);
        
        // Verify file permissions
        self.verify_file_permissions(path)?;
        
        // Load and parse configuration
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
                    "Insecure file permissions: {:o}, expected: {:o}",
                    mode, self.file_permissions
                ).into());
            }
        }
        
        Ok(())
    }
}
```

## Security Monitoring

### Audit Logging

Implement comprehensive audit logging:

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
            ip_address: None, // Would be populated from request context
            success,
        };
        
        writeln!(self.log_file, "{}", serde_json::to_string(&event)?)?;
        Ok(())
    }
}
```

### Intrusion Detection

Implement basic intrusion detection:

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
        
        // Remove attempts older than 1 hour
        attempts.retain(|&attempt| now.duration_since(attempt) < Duration::from_secs(3600));
        
        attempts.push(now);
        
        // Block user if more than 5 failed attempts in 1 hour
        if attempts.len() > 5 {
            self.blocked_users.insert(user_id.to_string(), now);
            tracing::warn!("User {} blocked due to repeated failed attempts", user_id);
            true
        } else {
            false
        }
    }
    
    pub fn is_blocked(&self, user_id: &str) -> bool {
        if let Some(&blocked_at) = self.blocked_users.get(user_id) {
            // Block for 24 hours
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

## Security Best Practices

### Development Guidelines

1. **Principle of Least Privilege** - Grant minimal necessary permissions
2. **Defense in Depth** - Implement multiple security layers
3. **Fail Securely** - Ensure failures don't expose sensitive information
4. **Input Validation** - Validate all input from untrusted sources
5. **Output Encoding** - Properly encode output to prevent injection

### Deployment Security

1. **Use HTTPS/WSS only** - Never transmit credentials over unencrypted connections
2. **Secure environment variables** - Use proper secrets management
3. **Regular updates** - Keep dependencies and runtime updated
4. **Network isolation** - Use firewalls and network segmentation
5. **Monitoring and alerting** - Implement security monitoring

### Code Review Checklist

- [ ] No hardcoded credentials or secrets
- [ ] Input validation implemented for all user input
- [ ] Proper error handling without information disclosure
- [ ] Authentication and authorization checks in place
- [ ] Sensitive data properly encrypted/hashed
- [ ] Audit logging for security-relevant events
- [ ] Rate limiting implemented for sensitive operations
- [ ] Dependencies are up to date and from trusted sources

## Security Testing

### Penetration Testing

Regularly test your bot's security:

```rust
// Security test framework
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sql_injection_protection() {
        let validator = InputValidator::new();
        
        let malicious_inputs = vec![
            "'; DROP TABLE users; --",
            "1' OR '1'='1",
            "UNION SELECT password FROM users",
        ];
        
        for input in malicious_inputs {
            assert!(validator.validate_message_content(input).is_err());
        }
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let mut rate_limiter = SecurityRateLimiter::new();
        
        // Test rapid-fire attempts
        for i in 0..15 {
            let result = rate_limiter.check_rate_limit("test_user", "test_command");
            if i >= 10 {
                assert!(result.is_err()); // Should be rate limited
            }
        }
    }
    
    #[tokio::test]
    async fn test_permission_bypass() {
        let perm_manager = PermissionManager::new();
        
        // Test that users without permissions can't execute admin commands
        assert!(!perm_manager.has_permission("regular_user", &Permission::BanMembers));
        assert!(perm_manager.check_command_permission("regular_user", "ban").await.is_err());
    }
}
```

By following these security guidelines and implementing the suggested measures, you can build robust and secure QQ Guild bots that protect both your infrastructure and your users' data.
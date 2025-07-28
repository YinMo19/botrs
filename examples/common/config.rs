//! Configuration loading utilities for BotRS examples.
//!
//! This module provides functionality to load bot configuration from TOML files,
//! environment variables, and command line arguments.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

/// Main configuration structure for bot examples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Bot configuration section
    pub bot: BotConfig,
    /// Logging configuration section
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Network configuration section
    #[serde(default)]
    pub network: NetworkConfig,
}

/// Bot-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotConfig {
    /// QQ Bot Application ID
    pub app_id: String,
    /// QQ Bot Secret
    pub secret: String,
    /// Whether to use sandbox environment
    #[serde(default)]
    pub sandbox: bool,
}

/// Logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Whether to log to file
    #[serde(default)]
    pub log_to_file: bool,
    /// Log file path
    #[serde(default = "default_log_file")]
    pub log_file: String,
}

/// Network configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// HTTP request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    /// Maximum number of reconnection attempts
    #[serde(default = "default_max_reconnects")]
    pub max_reconnects: u32,
    /// Reconnection delay in seconds
    #[serde(default = "default_reconnect_delay")]
    pub reconnect_delay: u64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            log_to_file: false,
            log_file: default_log_file(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            timeout: default_timeout(),
            max_reconnects: default_max_reconnects(),
            reconnect_delay: default_reconnect_delay(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_file() -> String {
    "bot.log".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_max_reconnects() -> u32 {
    5
}

fn default_reconnect_delay() -> u64 {
    5
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from environment variables.
    ///
    /// Expected environment variables:
    /// - QQ_BOT_APP_ID: Bot application ID
    /// - QQ_BOT_SECRET: Bot secret
    /// - QQ_BOT_SANDBOX: Whether to use sandbox (optional, default: false)
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id =
            env::var("QQ_BOT_APP_ID").map_err(|_| "QQ_BOT_APP_ID environment variable not set")?;
        let secret =
            env::var("QQ_BOT_SECRET").map_err(|_| "QQ_BOT_SECRET environment variable not set")?;
        let sandbox = env::var("QQ_BOT_SANDBOX")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        Ok(Config {
            bot: BotConfig {
                app_id,
                secret,
                sandbox,
            },
            logging: LoggingConfig::default(),
            network: NetworkConfig::default(),
        })
    }

    /// Load configuration with fallback priority:
    /// 1. TOML file if it exists
    /// 2. Environment variables
    /// 3. Command line arguments (app_id and secret)
    pub fn load_with_fallback(
        config_path: Option<&str>,
        app_id: Option<String>,
        secret: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Try loading from file first
        if let Some(path) = config_path {
            if Path::new(path).exists() {
                return Self::from_file(path);
            }
        }

        // Try default config file locations
        for default_path in &["config.toml", "examples/config.toml"] {
            if Path::new(default_path).exists() {
                return Self::from_file(default_path);
            }
        }

        // Try environment variables
        if env::var("QQ_BOT_APP_ID").is_ok() && env::var("QQ_BOT_SECRET").is_ok() {
            return Self::from_env();
        }

        // Try command line arguments
        if let (Some(app_id), Some(secret)) = (app_id, secret) {
            return Ok(Config {
                bot: BotConfig {
                    app_id,
                    secret,
                    sandbox: false,
                },
                logging: LoggingConfig::default(),
                network: NetworkConfig::default(),
            });
        }

        Err("No configuration source found. Please provide a config file, environment variables, or command line arguments.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_toml() {
        let toml_content = r#"
[bot]
app_id = "123456789"
secret = "test_secret"
sandbox = true

[logging]
level = "debug"
log_to_file = true
log_file = "test.log"

[network]
timeout = 60
max_reconnects = 10
reconnect_delay = 10
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), toml_content).unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();
        assert_eq!(config.bot.app_id, "123456789");
        assert_eq!(config.bot.secret, "test_secret");
        assert!(config.bot.sandbox);
        assert_eq!(config.logging.level, "debug");
        assert!(config.logging.log_to_file);
        assert_eq!(config.network.timeout, 60);
    }

    #[test]
    fn test_config_defaults() {
        let logging = LoggingConfig::default();
        assert_eq!(logging.level, "info");
        assert!(!logging.log_to_file);

        let network = NetworkConfig::default();
        assert_eq!(network.timeout, 30);
        assert_eq!(network.max_reconnects, 5);
    }
}

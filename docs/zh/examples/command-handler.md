# 命令处理器示例

此示例演示如何创建一个结构化的命令处理系统，支持不同类型的命令、权限检查、参数验证和错误处理。

## 概述

命令处理器提供了一种组织和管理机器人命令的结构化方法。它支持命令注册、参数解析、权限验证和冷却时间管理。

## 基本命令结构

```rust
use botrs::{Client, Context, EventHandler, Message, Ready, Intents, Token};
use std::collections::HashMap;
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
    pub requires_permission: bool,
}

pub struct CommandHandler {
    pub commands: HashMap<String, Command>,
    pub prefix: String,
}

impl CommandHandler {
    pub fn new(prefix: &str) -> Self {
        let mut handler = Self {
            commands: HashMap::new(),
            prefix: prefix.to_string(),
        };
        handler.register_default_commands();
        handler
    }

    fn register_default_commands(&mut self) {
        // 基础命令
        self.register_command(Command {
            name: "ping".to_string(),
            description: "测试机器人响应".to_string(),
            usage: "!ping".to_string(),
            min_args: 0,
            max_args: Some(0),
            requires_permission: false,
        });

        self.register_command(Command {
            name: "help".to_string(),
            description: "显示可用命令".to_string(),
            usage: "!help [命令名]".to_string(),
            min_args: 0,
            max_args: Some(1),
            requires_permission: false,
        });

        self.register_command(Command {
            name: "echo".to_string(),
            description: "回声指定的消息".to_string(),
            usage: "!echo <消息>".to_string(),
            min_args: 1,
            max_args: None,
            requires_permission: false,
        });

        // 管理员命令
        self.register_command(Command {
            name: "kick".to_string(),
            description: "踢出用户".to_string(),
            usage: "!kick <@用户> [原因]".to_string(),
            min_args: 1,
            max_args: None,
            requires_permission: true,
        });

        self.register_command(Command {
            name: "mute".to_string(),
            description: "禁言用户".to_string(),
            usage: "!mute <@用户> [时长] [原因]".to_string(),
            min_args: 1,
            max_args: None,
            requires_permission: true,
        });
    }

    pub fn register_command(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn parse_command(&self, content: &str) -> Option<ParsedCommand> {
        if !content.starts_with(&self.prefix) {
            return None;
        }

        let content = &content[self.prefix.len()..];
        let parts: Vec<&str> = content.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command_name = parts[0].to_lowercase();
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        let raw_args = if parts.len() > 1 {
            content[parts[0].len()..].trim().to_string()
        } else {
            String::new()
        };

        Some(ParsedCommand {
            name: command_name,
            args,
            raw_args,
        })
    }
}

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
    pub raw_args: String,
}
```

## 机器人实现

```rust
pub struct CommandBot {
    command_handler: CommandHandler,
}

impl CommandBot {
    pub fn new() -> Self {
        Self {
            command_handler: CommandHandler::new("!"),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for CommandBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("命令机器人 {} 已准备就绪！", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        // 跳过机器人消息
        if msg.is_from_bot() {
            return;
        }

        let content = match &msg.content {
            Some(content) => content,
            None => return,
        };

        // 解析命令
        let parsed_command = match self.command_handler.parse_command(content) {
            Some(cmd) => cmd,
            None => return, // 不是命令
        };

        // 查找命令
        let command = match self.command_handler.commands.get(&parsed_command.name) {
            Some(cmd) => cmd,
            None => {
                let _ = msg.reply(&ctx.api, &ctx.token, &format!("未知命令：{}。使用 `!help` 查看可用命令。", parsed_command.name)).await;
                return;
            }
        };

        // 验证参数数量
        if let Err(error_msg) = self.validate_command_args(command, &parsed_command.args) {
            let _ = msg.reply(&ctx.api, &ctx.token, &error_msg).await;
            return;
        }

        // 检查权限
        if command.requires_permission {
            if !self.check_permission(&ctx, &msg).await {
                let _ = msg.reply(&ctx.api, &ctx.token, "你没有权限执行此命令。").await;
                return;
            }
        }

        // 执行命令
        if let Err(e) = self.execute_command(&ctx, &msg, &parsed_command).await {
            warn!("执行命令失败：{}", e);
            let _ = msg.reply(&ctx.api, &ctx.token, "命令执行失败。").await;
        }
    }

    async fn error(&self, error: botrs::BotError) {
        error!("事件处理器错误：{}", error);
    }
}

impl CommandBot {
    fn validate_command_args(&self, command: &Command, args: &[String]) -> Result<(), String> {
        if args.len() < command.min_args {
            return Err(format!(
                "参数不足。需要至少 {} 个参数。\n用法：{}",
                command.min_args, command.usage
            ));
        }

        if let Some(max_args) = command.max_args {
            if args.len() > max_args {
                return Err(format!(
                    "参数过多。最多接受 {} 个参数。\n用法：{}",
                    max_args, command.usage
                ));
            }
        }

        Ok(())
    }

    async fn check_permission(&self, ctx: &Context, msg: &Message) -> bool {
        // 简化的权限检查 - 在实际应用中，你需要查询用户的角色和权限
        if let Some(author) = &msg.author {
            if let Some(member) = &msg.member {
                // 检查是否为管理员或拥有特定权限
                return self.has_permission(member);
            }
        }
        false
    }

    fn has_permission(&self, member: &botrs::Member) -> bool {
        // 简化实现 - 检查角色或权限
        if let Some(roles) = &member.roles {
            for role in roles {
                // 假设管理员角色 ID
                if role == "管理员角色ID" {
                    return true;
                }
            }
        }
        false
    }

    async fn execute_command(&self, ctx: &Context, msg: &Message, parsed_command: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        match parsed_command.name.as_str() {
            "ping" => self.handle_ping(ctx, msg).await,
            "help" => self.handle_help(ctx, msg, &parsed_command.args).await,
            "echo" => self.handle_echo(ctx, msg, &parsed_command.raw_args).await,
            "kick" => self.handle_kick(ctx, msg, &parsed_command.args).await,
            "mute" => self.handle_mute(ctx, msg, &parsed_command.args).await,
            _ => {
                msg.reply(&ctx.api, &ctx.token, "命令未实现。").await?;
                Ok(())
            }
        }
    }

    async fn handle_ping(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let response = msg.reply(&ctx.api, &ctx.token, "🏓 计算延迟中...").await?;
        let latency = start.elapsed().as_millis();
        
        // 更新消息显示实际延迟
        let updated_content = format!("🏓 Pong！延迟：{}ms", latency);
        // 注意：这里需要消息编辑功能，当前可能不可用
        msg.reply(&ctx.api, &ctx.token, &updated_content).await?;
        Ok(())
    }

    async fn handle_help(&self, ctx: &Context, msg: &Message, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            // 显示所有命令
            let mut help_text = "**可用命令：**\n".to_string();
            for (name, command) in &self.command_handler.commands {
                let permission_text = if command.requires_permission {
                    " 🔒"
                } else {
                    ""
                };
                help_text.push_str(&format!(
                    "• `{}` - {}{}\n",
                    command.usage, command.description, permission_text
                ));
            }
            help_text.push_str("\n使用 `!help <命令名>` 获取特定命令的详细信息。");
            msg.reply(&ctx.api, &ctx.token, &help_text).await?;
        } else {
            // 显示特定命令的帮助
            let command_name = &args[0].to_lowercase();
            if let Some(command) = self.command_handler.commands.get(command_name) {
                let permission_text = if command.requires_permission {
                    "\n🔒 **需要权限**"
                } else {
                    ""
                };
                let help_text = format!(
                    "**{}**\n{}\n\n**用法：** `{}`{}",
                    command.name, command.description, command.usage, permission_text
                );
                msg.reply(&ctx.api, &ctx.token, &help_text).await?;
            } else {
                msg.reply(&ctx.api, &ctx.token, &format!("找不到命令：{}", command_name)).await?;
            }
        }
        Ok(())
    }

    async fn handle_echo(&self, ctx: &Context, msg: &Message, raw_args: &str) -> Result<(), Box<dyn std::error::Error>> {
        msg.reply(&ctx.api, &ctx.token, raw_args).await?;
        Ok(())
    }

    async fn handle_kick(&self, ctx: &Context, msg: &Message, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let user_mention = &args[0];
        let reason = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            "未指定原因".to_string()
        };

        // 提取用户 ID
        if let Some(user_id) = self.extract_user_id_from_mention(user_mention) {
            // 在实际实现中，这里会调用踢出用户的 API
            let response = format!("已踢出用户 {} 。原因：{}", user_mention, reason);
            msg.reply(&ctx.api, &ctx.token, &response).await?;
        } else {
            msg.reply(&ctx.api, &ctx.token, "无效的用户提及。请使用 @用户 格式。").await?;
        }
        Ok(())
    }

    async fn handle_mute(&self, ctx: &Context, msg: &Message, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let user_mention = &args[0];
        let duration = if args.len() > 1 {
            &args[1]
        } else {
            "10m"
        };
        let reason = if args.len() > 2 {
            args[2..].join(" ")
        } else {
            "未指定原因".to_string()
        };

        if let Some(user_id) = self.extract_user_id_from_mention(user_mention) {
            // 在实际实现中，这里会调用禁言用户的 API
            let response = format!("已禁言用户 {} {} 。原因：{}", user_mention, duration, reason);
            msg.reply(&ctx.api, &ctx.token, &response).await?;
        } else {
            msg.reply(&ctx.api, &ctx.token, "无效的用户提及。请使用 @用户 格式。").await?;
        }
        Ok(())
    }

    fn extract_user_id_from_mention(&self, mention: &str) -> Option<String> {
        // 简化的用户 ID 提取 - 实际实现可能更复杂
        if mention.starts_with("<@") && mention.ends_with(">") {
            let id = mention.trim_start_matches("<@").trim_end_matches(">");
            Some(id.to_string())
        } else {
            None
        }
    }
}
```

## 主函数

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("botrs=info,command_bot=info")
        .init();

    info!("启动命令机器人...");

    let token = Token::new(
        std::env::var("QQ_BOT_APP_ID")?,
        std::env::var("QQ_BOT_SECRET")?,
    );

    let intents = Intents::default()
        .with_public_guild_messages()
        .with_direct_message()
        .with_guilds()
        .with_guild_members();

    let mut client = Client::new(token, intents, CommandBot::new(), false)?;
    client.start().await?;

    Ok(())
}
```

## 高级功能

### 自定义命令注册

```rust
impl CommandBot {
    pub fn register_custom_command(&mut self, command: Command) {
        self.command_handler.register_command(command);
    }

    pub fn create_custom_commands(&mut self) {
        // 用户信息命令
        self.register_custom_command(Command {
            name: "userinfo".to_string(),
            description: "显示用户信息".to_string(),
            usage: "!userinfo [@用户]".to_string(),
            min_args: 0,
            max_args: Some(1),
            requires_permission: false,
        });

        // 服务器信息命令
        self.register_custom_command(Command {
            name: "serverinfo".to_string(),
            description: "显示服务器信息".to_string(),
            usage: "!serverinfo".to_string(),
            min_args: 0,
            max_args: Some(0),
            requires_permission: false,
        });

        // 清理消息命令
        self.register_custom_command(Command {
            name: "clear".to_string(),
            description: "清理指定数量的消息".to_string(),
            usage: "!clear <数量>".to_string(),
            min_args: 1,
            max_args: Some(1),
            requires_permission: true,
        });
    }
}
```

### 冷却时间系统

```rust
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

pub struct CooldownManager {
    cooldowns: HashMap<String, Instant>,
}

impl CooldownManager {
    pub fn new() -> Self {
        Self {
            cooldowns: HashMap::new(),
        }
    }

    pub fn check_cooldown(&mut self, user_id: &str, command: &str, duration: Duration) -> bool {
        let key = format!("{}:{}", user_id, command);
        let now = Instant::now();

        if let Some(last_used) = self.cooldowns.get(&key) {
            if now.duration_since(*last_used) < duration {
                return false; // 仍在冷却中
            }
        }

        self.cooldowns.insert(key, now);
        true
    }

    pub fn get_remaining_cooldown(&self, user_id: &str, command: &str, duration: Duration) -> Option<Duration> {
        let key = format!("{}:{}", user_id, command);
        if let Some(last_used) = self.cooldowns.get(&key) {
            let elapsed = Instant::now().duration_since(*last_used);
            if elapsed < duration {
                return Some(duration - elapsed);
            }
        }
        None
    }
}
```

### 命令中间件

```rust
#[async_trait::async_trait]
pub trait CommandMiddleware {
    async fn before_command(
        &self,
        ctx: &Context,
        msg: &Message,
        command: &Command,
    ) -> Result<bool, Box<dyn std::error::Error>>; // 返回 false 取消命令执行

    async fn after_command(
        &self,
        ctx: &Context,
        msg: &Message,
        command: &Command,
        result: &Result<(), Box<dyn std::error::Error>>,
    );
}

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl CommandMiddleware for LoggingMiddleware {
    async fn before_command(
        &self,
        _ctx: &Context,
        msg: &Message,
        command: &Command,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        info!(
            "用户 {} 执行命令：{}",
            msg.author.as_ref()
                .and_then(|a| a.username.as_deref())
                .unwrap_or("Unknown"),
            command.name
        );
        Ok(true)
    }

    async fn after_command(
        &self,
        _ctx: &Context,
        _msg: &Message,
        command: &Command,
        result: &Result<(), Box<dyn std::error::Error>>,
    ) {
        match result {
            Ok(_) => info!("命令 {} 执行成功", command.name),
            Err(e) => warn!("命令 {} 执行失败：{}", command.name, e),
        }
    }
}
```

## 使用示例

### 基础命令

```bash
!ping                    # 测试机器人响应
!help                    # 显示所有命令
!help ping              # 显示特定命令的帮助
!echo Hello World       # 回声消息
```

### 管理员命令

```bash
!kick @user 违规行为     # 踢出用户
!mute @user 30m 垃圾信息 # 禁言用户30分钟
```

### 错误处理

机器人会自动处理各种错误情况：
- 未知命令
- 参数不足或过多
- 权限不足
- 执行错误

## 最佳实践

1. **参数验证**：始终验证命令参数的数量和格式
2. **权限检查**：对敏感命令实施适当的权限控制
3. **错误处理**：提供清晰的错误消息和使用说明
4. **日志记录**：记录命令执行情况以便调试和监控
5. **冷却时间**：防止命令滥用和垃圾信息
6. **中间件**：使用中间件模式添加横切关注点

## 相关链接

- [回声机器人](./echo-bot.md) - 了解基本消息处理
- [富消息](./rich-messages.md) - 创建更丰富的响应
- [事件处理](./event-handling.md) - 处理其他类型的事件
- [错误恢复](./error-recovery.md) - 实现健壮的错误处理
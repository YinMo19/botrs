# 安装

开始使用 BotRS 非常简单。本指南将引导您将 BotRS 添加到 Rust 项目中并设置必要的依赖项。

## 前置条件

在安装 BotRS 之前，请确保您拥有以下内容：

- **Rust 1.70 或更高版本**：BotRS 使用现代 Rust 功能，需要较新的编译器版本
- **Cargo**：Rust 的包管理器（包含在 Rust 安装中）
- **QQ 频道机器人凭据**：从 QQ 频道开发者门户获取应用 ID 和密钥

### 安装 Rust

如果您还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 并按照您平台的安装说明操作：

```bash
# 安装 Rust（类 Unix 系统）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 获取机器人凭据

1. 访问 [QQ 频道开发者门户](https://bot.q.qq.com/)
2. 创建新应用程序或选择现有应用程序
3. 记下您的**应用 ID** 和**密钥** - 您将需要这些来进行身份验证

## 将 BotRS 添加到您的项目

### 创建新项目

首先创建一个新的 Rust 项目：

```bash
cargo new my-qq-bot
cd my-qq-bot
```

### 添加依赖项

将 BotRS 和必需的依赖项添加到您的 `Cargo.toml`：

```toml
[dependencies]
# BotRS 框架
botrs = "0.2.5"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 日志记录（推荐）
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 异步 trait 支持
async-trait = "0.1"
```

### 可选依赖项

根据您机器人的需求，您可能想要添加：

```toml
[dependencies]
# 用于配置文件
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# 用于命令行参数解析
clap = { version = "4.0", features = ["derive"] }

# 用于环境变量处理
dotenvy = "0.15"

# 用于 HTTP 客户端自定义
reqwest = { version = "0.11", features = ["json"] }

# 用于日期/时间处理
chrono = { version = "0.4", features = ["serde"] }
```

## 功能标志

BotRS 提供几个功能标志来自定义您的安装：

```toml
[dependencies]
botrs = { version = "0.2.5", features = ["examples"] }
```

### 可用功能

- **`examples`**：启用示例特定的依赖项（clap、toml）
- **默认功能**：核心功能始终包含

## 验证安装

创建一个简单的测试来验证 BotRS 是否正确安装：

```rust
// src/main.rs
use botrs::{Client, EventHandler, Intents, Token};

struct TestBot;

#[async_trait::async_trait]
impl EventHandler for TestBot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("BotRS 安装成功！");
    println!("版本：{}", botrs::VERSION);
    Ok(())
}
```

运行测试：

```bash
cargo run
```

您应该看到指示 BotRS 正确安装和版本号的输出。

## 开发依赖项

对于开发和测试，请考虑添加这些依赖项：

```toml
[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
```

## IDE 设置

### VS Code

为了在 VS Code 中获得最佳开发体验：

1. 安装 **rust-analyzer** 扩展
2. 配置您的设置：

```json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

### 其他 IDE

- **IntelliJ IDEA**：使用 Rust 插件
- **Vim/Neovim**：使用 rust-analyzer LSP 配置
- **Emacs**：使用 rustic-mode 与 rust-analyzer

## 常见安装问题

### 编译错误

**问题**：编译失败，缺少功能
```
error: failed to resolve: use of undeclared crate or module `tokio`
```

**解决方案**：确保启用所有必需的功能：
```toml
tokio = { version = "1.0", features = ["full"] }
```

**问题**：异步 trait 编译错误
```
error: async trait not supported
```

**解决方案**：添加 async-trait 依赖项：
```toml
async-trait = "0.1"
```

### 版本冲突

**问题**：依赖项版本冲突
```
error: failed to select a version for `serde`
```

**解决方案**：使用 `cargo tree` 识别冲突并指定兼容版本：
```bash
cargo tree
```

### 网络问题

**问题**：由于网络限制导致下载失败

**解决方案**：配置 cargo 使用代理或镜像：
```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "mirror"

[source.mirror]
registry = "https://crates.io/api/v1/crates"
```

## 环境设置

### 环境变量

为开发创建 `.env` 文件（可选）：

```bash
# .env
QQ_BOT_APP_ID=你的应用ID
QQ_BOT_SECRET=你的密钥
RUST_LOG=botrs=debug,my_qq_bot=info
```

添加到 `.gitignore`：
```gitignore
.env
target/
Cargo.lock  # 仅适用于库
```

### 配置文件

创建配置模板：

```toml
# config.toml
[bot]
app_id = "你的应用ID"
secret = "你的密钥"
sandbox = false

[logging]
level = "info"
```

## Docker 设置（可选）

对于容器化部署：

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-qq-bot /usr/local/bin/my-qq-bot

CMD ["my-qq-bot"]
```

## 下一步

现在 BotRS 已安装，您可以：

1. **[快速开始](/zh/guide/quick-start)** - 创建您的第一个机器人
2. **[配置](/zh/guide/configuration)** - 设置机器人凭据
3. **[客户端与事件处理](/zh/guide/client-handler)** - 学习核心概念

## 故障排除

如果您在安装过程中遇到问题：

1. 查看 [GitHub Issues](https://github.com/YinMo19/botrs/issues) 页面
2. 验证您的 Rust 版本：`rustc --version`
3. 更新您的工具链：`rustup update`
4. 清理并重建：`cargo clean && cargo build`

如需更多帮助，请随时在 GitHub 仓库上开启一个 issue，包含：
- 您的 Rust 版本
- 完整的错误消息
- 您的 `Cargo.toml` 配置
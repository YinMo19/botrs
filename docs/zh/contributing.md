# 贡献到 BotRS

感谢您对贡献 BotRS 的兴趣！本指南将帮助您开始为项目做出贡献。

## 行为准则

通过参与此项目，您同意遵守我们的[行为准则](./CODE_OF_CONDUCT.md)。请在贡献之前阅读。

## 开始使用

### 前置条件

- Rust 1.70 或更高版本
- Git
- 一个 QQ 频道机器人应用程序（用于测试）

### 设置开发环境

1. 在 GitHub 上 Fork 仓库
2. 在本地克隆您的 Fork：
   ```bash
   git clone https://github.com/YOUR_USERNAME/botrs.git
   cd botrs
   ```

3. 添加上游仓库：
   ```bash
   git remote add upstream https://github.com/YinMo19/botrs.git
   ```

4. 安装依赖项：
   ```bash
   cargo build
   ```

5. 运行测试以确保一切正常：
   ```bash
   cargo test
   ```

## 开发工作流程

### 在进行更改之前

1. 为您的功能或修复创建新分支：
   ```bash
   git checkout -b feature/你的功能名称
   ```

2. 保持您的分支与上游同步：
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### 进行更改

1. 遵循我们的[编码标准](#编码标准)编写代码
2. 为新功能添加测试
3. 如果需要，更新文档
4. 确保所有测试通过：
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

### 提交更改

1. 使用清晰、描述性的消息提交更改：
   ```bash
   git commit -m "feat: 添加对新消息类型的支持"
   ```

2. 推送到您的 Fork：
   ```bash
   git push origin feature/你的功能名称
   ```

3. 在 GitHub 上创建 Pull Request

## 贡献类型

### 错误报告

在报告错误时，请包括：

- 问题的清晰描述
- 重现问题的步骤
- 预期与实际行为
- 您的环境（Rust 版本、操作系统等）
- 相关的代码示例或日志

使用我们的[错误报告模板](./bug_report.md)。

### 功能请求

对于功能请求，请包括：

- 功能的清晰描述
- 用例和动机
- 来自其他库的任何相关示例
- 愿意自己实现功能

使用我们的[功能请求模板](./feature_request.md)。

### 代码贡献

我们欢迎以下贡献：

- 错误修复
- 新功能
- 性能改进
- 文档更新
- 示例代码
- 测试覆盖率改进

## 编码标准

### Rust 指南

- 遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 进行代码格式化
- 解决所有 `cargo clippy` 警告
- 为公共 API 编写全面的文档
- 为所有新功能添加测试

### 代码风格

```rust
// 使用描述性名称
pub struct MessageParams {
    content: Option<String>,
    embed: Option<MessageEmbed>,
}

// 记录公共 API
/// 创建具有文本内容的新消息。
///
/// # 参数
///
/// * `content` - 消息的文本内容
///
/// # 示例
///
/// ```
/// let params = MessageParams::new_text("你好，世界！");
/// ```
pub fn new_text(content: impl Into<String>) -> Self {
    Self {
        content: Some(content.into()),
        embed: None,
    }
}
```

### 错误处理

- 对可能失败的操作使用 `Result<T, BotError>`
- 提供有意义的错误消息
- 使用 `thiserror` 处理错误类型
- 在错误链中包含上下文

### 测试

- 为所有公共函数编写单元测试
- 对复杂工作流程使用集成测试
- 尽可能模拟外部依赖项
- 争取高测试覆盖率

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_params_creation() {
        let params = MessageParams::new_text("test");
        assert_eq!(params.content, Some("test".to_string()));
    }

    #[tokio::test]
    async fn test_api_call() {
        // 测试异步功能
    }
}
```

## 文档

### 代码文档

- 使用 `///` 注释记录所有公共 API
- 在文档中包含示例
- 解释复杂的算法和设计决策
- 使用 `cargo doc` 生成和审查文档

### 用户文档

- 添加功能时更新相关指南
- 在文档站点中包含示例
- 保持更新日志最新
- 如果需要，更新 README.md

## 测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行带输出的测试
cargo test -- --nocapture

# 运行集成测试
cargo test --test integration_tests
```

### 测试环境

设置测试环境变量：

```bash
export QQ_BOT_APP_ID="test_app_id"
export QQ_BOT_SECRET="test_secret"
export RUST_LOG="debug"
```

### 编写测试

- 测试成功和失败情况
- 使用描述性测试名称
- 保持测试专注和独立
- 尽可能模拟外部 API

## Pull Request 流程

### 提交之前

1. 确保所有测试通过
2. 运行 `cargo clippy` 并解决警告
3. 运行 `cargo fmt` 格式化代码
4. 如果需要，更新文档
5. 为重要更改添加更新日志条目

### PR 要求

- 清晰的标题和描述
- 引用相关问题
- 包含新功能的测试
- 更新文档
- 遵循我们的提交消息格式

### 提交消息格式

我们使用约定式提交：

```
type(scope): description

[可选的正文]

[可选的页脚]
```

类型：
- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更改
- `style`: 格式更改
- `refactor`: 代码重构
- `test`: 添加测试
- `chore`: 维护任务

示例：
```
feat(api): 添加对语音消息的支持
fix(websocket): 解决连接超时问题
docs(readme): 更新安装说明
```

### 审查流程

1. 维护者将审查您的 PR
2. 解决反馈和请求的更改
3. 一旦获得批准，您的 PR 将被合并
4. 您的贡献将在发布中得到认可

## 发布流程

### 版本控制

我们遵循[语义化版本控制](https://semver.org/)：

- `MAJOR`: 不兼容的 API 更改
- `MINOR`: 向后兼容的功能
- `PATCH`: 向后兼容的错误修复

### 发布清单

1. 更新 `Cargo.toml` 中的版本
2. 更新 `CHANGELOG.md`
3. 创建发布标签
4. 发布到 crates.io
5. 更新文档

## 社区

### 获取帮助

- [GitHub 讨论](https://github.com/YinMo19/botrs/discussions) 用于问题
- [Discord 服务器](https://discord.gg/botrs) 用于实时聊天
- [GitHub Issues](https://github.com/YinMo19/botrs/issues) 用于错误和功能

### 认可

贡献者在以下方面得到认可：

- 发布说明
- README 中的贡献者部分
- 文档中的名人堂

## 许可证

通过为 BotRS 做出贡献，您同意您的贡献将在 MIT 许可证下获得许可。

## 有问题吗？

如果您对贡献有疑问，请：

1. 检查现有的问题和讨论
2. 在我们的 Discord 服务器中询问
3. 在 GitHub 上开启讨论
4. 直接联系维护者

感谢您为 BotRS 做出贡献！🚀

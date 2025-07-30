# 更新日志

BotRS 的所有重要更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)，
该项目遵循 [语义化版本控制](https://semver.org/spec/v2.0.0.html)。

## [未发布]

### 新增
- 支持英文和中文的全面文档网站
- 所有核心组件的 API 参考文档
- 入门示例和教程
- 配置指南和最佳实践

### 更改
- 文档改进和重构

## [0.2.5] - 2025-07-30

### 新增
- 增加消息参数验证
- 增强 API 响应中的错误上下文
- 支持更多消息附件类型

### 修复
- WebSocket 连接处理中的内存泄漏
- 事件分派中的竞态条件
- 空消息内容的错误处理

### 更改
- 通过更好的重试逻辑提高连接稳定性
- 将依赖项更新到最新版本

## [0.2.0] - 2025-07-29

### 新增
- **新的结构化消息 API**：使用结构化参数完全重新设计消息发送
- 支持构建器模式的频道消息 `MessageParams`
- 群消息 `GroupMessageParams`
- 私聊消息 `C2CMessageParams`
- 私信 `DirectMessageParams`
- 新方法：`post_message_with_params`、`post_group_message_with_params`、`post_c2c_message_with_params`、`post_dms_with_params`
- 全面支持所有 QQ 频道消息类型（文本、嵌入内容、文件、Markdown、键盘、ARK 消息）
- 增强的文件上传功能，具有适当的 MIME 类型检测
- 消息引用和回复功能
- 交互式键盘和按钮支持
- 论坛和话题管理 API

### 更改
- **破坏性变更**：从多个 `None` 参数迁移到结构化参数对象
- 使用构建器模式改进 API 人体工程学（`.with_reply()`、`.with_file_image()` 等）
- 通过编译时参数验证提高类型安全性
- 增强错误消息，提供更多上下文
- 优化消息处理中的内存使用

### 已弃用
- 旧的消息 API 方法（`post_message`、`post_group_message`、`post_c2c_message`、`post_dms`）
- 多个 `None` 参数模式（仍然有效但已弃用）

### 修复
- 不稳定网络条件下的 WebSocket 重连问题
- 特殊字符的消息编码问题
- 长时间运行的机器人实例中的内存泄漏
- 速率限制边缘情况

### 安全
- 改进令牌验证和错误处理
- 更好的用户提供内容输入净化

## [0.1.3] - 2025-07-29

### 新增
- 支持群消息事件（`GROUP_ADD_ROBOT`、`GROUP_DEL_ROBOT`、`GROUP_MSG_RECEIVE`、`GROUP_MSG_REJECT`）
- C2C（客户端到客户端）消息处理（`FRIEND_ADD`、`FRIEND_DEL`、`C2C_MSG_RECEIVE`、`C2C_MSG_REJECT`）
- 音频和直播频道成员管理
- 消息表情回应 API（`PUT /channels/{channel_id}/messages/{message_id}/reactions/{type}`）
- 论坛话题创建和管理
- 定时消息支持
- PIN 消息功能
- 高级权限管理 API

### 更改
- 改进事件处理器 trait，具有更细粒度的事件类型
- 更好的 API 调用错误传播
- 使用结构化输出增强日志记录
- 更新到最新的 QQ 频道 API 规范

### 修复
- 新消息格式的事件解析问题
- 连接稳定性改进
- 内存使用优化

## [0.1.2] - 2025-07-29

### 新增
- 消息审核事件处理（`MESSAGE_AUDIT_PASS`、`MESSAGE_AUDIT_REJECT`）
- 增强的频道成员事件支持
- 更好的 WebSocket 错误恢复
- API 调用的可配置重试机制

### 更改
- 通过更多示例改进文档
- 具有更具体错误信息的更好错误类型
- 增强高吞吐量场景的性能

### 修复
- 消息内容中特殊字符的问题
- 某些网络条件下的 WebSocket 连接断开
- 事件处理中的内存泄漏

## [0.1.1] - 2025-07-29

### 新增
- 基本消息撤回功能
- 增强的文件上传支持，具有进度跟踪
- 与 `tracing` crate 更好的日志集成

### 修复
- 嵌入内容消息解析中的关键错误
- 机器人用户识别问题
- WebSocket 心跳时间问题

### 更改
- 改进 API 响应解析
- 更好的速率限制处理

## [0.1.0] - 2025-07-29

### 新增
- BotRS 初始发布
- 核心 WebSocket 网关连接处理
- 基本消息发送和接收
- 使用 `EventHandler` trait 的事件驱动架构
- 支持频道消息、私信和系统事件
- 用于事件过滤的 Intent 系统
- 内置速率限制和重试逻辑
- 使用 `BotError` 类型的全面错误处理
- 与 Tokio 异步运行时集成
- 支持嵌入内容、文件和富文本消息内容
- 频道和子频道管理 API
- 成员和角色管理
- 基本身份验证和令牌管理

### 核心功能
- `Client` - 具有 WebSocket 管理的主要机器人客户端
- `EventHandler` - 处理各种机器人事件的 trait
- `BotApi` - QQ 频道端点的 REST API 客户端
- `Token` - 身份验证和凭据管理
- `Intents` - 事件订阅配置
- 消息类型：`Message`、`DirectMessage`、`GroupMessage`
- 频道类型：`Guild`、`Channel`、`Member`、`Role`
- 全面的错误处理和日志记录

### 支持的事件
- `READY` - 机器人连接建立
- `GUILD_CREATE`、`GUILD_UPDATE`、`GUILD_DELETE` - 频道生命周期
- `CHANNEL_CREATE`、`CHANNEL_UPDATE`、`CHANNEL_DELETE` - 子频道管理
- `GUILD_MEMBER_ADD`、`GUILD_MEMBER_UPDATE`、`GUILD_MEMBER_REMOVE` - 成员事件
- `AT_MESSAGE_CREATE` - 消息提及
- `DIRECT_MESSAGE_CREATE` - 私人消息
- `MESSAGE_DELETE` - 消息删除

## 迁移指南

### 从 0.1.x 迁移到 0.2.x

v0.2.0 的主要变化是引入了结构化消息参数。以下是迁移方法：

#### 旧 API（已弃用）
```rust
// 多个 None 参数 - 令人困惑且容易出错
api.post_message(
    token, "channel_id", Some("你好！"),
    None, None, None, None, None, None, None, None, None
).await?;
```

#### 新 API（推荐）
```rust
use botrs::models::message::MessageParams;

// 清洁、可读、类型安全
let params = MessageParams::new_text("你好！")
    .with_reply("message_id")
    .with_markdown(true);
api.post_message_with_params(token, "channel_id", params).await?;
```

#### 方法映射
- `post_message` → `post_message_with_params`
- `post_group_message` → `post_group_message_with_params`
- `post_c2c_message` → `post_c2c_message_with_params`
- `post_dms` → `post_dms_with_params`

### 0.2.0 中的破坏性变更

1. **消息 API 结构**：参数对象替换位置参数
2. **导入路径**：一些消息类型移动到 `botrs::models::message`
3. **构建器模式**：用于参数构造的新 `.with_*()` 方法
4. **默认值**：使用 `..Default::default()` 而不是多个 `None`

## 安全公告

### RUSTSEC-2023-0001（在 0.1.2 中解决）
- **问题**：WebSocket 连接处理中的潜在内存泄漏
- **影响**：长时间运行的机器人可能会遇到内存使用增加
- **解决方案**：修复事件循环中的连接清理
- **影响版本**：0.1.0、0.1.1
- **修复版本**：0.1.2+

## 弃用通知

### v0.1.x 消息 API（在 0.2.0 中弃用）
具有多个 `None` 参数的旧消息 API 已弃用，将在 v0.3.0 中删除。请迁移到新的结构化参数 API。

```rust
// ❌ 已弃用 - 将在 v0.3.0 中删除
api.post_message(token, channel, Some(content), None, None, None, None, None, None, None, None, None).await?;

// ✅ 新 API - 推荐
let params = MessageParams::new_text(content);
api.post_message_with_params(token, channel, params).await?;
```

## 版本支持

| 版本 | 状态 | 生命周期结束 |
|------|------|-------------|
| 0.2.x | ✅ 活跃 | 待定 |
| 0.1.x | ⚠️ 仅安全修复 | 2024-06-01 |

## 贡献

有关对 BotRS 做出贡献的指南，请参阅 [CONTRIBUTING.md](contributing.md)。

## 链接

- [仓库](https://github.com/YinMo19/botrs)
- [文档](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [问题](https://github.com/YinMo19/botrs/issues)

# 更新日志

本页概述当前文档对应的 BotRS 版本线。主 API 表面保持较小，围绕 examples 和 gateway 事件处理实际使用的运行路径组织。

## [0.14.0] - 当前版本

- WebSocket 和签名 Webhook 共用类型化事件。处理器可以返回 Result，并通过问号操作符传播错误。
- 消息会话使用平台提供的回复 ID，兼容 QQ 的最小消息载荷。REST 错误保留 HTTP 状态、QQ 错误码、trace ID 和响应体。
- 最低 Rust 版本为 1.88。频道序列号变为可选值，群和 C2C 使用类型化的 Keyboard。

## [0.13.0]

### 当前能力

- `Client` 负责网关生命周期，并把类型化事件分发给 `EventHandler`。
- 事件回调收到 session 对象；session 暴露共享 `BotApi`，并提供当前事件的回复辅助方法。
- `BotApi` 覆盖 bot 信息、网关发现、消息发送/撤回、群/C2C 文件上传、频道资源、身份组、禁言、子频道权限、公告、日程、API 权限、表情回应、精华消息和音频控制。
- 消息发送在常见回复场景使用 session helper，底层调用使用 `MessageParams`、`GroupMessageParams`、`C2CMessageParams` 和 `DirectMessageParams` 构造器。
- 网关事件会解析成消息、私信、群/C2C、表情回应、互动、频道、子频道、成员、审核、管理、音频和论坛等类型化 payload。

### 说明

- 公开文档聚焦当前 API 能构建什么。
- `examples/` 目录下的示例是端到端使用方式的最佳入口。

## 链接

- [仓库](https://github.com/YinMo19/botrs)
- [文档](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [问题](https://github.com/YinMo19/botrs/issues)

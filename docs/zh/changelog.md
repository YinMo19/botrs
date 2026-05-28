# 更新日志

本页概述当前文档对应的 BotRS 版本线。主 API 表面保持较小，围绕 examples 和 gateway 事件处理实际使用的运行路径组织。

## [0.13.0] - 当前版本

### 当前能力

- `Client` 负责网关生命周期，并把类型化事件分发给 `EventHandler`。
- `Context` 会解引用到共享的 `BotApi`，handler 代码可以直接调用 REST 方法。
- `BotApi` 覆盖 bot 信息、网关发现、消息发送、消息撤回、群/C2C 文件上传、公告、日程、API 权限、表情回应和精华消息。
- 消息发送使用 `MessageParams`、`GroupMessageParams`、`C2CMessageParams` 和 `DirectMessageParams`。
- 网关事件会解析成消息、私信、群/C2C、表情回应、互动、频道、子频道、成员、审核、管理、音频和论坛等类型化 payload。

### 说明

- `Cargo.toml` 中的 crate 版本是 `0.13.0`。
- 公开文档聚焦当前 API 能构建什么。
- `examples/` 目录下的示例是端到端使用方式的最佳入口。

## 链接

- [仓库](https://github.com/YinMo19/botrs)
- [文档](https://docs.rs/botrs)
- [Crates.io](https://crates.io/crates/botrs)
- [问题](https://github.com/YinMo19/botrs/issues)

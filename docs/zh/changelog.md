# 更新日志

BotRS 的所有重要更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)，
该项目遵循 [语义化版本控制](https://semver.org/spec/v2.0.0.html)。

## [未发布]

### 移除
- 移除 `src/api/compat` 下的 `BotApi` PascalCase 门面（`PostMessage`、`Channels`、`PostAudio` 等）。请使用原生 snake_case `BotApi` 方法。
- 移除顶层 Go 风格 OpenAPI 兼容模块（`facade`、`openapi`、`options`）以及只服务于该门面的 HTTP filter 注册表。
- 移除不属于 REST 或网关 wire 行为的独立 Botgo helper 门面（`log`、`remote`、`search`、`version`）。
- 移除 Go 风格错误与 session helper 名称（`Err`、`New`、`Error`、`CanNotResume`、`CheckSessionLimit` 等），改用原生 Rust 命名。
- 移除未被运行路径使用的 Go 风格 `websocket` 门面；网关处理请使用 Rust 原生的 `Client`、`Gateway` 和 `session_manager`。
- 移除 Go 风格 event、webhook 和 signature helper 名称（`ParseData`、`RegisterHandlers`、`HTTPHandler`、`Gen*ACK`、`Generate`、`Verify` 等），改用原生 Rust 命名。
- 移除 Go 风格 token/API/constant helper 名称（`NewQQBotTokenSource`、`GetAppID`、`StartRefreshAccessToken`、`APIv1`、`HeaderTraceID` 等），改用原生 Rust 命名。
- 移除 Go 风格 intent 常量（`IntentGuilds`、`IntentGroupMessages`、`IntentNone` 等）；请使用 `Intents::GUILDS`、`Intents::PUBLIC_MESSAGES` 或 `0`。
- 移除 Go 风格 enum 常量别名（`AudioStatusStart`、`InteractionTypePing`、`LayoutTypeImageText` 等）；请使用 enum variant 或 Rust 风格常量。
- 移除 Go 风格 pager、成员删除 option 和消息辅助名称（`QueryParams`、`WithAddBlackList`、`GetEventID`、`GetSendType`、`APIMessage` 等），改用原生 Rust 方法。
- 移除 Go 风格消息 mention 辅助名称（`MentionUser`、`MentionAllUser`、`MentionChannel`、`Emoji`、`ETLInput`、`ParseCommand`）；请使用 `mention_user`、`mention_all_user`、`mention_channel`、`emoji`、`etl_input` 和 `parse_command`。
- 移除 Go 风格消息 payload 与 keyboard 常量别名（`TextMsg`、`MarkdownMsg`、`RichMedia`、`ActionTypeURL`、`PermissionTypAll` 等）；请使用 `MessageCreateType`、`SendType` 以及 Rust 风格 `ACTION_TYPE_*` / `PERMISSION_TYPE_*` 常量。
- 移除 Go 风格和 botpy 风格消息内容类型别名（`ArkKV`、`ArkObjKV`、`MessageKeyboard`、`CustomKeyboard`、`TemplateID`、`MarkdownParams` 等）；请使用具体 Rust 类型，例如 `ArkKv`、`Keyboard`、`KeyboardContent`、`KeyboardTemplateId` 和 `MarkdownParam`。
- 将 Go 风格错误码常量（`CodeNeedReConnect`、`WSCodeBackendAuthenticationFail`、`APICodeTokenExpireOrNotExist` 等）改名为 Rust 风格 `CODE_*`、`WS_CODE_*` 和 `API_CODE_*`。
- 移除 Go 风格 channel enum 别名和重复 enum-value 常量（`ChannelTypeText`、`ChannelSubTypeChat`、`ChannelPrivateTypePublic`、`SpeakPermissionTypePublic`、`CHANNEL_TYPE_TEXT` 等）；请使用 `ChannelType::Text` 这类枚举 variant。
- 移除 Go 风格 gateway opcode/event 别名（`OPCode`、`WSDispatchEvent`、`WSIdentity`、`HTTPCallbackAck`、`EventMessageCreate`、`OPMeans` 等）；请使用 `OpCode`、`WS_DISPATCH_EVENT`、`WS_IDENTIFY`、`HTTP_CALLBACK_ACK`、`EVENT_MESSAGE_CREATE` 和 `op_meaning`。
- 移除冗余 Rust 方法别名（`BotApi::me`、`BotApi::me_guilds`、`BotApi::get_ws_url`、`BotApi::get_permissions`、`BotApi::patch_message` 及对应 `Context` wrapper）；请使用 `get_bot_info`、`get_guilds`、`get_gateway`、`get_api_permissions` 和 `edit_message`。
- 移除 botpy 风格重复方法（`BotApi::create_dms`、`Context::create_dms`、`BotApi::get_delete_member` 和 `Context::get_delete_member`）；请使用 `create_direct_message` 和 `delete_member`。
- 移除冗余 `Context` 方法别名（`add_reaction`、`remove_reaction`、`pin_message`、`unpin_message`、`add_guild_role_member` 和 `remove_guild_role_member`）；请使用 `put_reaction`、`delete_reaction`、`put_pin`、`delete_pin`、`create_guild_role_member` 和 `delete_guild_role_member`。
- 移除兼容类型别名（`WebsocketAP`、`DirectMessageSession`、`ReactionUser`、`MessageReactionUsers`、`Announces`、`EnterAIO`、`HTTPIdentity`、`HTTPReady`、`HTTPSession`、`WHValidationReq`、`WHValidationRsp`、`RoleID` 和 `Roles`）以及 Go 风格的 `SessionManager::Start` / `DefaultColor` 别名；请使用具体 Rust 类型和 `SessionManager::start` / `DEFAULT_ROLE_COLOR`。
- 移除已弃用的多 `Option` 消息发送方法（`post_message`、`post_group_message`、`post_c2c_message`、`post_dms`）、`*_with_params` 兼容名及其 `Context` wrapper。请使用 `send_message`、`send_group_message`、`send_c2c_message` 和 `send_direct_message`。
- 移除冗余的 `post_message_api` 和 `patch_message_api` 别名。

## [0.11.0] - 2026-05-25

### 更改
- 继续对齐 QQ 机器人开放接口的 user、guild/member/role、mute、manage event、gateway/webhook、interaction search、message audit 和 message setting DTO。
- 更新相关 DTO 的零值和 `omitempty` JSON wire shape，与协议规范保持一致。

### 修复
- 本地 gateway/event 辅助上下文字段不再进入纯 payload DTO 的 JSON。

## [0.10.0] - 2026-05-25

### 更改
- 对齐 channel、schedule、audio、announce 和 API permission DTO 的零值 wire shape，与 QQ 机器人开放接口保持一致。
- `PostAudio`、`post_audio` 和 `Context::post_audio` 现在使用 `AudioControl`；音频事件继续使用 `AudioAction`。
- `AudioStatus` 现在按协议定义的数字状态值进行序列化和反序列化。
- 网关文档改为描述固定重连节流策略，不再描述指数 backoff。

### 修复
- `ListSchedules` 现在总是发送 `since` 查询参数，与协议规定的行为一致。

## [0.9.0] - 2026-05-25

### 更改
- `MessageReaction` 和 `WSMessageReactionData` 对齐为 QQ 机器人开放接口的纯消息表情表态 DTO。
- reaction 的 `Emoji`、`ReactionTarget` 和 `MessageReaction` 字段改为必填，不再对缺失协议字段静默填空。
- 论坛 `ThreadInfo.title` 和 `ThreadInfo.content` 对齐 QQ 机器人开放接口，保留原始字符串。
- GitHub Actions 升级到当前主版本，规避 Node 20 action deprecation 风险。

### 修复
- 表情表态事件解析遇到畸形 payload 时会向外返回错误，不再构造空字段的半成品事件。

## [0.8.0] - 2026-05-25

### 更改
- `DirectMessage` 对齐为 QQ 机器人开放接口的私信会话 DTO。
- `direct_message_create` 现在接收普通 `Message`,与协议定义的 `WSDirectMessageData` 保持一致。
- 私信示例和 API 文档改为使用 `DirectMessageParams` 与 `send_direct_message`。

### 修复
- 补齐 message、guild 和 interaction wire format 的 DTO 对齐，与 QQ 机器人开放接口一致。
- 消息辅助函数的 mention 处理和命令切分行为与协议规定保持一致。

## [0.7.0] - 2026-05-25

### 新增
- 增加与 QQ 机器人开放接口对齐的 OpenAPI v1 具体实现常量 `HeaderCallbackAppID` 和 `MaxIdleConns`。
- 增加 `Session::from_app_id`，用于 HTTP callback payload session。

### 更改
- OpenAPI 实例现在会保存 app ID 状态，并在请求中发送 `X-Union-Appid`。
- `PutInteraction` 现在使用 OpenAPI 实例 app ID 设置 `X-Callback-AppID`，与协议规范对齐。
- HTTP webhook dispatch payload 现在会带上包含 app ID 的 session。

### 修复
- 网关返回不可 resume 错误后，重新入队前会清理过期 session ID 和 sequence。
- 兼容单条消息接口的旧包装响应格式（`{"message": ...}`）。

## [0.6.0] - 2026-05-25

### 新增
- 对齐 QQ 机器人开放接口的 `WSPayload.session`、`C2CFriendData`、`WSC2CFriendData`、消息辅助接口、分页辅助接口和 OpenAPI facade 名称。
- 增加 `APIMessage`、`GetEventID`、`GetSendType`、`QueryParams`、OpenAPI 分组别名、`openapi::Register` 和 `DefaultImpl` 兼容入口。

### 更改
- WebSocket 事件 payload DTO 和 C2C 好友事件改为更贴近 QQ 机器人开放接口的纯数据模型形态。
- 保留 Rust 风格小写 helper，同时增加大驼峰风格导出名称，便于迁移。

## [0.5.0] - 2026-05-25

### 新增
- 对齐 QQ 机器人开放接口的 OpenAPI facade、版本注册表、请求选项、HTTP filter 和 token 后台刷新辅助接口。

### 更改
- OpenAPI 成功状态码处理与 QQ 机器人开放接口对齐，仅 `200` 和 `204` 视为成功。
- 克隆后的 token source 共享 token 缓存，并移除 unsafe 状态写入。
- 刷新 lockfile，避免已 yanked 的传递依赖。

### 修复
- WebSocket 重连成功后重置 backoff，避免后续一直使用 40s 间隔。
- 隔离 OpenAPI registry 测试，避免并行测试随机失败。

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
- 新方法：`send_message`、`send_group_message`、`send_c2c_message`、`send_direct_message`
- 全面支持所有 QQ 频道消息类型（文本、嵌入内容、文件、Markdown、键盘、ARK 消息）
- 增强的文件上传功能，具有适当的 MIME 类型检测
- 消息引用和回复功能
- 交互式键盘和按钮支持
- 论坛和话题管理 API

### 更改
- **破坏性变更**：从多个 `None` 参数迁移到结构化参数对象
- 使用构建器模式改进 API 人体工程学，例如 `.with_reply()`
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
api.send_message("channel_id", params).await?;
```

#### 方法映射
- `post_message` → `send_message`
- `post_group_message` → `send_group_message`
- `post_c2c_message` → `send_c2c_message`
- `post_dms` → `send_direct_message`

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

## 已移除 API

### v0.1.x 消息 API
具有多个 `None` 参数的旧消息 API 已移除。请使用新的结构化参数 API。

```rust
let params = MessageParams::new_text(content);
api.send_message(channel, params).await?;
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

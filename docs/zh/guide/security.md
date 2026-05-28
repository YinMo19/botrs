# 安全

安全边界主要集中在 token、运行环境和日志。bot 的业务代码通常运行在 gateway 事件回调里，再通过事件 session 发起 REST 调用。

## Token

`Token` 持有 `app_id` 和 `secret`。不要把 secret 写进仓库、日志或 panic 信息里。需要打印时使用 `safe_display()`：

```rust
tracing::info!("starting with {}", token.safe_display());
```

推荐用 `Token::from_env()` 从环境变量加载：

- `QQ_BOT_APP_ID`
- `QQ_BOT_SECRET`

`Token::validate()` 只检查字段非空，不会向平台发请求验证真伪。真正的认证结果来自 REST 调用或 gateway identify。

## 沙箱与正式环境

`Client::new(token, intents, handler, is_sandbox)` 的最后一个参数决定使用沙箱还是生产环境。开发和验证阶段建议用 sandbox，确认事件、权限和消息发送逻辑都正常后再切到生产。

独立构造 `HttpClient` 时同样要传入 sandbox 标志：

```rust
let http = HttpClient::new(30, true)?;
```

## 日志

网关和 HTTP 层会通过 `tracing` 输出调试信息。生产环境不要开启过宽的 debug 日志并把请求体原样收集到外部系统，尤其是包含用户消息内容、openid 或 token 相关错误的日志。

## 参见

- [Token](../api/token.md)
- [配置](./configuration.md)
- [网关](./gateway.md)

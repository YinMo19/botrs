# 错误处理

`botrs` 中所有可能失败的函数都返回 `Result<T, BotError>`。`BotError` 是一个 `thiserror` 枚举，通常只需要匹配少数几个变体，其余用 `?` 透传。

## 实际会遇到的变体

最常匹配的是 QQ API 与网关产生的几种结果：

- `BotError::Api { code, message }` —— 任意 REST 调用被 QQ 拒绝。`code` 为平台错误码，`message` 为可读原因。
- `BotError::AuthenticationFailed(String)` —— HTTP 401，`Token` 错误或被吊销。
- `BotError::Forbidden(String)` —— HTTP 403，机器人无权执行。
- `BotError::NotFound(String)` —— HTTP 404，目标资源（频道、消息、成员）不存在。
- `BotError::MethodNotAllowed(String)` —— HTTP 405，端点或方法错误。
- `BotError::Server(String)` —— HTTP 500 / 504，并带有解析后的平台诊断信息。是否可重试需要看其中的 code/message。
- `BotError::SequenceNumber(String)` —— 低层状态码映射中保留的 sequence/order 错误；普通 REST 429 会作为 `RateLimit` 返回。
- `BotError::RateLimit { retry_after }` —— 显式限流响应，`retry_after` 单位为秒。
- `BotError::Auth(String)` / `BotError::Config(String)` / `BotError::InvalidData(String)` —— 本地校验失败（错误的 app id、错误的环境变量、无法序列化的载荷）。
- `BotError::Connection(String)` / `BotError::Gateway(String)` / `BotError::Session(String)` —— 网关生命周期失败。
- `BotError::Timeout` —— 请求未在 `HttpClient` 配置的超时内完成。
- `BotError::Http(reqwest::Error)` / `BotError::WebSocket(...)` / `BotError::Json(...)` / `BotError::Url(...)` / `BotError::Io(...)` —— 传输层包装，`Display` 输出已足够诊断。
- `BotError::Sdk(SdkError)` —— 内部 SDK 错误（携带数值 code 与 trace ID），用于网关状态机；用于诊断后透传即可。

## 错误如何抵达你

REST 调用（`BotApi::*`）直接返回错误。处理器内部由你决定如何处理：

```rust
async fn message_create(&self, mut session: ChannelReplySession) {
    let params = MessageParams::new_text("你好");
    if let Err(e) = session.send_message(params).await {
        match e {
            BotError::RateLimit { retry_after } => {
                tracing::warn!(retry_after, "限流，放弃回复");
            }
            BotError::Forbidden(_) => {
                tracing::warn!("缺少该频道权限");
            }
            other => tracing::error!(error = %other, "发送失败"),
        }
    }
}
```

在调用你的 handler 之前，如果分派准备阶段失败，框架会调用一次 `EventHandler::error(&self, error: BotError)` 并继续消费后续事件。payload 解析失败会被记录并丢弃；handler panic 不会被转换成 `BotError`。需要接入指标或告警时可以重写 `error`，但框架不会替你重试原事件。

## 重试辅助方法

`BotError::is_retryable(&self)` 在以下变体上返回 `true`：`Http`（仅 timeout 与 connect 错误）、`WebSocket`、`Connection`、`Gateway`、`Timeout`、`RateLimit`。`BotError::retry_after(&self)` 给出建议等待秒数（`RateLimit` 即 `retry_after`、`Connection` 为 5、`Gateway` 为 1、`Timeout` 为 3、其余可重试为 1，否则 `None`）。

这两者只是建议。框架本身不会替你重试 —— 每个 `BotApi` 调用首次失败就立即返回，网关重连由 `Gateway::session_start_interval` 节流，并不参考 `retry_after`。需要在某次调用上加重试，请自行编写循环并参考 `is_retryable` / `retry_after`：

```rust
loop {
    match session.send_message(params.clone()).await {
        Ok(resp) => break Ok(resp),
        Err(e) if e.is_retryable() => {
            if let Some(secs) = e.retry_after() {
                tokio::time::sleep(Duration::from_secs(secs)).await;
                continue;
            }
            break Err(e);
        }
        Err(e) => break Err(e),
    }
}
```

请控制循环上限，否则处理器中的死循环会阻塞分派任务。

## 状态码映射

HTTP 层会先把状态码映射为 `BotError` 变体，再从 `BotApi` 调用返回：

| 状态码      | 变体                           |
|-------------|--------------------------------|
| 401         | `AuthenticationFailed`        |
| 403         | `Forbidden`                   |
| 404         | `NotFound`                    |
| 405         | `MethodNotAllowed`            |
| 429         | `RateLimit`                   |
| 500 / 504   | `Server`                      |
| 其他        | `Api { code: status, message }` |

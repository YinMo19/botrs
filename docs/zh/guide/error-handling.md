# 错误处理

`botrs` 中所有可能失败的函数都返回 `Result<T, BotError>`。`BotError` 是一个 `thiserror` 枚举，通常只需要匹配少数几个变体，其余用 `?` 透传。

## 实际会遇到的变体

最常匹配的是 QQ API 与网关产生的几种结果：

- `BotError::Api { code, message }` —— 任意 REST 调用被 QQ 拒绝。`code` 为平台错误码，`message` 为可读原因。
- `BotError::AuthenticationFailed(String)` —— HTTP 401，`Token` 错误或被吊销。
- `BotError::Forbidden(String)` —— HTTP 403，机器人无权执行。
- `BotError::NotFound(String)` —— HTTP 404，目标资源（频道、消息、成员）不存在。
- `BotError::MethodNotAllowed(String)` —— HTTP 405，端点或方法错误。
- `BotError::Server(String)` —— HTTP 500 / 504，QQ 侧故障，可重试。
- `BotError::SequenceNumber(String)` —— HTTP 429 的 sequence 错误，与下方的 `RateLimit` 不同，通常是消息顺序而非限流。
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

事件分派本身失败时（载荷异常、处理器内部错误等），框架会调用一次 `EventHandler::error(&self, error: BotError)` 并继续消费后续事件。默认实现使用 `error!` 记录日志。需要接入指标、告警或自定义 panic 处理时可以重写它 —— 但请注意框架不会替你重试原事件。

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
| 429         | `SequenceNumber`              |
| 500 / 504   | `Server`                      |
| 其他        | `Api { code: status, message }` |

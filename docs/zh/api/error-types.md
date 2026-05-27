# 错误类型

BotRS 中所有可能失败的调用都返回 `Result<T, BotError>`。`BotError` 是一个 `enum`，覆盖传输、协议、本地校验等失败场景，并通过 `Sdk` 变体兜底承载所有 QQ 侧返回的错误码。

```rust
pub type Result<T> = std::result::Result<T, BotError>;
```

`BotError` 由 `thiserror::Error` 派生，自动实现了 `std::error::Error`，并提供可读的 `Display` 字符串。

## 变体一览

| 变体                                          | 来源                                                    |
|-----------------------------------------------|---------------------------------------------------------|
| `Sdk(SdkError)`                               | QQ 定义的错误码，可能附带响应体。                       |
| `Http(reqwest::Error)`                        | HTTP 客户端的传输 / TLS / DNS / 超时错误。              |
| `WebSocket(Box<tungstenite::Error>)`          | 网关 WebSocket 失败。                                   |
| `Json(serde_json::Error)`                     | 载荷反序列化失败。                                      |
| `Url(url::ParseError)`                        | 构造请求 URL 失败。                                     |
| `Api { code: u32, message: String }`          | 非 2xx 响应及解析后的响应体。                           |
| `AuthenticationFailed(String)`                | 令牌签名或 `Authorization` 头被拒绝。                   |
| `NotFound(String)`                            | 404 / 资源不存在。                                      |
| `MethodNotAllowed(String)`                    | 405。                                                   |
| `Forbidden(String)`                           | 403 / 权限不足。                                        |
| `RateLimit { retry_after: u64 }`              | 429，附带解析后的 `Retry-After`（秒）。                 |
| `SequenceNumber(String)`                      | 网关序号偏移，需要 resume 或重连。                      |
| `Server(String)`                              | 5xx 响应，且未携带结构化错误码。                        |
| `Auth(String)`                                | 更高层级的鉴权 / 会话问题。                             |
| `Connection(String)`                          | 网关生命周期或心跳失败。                                |
| `Config(String)`                              | 请求开始之前的配置错误。                                |
| `InvalidData(String)`                         | 本地校验（例如非法的权限串）失败。                      |
| `Timeout`                                     | 内部各阶段抛出的网络超时。                              |
| `Gateway(String)`                             | 其他不属于上述任一变体的网关协议错误。                  |
| `Session(String)`                             | Webhook 会话状态机错误。                                |
| `Internal(String)`                            | 框架自身的不变量被破坏（视为 bug）。                    |
| `Io(std::io::Error)`                          | 本地 I/O，例如文件上传。                                |
| `NotImplemented(String)`                      | 接口可达，但尚未在框架里实现。                          |

`Sdk` 包装的是框架内部的错误码载体 `SdkError`，其方法包括 `code()`、`message()`、`trace_id()`。框架将 QQ 定义的所有数值错误码（例如 9999、`CodeNeedReConnect`、`CodeConnCloseCantResume` 等）经 `Sdk` 派发。

## 构造辅助

`BotError` 上提供了一组短构造器，便于在框架内部构建错误：

- `BotError::invalid_data(msg)` —— 快捷构造 `InvalidData`。
- `BotError::auth(msg)` / `BotError::config(msg)` / `BotError::internal(msg)` —— 各自对应的变体。

## 模式匹配

```rust
use botrs::BotError;

match api.get_guild(&token, "guild_id").await {
    Ok(guild) => println!("{}", guild.name),
    Err(BotError::Http(e)) if e.is_timeout() => warn!("timed out"),
    Err(BotError::RateLimit { retry_after }) => {
        tokio::time::sleep(Duration::from_secs(retry_after)).await;
    }
    Err(BotError::Forbidden(msg)) => warn!("permission denied: {msg}"),
    Err(BotError::Api { code, message }) => warn!("qq api error {code}: {message}"),
    Err(other) => return Err(other),
}
```

需要根据协议错误码做分支时：

```rust
if let BotError::Sdk(ref err) = e {
    match err.code() {
        CODE_NEED_RECONNECT => /* 关闭会话并重连 */,
        CODE_CONN_CLOSE_CANT_RESUME => /* 抛弃会话，重新 identify */,
        _ => {}
    }
}
```

这些常量定义在 `crate::constant` 模块。

## 自动转换

`BotError` 为常用的内部错误类型实现了 `From`（`reqwest::Error`、`serde_json::Error`、`url::ParseError`、`std::io::Error`），因此 `async fn -> Result<...>` 中可以直接使用 `?`。

## 参见

- [Bot API](./bot-api.md) —— 可能产生这些错误的全部接口。
- [Token](./token.md) —— 凭证刷新逻辑，会触发 `Auth` 错误。
- [网关指南](../guide/gateway.md) —— 框架如何对 `Connection` / `SequenceNumber` 错误做出反应。

# 安全

框架在安全方面只关心三件事：不要把 `Token` 泄漏到日志或仓库里、收 Webhook 回调前先校验签名、按场景选对环境（沙箱／正式）。其余通用运维加固由你的部署体系负责，框架不参与。

## Token 管理

`Token` 由 `app_id` 与 `secret` 组成。不要打印或序列化原始字段 —— `Token` 的 `Debug` 实现已脱敏，但 format 字符串、JSON 序列化、panic 栈仍可能泄漏。需要在日志中展示时使用 `safe_display()`：

```rust
tracing::info!("starting with {}", token.safe_display());
// "Token { app_id: 1234, secret: 1234****abcd }"
```

推荐使用 `Token::from_env()` 加载，它读取 `QQ_BOT_APP_ID` 与 `QQ_BOT_SECRET` 并校验非空（否则返回 `BotError::Config`）。永远不要把真实凭据提交到仓库，使用环境变量、密钥管理服务或工作树外的文件。

`Token::validate()` 仅检查非空，不会向 QQ 验证凭据是否有效；后者由网关／REST 接口决定，`client.start()` 成功才是真正的可用信号。

## Webhook 签名校验

若你选择走 QQ 的 HTTP 回调（而非网关连接），每条分派都会带 `X-Signature-Ed25519` 与 `X-Signature-Timestamp`。框架在 `botrs::signature` 提供校验辅助：

```rust
use botrs::signature::{HEADER_SIGNATURE, HEADER_TIMESTAMP, verify};

fn handle(headers: &reqwest::header::HeaderMap, body: &[u8], bot_secret: &str) -> botrs::Result<()> {
    if !verify(bot_secret, headers, body)? {
        return Err(botrs::BotError::auth("bad signature"));
    }
    // 签名通过；继续反序列化并分派
    Ok(())
}
```

`verify` 会从 header 中读取 `X-Signature-Ed25519`（hex 编码签名）与 `X-Signature-Timestamp`，按 QQ 互动回调文档描述的方案从机器人 secret 推导签名密钥，仅在签名通过时返回 `Ok(true)`。`generate(secret, headers, body)` 是其反向函数，常用于测试或你自行实现签名时。`HEADER_SIGNATURE` 与 `HEADER_TIMESTAMP` 常量也已导出，避免硬编码。

校验失败的请求必须立即拒绝。一旦放行未签名或签名错误的载荷，攻击者就能伪造事件触达你的处理器。

## 沙箱与正式

`Client::new(token, intents, handler, is_sandbox)` 通过布尔参数选择基础地址。开发期使用沙箱：网关与 REST 端点接受同一个 `Token`，但运行在面向测试的隔离环境，速率限制更宽松。准备上线时再把 `is_sandbox` 设为 `false`。

沙箱地址常量：`botrs::SANDBOX_API_URL`（`https://sandbox.api.sgroup.qq.com`），正式地址：`botrs::DEFAULT_API_URL`（`https://api.sgroup.qq.com`）。

## 本指南不涉及的内容

通用运维加固（TLS 终止、机密轮换频率、入侵检测、限流中间件、审计日志、漏洞扫描等）不在框架职责内 —— 这些应在部署体系里解决，沿用你团队既有的方案即可。

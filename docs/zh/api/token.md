# Token

`Token` 持有机器人的 `app_id` 与 `secret`，负责签名 API 请求，并把访问令牌缓存到所有 `Token` 克隆共享的状态里。缓存按需刷新，受内部 mutex 保护，因此 `Token` 在多任务之间共享非常安全且开销很小。

```rust
pub struct Token { /* 私有字段 */ }
```

`Token` 实现了 `Clone`、`Debug`、`Serialize`、`Deserialize`。序列化时缓存会被丢弃，便于在配置文件里往返携带凭证。

## 构造

- `Token::new(app_id, secret)` —— 显式凭证。
- `Token::from_env()` —— 读取 `QQ_BOT_APP_ID` / `QQ_BOT_SECRET`。变量缺失时返回 `BotError::Config`；变量存在但为空时会在校验阶段返回 `BotError::Auth`。

```rust
let token = Token::new("123456789", "abcdef...");
let token = Token::from_env()?;       // 走环境变量
```

## 访问器

- `app_id()` / `secret()` —— 借用凭证字符串（`&str`）。
- `safe_display()` —— 返回适合日志的脱敏字符串（如 `Token { app_id: 123456789, secret: abcd****wxyz }`），任何想打印 token 的地方都应改用这个。

## 鉴权

- `authorization_header().await` —— 构造适合 HTTP `Authorization` 头使用的 `QQBot <access-token>` 值，过期时会自动刷新。
- `bot_token().await` —— 返回同一个 `QQBot <access-token>` 值，由网关 `Identify` 载荷使用。

两者共用同一份访问令牌缓存。并发调用方在需要刷新时会短暂地等待 refresh mutex，然后无竞争地读到最新值。

## 校验

`validate()` 检查 app id 与 secret 是否非空，失败时返回 `BotError::Auth`。建议在 `Token::new(...)` 之后立即调用，启动阶段就把凭证错误暴露出来。

## 刷新机制

框架会监控访问令牌的有效期，在到期前自动续期。如果续期失败，下一个调用方会收到 `BotError::Auth` 并附带上游错误信息。`Token` 没有公开的 `refresh()` 方法——刷新由 `authorization_header()` / `bot_token()` 自己驱动。

由于克隆共享同一份 `Arc<Mutex<TokenState>>`，任何一个克隆触发的刷新都会立刻对其他克隆可见，即便在高并发下，一个有效期窗口内也只刷新一次。

## 示例

**从环境变量构造并在启动时校验一次：**

```rust
let token = Token::from_env()?;
token.validate()?;
```

**手工构造 HTTP 请求：**

```rust
let auth = token.authorization_header().await?;
let request = client
    .get("https://api.example.com/whatever")
    .header("Authorization", auth)
    .send()
    .await?;
```

**安全打印 token：**

```rust
info!("starting bot {}", token.safe_display());
```

## 安全建议

- 永远不要 `Debug`-print `Token`，请走 `safe_display()`。
- 凭证优先来自环境变量或秘钥管理服务，不要提交到代码仓库的配置文件里。
- 若怀疑密钥泄漏，先在 QQ 开放平台轮换密钥；轮换后下一次 `authorization_header()` 会返回 `BotError::Auth`，再用新密钥构造新的 `Token` 即可。

## 参见

- [Bot API](./bot-api.md) —— 所有接口最终都会消费 `Token`。
- [Sessions](./context.md) —— 暴露持有 token 的 `BotApi`，回调里无需手动传 token。
- [错误类型](./error-types.md) —— 上文涉及的 `BotError::Auth` 与 `BotError::Config` 行为。

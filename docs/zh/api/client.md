# 客户端 API 参考

`Client` 是创建和管理 QQ 频道机器人的主要入口点。它处理 WebSocket 连接、身份验证，并将事件分派给您的事件处理器。

## 概述

```rust
use botrs::{Client, EventHandler, Intents, Token};

pub struct Client<H: EventHandler> {
    // 内部字段...
}
```

`Client` 管理：
- 到 QQ 频道网关的 WebSocket 连接
- 与 QQ 服务器的身份验证
- 事件分派到您的 `EventHandler`
- 自动重连和心跳处理
- 速率限制和请求管理

## 构造函数

### `new`

创建新的客户端实例。

```rust
pub fn new(
    token: Token,
    intents: Intents,
    handler: H,
    use_sandbox: bool,
) -> Result<Self>
```

#### 参数

- `token`: 包含您的应用 ID 和密钥的身份验证令牌
- `intents`: 事件订阅配置
- `handler`: 实现 `EventHandler` trait 的事件处理器
- `use_sandbox`: 是否使用沙盒环境进行测试

#### 返回值

返回 `Result<Client<H>, BotError>` - 客户端实例或初始化失败时的错误。

#### 示例

```rust
use botrs::{Client, EventHandler, Intents, Token};

struct MyHandler;

#[async_trait::async_trait]
impl EventHandler for MyHandler {
    // 事件处理方法...
}

let token = Token::new("你的应用ID", "你的密钥");
let intents = Intents::default().with_public_guild_messages();
let handler = MyHandler;

let client = Client::new(token, intents, handler, false)?;
```

## 方法

### `start`

启动机器人并开始监听事件。此方法会阻塞直到连接关闭。

```rust
pub async fn start(&mut self) -> Result<()>
```

#### 返回值

返回 `Result<(), BotError>` - 机器人优雅停止时返回 `Ok(())`，连接失败时返回错误。

#### 示例

```rust
let mut client = Client::new(token, intents, handler, false)?;
client.start().await?;
```

### `stop`

优雅地停止机器人并关闭 WebSocket 连接。

```rust
pub async fn stop(&mut self) -> Result<()>
```

#### 返回值

返回 `Result<(), BotError>` - 成功停止时返回 `Ok(())`，停止失败时返回错误。

#### 示例

```rust
// 在另一个任务或信号处理器中
client.stop().await?;
```

### `is_connected`

检查客户端当前是否连接到网关。

```rust
pub fn is_connected(&self) -> bool
```

#### 返回值

连接时返回 `true`，否则返回 `false`。

#### 示例

```rust
if client.is_connected() {
    println!("机器人在线");
} else {
    println!("机器人离线");
}
```

### `get_session_info`

获取当前会话的信息。

```rust
pub fn get_session_info(&self) -> Option<&ConnectionSession>
```

#### 返回值

连接时返回 `Some(&ConnectionSession)`，断开连接时返回 `None`。

#### 示例

```rust
if let Some(session) = client.get_session_info() {
    println!("会话 ID: {}", session.session_id);
    println!("分片: {}/{}", session.shard_id, session.shard_count);
}
```

## 配置

### 环境 URL

客户端自动选择适当的 API 端点：

- **生产环境**: `https://api.sgroup.qq.com`
- **沙盒环境**: `https://sandbox.api.sgroup.qq.com`

### 连接设置

默认连接设置：

- **WebSocket URL**: `wss://api.sgroup.qq.com/websocket`
- **超时**: HTTP 请求 30 秒
- **心跳**: 根据服务器要求自动
- **重连**: 自动重连，指数退避

## 错误处理

客户端可能返回各种错误：

```rust
use botrs::BotError;

match client.start().await {
    Ok(_) => println!("机器人优雅停止"),
    Err(BotError::Authentication(e)) => eprintln!("认证错误: {}", e),
    Err(BotError::Network(e)) => eprintln!("网络错误: {}", e),
    Err(BotError::Gateway(e)) => eprintln!("网关错误: {}", e),
    Err(e) => eprintln!("其他错误: {}", e),
}
```

## 事件流程

1. **连接**: 客户端连接到 WebSocket 网关
2. **身份验证**: 发送包含令牌和 intent 的识别负载
3. **就绪**: 接收就绪事件，机器人现在在线
4. **事件循环**: 持续接收和分派事件
5. **重连**: 如果连接断开自动重连

## 线程安全

`Client` 设计为从单个异步任务使用。对于多线程应用程序，请将其包装在适当的同步原语中：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let client = Arc::new(Mutex::new(client));
```

## 示例

### 基础机器人

```rust
use botrs::{Client, Context, EventHandler, Intents, Message, Ready, Token};

struct BasicBot;

#[async_trait::async_trait]
impl EventHandler for BasicBot {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("机器人就绪: {}", ready.user.username);
    }

    async fn message_create(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let _ = message.reply(&ctx.api, &ctx.token, "Pong!").await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("应用ID", "密钥");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, BasicBot, false)?;
    
    client.start().await?;
    Ok(())
}
```

### 带优雅关闭的机器人

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Token::new("应用ID", "密钥");
    let intents = Intents::default().with_public_guild_messages();
    let mut client = Client::new(token, intents, MyHandler, false)?;
    
    // 在后台任务中启动机器人
    let client_handle = tokio::spawn(async move {
        client.start().await
    });
    
    // 等待 Ctrl+C
    signal::ctrl_c().await?;
    println!("收到关闭信号");
    
    // 停止机器人
    client_handle.abort();
    
    Ok(())
}
```

### 多个 Intent

```rust
let intents = Intents::default()
    .with_public_guild_messages()
    .with_direct_message()
    .with_guilds()
    .with_guild_members();

let mut client = Client::new(token, intents, handler, false)?;
```

## 另请参阅

- [`EventHandler`](./event-handler.md) - 定义机器人如何响应事件
- [`Context`](./context.md) - 在事件处理器中访问 API 客户端和令牌
- [`Intents`](./intents.md) - 配置要接收的事件
- [`Token`](./token.md) - 身份验证和凭据管理
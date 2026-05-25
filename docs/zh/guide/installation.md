# 安装

在 `Cargo.toml` 中加入 `botrs` 以及异步运行时：

```toml
[dependencies]
botrs = "0.11.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

`tokio` 必须开启 `features = ["full"]`，因为框架会启动网关／心跳任务并依赖定时器与 I/O 驱动。`async-trait` 也是必需的，`EventHandler` 使用 `#[async_trait::async_trait]` 标注。

## 特性开关

crate 默认不启用任何特性。唯一可能用到的开关是 `examples`，它会引入 `clap` 与 `toml`，用于 `examples/` 目录下的可执行示例。普通使用者不需要它。

```toml
botrs = { version = "0.11.0", features = ["examples"] }
```

## 安装自检

```rust
use botrs::{Token, Intents};

fn main() {
    let _ = Token::new("app_id", "secret");
    let _ = Intents::default();
    println!("botrs {}", botrs::VERSION);
}
```

若能编译通过即可继续：

- [快速开始](/zh/guide/quick-start) —— 一个最小可运行机器人。
- [配置](/zh/guide/configuration) —— `Token::from_env` 与沙箱／正式环境切换。

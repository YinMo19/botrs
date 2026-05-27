# 网关

网关是机器人与 QQ 之间的 WebSocket 连接。`Client` 已经替你构建并运行它，大多数使用者无需直接接触网关类型。本页只描述框架代为处理的生命周期，以及少量影响它的可调参数。

## 客户端管理的生命周期

调用 `client.start().await` 后，框架会：

1. 校验 `Token` 并通过 `BotApi::get_bot_info` 拉取当前机器人信息。
2. 调用 `BotApi::get_gateway` 获取 WebSocket URL、推荐 shard 数与会话启动限额。
3. 用 `check_session_limit` 校验限额（若当日额度已耗尽返回 `BotError::Sdk`）。
4. 根据 `session_start_limit.max_concurrency` 调用 `Gateway::session_start_interval` 计算重连间隔。
5. 启动会话管理器，为每个 shard 打开一个 `Gateway`，把事件汇聚到客户端读取的通道。

客户端进入主循环：每个分派事件都会被解码为对应的载荷类型，并路由到 `EventHandler` 中的回调。

## 心跳

收到 `HELLO` op 后，网关记录服务端给出的 `heartbeat_interval`（毫秒）并启动心跳任务。每次心跳发送 op-1 携带最后一次收到的 `s`；如果下一次心跳前没有收到 `HEARTBEAT_ACK`，连接被判定为死连接并关闭，由重连路径接管。

你无需自己发送心跳。心跳健康状况可通过 tracing 观察 —— 网关在 `debug` 级别打印 ack 延迟。

## Resume 与 Identify

正常断开后，网关会先用缓存的 `session_id` 与 `last_seq` 尝试 `RESUME`。若服务端回复 `INVALID_SESSION`（或关闭码命中 `can_not_resume` 列表），下一次尝试将退化为重新 `IDENTIFY`。命中 `can_not_identify` 列表的关闭码（例如 `4014` 表示“intent 未授权”）会让网关停止重连，客户端把它作为 `BotError::Gateway` 暴露。

## 重连节流

框架遵循官方指引：不要紧凑循环重连。两次 `connect_once` 之间的间隔来自 `Gateway::session_start_interval(max_concurrency)`，实现为 `round(2 / max_concurrency)`，下限为一秒。`max_concurrency = 1` 时为 2 秒，更高并发档位等比缩短。

如需自定义间隔（例如测试），在驱动 `Gateway` 之前调用 `Gateway::with_reconnect_interval(Duration::from_secs(n))`。零会被归一化为一秒，避免病态循环。

## 状态查询

正在运行的 `Gateway` 暴露若干便于观测的 `pub fn`：

- `is_ready() -> bool` —— 收到首个 `READY` 后变为 `true`。
- `can_reconnect() -> bool` —— 一旦遇到不可恢复的关闭码就翻转为 `false`。
- `session_id() -> Option<&str>` —— resume 会话 id，identify 完成前为 `None`。
- `last_sequence() -> u64` —— 最近一次的 `s` 值，心跳也会用到。

通常这些方法在自定义会话管理器中使用。客户端默认使用 `new_session_manager()` 构造的实现。

## Sharding

shard 数量来自 `gateway_info.shards`（即 `BotApi::get_gateway` 的返回）。每个 shard 是独立的 WebSocket 连接，由会话管理器按计算出的间隔节流。`Client` 没有手动 shard 设置，若需自定义拓扑，可自行构造 `Gateway::new(url, token, intents, Some([shard_id, total]))`，并通过 `set_session_manager_factory` 注入自己的会话管理器。

## 相关类型

- `botrs::session_manager::Session`、`botrs::session_manager::SessionManager` 与 `botrs::session_manager::ChanManager` 是网关运行时使用的公开会话管理类型。
- 公共端点常量包括 `botrs::DEFAULT_WS_URL`（`wss://api.sgroup.qq.com/websocket`）与 `botrs::SANDBOX_API_URL`。

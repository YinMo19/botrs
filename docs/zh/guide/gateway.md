# 网关

网关由 `Client` 管理。应用代码通常通过实现 `EventHandler` 来处理网关流量。

## 生命周期

调用 `client.start().await` 后，框架会：

1. 校验 `Token`；
2. 创建共享的 `BotApi`；
3. 通过 `get_bot_info` 获取 bot 信息；
4. 通过 `get_gateway` 获取网关元信息；
5. 根据返回的 URL、shard 数和 session start limit 创建 shard session；
6. 连接每个 shard，完成鉴权，发送心跳，并把 gateway dispatch payload 转发到 client 事件循环。

随后 client 会把每个 gateway dispatch 解析成对应的 Rust 模型，并调用匹配的 `EventHandler` 方法。

## 心跳与重连

收到 `HELLO` 后，runtime 使用服务端给出的 heartbeat interval 启动心跳任务。心跳会携带最后一次收到的 sequence number。socket 关闭时，runtime 会用缓存的 session id 和 sequence number 尝试 resume。不可 resume 的关闭码会触发新的 identify；致命 identify 错误会停止该 shard 的重连循环，并由 session manager 记录日志。

session manager 会根据 `session_start_limit.max_concurrency` 间隔启动 shard，避免网络恢复后集中重连。

## 事件分发

已知事件名会解析成类型化 payload。例如：

- `READY` -> `ready(ReadySession)`
- `AT_MESSAGE_CREATE` -> `message_create(ChannelReplySession)`
- `DIRECT_MESSAGE_CREATE` -> `direct_message_create(DirectReplySession)`
- `GROUP_AT_MESSAGE_CREATE` -> `group_message_create(GroupReplySession)`
- `C2C_MESSAGE_CREATE` -> `c2c_message_create(C2CReplySession)`
- `MESSAGE_REACTION_ADD` / `MESSAGE_REACTION_REMOVE` -> 表情回应回调
- guild、channel、member、manage、audio、forum、open-forum 事件 -> 对应回调

未知事件名会进入 `unknown_event(UnknownEventSession)`，这样平台新增事件时仍然可以被观察到。

## 可配置项

公开配置项保持很小：

- `Client::new(token, intents, handler, is_sandbox)`
- `Client::with_config(token, intents, handler, timeout, is_sandbox)`
- `Intents` 控制 QQ 会发送哪些 gateway 事件类别。
- `is_sandbox` 控制 REST 和 gateway discovery 是否使用沙箱环境。

## 参见

- [Client](../api/client.md)
- [EventHandler](../api/event-handler.md)
- [Intents](./intents.md)

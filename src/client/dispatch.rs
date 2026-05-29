use super::prelude::*;
use super::{Client, Context, EventHandler};

impl<H: EventHandler + 'static> Client<H> {
    /// Converts one gateway envelope into the typed event passed to `EventHandler`.
    ///
    /// Unknown event names are forwarded unchanged so callers can still observe
    /// newer gateway events before this crate has first-class models for them.
    pub(super) async fn handle_event(&self, ctx: Context, event: GatewayEvent) -> Result<()> {
        debug!("Handling event: {:?}", event.event_type);

        let event_type = event.event_type.as_deref().map(str::to_ascii_uppercase);

        // Some payloads carry a fallback event ID inside `data.id`; pick the gateway
        // envelope id first and fall back to the inline value.
        fn payload_event_id(
            envelope_id: &Option<String>,
            data: &serde_json::Value,
        ) -> Option<String> {
            envelope_id.clone().or_else(|| {
                data.get("id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
        }

        fn fallback_event_id(
            event_name: &str,
            envelope_id: &Option<String>,
            sequence: Option<u64>,
        ) -> String {
            envelope_id
                .clone()
                .unwrap_or_else(|| format!("{}_{}", event_name, sequence.unwrap_or(0)))
        }

        fn parse_event_json<T>(event_name: &str, data: &serde_json::Value) -> Option<T>
        where
            T: serde::de::DeserializeOwned,
        {
            match serde_json::from_value::<T>(data.clone()) {
                Ok(value) => Some(value),
                Err(e) => {
                    error!("Failed to parse {} event: {}", event_name, e);
                    debug!(
                        "Raw event data: {}",
                        serde_json::to_string_pretty(data).unwrap_or_default()
                    );
                    None
                }
            }
        }

        fn parse_message_delete(
            event_name: &str,
            event_id: String,
            data: &serde_json::Value,
        ) -> Option<MessageDelete> {
            let mut value = if data.get("message").is_some() {
                parse_event_json::<MessageDelete>(event_name, data)?
            } else {
                let message = parse_event_json::<Message>(event_name, data)?;
                MessageDelete {
                    message,
                    op_user: User::default(),
                    event_id: None,
                }
            };

            value.event_id = Some(event_id.clone());
            value.message.event_id = Some(event_id);
            Some(value)
        }

        macro_rules! dispatch_with_event_id {
            ($event_name:literal, $event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = fallback_event_id($event_name, &event.id, event.sequence);
                    if let Some(mut value) = parse_event_json::<$event_type>($event_name, &data) {
                        value.event_id = Some(event_id);
                        self.handler.$handler(EventSession::new(ctx, value)).await;
                    }
                }
            }};
        }

        macro_rules! dispatch_reply_session {
            ($event_name:literal, $event_type:ty, $session_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = fallback_event_id($event_name, &event.id, event.sequence);
                    if let Some(mut value) = parse_event_json::<$event_type>($event_name, &data) {
                        value.event_id = Some(event_id);
                        match <$session_type>::new(ctx, value) {
                            Ok(session) => self.handler.$handler(session).await,
                            Err(error) => self.handler.error(error).await,
                        }
                    }
                }
            }};
        }

        macro_rules! dispatch_message_delete {
            ($event_name:literal, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = fallback_event_id($event_name, &event.id, event.sequence);
                    if let Some(value) = parse_message_delete($event_name, event_id, &data) {
                        self.handler.$handler(EventSession::new(ctx, value)).await;
                    }
                }
            }};
        }

        macro_rules! dispatch_json {
            ($event_name:literal, $event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    if let Some(value) = parse_event_json::<$event_type>($event_name, &data) {
                        self.handler.$handler(EventSession::new(ctx, value)).await;
                    }
                }
            }};
        }

        macro_rules! dispatch_payload_new {
            ($event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let value = <$event_type>::new(event_id, &data);
                    self.handler.$handler(EventSession::new(ctx, value)).await;
                }
            }};
        }

        macro_rules! dispatch_manage_session {
            ($event_type:ty, $session_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let value = <$event_type>::new(event_id, &data);
                    self.handler
                        .$handler(<$session_type>::new(ctx, value))
                        .await;
                }
            }};
        }

        macro_rules! dispatch_reaction {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let reaction = Reaction::new(event.id, &data)?;
                    self.handler
                        .$handler(EventSession::new(ctx, reaction))
                        .await;
                }
            }};
        }

        macro_rules! dispatch_audio {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(event.id, audio_action);
                    self.handler.$handler(EventSession::new(ctx, audio)).await;
                }
            }};
        }

        macro_rules! dispatch_forum {
            ($event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let value = <$event_type>::new(event.id, &data);
                    self.handler.$handler(EventSession::new(ctx, value)).await;
                }
            }};
        }

        macro_rules! dispatch_open_forum {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(&data);
                    thread.event_id = event.id;
                    self.handler.$handler(EventSession::new(ctx, thread)).await;
                }
            }};
        }

        match event_type.as_deref() {
            Some("READY") => {
                if let Some(data) = event.data {
                    match serde_json::from_value::<Ready>(data.clone()) {
                        Ok(ready) => {
                            info!("Bot is ready! Session ID: {}", ready.session_id);
                            self.handler.ready(EventSession::new(ctx, ready)).await;
                        }
                        Err(e) => {
                            error!("Failed to parse READY event: {}", e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Some("RESUMED") => {
                self.handler.resumed(EventSession::new(ctx, ())).await;
            }
            Some("AT_MESSAGE_CREATE") => {
                dispatch_reply_session!(
                    "AT_MESSAGE_CREATE",
                    Message,
                    ChannelReplySession,
                    message_create
                );
            }
            Some("DIRECT_MESSAGE_CREATE") => {
                dispatch_reply_session!(
                    "DIRECT_MESSAGE_CREATE",
                    Message,
                    DirectReplySession,
                    direct_message_create
                );
            }
            Some("GROUP_AT_MESSAGE_CREATE") => {
                dispatch_reply_session!(
                    "GROUP_AT_MESSAGE_CREATE",
                    GroupMessage,
                    GroupReplySession,
                    group_message_create
                );
            }
            Some("C2C_MESSAGE_CREATE") => {
                dispatch_reply_session!(
                    "C2C_MESSAGE_CREATE",
                    C2CMessage,
                    C2CReplySession,
                    c2c_message_create
                );
            }
            Some("SUBSCRIBE_MESSAGE_STATUS") => {
                dispatch_payload_new!(SubscribeMessageStatusData, subscribe_message_status);
            }
            Some("ENTER_AIO") => {
                dispatch_payload_new!(EnterAioEvent, enter_aio);
            }
            Some("DIRECT_MESSAGE_DELETE") => {
                dispatch_message_delete!("DIRECT_MESSAGE_DELETE", direct_message_delete);
            }
            Some("PUBLIC_MESSAGE_DELETE") => {
                dispatch_message_delete!("PUBLIC_MESSAGE_DELETE", public_message_delete);
            }
            Some("MESSAGE_DELETE") => {
                dispatch_message_delete!("MESSAGE_DELETE", message_delete);
            }
            Some("MESSAGE_REACTION_ADD") => {
                dispatch_reaction!(message_reaction_add);
            }
            Some("MESSAGE_REACTION_REMOVE") => {
                dispatch_reaction!(message_reaction_remove);
            }
            Some("INTERACTION_CREATE") => {
                if let Some(data) = event.data {
                    let interaction = Interaction::new(event.id, &data);
                    self.handler
                        .interaction_create(EventSession::new(ctx, interaction))
                        .await;
                }
            }
            Some("AUDIO_START") => {
                dispatch_audio!(audio_start);
            }
            Some("AUDIO_FINISH") => {
                dispatch_audio!(audio_finish);
            }
            Some("ON_MIC") => {
                dispatch_audio!(on_mic);
            }
            Some("OFF_MIC") => {
                dispatch_audio!(off_mic);
            }
            Some("GUILD_CREATE") => {
                dispatch_json!("GUILD_CREATE", Guild, guild_create);
            }
            Some("GUILD_UPDATE") => {
                dispatch_json!("GUILD_UPDATE", Guild, guild_update);
            }
            Some("GUILD_DELETE") => {
                dispatch_json!("GUILD_DELETE", Guild, guild_delete);
            }
            Some("CHANNEL_CREATE") => {
                dispatch_json!("CHANNEL_CREATE", Channel, channel_create);
            }
            Some("CHANNEL_UPDATE") => {
                dispatch_json!("CHANNEL_UPDATE", Channel, channel_update);
            }
            Some("CHANNEL_DELETE") => {
                dispatch_json!("CHANNEL_DELETE", Channel, channel_delete);
            }
            Some("GUILD_MEMBER_ADD") => {
                dispatch_json!("GUILD_MEMBER_ADD", Member, guild_member_add);
            }
            Some("GUILD_MEMBER_UPDATE") => {
                dispatch_json!("GUILD_MEMBER_UPDATE", Member, guild_member_update);
            }
            Some("GUILD_MEMBER_REMOVE") => {
                dispatch_json!("GUILD_MEMBER_REMOVE", Member, guild_member_remove);
            }
            Some("MESSAGE_AUDIT_PASS") => {
                dispatch_with_event_id!("MESSAGE_AUDIT_PASS", MessageAudit, message_audit_pass);
            }
            Some("MESSAGE_AUDIT_REJECT") => {
                dispatch_with_event_id!("MESSAGE_AUDIT_REJECT", MessageAudit, message_audit_reject);
            }
            Some("FRIEND_ADD") => {
                dispatch_manage_session!(C2CManageEvent, C2CManageSession, friend_add);
            }
            Some("FRIEND_DEL") => {
                dispatch_manage_session!(C2CManageEvent, C2CManageSession, friend_del);
            }
            Some("C2C_MSG_REJECT") => {
                dispatch_manage_session!(C2CManageEvent, C2CManageSession, c2c_msg_reject);
            }
            Some("C2C_MSG_RECEIVE") => {
                dispatch_manage_session!(C2CManageEvent, C2CManageSession, c2c_msg_receive);
            }
            Some("GROUP_ADD_ROBOT") => {
                dispatch_manage_session!(GroupManageEvent, GroupManageSession, group_add_robot);
            }
            Some("GROUP_DEL_ROBOT") => {
                dispatch_manage_session!(GroupManageEvent, GroupManageSession, group_del_robot);
            }
            Some("GROUP_MSG_REJECT") => {
                dispatch_manage_session!(GroupManageEvent, GroupManageSession, group_msg_reject);
            }
            Some("GROUP_MSG_RECEIVE") => {
                dispatch_manage_session!(GroupManageEvent, GroupManageSession, group_msg_receive);
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(data);
                    self.handler
                        .audio_or_live_channel_member_enter(EventSession::new(ctx, audio))
                        .await;
                }
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(data);
                    self.handler
                        .audio_or_live_channel_member_exit(EventSession::new(ctx, audio))
                        .await;
                }
            }
            Some("FORUM_THREAD_CREATE") => {
                dispatch_forum!(Thread, forum_thread_create);
            }
            Some("FORUM_THREAD_UPDATE") => {
                dispatch_forum!(Thread, forum_thread_update);
            }
            Some("FORUM_THREAD_DELETE") => {
                dispatch_forum!(Thread, forum_thread_delete);
            }
            Some("FORUM_POST_CREATE") => {
                dispatch_forum!(Post, forum_post_create);
            }
            Some("FORUM_POST_DELETE") => {
                dispatch_forum!(Post, forum_post_delete);
            }
            Some("FORUM_REPLY_CREATE") => {
                dispatch_forum!(Reply, forum_reply_create);
            }
            Some("FORUM_REPLY_DELETE") => {
                dispatch_forum!(Reply, forum_reply_delete);
            }
            Some("FORUM_PUBLISH_AUDIT_RESULT") => {
                if let Some(data) = event.data {
                    let result = ForumAuditResult::new(event.id, &data);
                    self.handler
                        .forum_publish_audit_result(EventSession::new(ctx, result))
                        .await;
                }
            }
            Some("OPEN_FORUM_THREAD_CREATE") => {
                dispatch_open_forum!(open_forum_thread_create);
            }
            Some("OPEN_FORUM_THREAD_UPDATE") => {
                dispatch_open_forum!(open_forum_thread_update);
            }
            Some("OPEN_FORUM_THREAD_DELETE") => {
                dispatch_open_forum!(open_forum_thread_delete);
            }
            Some("OPEN_FORUM_POST_CREATE") => {
                dispatch_open_forum!(open_forum_post_create);
            }
            Some("OPEN_FORUM_POST_DELETE") => {
                dispatch_open_forum!(open_forum_post_delete);
            }
            Some("OPEN_FORUM_REPLY_CREATE") => {
                dispatch_open_forum!(open_forum_reply_create);
            }
            Some("OPEN_FORUM_REPLY_DELETE") => {
                dispatch_open_forum!(open_forum_reply_delete);
            }
            _ => {
                debug!("Unknown event type: {:?}", event.event_type);
                self.handler
                    .unknown_event(EventSession::new(ctx, event))
                    .await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[derive(Default)]
    struct DeleteCounters {
        message_delete: AtomicUsize,
        public_message_delete: AtomicUsize,
        message_create_count: AtomicUsize,
        friend_add: AtomicUsize,
        inline_event_id_seen: AtomicUsize,
        fallback_event_id_seen: AtomicUsize,
    }

    struct CountingHandler(Arc<DeleteCounters>);

    #[async_trait::async_trait]
    impl EventHandler for CountingHandler {
        async fn message_create(&self, session: ChannelReplySession) {
            self.0.message_create_count.fetch_add(1, Ordering::SeqCst);
            let message = session.message();
            if message.event_id.as_deref() == Some("AT_MESSAGE_CREATE_42") {
                self.0.fallback_event_id_seen.fetch_add(1, Ordering::SeqCst);
            }
        }

        async fn message_delete(&self, _session: MessageDeleteSession) {
            self.0.message_delete.fetch_add(1, Ordering::SeqCst);
        }

        async fn public_message_delete(&self, _session: MessageDeleteSession) {
            self.0.public_message_delete.fetch_add(1, Ordering::SeqCst);
        }

        async fn friend_add(&self, session: C2CManageSession) {
            self.0.friend_add.fetch_add(1, Ordering::SeqCst);
            if session.event().event_id.as_deref() == Some("inline-event-id") {
                self.0.inline_event_id_seen.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    #[tokio::test]
    async fn public_message_delete_dispatches_to_public_handler() {
        let counters = Arc::new(DeleteCounters::default());
        let handler = CountingHandler(counters.clone());
        let client = Client::new(
            Token::new("test_app", "test_secret"),
            Intents::default(),
            handler,
            false,
        )
        .expect("client");
        let ctx = Context::new(client.api.clone());
        let event = GatewayEvent {
            id: Some("event_id".to_string()),
            event_type: Some("PUBLIC_MESSAGE_DELETE".to_string()),
            data: Some(json!({
                "message": {
                    "id": "message_id",
                    "content": "deleted",
                    "channel_id": "channel_id",
                    "guild_id": "guild_id",
                    "author": {
                        "id": "author_id",
                        "username": "author",
                        "bot": false
                    },
                    "seq_in_channel": "1",
                    "timestamp": "2024-01-01T00:00:00+08:00"
                },
                "op_user": {
                    "id": "operator_id",
                    "username": "operator"
                }
            })),
            sequence: Some(1),
            opcode: 0,
        };

        client.handle_event(ctx, event).await.expect("dispatch");

        assert_eq!(counters.message_delete.load(Ordering::SeqCst), 0);
        assert_eq!(counters.public_message_delete.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn generated_dispatch_keeps_event_id_fallbacks() {
        let counters = Arc::new(DeleteCounters::default());
        let handler = CountingHandler(counters.clone());
        let client = Client::new(
            Token::new("test_app", "test_secret"),
            Intents::default(),
            handler,
            false,
        )
        .expect("client");
        let ctx = Context::new(client.api.clone());

        client
            .handle_event(
                ctx.clone(),
                GatewayEvent {
                    id: None,
                    event_type: Some("AT_MESSAGE_CREATE".to_string()),
                    data: Some(json!({
                        "id": "message_id",
                        "content": "hello",
                        "channel_id": "channel_id",
                        "guild_id": "guild_id",
                        "author": {
                            "id": "author_id",
                            "username": "author",
                            "bot": false
                        },
                        "seq_in_channel": "1",
                        "timestamp": "2024-01-01T00:00:00+08:00"
                    })),
                    sequence: Some(42),
                    opcode: 0,
                },
            )
            .await
            .expect("dispatch at message");

        client
            .handle_event(
                ctx,
                GatewayEvent {
                    id: None,
                    event_type: Some("FRIEND_ADD".to_string()),
                    data: Some(json!({
                        "id": "inline-event-id",
                        "openid": "openid-1",
                        "timestamp": 1710000000
                    })),
                    sequence: Some(7),
                    opcode: 0,
                },
            )
            .await
            .expect("dispatch friend add");

        assert_eq!(counters.message_create_count.load(Ordering::SeqCst), 1);
        assert_eq!(counters.fallback_event_id_seen.load(Ordering::SeqCst), 1);
        assert_eq!(counters.friend_add.load(Ordering::SeqCst), 1);
        assert_eq!(counters.inline_event_id_seen.load(Ordering::SeqCst), 1);
    }
}

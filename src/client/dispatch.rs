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

        macro_rules! dispatch_from_data {
            ($event_name:literal, $event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = fallback_event_id($event_name, &event.id, event.sequence);
                    let value = <$event_type>::from_data(event_id, data);
                    self.handler.$handler(ctx, value).await;
                }
            }};
        }

        macro_rules! dispatch_json {
            ($event_name:literal, $event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    match serde_json::from_value::<$event_type>(data.clone()) {
                        Ok(value) => {
                            self.handler.$handler(ctx, value).await;
                        }
                        Err(e) => {
                            error!("Failed to parse {} event: {}", $event_name, e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }};
        }

        macro_rules! dispatch_payload_new {
            ($event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let value = <$event_type>::new(event_id, &data);
                    self.handler.$handler(ctx, value).await;
                }
            }};
        }

        macro_rules! dispatch_reaction {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let reaction = Reaction::new(event.id, &data)?;
                    self.handler.$handler(ctx, reaction).await;
                }
            }};
        }

        macro_rules! dispatch_audio {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(event.id, audio_action);
                    self.handler.$handler(ctx, audio).await;
                }
            }};
        }

        macro_rules! dispatch_forum {
            ($event_type:ty, $handler:ident) => {{
                if let Some(data) = event.data {
                    let value = <$event_type>::new(event.id, &data);
                    self.handler.$handler(ctx, value).await;
                }
            }};
        }

        macro_rules! dispatch_open_forum {
            ($handler:ident) => {{
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(&data);
                    thread.event_id = event.id;
                    self.handler.$handler(ctx, thread).await;
                }
            }};
        }

        match event_type.as_deref() {
            Some("READY") => {
                if let Some(data) = event.data {
                    match serde_json::from_value::<Ready>(data.clone()) {
                        Ok(ready) => {
                            info!("Bot is ready! Session ID: {}", ready.session_id);
                            self.handler.ready(ctx, ready).await;
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
                self.handler.resumed(ctx).await;
            }
            Some("AT_MESSAGE_CREATE") => {
                dispatch_from_data!("AT_MESSAGE_CREATE", Message, message_create);
            }
            Some("DIRECT_MESSAGE_CREATE") => {
                dispatch_from_data!("DIRECT_MESSAGE_CREATE", Message, direct_message_create);
            }
            Some("GROUP_AT_MESSAGE_CREATE") => {
                dispatch_from_data!(
                    "GROUP_AT_MESSAGE_CREATE",
                    GroupMessage,
                    group_message_create
                );
            }
            Some("C2C_MESSAGE_CREATE") => {
                dispatch_from_data!("C2C_MESSAGE_CREATE", C2CMessage, c2c_message_create);
            }
            Some("SUBSCRIBE_MESSAGE_STATUS") => {
                dispatch_payload_new!(SubscribeMessageStatusData, subscribe_message_status);
            }
            Some("ENTER_AIO") => {
                dispatch_payload_new!(EnterAioEvent, enter_aio);
            }
            Some("DIRECT_MESSAGE_DELETE") => {
                dispatch_from_data!(
                    "DIRECT_MESSAGE_DELETE",
                    MessageDelete,
                    direct_message_delete
                );
            }
            Some("PUBLIC_MESSAGE_DELETE") => {
                dispatch_from_data!(
                    "PUBLIC_MESSAGE_DELETE",
                    MessageDelete,
                    public_message_delete
                );
            }
            Some("MESSAGE_DELETE") => {
                dispatch_from_data!("MESSAGE_DELETE", MessageDelete, message_delete);
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
                    self.handler.interaction_create(ctx, interaction).await;
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
                dispatch_from_data!("GUILD_CREATE", Guild, guild_create);
            }
            Some("GUILD_UPDATE") => {
                dispatch_from_data!("GUILD_UPDATE", Guild, guild_update);
            }
            Some("GUILD_DELETE") => {
                dispatch_from_data!("GUILD_DELETE", Guild, guild_delete);
            }
            Some("CHANNEL_CREATE") => {
                dispatch_from_data!("CHANNEL_CREATE", Channel, channel_create);
            }
            Some("CHANNEL_UPDATE") => {
                dispatch_from_data!("CHANNEL_UPDATE", Channel, channel_update);
            }
            Some("CHANNEL_DELETE") => {
                dispatch_from_data!("CHANNEL_DELETE", Channel, channel_delete);
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
                dispatch_from_data!("MESSAGE_AUDIT_PASS", MessageAudit, message_audit_pass);
            }
            Some("MESSAGE_AUDIT_REJECT") => {
                dispatch_from_data!("MESSAGE_AUDIT_REJECT", MessageAudit, message_audit_reject);
            }
            Some("FRIEND_ADD") => {
                dispatch_payload_new!(C2CManageEvent, friend_add);
            }
            Some("FRIEND_DEL") => {
                dispatch_payload_new!(C2CManageEvent, friend_del);
            }
            Some("C2C_MSG_REJECT") => {
                dispatch_payload_new!(C2CManageEvent, c2c_msg_reject);
            }
            Some("C2C_MSG_RECEIVE") => {
                dispatch_payload_new!(C2CManageEvent, c2c_msg_receive);
            }
            Some("GROUP_ADD_ROBOT") => {
                dispatch_payload_new!(GroupManageEvent, group_add_robot);
            }
            Some("GROUP_DEL_ROBOT") => {
                dispatch_payload_new!(GroupManageEvent, group_del_robot);
            }
            Some("GROUP_MSG_REJECT") => {
                dispatch_payload_new!(GroupManageEvent, group_msg_reject);
            }
            Some("GROUP_MSG_RECEIVE") => {
                dispatch_payload_new!(GroupManageEvent, group_msg_receive);
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(data);
                    self.handler
                        .audio_or_live_channel_member_enter(ctx, audio)
                        .await;
                }
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(data);
                    self.handler
                        .audio_or_live_channel_member_exit(ctx, audio)
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
                    self.handler.forum_publish_audit_result(ctx, result).await;
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
                self.handler.unknown_event(ctx, event).await;
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
        at_message_create: AtomicUsize,
        friend_add: AtomicUsize,
        inline_event_id_seen: AtomicUsize,
        fallback_event_id_seen: AtomicUsize,
    }

    struct CountingHandler(Arc<DeleteCounters>);

    #[async_trait::async_trait]
    impl EventHandler for CountingHandler {
        async fn message_create(&self, _ctx: Context, message: Message) {
            self.0.at_message_create.fetch_add(1, Ordering::SeqCst);
            if message.event_id.as_deref() == Some("AT_MESSAGE_CREATE_42") {
                self.0.fallback_event_id_seen.fetch_add(1, Ordering::SeqCst);
            }
        }

        async fn message_delete(&self, _ctx: Context, _message: MessageDelete) {
            self.0.message_delete.fetch_add(1, Ordering::SeqCst);
        }

        async fn public_message_delete(&self, _ctx: Context, _message: MessageDelete) {
            self.0.public_message_delete.fetch_add(1, Ordering::SeqCst);
        }

        async fn friend_add(&self, _ctx: Context, event: C2CManageEvent) {
            self.0.friend_add.fetch_add(1, Ordering::SeqCst);
            if event.event_id.as_deref() == Some("inline-event-id") {
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
                    "channel_id": "channel_id",
                    "guild_id": "guild_id"
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
                        "channel_id": "channel_id",
                        "guild_id": "guild_id"
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

        assert_eq!(counters.at_message_create.load(Ordering::SeqCst), 1);
        assert_eq!(counters.fallback_event_id_seen.load(Ordering::SeqCst), 1);
        assert_eq!(counters.friend_add.load(Ordering::SeqCst), 1);
        assert_eq!(counters.inline_event_id_seen.load(Ordering::SeqCst), 1);
    }
}

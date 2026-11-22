use super::prelude::*;
use super::{Client, Context, Event, EventHandler};
use serde::de::DeserializeOwned;
use serde_json::Value;

fn decode<T: DeserializeOwned>(name: &str, data: Value) -> Result<T> {
    serde_json::from_value(data).map_err(|err| BotError::invalid_data(format!("{name}: {err}")))
}

fn decode_delete(name: &str, data: Value, id: Option<String>) -> Result<MessageDelete> {
    let mut deleted = if data.get("message").is_some() {
        decode::<MessageDelete>(name, data)?
    } else {
        MessageDelete {
            message: decode(name, data)?,
            op_user: User::default(),
            event_id: None,
        }
    };
    deleted.message.event_id = id.clone();
    deleted.event_id = id;
    Ok(deleted)
}

fn decode_audio(name: &str, data: Value, id: Option<String>) -> Result<Audio> {
    Ok(Audio::new(id, decode(name, data)?))
}

fn decode_audit(name: &str, data: Value, id: Option<String>) -> Result<MessageAudit> {
    let mut audit: MessageAudit = decode(name, data)?;
    audit.event_id = id;
    Ok(audit)
}

impl Event {
    fn decode(ctx: Context, mut event: GatewayEvent) -> Result<Self> {
        let name = event
            .event_type
            .as_deref()
            .filter(|name| !name.is_empty())
            .ok_or_else(|| BotError::invalid_data("dispatch event is missing its type"))?
            .to_ascii_uppercase();
        let mut payload = || {
            event
                .data
                .take()
                .ok_or_else(|| BotError::invalid_data(format!("{name}: missing event data")))
        };
        macro_rules! typed {
            () => {
                EventSession::new(ctx, decode(&name, payload()?)?)
            };
            ($decode:ident) => {
                EventSession::new(ctx, $decode(&name, payload()?, event.id)?)
            };
        }
        macro_rules! message {
            ($message:ty, $session:ty) => {{
                let mut message: $message = decode(&name, payload()?)?;
                message.event_id = event.id;
                <$session>::new(ctx, message)?
            }};
        }
        macro_rules! with_id {
            ($ty:ty) => {
                EventSession::new(ctx, <$ty>::new(event.id, payload()?)?)
            };
        }
        macro_rules! management {
            ($ty:ty, $session:ty) => {{
                let data = payload()?;
                let id = event
                    .id
                    .or_else(|| data.get("id")?.as_str().map(str::to_owned));
                <$session>::new(ctx, <$ty>::new(id, data)?)
            }};
        }
        Ok(match name.as_str() {
            "READY" => Event::Ready(typed!()),
            "RESUMED" => Event::Resumed(EventSession::new(ctx, ())),
            "MESSAGE_CREATE" | "AT_MESSAGE_CREATE" => {
                Event::MessageCreate(message!(Message, ChannelReplySession))
            }
            "DIRECT_MESSAGE_CREATE" => {
                Event::DirectMessageCreate(message!(Message, DirectReplySession))
            }
            "GROUP_AT_MESSAGE_CREATE" => {
                Event::GroupMessageCreate(message!(GroupMessage, GroupReplySession))
            }
            "C2C_MESSAGE_CREATE" => Event::C2CMessageCreate(message!(C2CMessage, C2CReplySession)),
            "DIRECT_MESSAGE_DELETE" => Event::DirectMessageDelete(typed!(decode_delete)),
            "MESSAGE_DELETE" => Event::MessageDelete(typed!(decode_delete)),
            "PUBLIC_MESSAGE_DELETE" => Event::PublicMessageDelete(typed!(decode_delete)),
            "MESSAGE_REACTION_ADD" => Event::MessageReactionAdd(with_id!(Reaction)),
            "MESSAGE_REACTION_REMOVE" => Event::MessageReactionRemove(with_id!(Reaction)),
            "INTERACTION_CREATE" => Event::InteractionCreate(with_id!(Interaction)),
            "AUDIO_START" => Event::AudioStart(typed!(decode_audio)),
            "AUDIO_FINISH" => Event::AudioFinish(typed!(decode_audio)),
            "AUDIO_ON_MIC" | "ON_MIC" => Event::OnMic(typed!(decode_audio)),
            "AUDIO_OFF_MIC" | "OFF_MIC" => Event::OffMic(typed!(decode_audio)),
            "GUILD_CREATE" => Event::GuildCreate(typed!()),
            "GUILD_UPDATE" => Event::GuildUpdate(typed!()),
            "GUILD_DELETE" => Event::GuildDelete(typed!()),
            "CHANNEL_CREATE" => Event::ChannelCreate(typed!()),
            "CHANNEL_UPDATE" => Event::ChannelUpdate(typed!()),
            "CHANNEL_DELETE" => Event::ChannelDelete(typed!()),
            "GUILD_MEMBER_ADD" => Event::GuildMemberAdd(typed!()),
            "GUILD_MEMBER_UPDATE" => Event::GuildMemberUpdate(typed!()),
            "GUILD_MEMBER_REMOVE" => Event::GuildMemberRemove(typed!()),
            "MESSAGE_AUDIT_PASS" => Event::MessageAuditPass(typed!(decode_audit)),
            "MESSAGE_AUDIT_REJECT" => Event::MessageAuditReject(typed!(decode_audit)),
            "FRIEND_ADD" => Event::FriendAdd(management!(C2CManageEvent, C2CManageSession)),
            "FRIEND_DEL" => Event::FriendDel(management!(C2CManageEvent, C2CManageSession)),
            "C2C_MSG_REJECT" => Event::C2CMsgReject(management!(C2CManageEvent, C2CManageSession)),
            "C2C_MSG_RECEIVE" => {
                Event::C2CMsgReceive(management!(C2CManageEvent, C2CManageSession))
            }
            "SUBSCRIBE_MESSAGE_STATUS" => Event::SubscribeMessageStatus(management!(
                SubscribeMessageStatusData,
                SubscribeMessageStatusSession
            )),
            "ENTER_AIO" => Event::EnterAio(management!(EnterAioEvent, EnterAioSession)),
            "GROUP_ADD_ROBOT" => {
                Event::GroupAddRobot(management!(GroupManageEvent, GroupManageSession))
            }
            "GROUP_DEL_ROBOT" => {
                Event::GroupDelRobot(management!(GroupManageEvent, GroupManageSession))
            }
            "GROUP_MSG_REJECT" => {
                Event::GroupMsgReject(management!(GroupManageEvent, GroupManageSession))
            }
            "GROUP_MSG_RECEIVE" => {
                Event::GroupMsgReceive(management!(GroupManageEvent, GroupManageSession))
            }
            "AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER" => Event::AudioOrLiveChannelMemberEnter(typed!()),
            "AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT" => Event::AudioOrLiveChannelMemberExit(typed!()),
            "FORUM_THREAD_CREATE" => Event::ForumThreadCreate(with_id!(Thread)),
            "FORUM_THREAD_UPDATE" => Event::ForumThreadUpdate(with_id!(Thread)),
            "FORUM_THREAD_DELETE" => Event::ForumThreadDelete(with_id!(Thread)),
            "FORUM_POST_CREATE" => Event::ForumPostCreate(with_id!(Post)),
            "FORUM_POST_DELETE" => Event::ForumPostDelete(with_id!(Post)),
            "FORUM_REPLY_CREATE" => Event::ForumReplyCreate(with_id!(Reply)),
            "FORUM_REPLY_DELETE" => Event::ForumReplyDelete(with_id!(Reply)),
            "FORUM_PUBLISH_AUDIT_RESULT" => {
                Event::ForumPublishAuditResult(with_id!(ForumAuditResult))
            }
            "OPEN_FORUM_THREAD_CREATE" => Event::OpenForumThreadCreate(with_id!(OpenThread)),
            "OPEN_FORUM_THREAD_UPDATE" => Event::OpenForumThreadUpdate(with_id!(OpenThread)),
            "OPEN_FORUM_THREAD_DELETE" => Event::OpenForumThreadDelete(with_id!(OpenThread)),
            "OPEN_FORUM_POST_CREATE" => Event::OpenForumPostCreate(with_id!(OpenThread)),
            "OPEN_FORUM_POST_DELETE" => Event::OpenForumPostDelete(with_id!(OpenThread)),
            "OPEN_FORUM_REPLY_CREATE" => Event::OpenForumReplyCreate(with_id!(OpenThread)),
            "OPEN_FORUM_REPLY_DELETE" => Event::OpenForumReplyDelete(with_id!(OpenThread)),
            _ => Event::UnknownEvent(EventSession::new(ctx, event)),
        })
    }
}

impl<H: EventHandler + 'static> Client<H> {
    pub(super) async fn handle_event(&self, ctx: Context, event: GatewayEvent) -> Result<()> {
        self.handler.handle(Event::decode(ctx, event)?).await
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

    // The five envelopes wrap the QQ message event examples from
    // https://github.com/tencent-connect/bot-docs/blob/645787a/docs/develop/api-v2/server-inter/message/send-receive/event.md
    #[tokio::test]
    async fn official_message_events_reach_the_correct_reply_session() {
        let events: Vec<GatewayEvent> =
            serde_json::from_str(include_str!("fixtures/message_events.json")).unwrap();
        for event in events {
            let kind = event.event_type.clone().unwrap();
            let client = Client::new(
                Token::new("test", "secret"),
                Intents::new(),
                move |decoded| {
                    let kind = kind.clone();
                    async move {
                        match (kind.as_str(), decoded) {
                            ("C2C_MESSAGE_CREATE", Event::C2CMessageCreate(s)) => {
                                assert_eq!(s.openid(), "E4F4AEA33253A2797FB897C50B81D7ED")
                            }
                            ("GROUP_AT_MESSAGE_CREATE", Event::GroupMessageCreate(s)) => {
                                assert_eq!(s.group_openid(), "C9F778FE6ADF9D1D1DBE395BF744A33A");
                                assert_eq!(
                                    s.message().author.member_openid,
                                    "E4F4AEA33253A2797FB897C50B81D7ED"
                                );
                            }
                            ("DIRECT_MESSAGE_CREATE", Event::DirectMessageCreate(s)) => {
                                assert_eq!(s.guild_id(), "18700000000001")
                            }
                            ("MESSAGE_CREATE" | "AT_MESSAGE_CREATE", Event::MessageCreate(s)) => {
                                assert_eq!(s.channel_id(), "100010");
                                assert!(s.message().seq_in_channel.is_none());
                            }
                            _ => panic!("incorrect session for {kind}"),
                        }
                        Ok(())
                    }
                },
                false,
            )
            .unwrap();
            client
                .handle_event(Context::new(client.api.clone()), event)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn microphone_events_accept_platform_and_legacy_names() {
        for (name, on) in [
            ("AUDIO_ON_MIC", true),
            ("AUDIO_OFF_MIC", false),
            ("ON_MIC", true),
            ("OFF_MIC", false),
        ] {
            let client = Client::new(
                Token::new("test", "secret"),
                Intents::new(),
                move |event| async move {
                    assert!(matches!(
                        (event, on),
                        (Event::OnMic(_), true) | (Event::OffMic(_), false)
                    ));
                    Ok(())
                },
                false,
            )
            .unwrap();
            let event = serde_json::from_value(json!({"op":0,"t":name,"d":{}})).unwrap();
            client
                .handle_event(Context::new(client.api.clone()), event)
                .await
                .unwrap();
        }
    }

    #[derive(Default)]
    struct DeleteCounters {
        message_delete: AtomicUsize,
        public_message_delete: AtomicUsize,
        message_create_count: AtomicUsize,
        friend_add: AtomicUsize,
        inline_event_id_seen: AtomicUsize,
        missing_event_id_seen: AtomicUsize,
    }

    struct CountingHandler(Arc<DeleteCounters>);

    #[async_trait::async_trait]
    impl EventHandler for CountingHandler {
        async fn message_create(&self, session: ChannelReplySession) {
            self.0.message_create_count.fetch_add(1, Ordering::SeqCst);
            let message = session.message();
            if message.event_id.is_none() {
                self.0.missing_event_id_seen.fetch_add(1, Ordering::SeqCst);
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
    async fn dispatch_preserves_real_ids_and_leaves_absent_ids_unset() {
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
        assert_eq!(counters.missing_event_id_seen.load(Ordering::SeqCst), 1);
        assert_eq!(counters.friend_add.load(Ordering::SeqCst), 1);
        assert_eq!(counters.inline_event_id_seen.load(Ordering::SeqCst), 1);
    }
}

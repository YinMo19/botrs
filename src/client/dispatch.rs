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
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("AT_MESSAGE_CREATE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = Message::from_data(ctx.api_clone(), event_id, data);
                    self.handler.message_create(ctx, message).await;
                }
            }
            Some("DIRECT_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("DIRECT_MESSAGE_CREATE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = Message::from_data(ctx.api_clone(), event_id, data);
                    self.handler.direct_message_create(ctx, message).await;
                }
            }
            Some("GROUP_AT_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("GROUP_AT_MESSAGE_CREATE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = GroupMessage::from_data(ctx.api_clone(), event_id, data);
                    self.handler.group_message_create(ctx, message).await;
                }
            }
            Some("C2C_MESSAGE_CREATE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("C2C_MESSAGE_CREATE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = C2CMessage::from_data(ctx.api_clone(), event_id, data);
                    self.handler.c2c_message_create(ctx, message).await;
                }
            }
            Some("SUBSCRIBE_MESSAGE_STATUS") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let status = SubscribeMessageStatusData::new(event_id, &data);
                    self.handler.subscribe_message_status(ctx, status).await;
                }
            }
            Some("ENTER_AIO") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let aio = EnterAioEvent::new(event_id, &data);
                    self.handler.enter_aio(ctx, aio).await;
                }
            }
            Some("DIRECT_MESSAGE_DELETE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("DIRECT_MESSAGE_DELETE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = MessageDelete::from_data(ctx.api_clone(), event_id, data);
                    self.handler.direct_message_delete(ctx, message).await;
                }
            }
            Some("PUBLIC_MESSAGE_DELETE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("PUBLIC_MESSAGE_DELETE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = MessageDelete::from_data(ctx.api_clone(), event_id, data);
                    self.handler.public_message_delete(ctx, message).await;
                }
            }
            Some("MESSAGE_DELETE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("MESSAGE_DELETE_{}", event.sequence.unwrap_or(0))
                    });
                    let message = MessageDelete::from_data(ctx.api_clone(), event_id, data);
                    self.handler.message_delete(ctx, message).await;
                }
            }
            Some("MESSAGE_REACTION_ADD") => {
                if let Some(data) = event.data {
                    let reaction = Reaction::new(ctx.api_clone(), event.id, &data)?;
                    self.handler.message_reaction_add(ctx, reaction).await;
                }
            }
            Some("MESSAGE_REACTION_REMOVE") => {
                if let Some(data) = event.data {
                    let reaction = Reaction::new(ctx.api_clone(), event.id, &data)?;
                    self.handler.message_reaction_remove(ctx, reaction).await;
                }
            }
            Some("INTERACTION_CREATE") => {
                if let Some(data) = event.data {
                    let interaction = Interaction::new(ctx.api_clone(), event.id, &data);
                    self.handler.interaction_create(ctx, interaction).await;
                }
            }
            Some("AUDIO_START") => {
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(ctx.api_clone(), event.id, audio_action);
                    self.handler.audio_start(ctx, audio).await;
                }
            }
            Some("AUDIO_FINISH") => {
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(ctx.api_clone(), event.id, audio_action);
                    self.handler.audio_finish(ctx, audio).await;
                }
            }
            Some("ON_MIC") => {
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(ctx.api_clone(), event.id, audio_action);
                    self.handler.on_mic(ctx, audio).await;
                }
            }
            Some("OFF_MIC") => {
                if let Some(data) = event.data {
                    let audio_action = AudioAction::from_value(&data);
                    let audio = Audio::new(ctx.api_clone(), event.id, audio_action);
                    self.handler.off_mic(ctx, audio).await;
                }
            }
            Some("GUILD_CREATE") => {
                if let Some(data) = event.data {
                    let event_id = event
                        .id
                        .unwrap_or_else(|| format!("GUILD_CREATE_{}", event.sequence.unwrap_or(0)));
                    let guild = Guild::from_data(ctx.api_clone(), event_id, data);
                    self.handler.guild_create(ctx, guild).await;
                }
            }
            Some("GUILD_UPDATE") => {
                if let Some(data) = event.data {
                    let event_id = event
                        .id
                        .unwrap_or_else(|| format!("GUILD_UPDATE_{}", event.sequence.unwrap_or(0)));
                    let guild = Guild::from_data(ctx.api_clone(), event_id, data);
                    self.handler.guild_update(ctx, guild).await;
                }
            }
            Some("GUILD_DELETE") => {
                if let Some(data) = event.data {
                    let event_id = event
                        .id
                        .unwrap_or_else(|| format!("GUILD_DELETE_{}", event.sequence.unwrap_or(0)));
                    let guild = Guild::from_data(ctx.api_clone(), event_id, data);
                    self.handler.guild_delete(ctx, guild).await;
                }
            }
            Some("CHANNEL_CREATE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("CHANNEL_CREATE_{}", event.sequence.unwrap_or(0))
                    });
                    let channel = Channel::from_data(ctx.api_clone(), event_id, data);
                    self.handler.channel_create(ctx, channel).await;
                }
            }
            Some("CHANNEL_UPDATE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("CHANNEL_UPDATE_{}", event.sequence.unwrap_or(0))
                    });
                    let channel = Channel::from_data(ctx.api_clone(), event_id, data);
                    self.handler.channel_update(ctx, channel).await;
                }
            }
            Some("CHANNEL_DELETE") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("CHANNEL_DELETE_{}", event.sequence.unwrap_or(0))
                    });
                    let channel = Channel::from_data(ctx.api_clone(), event_id, data);
                    self.handler.channel_delete(ctx, channel).await;
                }
            }
            Some("GUILD_MEMBER_ADD") => {
                if let Some(data) = event.data {
                    match serde_json::from_value::<Member>(data.clone()) {
                        Ok(member) => {
                            self.handler.guild_member_add(ctx, member).await;
                        }
                        Err(e) => {
                            error!("Failed to parse GUILD_MEMBER_ADD event: {}", e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Some("GUILD_MEMBER_UPDATE") => {
                if let Some(data) = event.data {
                    match serde_json::from_value::<Member>(data.clone()) {
                        Ok(member) => {
                            self.handler.guild_member_update(ctx, member).await;
                        }
                        Err(e) => {
                            error!("Failed to parse GUILD_MEMBER_UPDATE event: {}", e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Some("GUILD_MEMBER_REMOVE") => {
                if let Some(data) = event.data {
                    match serde_json::from_value::<Member>(data.clone()) {
                        Ok(member) => {
                            self.handler.guild_member_remove(ctx, member).await;
                        }
                        Err(e) => {
                            error!("Failed to parse GUILD_MEMBER_REMOVE event: {}", e);
                            debug!(
                                "Raw event data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                }
            }
            Some("MESSAGE_AUDIT_PASS") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("MESSAGE_AUDIT_PASS_{}", event.sequence.unwrap_or(0))
                    });
                    let audit = MessageAudit::from_data(ctx.api_clone(), event_id, data);
                    self.handler.message_audit_pass(ctx, audit).await;
                }
            }
            Some("MESSAGE_AUDIT_REJECT") => {
                if let Some(data) = event.data {
                    let event_id = event.id.unwrap_or_else(|| {
                        format!("MESSAGE_AUDIT_REJECT_{}", event.sequence.unwrap_or(0))
                    });
                    let audit = MessageAudit::from_data(ctx.api_clone(), event_id, data);
                    self.handler.message_audit_reject(ctx, audit).await;
                }
            }
            Some("FRIEND_ADD") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = C2CManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.friend_add(ctx, event).await;
                }
            }
            Some("FRIEND_DEL") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = C2CManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.friend_del(ctx, event).await;
                }
            }
            Some("C2C_MSG_REJECT") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = C2CManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.c2c_msg_reject(ctx, event).await;
                }
            }
            Some("C2C_MSG_RECEIVE") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = C2CManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.c2c_msg_receive(ctx, event).await;
                }
            }
            Some("GROUP_ADD_ROBOT") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = GroupManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.group_add_robot(ctx, event).await;
                }
            }
            Some("GROUP_DEL_ROBOT") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = GroupManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.group_del_robot(ctx, event).await;
                }
            }
            Some("GROUP_MSG_REJECT") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = GroupManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.group_msg_reject(ctx, event).await;
                }
            }
            Some("GROUP_MSG_RECEIVE") => {
                if let Some(data) = event.data {
                    let event_id = payload_event_id(&event.id, &data);
                    let event = GroupManageEvent::new(ctx.api_clone(), event_id, &data);
                    self.handler.group_msg_receive(ctx, event).await;
                }
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(ctx.api_clone(), data);
                    self.handler
                        .audio_or_live_channel_member_enter(ctx, audio)
                        .await;
                }
            }
            Some("AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT") => {
                if let Some(data) = event.data {
                    let audio = PublicAudio::new(ctx.api_clone(), data);
                    self.handler
                        .audio_or_live_channel_member_exit(ctx, audio)
                        .await;
                }
            }
            Some("FORUM_THREAD_CREATE") => {
                if let Some(data) = event.data {
                    let thread = Thread::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_thread_create(ctx, thread).await;
                }
            }
            Some("FORUM_THREAD_UPDATE") => {
                if let Some(data) = event.data {
                    let thread = Thread::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_thread_update(ctx, thread).await;
                }
            }
            Some("FORUM_THREAD_DELETE") => {
                if let Some(data) = event.data {
                    let thread = Thread::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_thread_delete(ctx, thread).await;
                }
            }
            Some("FORUM_POST_CREATE") => {
                if let Some(data) = event.data {
                    let post = Post::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_post_create(ctx, post).await;
                }
            }
            Some("FORUM_POST_DELETE") => {
                if let Some(data) = event.data {
                    let post = Post::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_post_delete(ctx, post).await;
                }
            }
            Some("FORUM_REPLY_CREATE") => {
                if let Some(data) = event.data {
                    let reply = Reply::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_reply_create(ctx, reply).await;
                }
            }
            Some("FORUM_REPLY_DELETE") => {
                if let Some(data) = event.data {
                    let reply = Reply::new(ctx.api_clone(), event.id, &data);
                    self.handler.forum_reply_delete(ctx, reply).await;
                }
            }
            Some("FORUM_PUBLISH_AUDIT_RESULT") => {
                if let Some(data) = event.data {
                    let result = ForumAuditResult::new(event.id, &data);
                    self.handler.forum_publish_audit_result(ctx, result).await;
                }
            }
            Some("OPEN_FORUM_THREAD_CREATE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_thread_create(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_THREAD_UPDATE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_thread_update(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_THREAD_DELETE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_thread_delete(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_POST_CREATE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_post_create(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_POST_DELETE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_post_delete(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_REPLY_CREATE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_reply_create(ctx, thread).await;
                }
            }
            Some("OPEN_FORUM_REPLY_DELETE") => {
                if let Some(data) = event.data {
                    let mut thread = OpenThread::new(ctx.api_clone(), &data);
                    thread.event_id = event.id;
                    self.handler.open_forum_reply_delete(ctx, thread).await;
                }
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
    }

    struct CountingHandler(Arc<DeleteCounters>);

    #[async_trait::async_trait]
    impl EventHandler for CountingHandler {
        async fn message_delete(&self, _ctx: Context, _message: MessageDelete) {
            self.0.message_delete.fetch_add(1, Ordering::SeqCst);
        }

        async fn public_message_delete(&self, _ctx: Context, _message: MessageDelete) {
            self.0.public_message_delete.fetch_add(1, Ordering::SeqCst);
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
}

use super::parsers;
use crate::api::BotApi;
use crate::models::robot::Robot;
use serde_json::Value;
use std::collections::HashMap;
use tracing::warn;

pub(crate) type ParserMap =
    HashMap<String, fn(&ConnectionState, &Value) -> Option<(&'static str, Value)>>;

/// Connection state for handling websocket events
pub struct ConnectionState {
    /// Robot information
    pub robot: Option<Robot>,
    /// API client
    pub(crate) api: BotApi,
    /// Event parsers
    parsers: ParserMap,
}

impl ConnectionState {
    /// Create a new connection state
    pub fn new(api: BotApi) -> Self {
        let mut state = Self {
            robot: None,
            api,
            parsers: HashMap::new(),
        };

        state.register_parsers();
        state
    }

    /// Register all event parsers
    fn register_parsers(&mut self) {
        self.parsers
            .insert("ready".to_string(), parsers::parse_ready);
        self.parsers
            .insert("resumed".to_string(), parsers::parse_resumed);

        // Guild events
        self.parsers
            .insert("guild_create".to_string(), parsers::parse_guild_create);
        self.parsers
            .insert("guild_update".to_string(), parsers::parse_guild_update);
        self.parsers
            .insert("guild_delete".to_string(), parsers::parse_guild_delete);

        // Channel events
        self.parsers
            .insert("channel_create".to_string(), parsers::parse_channel_create);
        self.parsers
            .insert("channel_update".to_string(), parsers::parse_channel_update);
        self.parsers
            .insert("channel_delete".to_string(), parsers::parse_channel_delete);

        // Member events
        self.parsers.insert(
            "guild_member_add".to_string(),
            parsers::parse_guild_member_add,
        );
        self.parsers.insert(
            "guild_member_update".to_string(),
            parsers::parse_guild_member_update,
        );
        self.parsers.insert(
            "guild_member_remove".to_string(),
            parsers::parse_guild_member_remove,
        );

        // Message events
        self.parsers
            .insert("message_create".to_string(), parsers::parse_message_create);
        self.parsers
            .insert("message_delete".to_string(), parsers::parse_message_delete);
        self.parsers.insert(
            "at_message_create".to_string(),
            parsers::parse_at_message_create,
        );
        self.parsers.insert(
            "public_message_delete".to_string(),
            parsers::parse_public_message_delete,
        );

        // Direct message events
        self.parsers.insert(
            "direct_message_create".to_string(),
            parsers::parse_direct_message_create,
        );
        self.parsers.insert(
            "direct_message_delete".to_string(),
            parsers::parse_direct_message_delete,
        );

        // Reaction events
        self.parsers.insert(
            "message_reaction_add".to_string(),
            parsers::parse_message_reaction_add,
        );
        self.parsers.insert(
            "message_reaction_remove".to_string(),
            parsers::parse_message_reaction_remove,
        );

        // Interaction events
        self.parsers.insert(
            "interaction_create".to_string(),
            parsers::parse_interaction_create,
        );

        // Audio events
        self.parsers
            .insert("audio_start".to_string(), parsers::parse_audio_start);
        self.parsers
            .insert("audio_finish".to_string(), parsers::parse_audio_finish);
        self.parsers
            .insert("on_mic".to_string(), parsers::parse_on_mic);
        self.parsers
            .insert("off_mic".to_string(), parsers::parse_off_mic);

        // Public audio events
        self.parsers.insert(
            "audio_or_live_channel_member_enter".to_string(),
            parsers::parse_audio_or_live_channel_member_enter,
        );
        self.parsers.insert(
            "audio_or_live_channel_member_exit".to_string(),
            parsers::parse_audio_or_live_channel_member_exit,
        );

        // Forum events
        self.parsers.insert(
            "forum_thread_create".to_string(),
            parsers::parse_forum_thread_create,
        );
        self.parsers.insert(
            "forum_thread_update".to_string(),
            parsers::parse_forum_thread_update,
        );
        self.parsers.insert(
            "forum_thread_delete".to_string(),
            parsers::parse_forum_thread_delete,
        );
        self.parsers.insert(
            "forum_post_create".to_string(),
            parsers::parse_forum_post_create,
        );
        self.parsers.insert(
            "forum_post_delete".to_string(),
            parsers::parse_forum_post_delete,
        );
        self.parsers.insert(
            "forum_reply_create".to_string(),
            parsers::parse_forum_reply_create,
        );
        self.parsers.insert(
            "forum_reply_delete".to_string(),
            parsers::parse_forum_reply_delete,
        );
        self.parsers.insert(
            "forum_publish_audit_result".to_string(),
            parsers::parse_forum_publish_audit_result,
        );

        // Open forum events
        self.parsers.insert(
            "open_forum_thread_create".to_string(),
            parsers::parse_open_forum_thread_create,
        );
        self.parsers.insert(
            "open_forum_thread_update".to_string(),
            parsers::parse_open_forum_thread_update,
        );
        self.parsers.insert(
            "open_forum_thread_delete".to_string(),
            parsers::parse_open_forum_thread_delete,
        );
        self.parsers.insert(
            "open_forum_post_create".to_string(),
            parsers::parse_open_forum_post_create,
        );
        self.parsers.insert(
            "open_forum_post_delete".to_string(),
            parsers::parse_open_forum_post_delete,
        );
        self.parsers.insert(
            "open_forum_reply_create".to_string(),
            parsers::parse_open_forum_reply_create,
        );
        self.parsers.insert(
            "open_forum_reply_delete".to_string(),
            parsers::parse_open_forum_reply_delete,
        );

        // Group and C2C events
        self.parsers.insert(
            "group_at_message_create".to_string(),
            parsers::parse_group_at_message_create,
        );
        self.parsers.insert(
            "c2c_message_create".to_string(),
            parsers::parse_c2c_message_create,
        );
        self.parsers.insert(
            "group_add_robot".to_string(),
            parsers::parse_group_add_robot,
        );
        self.parsers.insert(
            "group_del_robot".to_string(),
            parsers::parse_group_del_robot,
        );
        self.parsers.insert(
            "group_msg_reject".to_string(),
            parsers::parse_group_msg_reject,
        );
        self.parsers.insert(
            "group_msg_receive".to_string(),
            parsers::parse_group_msg_receive,
        );
        self.parsers
            .insert("friend_add".to_string(), parsers::parse_friend_add);
        self.parsers
            .insert("friend_del".to_string(), parsers::parse_friend_del);
        self.parsers
            .insert("c2c_msg_reject".to_string(), parsers::parse_c2c_msg_reject);
        self.parsers.insert(
            "c2c_msg_receive".to_string(),
            parsers::parse_c2c_msg_receive,
        );

        // Message audit events
        self.parsers.insert(
            "message_audit_pass".to_string(),
            parsers::parse_message_audit_pass,
        );
        self.parsers.insert(
            "message_audit_reject".to_string(),
            parsers::parse_message_audit_reject,
        );
    }

    /// Parse an event and return the event name and data for dispatching
    pub fn parse_event(&self, event_type: &str, payload: &Value) -> Option<(&'static str, Value)> {
        if let Some(parser) = self.parsers.get(event_type) {
            parser(self, payload)
        } else {
            warn!("Unknown event type: {}", event_type);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> ConnectionState {
        let http = crate::http::HttpClient::new(30, false).unwrap();
        ConnectionState::new(BotApi::new(http))
    }

    #[test]
    fn open_forum_parsers_match_botpy_shape() {
        let state = test_state();
        let payload = serde_json::json!({
            "id": "event-1",
            "d": {
                "guild_id": "guild-1",
                "channel_id": "channel-1",
                "author_id": "author-1",
                "thread_info": {
                    "thread_id": "thread-1"
                },
                "post_info": {
                    "thread_id": "thread-1",
                    "post_id": "post-1"
                },
                "reply_info": {
                    "thread_id": "thread-1",
                    "post_id": "post-1",
                    "reply_id": "reply-1"
                }
            }
        });

        for event_type in [
            "open_forum_thread_create",
            "open_forum_thread_update",
            "open_forum_thread_delete",
            "open_forum_post_create",
            "open_forum_post_delete",
            "open_forum_reply_create",
            "open_forum_reply_delete",
        ] {
            let (name, value) = state.parse_event(event_type, &payload).unwrap();
            assert_eq!(name, event_type);
            assert_eq!(
                value,
                serde_json::json!({
                    "channel_id": "channel-1",
                    "guild_id": "guild-1",
                    "author_id": "author-1"
                })
            );
        }
    }
}

use super::payload::PayloadData;
use crate::forum::{ForumAuditResult, Post, Reply, Thread};
use crate::interaction::Interaction;
use crate::manage::{C2CFriendData, EnterAioEvent, SubscribeMessageStatusData};
use crate::models::{
    api::AudioAction,
    channel::Channel,
    gateway::WSPayload,
    guild::{Guild, Member},
    message::{Message, MessageAudit, MessageDelete},
};
use crate::reaction::MessageReaction;

macro_rules! typed_handler {
    ($fn_name:ident, $data:ty) => {
        pub(super) fn $fn_name(payload: &mut WSPayload, message: &[u8]) -> crate::Result<()> {
            let _: $data = <$data as PayloadData>::parse_from_payload(payload, message)?;
            Ok(())
        }
    };
}

typed_handler!(guild_handler, Guild);
typed_handler!(guild_member_handler, Member);
typed_handler!(channel_handler, Channel);
typed_handler!(message_handler, Message);
typed_handler!(message_delete_handler, MessageDelete);
typed_handler!(message_reaction_handler, MessageReaction);
typed_handler!(at_message_handler, Message);
typed_handler!(public_message_delete_handler, MessageDelete);
typed_handler!(direct_message_handler, Message);
typed_handler!(direct_message_delete_handler, MessageDelete);
typed_handler!(audio_handler, AudioAction);
typed_handler!(thread_handler, Thread);
typed_handler!(post_handler, Post);
typed_handler!(reply_handler, Reply);
typed_handler!(forum_audit_handler, ForumAuditResult);
typed_handler!(message_audit_handler, MessageAudit);
typed_handler!(interaction_handler, Interaction);
typed_handler!(group_at_message_handler, Message);
typed_handler!(c2c_message_handler, Message);
typed_handler!(subscribe_msg_status_handler, SubscribeMessageStatusData);
typed_handler!(c2c_friend_handler, C2CFriendData);
typed_handler!(enter_aio_handler, EnterAioEvent);

//! Event-scoped API sessions.
//!
//! These types wrap the stateless [`BotApi`](crate::BotApi) with the routing
//! information from one incoming event. They automatically attach reply ids and
//! monotonically increasing `msg_seq` values when sending follow-up messages.

mod c2c;
mod channel;
mod direct;
mod event;
mod group;
mod manage;

pub use c2c::C2CReplySession;
pub use channel::ChannelReplySession;
pub use direct::DirectReplySession;
pub use event::EventSession;
pub use group::GroupReplySession;
pub use manage::{C2CManageSession, GroupManageSession};

use crate::audio::{Audio, PublicAudio};
use crate::forum::{ForumAuditResult, OpenThread, Post, Reply, Thread};
use crate::interaction::Interaction;
use crate::manage::{EnterAioEvent, SubscribeMessageStatusData};
use crate::models::channel::Channel;
use crate::models::gateway::{GatewayEvent, Ready};
use crate::models::guild::{Guild, Member};
use crate::models::message::{MessageAudit, MessageDelete};
use crate::reaction::Reaction;

pub type ReadySession = EventSession<Ready>;
pub type ResumeSession = EventSession<()>;
pub type MessageDeleteSession = EventSession<MessageDelete>;
pub type ReactionSession = EventSession<Reaction>;
pub type InteractionSession = EventSession<Interaction>;
pub type AudioSession = EventSession<Audio>;
pub type GuildSession = EventSession<Guild>;
pub type ChannelSession = EventSession<Channel>;
pub type MemberSession = EventSession<Member>;
pub type MessageAuditSession = EventSession<MessageAudit>;
pub type SubscribeMessageStatusSession = EventSession<SubscribeMessageStatusData>;
pub type EnterAioSession = EventSession<EnterAioEvent>;
pub type PublicAudioSession = EventSession<PublicAudio>;
pub type ThreadSession = EventSession<Thread>;
pub type PostSession = EventSession<Post>;
pub type ForumReplySession = EventSession<Reply>;
pub type ForumAuditSession = EventSession<ForumAuditResult>;
pub type OpenForumSession = EventSession<OpenThread>;
pub type UnknownEventSession = EventSession<GatewayEvent>;

pub(crate) fn advance_msg_seq(next_msg_seq: &mut u32, msg_seq: Option<u32>) {
    match msg_seq {
        Some(0) | None => {
            *next_msg_seq = next_msg_seq.saturating_add(1);
        }
        Some(seq) => {
            *next_msg_seq = (*next_msg_seq).max(seq.saturating_add(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::api_impl::BotApi;
    use crate::client::Context;
    use crate::http::HttpClient;
    use crate::models::message::{GroupMessage, GroupMessageParams};
    use crate::token_impl::Token;

    use super::GroupReplySession;

    fn test_context() -> Context {
        let http = HttpClient::new(30, false).unwrap();
        let api = BotApi::new(http, Token::new("app-id", "secret"));
        Context::new(Arc::new(api))
    }

    #[test]
    fn group_session_fills_reply_fields_and_increments_msg_seq() {
        let message = GroupMessage {
            id: Some("message-1".to_string()),
            group_openid: Some("group-openid-1".to_string()),
            event_id: Some("event-1".to_string()),
            ..Default::default()
        };
        let mut session = GroupReplySession::new(test_context(), message).unwrap();

        let mut first = GroupMessageParams::new_text("first");
        session.prepare_message(&mut first);
        assert_eq!(first.msg_id.as_deref(), Some("message-1"));
        assert_eq!(first.event_id.as_deref(), Some("event-1"));
        assert_eq!(first.msg_seq, Some(1));

        let mut second = GroupMessageParams::new_text("second");
        session.prepare_message(&mut second);
        assert_eq!(second.msg_seq, Some(2));
    }

    #[test]
    fn group_session_respects_explicit_msg_seq_and_advances_after_it() {
        let message = GroupMessage {
            id: Some("message-1".to_string()),
            group_openid: Some("group-openid-1".to_string()),
            event_id: Some("event-1".to_string()),
            ..Default::default()
        };
        let mut session = GroupReplySession::new(test_context(), message).unwrap();

        let mut explicit = GroupMessageParams {
            msg_seq: Some(42),
            ..GroupMessageParams::new_text("manual")
        };
        session.prepare_message(&mut explicit);
        assert_eq!(explicit.msg_seq, Some(42));

        let mut automatic = GroupMessageParams::new_text("automatic");
        session.prepare_message(&mut automatic);
        assert_eq!(automatic.msg_seq, Some(43));
    }
}

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

fn validate_reply(message_id: &str, target: &str) -> crate::Result<()> {
    if message_id.is_empty() || target.is_empty() {
        return Err(crate::BotError::invalid_data(
            "message requires a non-empty ID and reply target",
        ));
    }
    Ok(())
}

fn prepare_reply(
    message_id: &str,
    msg_id: &mut Option<String>,
    event_id: Option<&str>,
    next_seq: &mut u32,
    msg_seq: &mut Option<u32>,
) {
    if msg_id.is_none() && event_id.is_none() {
        *msg_id = Some(message_id.to_owned());
    }
    advance_msg_seq(next_seq, msg_seq);
}

fn advance_msg_seq(next: &mut u32, msg_seq: &mut Option<u32>) {
    let seq = *msg_seq.get_or_insert(*next);
    *next = if seq == 0 {
        next.saturating_add(1)
    } else {
        (*next).max(seq.saturating_add(1))
    };
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
    fn old_payloads_keep_their_reply_targets_and_empty_targets_fail_early() {
        let mut group = GroupMessage {
            id: "msg".into(),
            group_id: "old-group".into(),
            ..Default::default()
        };
        let s = GroupReplySession::new(test_context(), group.clone()).unwrap();
        assert_eq!(s.group_openid(), "old-group");
        group.group_id.clear();
        assert!(GroupReplySession::new(test_context(), group).is_err());

        let mut c2c = crate::models::message::C2CMessage {
            id: "msg".into(),
            ..Default::default()
        };
        c2c.author.id = "old-user".into();
        let session = super::C2CReplySession::new(test_context(), c2c.clone()).unwrap();
        assert_eq!(session.openid(), "old-user");
        c2c.author.user_openid = "user-openid".into();
        let session = super::C2CReplySession::new(test_context(), c2c.clone()).unwrap();
        assert_eq!(session.openid(), "user-openid");
        c2c.id.clear();
        assert!(super::C2CReplySession::new(test_context(), c2c).is_err());
    }

    #[test]
    fn group_session_fills_reply_fields_and_increments_msg_seq() {
        let message = GroupMessage {
            id: "message-1".to_string(),
            group_openid: "group-openid-1".to_string(),
            event_id: Some("event-1".to_string()),
            ..Default::default()
        };
        let mut session = GroupReplySession::new(test_context(), message).unwrap();

        let mut first = GroupMessageParams::new_text("first");
        session.prepare_message(&mut first);
        assert_eq!(first.msg_id.as_deref(), Some("message-1"));
        assert!(first.event_id.is_none());
        assert_eq!(first.msg_seq, Some(1));

        let mut second = GroupMessageParams::new_text("second");
        second.event_id = Some("explicit-event".into());
        session.prepare_message(&mut second);
        assert_eq!(second.msg_seq, Some(2));
        assert!(second.msg_id.is_none());
        assert_eq!(second.event_id.as_deref(), Some("explicit-event"));
    }

    #[test]
    fn group_session_respects_explicit_msg_seq_and_advances_after_it() {
        let message = GroupMessage {
            id: "message-1".to_string(),
            group_openid: "group-openid-1".to_string(),
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

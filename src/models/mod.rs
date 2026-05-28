//! Data models for the BotRS library.
//!
//! All DTOs and gateway event payloads are exposed from this namespace. Prefer
//! importing models from their domain module, for example
//! `botrs::models::message::Message` or `botrs::models::guild::Member`.
//!
//! `botrs::models::prelude` is the single flat convenience import for examples
//! and small bots. The crate root intentionally only re-exports runtime entry
//! points such as `Client`, `BotApi`, `Token`, and `Intents`.

pub mod announce;
pub mod api;
pub mod audio {
    //! Audio gateway event payloads.

    pub use crate::audio::{Audio, PublicAudio, PublicAudioType};
}
pub mod channel;
pub mod emoji;
pub mod forum {
    //! Forum gateway event payloads.

    pub use crate::forum::{
        ForumAuditResult, OpenThread, Post, PostInfo, Reply, ReplyInfo, Thread, ThreadInfo,
    };
}
pub mod gateway;
pub mod guild;
pub mod interaction {
    //! Interaction gateway event payloads.

    pub use crate::interaction::{
        Interaction, InteractionData, InteractionDataType, InteractionType, Resolved,
    };
}
pub mod manage {
    //! C2C, group, subscription, and AIO management event payloads.

    pub use crate::manage::{
        C2CFriendData, C2CManageEvent, EnterAioEvent, GroupManageEvent, SubscribeMessageStatusData,
        SubscribeMsgTemplateResult,
    };
}
pub mod message;
pub mod permission;
pub mod reaction {
    //! Message reaction gateway event and API payloads.

    pub use crate::reaction::{
        MessageReaction, Reaction, ReactionTarget, ReactionTargetType, ReactionUsers,
    };
}
pub mod schedule;
pub(crate) mod serde_helpers;
pub mod user;

/// Flat model import set for applications that prefer a compact import list.
///
/// This is intentionally the only flattened model namespace. The canonical
/// paths remain the domain modules above.
pub mod prelude {
    pub use super::announce::{Announce, AnnouncesType, RecommendChannel};
    pub use super::api::{
        ApiError, AudioAction, BotInfo, GatewayResponse, MessageResponse, PinsMessage, RateLimit,
        SessionStartLimit, ShardConfig,
    };
    pub use super::audio::{Audio, PublicAudio, PublicAudioType};
    pub use super::channel::{Channel, ChannelSubType, ChannelType, PrivateType, SpeakPermission};
    pub use super::emoji::{Emoji, EmojiType};
    pub use super::forum::{
        ForumAuditResult, OpenThread, Post, PostInfo, Reply, ReplyInfo, Thread, ThreadInfo,
    };
    pub use super::gateway::{GatewayEvent, Hello, Identify, IdentifyProperties, Ready, Resume};
    pub use super::guild::{Guild, Member, Member as GuildMember};
    pub use super::interaction::{
        Interaction, InteractionData, InteractionDataType, InteractionType, Resolved,
    };
    pub use super::manage::{
        C2CFriendData, C2CManageEvent, EnterAioEvent, GroupManageEvent, SubscribeMessageStatusData,
        SubscribeMsgTemplateResult,
    };
    pub use super::message::{
        Ark, ArkKv, ArkObj, ArkObjKv, C2CMessage, C2CMessageParams, C2CMessageUser, DirectMessage,
        DirectMessageParams, DirectMessageToCreate, Embed, EmbedField, EmbedThumbnail,
        GroupMessage, GroupMessageParams, GroupMessageUser, Keyboard, KeyboardButton,
        KeyboardButtonAction, KeyboardButtonPermission, KeyboardButtonRenderData, KeyboardContent,
        KeyboardModal, KeyboardPayload, KeyboardRow, KeyboardStyle, KeyboardSubscribeData,
        KeyboardTemplateId, MarkdownParam, MarkdownPayload, MarkdownStyle, Media, Message,
        MessageAttachment, MessageAudit, MessageCreateType, MessageDelete, MessageMember,
        MessageParams, MessageReference, MessageScene, MessageUser, Reference,
    };
    pub use super::permission::{
        APIPermission, APIPermissionDemand, APIPermissionDemandIdentify, APIPermissions,
    };
    pub use super::reaction::{
        MessageReaction, Reaction, ReactionTarget, ReactionTargetType, ReactionUsers,
    };
    pub use super::schedule::{RemindType, Schedule, ScheduleWrapper};
    pub use super::user::{Member as UserMember, User};
    pub use super::{Snowflake, Timestamp};
}

/// A snowflake ID used throughout the QQ Guild API.
pub type Snowflake = String;

/// Timestamp string used by the API.
pub type Timestamp = String;

#[cfg(test)]
mod tests {
    #[test]
    fn snowflake_and_timestamp_are_strings() {
        let snowflake: super::Snowflake = "123".to_string();
        let timestamp: super::Timestamp = "2024-01-01T00:00:00Z".to_string();
        assert_eq!(snowflake, "123");
        assert_eq!(timestamp, "2024-01-01T00:00:00Z");
    }
}

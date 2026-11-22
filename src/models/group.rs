//! Per-group bot settings returned by the group OpenAPI.

use serde::{Deserialize, Serialize};

/// Message visibility selected for this bot in a particular QQ group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GroupReceiveMode {
    /// QQ delivers messages mentioning the bot.
    OnlyMention,
    /// QQ may include recent group context when the bot is mentioned.
    MentionAndContext,
    /// QQ delivers group messages including ordinary conversation.
    All,
    /// A wire value introduced by a newer platform version.
    #[serde(untagged)]
    Other(String),
}

/// A member's role in a QQ group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum GroupMemberRole {
    Member,
    Admin,
    Owner,
    #[serde(untagged)]
    Other(String),
}

/// Snapshot returned by [`BotApi::get_group_bot_state`](crate::BotApi::get_group_bot_state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupBotState {
    pub member_openid: String,
    /// RFC 3339 timestamp.
    pub joined_at: super::Timestamp,
    /// Whether the group permits proactive messages independently of its receive mode.
    pub allow_proactive_msg: bool,
    pub recv_msg_setting: GroupReceiveMode,
    pub member_role: GroupMemberRole,
}

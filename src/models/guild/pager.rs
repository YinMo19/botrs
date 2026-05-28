use super::Member;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pager for listing guilds the bot belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GuildPager {
    pub before: Option<String>,
    pub after: Option<String>,
    pub limit: Option<u32>,
}

impl GuildPager {
    pub(crate) fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit.filter(|limit| *limit != 0) {
            query.insert("limit", limit.to_string());
        }
        if let Some(after) = self.after.as_ref().filter(|value| !value.is_empty()) {
            query.insert("after", after.clone());
        } else if let Some(before) = self.before.as_ref().filter(|value| !value.is_empty()) {
            query.insert("before", before.clone());
        }
        query
    }
}

/// Pager for listing guild members.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GuildMembersPager {
    pub after: Option<String>,
    pub limit: Option<u32>,
}

impl GuildMembersPager {
    pub(crate) fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit.filter(|limit| *limit != 0) {
            query.insert("limit", limit.to_string());
        }
        if let Some(after) = self.after.as_ref().filter(|value| !value.is_empty()) {
            query.insert("after", after.clone());
        }
        query
    }
}

/// Pager for listing members in a role.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GuildRoleMembersPager {
    pub start_index: Option<String>,
    pub limit: Option<u32>,
}

impl GuildRoleMembersPager {
    pub(crate) fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut query = HashMap::new();
        if let Some(limit) = self.limit.filter(|limit| *limit != 0) {
            query.insert("limit", limit.to_string());
        }
        if let Some(start_index) = self.start_index.as_ref().filter(|value| !value.is_empty()) {
            query.insert("start_index", start_index.clone());
        }
        query
    }
}

/// Members in a role plus the next cursor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GuildRoleMembers {
    #[serde(default)]
    pub data: Vec<Member>,
    #[serde(default)]
    pub next: String,
}

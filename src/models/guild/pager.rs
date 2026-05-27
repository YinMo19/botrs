use crate::models::Pager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn insert_query_param(query: &mut HashMap<String, String>, key: &str, value: &Option<String>) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        query.insert(key.to_string(), value.clone());
    }
}

/// Pager for guild member list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildMembersPager {
    /// Read members after this user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildMembersPager {
    /// Creates a new guild members pager.
    pub fn new(after: impl Into<String>, limit: impl ToString) -> Self {
        Self {
            after: Some(after.into()),
            limit: Some(limit.to_string()),
        }
    }

    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "after", &self.after);
        query
    }
}

impl Pager for GuildMembersPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildMembersPager::query_params(self)
    }
}

/// Pager for guild role member list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildRoleMembersPager {
    /// Start index from the previous response's `next` value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<String>,
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildRoleMembersPager {
    /// Creates a new guild role members pager.
    pub fn new(start_index: impl Into<String>, limit: impl ToString) -> Self {
        Self {
            start_index: Some(start_index.into()),
            limit: Some(limit.to_string()),
        }
    }

    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "start_index", &self.start_index);
        query
    }
}

impl Pager for GuildRoleMembersPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildRoleMembersPager::query_params(self)
    }
}

/// Pager for current-user guild list requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GuildPager {
    /// Read guilds before this guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Read guilds after this guild ID. Takes precedence over `before`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl GuildPager {
    /// Creates an empty guild pager.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    pub fn with_before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    pub fn with_limit(mut self, limit: impl ToString) -> Self {
        self.limit = Some(limit.to_string());
        self
    }

    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> HashMap<String, String> {
        let mut query = HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "after", &self.after);
        if !query.contains_key("after") {
            insert_query_param(&mut query, "before", &self.before);
        }
        query
    }
}

impl Pager for GuildPager {
    fn query_params(&self) -> HashMap<String, String> {
        GuildPager::query_params(self)
    }
}

use crate::models::Pager;
use serde::{Deserialize, Serialize};

fn insert_query_param(
    query: &mut std::collections::HashMap<String, String>,
    key: &str,
    value: &Option<String>,
) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        query.insert(key.to_string(), value.clone());
    }
}

/// Pager for message reaction users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageReactionPager {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>,
    /// Page size, 1-1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl MessageReactionPager {
    /// Creates a new message reaction pager.
    pub fn new(cookie: Option<impl Into<String>>, limit: Option<impl ToString>) -> Self {
        Self {
            cookie: cookie.map(Into::into),
            limit: limit.map(|value| value.to_string()),
        }
    }

    /// Converts the pager to query parameters.
    pub fn query_params(&self) -> std::collections::HashMap<String, String> {
        let mut query = std::collections::HashMap::new();
        insert_query_param(&mut query, "limit", &self.limit);
        insert_query_param(&mut query, "cookie", &self.cookie);
        query
    }
}

impl Pager for MessageReactionPager {
    fn query_params(&self) -> std::collections::HashMap<String, String> {
        MessageReactionPager::query_params(self)
    }
}

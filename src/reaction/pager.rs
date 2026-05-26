use crate::models::Pager;
use serde::{Deserialize, Serialize};

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
        if let Some(limit) = &self.limit {
            query.insert("limit".to_string(), limit.clone());
        }
        if let Some(cookie) = &self.cookie {
            query.insert("cookie".to_string(), cookie.clone());
        }
        query
    }

    /// Query parameter accessor.
    #[allow(non_snake_case)]
    pub fn QueryParams(&self) -> std::collections::HashMap<String, String> {
        self.query_params()
    }
}

impl Pager for MessageReactionPager {
    fn query_params(&self) -> std::collections::HashMap<String, String> {
        MessageReactionPager::query_params(self)
    }
}

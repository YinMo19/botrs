use crate::models::api::BotInfo;
use crate::models::{HasId, Snowflake};
use serde::{Deserialize, Serialize};

/// Represents a user in the QQ Guild system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct User {
    /// The user's unique ID
    #[serde(default)]
    pub id: Snowflake,
    /// The user's username
    #[serde(default)]
    pub username: String,
    /// The user's avatar hash
    #[serde(default)]
    pub avatar: String,
    /// Whether the user is a bot
    #[serde(default)]
    pub bot: bool,
    /// The user's union openid (for group/C2C messages)
    #[serde(default)]
    pub union_openid: String,
    /// The user's union user account
    #[serde(default)]
    pub union_user_account: String,
}

impl HasId for User {
    fn id(&self) -> Option<&Snowflake> {
        Some(&self.id)
    }
}

impl From<BotInfo> for User {
    fn from(bot: BotInfo) -> Self {
        Self {
            id: bot.id,
            username: bot.username,
            avatar: bot.avatar,
            bot: bot.bot,
            union_openid: bot.union_openid,
            union_user_account: bot.union_user_account,
        }
    }
}

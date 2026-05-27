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

impl User {
    /// Creates a new user.
    pub fn new(id: impl Into<Snowflake>, username: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            avatar: String::new(),
            bot: false,
            union_openid: String::new(),
            union_user_account: String::new(),
        }
    }

    /// Gets the user's avatar URL if they have one.
    ///
    /// Returns the full URL to the user's avatar image.
    pub fn avatar_url(&self) -> Option<String> {
        (!self.avatar.is_empty()).then(|| {
            format!(
                "https://thirdqq.qlogo.cn/headimg_dl?dst_uin={}&spec=640",
                self.id
            )
        })
    }

    /// Gets the user's display name.
    ///
    /// This is the same as the username for regular users.
    pub fn display_name(&self) -> &str {
        &self.username
    }

    /// Returns true if this user is a bot.
    pub fn is_bot(&self) -> bool {
        self.bot
    }

    /// Returns true if this user is a human.
    pub fn is_human(&self) -> bool {
        !self.bot
    }

    /// Gets the user's mention string.
    ///
    /// Returns a string that can be used to mention this user in messages.
    pub fn mention(&self) -> String {
        format!("<@!{}>", self.id)
    }
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

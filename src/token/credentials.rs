use serde::{Deserialize, Serialize};

pub const TypeBearer: &str = "Bearer";
pub const TypeQQBot: &str = "QQBot";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QQBotCredentials {
    #[serde(alias = "appid", alias = "appId")]
    pub app_id: String,
    #[serde(alias = "secret", alias = "appSecret")]
    pub app_secret: String,
}

pub type QQBotTokenSource = crate::token::Token;

#[allow(non_snake_case)]
pub fn NewQQBotTokenSource(credentials: &QQBotCredentials) -> QQBotTokenSource {
    crate::token::Token::new(&credentials.app_id, &credentials.app_secret)
}

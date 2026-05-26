use crate::api::BotApi;
use crate::error::Result;
use crate::models::{
    api::MessageResponse,
    message::{Message, SettingGuide, SettingGuideToCreate},
};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

impl BotApi {
    pub(crate) fn parse_message_response(response: Value) -> Result<Message> {
        if response
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .is_empty()
            && let Some(message) = response.get("message")
        {
            return Self::decode_json(message.clone());
        }
        Self::decode_json(response)
    }

    pub(crate) fn mention_content(user_ids: &[String]) -> String {
        user_ids
            .iter()
            .map(|user_id| format!("<@{user_id}>"))
            .collect()
    }

    pub(crate) fn channel_setting_guide_body(user_ids: &[String]) -> SettingGuideToCreate {
        SettingGuideToCreate {
            content: Some(Self::mention_content(user_ids)),
            setting_guide: None,
        }
    }

    pub(crate) fn dm_setting_guide_body(jump_guild_id: &str) -> SettingGuideToCreate {
        SettingGuideToCreate {
            content: None,
            setting_guide: Some(SettingGuide {
                guild_id: jump_guild_id.to_string(),
            }),
        }
    }

    pub(crate) async fn request_message_response_body<B>(
        &self,
        token: &Token,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<MessageResponse>
    where
        B: Serialize + ?Sized,
    {
        let body = serde_json::to_value(body)?;
        self.request_json(token, method, path, None::<&()>, Some(&body))
            .await
    }
}

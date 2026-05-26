use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::{api::MessageResponse, message::Message};
use crate::token::Token;
use reqwest::Method;

impl BotApi {
    /// Posts a channel setting guide message.
    pub async fn post_setting_guide(
        &self,
        token: &Token,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<MessageResponse> {
        self.post_setting_guide_result(token, channel_id, at_user_ids)
            .await
    }

    /// Posts a channel setting guide message and returns the full message.
    pub async fn post_setting_guide_message(
        &self,
        token: &Token,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<Message> {
        self.post_setting_guide_result(token, channel_id, at_user_ids)
            .await
    }

    async fn post_setting_guide_result<T>(
        &self,
        token: &Token,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let body = Self::channel_setting_guide_body(&at_user_ids);
        let path = resource::channel_setting_guide(channel_id);
        self.request_json(token, Method::POST, &path, None::<&()>, Some(&body))
            .await
    }
}

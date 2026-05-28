use crate::api_impl::BotApi;
use crate::error::Result;
use crate::models::api::MessageResponse;
use reqwest::Method;
use serde::Serialize;

impl BotApi {
    pub(crate) async fn request_message_response_body<B>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<MessageResponse>
    where
        B: Serialize + ?Sized,
    {
        let body = serde_json::to_value(body)?;
        self.request_json(method, path, None::<&()>, Some(&body))
            .await
    }
}

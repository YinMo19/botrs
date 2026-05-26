use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::message::{
    DirectMessageSession, DirectMessageToCreate, Message, MessageToCreate,
};
use crate::options::{OpenApiOption, Options};
use reqwest::Method;
use serde_json::Value;

impl BotApi {
    /// Direct-message session creation API.
    #[allow(non_snake_case)]
    pub async fn CreateDirectMessage(
        &self,
        dm: &DirectMessageToCreate,
    ) -> Result<DirectMessageSession> {
        self.CreateDirectMessage_with_options(dm, Self::no_options())
            .await
    }

    /// Direct-message session creation API with request options.
    #[allow(non_snake_case)]
    pub async fn CreateDirectMessage_with_options<I, O>(
        &self,
        dm: &DirectMessageToCreate,
        options: I,
    ) -> Result<DirectMessageSession>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self.create_direct_message(self.token_required()?, dm).await;
        }
        self.request_options_json(
            &opts,
            Method::POST,
            resource::USER_ME_DMS,
            None::<&()>,
            Some(dm),
        )
        .await
    }

    /// Direct-message send API.
    #[allow(non_snake_case)]
    pub async fn PostDirectMessage(
        &self,
        dm: &DirectMessageSession,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.PostDirectMessage_with_options(dm, msg, Self::no_options())
            .await
    }

    /// Direct-message send API with request options.
    #[allow(non_snake_case)]
    pub async fn PostDirectMessage_with_options<I, O>(
        &self,
        dm: &DirectMessageSession,
        msg: &MessageToCreate,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        // Botgo reads dm.GuildID directly and leaves empty values to the request layer.
        let guild_id = dm.guild_id.as_str();
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_direct_message(self.token_required()?, guild_id, msg)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::dms_messages(guild_id),
            None::<&()>,
            Some(msg),
        )
        .await
    }

    /// Direct-message retract API.
    #[allow(non_snake_case)]
    pub async fn RetractDMMessage(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.RetractDMMessage_with_options(guild_id, message_id, Self::no_options())
            .await
    }

    /// Direct-message retract API with request options.
    #[allow(non_snake_case)]
    pub async fn RetractDMMessage_with_options<I, O>(
        &self,
        guild_id: &str,
        message_id: &str,
        options: I,
    ) -> Result<()>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .retract_dm_message(
                    self.token_required()?,
                    guild_id,
                    message_id,
                    Some(opts.hide_tip),
                )
                .await;
        }
        let query = Self::hide_tip_query(opts.hide_tip);
        self.request_options_json::<Value, _, ()>(
            &opts,
            Method::DELETE,
            &resource::dms_message(guild_id, message_id),
            query.as_ref(),
            None,
        )
        .await?;
        Ok(())
    }

    /// DM setting guide API.
    #[allow(non_snake_case)]
    pub async fn PostDMSettingGuide(
        &self,
        dm: &DirectMessageSession,
        jump_guild_id: &str,
    ) -> Result<Message> {
        self.PostDMSettingGuide_with_options(dm, jump_guild_id, Self::no_options())
            .await
    }

    /// DM setting guide API with request options.
    #[allow(non_snake_case)]
    pub async fn PostDMSettingGuide_with_options<I, O>(
        &self,
        dm: &DirectMessageSession,
        jump_guild_id: &str,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        // Botgo reads dm.GuildID directly and leaves empty values to the request layer.
        let guild_id = dm.guild_id.as_str();
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_dm_setting_guide_message(self.token_required()?, guild_id, jump_guild_id)
                .await;
        }
        let body = Self::dm_setting_guide_body(jump_guild_id);
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::dms_setting_guide(guild_id),
            None::<&()>,
            Some(&body),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn test_api(base_url: String) -> BotApi {
        let token = crate::Token::new("APPID_XXXXXX", "SECRET_XXXXXX");
        token
            .set_cached_access_token_for_test("ACCESS_TOKEN_XXXXXX")
            .await;
        let mut http = crate::http::HttpClient::new(30, false).unwrap();
        http.base_url = base_url;
        BotApi::with_token(http, token)
    }

    async fn spawn_json_server() -> (
        String,
        oneshot::Receiver<String>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = [0_u8; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);
            let first_line = request.lines().next().unwrap_or_default().to_string();
            let _ = tx.send(first_line);

            let body = r#"{"id":"message-1"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        (format!("http://{addr}"), rx, handle)
    }

    #[tokio::test]
    async fn dm_compat_routes_preserve_empty_guild_id() {
        let dm = DirectMessageSession::default();
        let msg = MessageToCreate::default();

        let (base_url, first_line, server) = spawn_json_server().await;
        let message = test_api(base_url)
            .await
            .PostDirectMessage(&dm, &msg)
            .await
            .unwrap();
        assert_eq!(message.id.as_deref(), Some("message-1"));
        assert_eq!(first_line.await.unwrap(), "POST /dms//messages HTTP/1.1");
        server.await.unwrap();

        let (base_url, first_line, server) = spawn_json_server().await;
        let message = test_api(base_url)
            .await
            .PostDMSettingGuide(&dm, "guild-1")
            .await
            .unwrap();
        assert_eq!(message.id.as_deref(), Some("message-1"));
        assert_eq!(
            first_line.await.unwrap(),
            "POST /dms//settingguide HTTP/1.1"
        );
        server.await.unwrap();
    }
}

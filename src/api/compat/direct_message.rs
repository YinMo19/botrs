use crate::api::{resource, BotApi};
use crate::error::Result;
use crate::models::message::{DirectMessageSession, DirectMessageToCreate, Message, MessageToCreate};
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
        let guild_id = dm.guild_id.as_deref().ok_or_else(|| {
            crate::BotError::invalid_data("direct message session missing guild_id")
        })?;
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
        let guild_id = dm.guild_id.as_deref().ok_or_else(|| {
            crate::BotError::invalid_data("direct message session missing guild_id")
        })?;
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

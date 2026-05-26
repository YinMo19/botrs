use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::models::message::{ApiMessage, Message, MessageToCreate, MessagesPager};
use crate::options::{OpenApiOption, Options};
use reqwest::Method;
use serde_json::Value;

impl BotApi {
    /// Single message fetch API.
    #[allow(non_snake_case)]
    pub async fn Message(&self, channel_id: &str, message_id: &str) -> Result<Message> {
        self.Message_with_options(channel_id, message_id, Self::no_options())
            .await
    }

    /// Single message fetch API with request options.
    #[allow(non_snake_case)]
    pub async fn Message_with_options<I, O>(
        &self,
        channel_id: &str,
        message_id: &str,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .get_message(self.token_required()?, channel_id, message_id)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::GET,
            &resource::channel_message(channel_id, message_id),
            None::<&()>,
            None::<&()>,
        )
        .await
    }

    /// Message list API.
    #[allow(non_snake_case)]
    pub async fn Messages(&self, channel_id: &str, pager: &MessagesPager) -> Result<Vec<Message>> {
        self.Messages_with_options(channel_id, pager, Self::no_options())
            .await
    }

    /// Message list API with request options.
    #[allow(non_snake_case)]
    pub async fn Messages_with_options<I, O>(
        &self,
        channel_id: &str,
        pager: &MessagesPager,
        options: I,
    ) -> Result<Vec<Message>>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .get_messages(self.token_required()?, channel_id, pager)
                .await;
        }
        let params = pager.query_params();
        self.request_options_json(
            &opts,
            Method::GET,
            &resource::channel_messages(channel_id),
            if params.is_empty() {
                None
            } else {
                Some(&params)
            },
            None::<&()>,
        )
        .await
    }

    /// Channel message send API.
    #[allow(non_snake_case)]
    pub async fn PostMessage(&self, channel_id: &str, msg: &MessageToCreate) -> Result<Message> {
        self.PostMessage_with_options(channel_id, msg, Self::no_options())
            .await
    }

    /// Channel message send API with request options.
    #[allow(non_snake_case)]
    pub async fn PostMessage_with_options<I, O>(
        &self,
        channel_id: &str,
        msg: &MessageToCreate,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_message_to_create(self.token_required()?, channel_id, msg)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::channel_messages(channel_id),
            None::<&()>,
            Some(msg),
        )
        .await
    }

    /// Channel message edit API.
    #[allow(non_snake_case)]
    pub async fn PatchMessage(
        &self,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
    ) -> Result<Message> {
        self.PatchMessage_with_options(channel_id, message_id, msg, Self::no_options())
            .await
    }

    /// Channel message edit API with request options.
    #[allow(non_snake_case)]
    pub async fn PatchMessage_with_options<I, O>(
        &self,
        channel_id: &str,
        message_id: &str,
        msg: &MessageToCreate,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .patch_message_to_create(self.token_required()?, channel_id, message_id, msg)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::PATCH,
            &resource::channel_message(channel_id, message_id),
            None::<&()>,
            Some(msg),
        )
        .await
    }

    /// Channel message retract API.
    #[allow(non_snake_case)]
    pub async fn RetractMessage(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.RetractMessage_with_options(channel_id, message_id, Self::no_options())
            .await
    }

    /// Channel message retract API with request options.
    #[allow(non_snake_case)]
    pub async fn RetractMessage_with_options<I, O>(
        &self,
        channel_id: &str,
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
                .recall_message_botgo(
                    self.token_required()?,
                    channel_id,
                    message_id,
                    opts.hide_tip,
                )
                .await;
        }
        let query = Self::hide_tip_query(opts.hide_tip);
        self.request_options_json::<Value, _, ()>(
            &opts,
            Method::DELETE,
            &resource::channel_message(channel_id, message_id),
            query.as_ref(),
            None,
        )
        .await?;
        Ok(())
    }

    /// Setting guide API.
    #[allow(non_snake_case)]
    pub async fn PostSettingGuide(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
    ) -> Result<Message> {
        self.PostSettingGuide_with_options(channel_id, at_user_ids, Self::no_options())
            .await
    }

    /// Setting guide API with request options.
    #[allow(non_snake_case)]
    pub async fn PostSettingGuide_with_options<I, O>(
        &self,
        channel_id: &str,
        at_user_ids: Vec<String>,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_setting_guide_message(self.token_required()?, channel_id, at_user_ids)
                .await;
        }
        let body = Self::channel_setting_guide_body(&at_user_ids);
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::channel_setting_guide(channel_id),
            None::<&()>,
            Some(&body),
        )
        .await
    }

    /// Group message send API.
    #[allow(non_snake_case)]
    pub async fn PostGroupMessage(
        &self,
        group_id: &str,
        msg: impl Into<ApiMessage>,
    ) -> Result<Message> {
        self.PostGroupMessage_with_options(group_id, msg, Self::no_options())
            .await
    }

    /// Group message send API with request options.
    #[allow(non_snake_case)]
    pub async fn PostGroupMessage_with_options<I, O>(
        &self,
        group_id: &str,
        msg: impl Into<ApiMessage>,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let msg = msg.into();
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_group_api_message(self.token_required()?, group_id, &msg)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::group_send(group_id, msg.send_type()),
            None::<&()>,
            Some(&msg),
        )
        .await
    }

    /// C2C message send API.
    #[allow(non_snake_case)]
    pub async fn PostC2CMessage(
        &self,
        user_id: &str,
        msg: impl Into<ApiMessage>,
    ) -> Result<Message> {
        self.PostC2CMessage_with_options(user_id, msg, Self::no_options())
            .await
    }

    /// C2C message send API with request options.
    #[allow(non_snake_case)]
    pub async fn PostC2CMessage_with_options<I, O>(
        &self,
        user_id: &str,
        msg: impl Into<ApiMessage>,
        options: I,
    ) -> Result<Message>
    where
        I: IntoIterator<Item = O>,
        O: Into<OpenApiOption>,
    {
        let msg = msg.into();
        let opts = Options::from_options(options);
        if opts.url.is_none() {
            return self
                .post_c2c_api_message(self.token_required()?, user_id, &msg)
                .await;
        }
        self.request_options_json(
            &opts,
            Method::POST,
            &resource::c2c_send(user_id, msg.send_type()),
            None::<&()>,
            Some(&msg),
        )
        .await
    }

    /// C2C message retract API.
    #[allow(non_snake_case)]
    pub async fn RetractC2CMessage(&self, user_id: &str, message_id: &str) -> Result<()> {
        self.RetractC2CMessage_with_options(user_id, message_id, Self::no_options())
            .await
    }

    /// C2C message retract API with request options.
    #[allow(non_snake_case)]
    pub async fn RetractC2CMessage_with_options<I, O>(
        &self,
        user_id: &str,
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
                .retract_c2c_message(
                    self.token_required()?,
                    user_id,
                    message_id,
                    Some(opts.hide_tip),
                )
                .await;
        }
        let query = Self::hide_tip_query(opts.hide_tip);
        self.request_options_json::<Value, _, ()>(
            &opts,
            Method::DELETE,
            &resource::c2c_message(user_id, message_id),
            query.as_ref(),
            None,
        )
        .await?;
        Ok(())
    }

    /// Group message retract API.
    #[allow(non_snake_case)]
    pub async fn RetractGroupMessage(&self, group_id: &str, message_id: &str) -> Result<()> {
        self.RetractGroupMessage_with_options(group_id, message_id, Self::no_options())
            .await
    }

    /// Group message retract API with request options.
    #[allow(non_snake_case)]
    pub async fn RetractGroupMessage_with_options<I, O>(
        &self,
        group_id: &str,
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
                .retract_group_message(
                    self.token_required()?,
                    group_id,
                    message_id,
                    Some(opts.hide_tip),
                )
                .await;
        }
        let query = Self::hide_tip_query(opts.hide_tip);
        self.request_options_json::<Value, _, ()>(
            &opts,
            Method::DELETE,
            &resource::group_message(group_id, message_id),
            query.as_ref(),
            None,
        )
        .await?;
        Ok(())
    }
}

use super::{APIVersion, BotApi, resource};
use crate::audio::AudioControl;
use crate::error::Result;
use crate::models::{
    announce::{Announce, AnnouncesType, ChannelAnnouncesToCreate, GuildAnnouncesToCreate},
    api::{GatewayResponse, PinsMessage},
    channel::{
        Channel, ChannelPermissions, ChannelRolesPermissions, ChannelValueObject,
        UpdateChannelPermissions,
    },
    guild::{
        Guild, GuildMembersPager, GuildPager, GuildRole, GuildRoleMembersPager, GuildRoles, Member,
        MemberAddRoleBody, UpdateGuildMute, UpdateGuildMuteResponse, UpdateResult,
    },
    message::{
        ApiMessage, DirectMessageSession, DirectMessageToCreate, Message, MessageToCreate,
        MessagesPager,
    },
    message_setting::MessageSetting,
    permission::{APIPermissionDemand, APIPermissionDemandToCreate, APIPermissions},
    schedule::Schedule,
    user::User,
    webhook::{HttpIdentity, HttpReady, HttpSession},
};
use crate::options::{OpenApiOption, Options};
use crate::reaction::{Emoji as ReactionEmoji, MessageReactionPager, ReactionUsers};
use crate::token::Token;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

impl BotApi {
    /// Channel announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateChannelAnnounces(
        &self,
        channel_id: &str,
        announce: &ChannelAnnouncesToCreate,
    ) -> Result<Announce> {
        self.create_channel_announce(self.token_required()?, channel_id, &announce.message_id)
            .await
    }

    /// Channel announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteChannelAnnounces(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_channel_announce(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Channel announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanChannelAnnounces(&self, channel_id: &str) -> Result<()> {
        self.clean_channel_announces(self.token_required()?, channel_id)
            .await
    }

    /// Guild announce creation API.
    #[allow(non_snake_case)]
    pub async fn CreateGuildAnnounces(
        &self,
        guild_id: &str,
        announce: &GuildAnnouncesToCreate,
    ) -> Result<Announce> {
        if !announce.recommend_channels.is_empty() {
            self.create_recommend_announce(
                self.token_required()?,
                guild_id,
                AnnouncesType::from(announce.announces_type as u8),
                announce.recommend_channels.clone(),
            )
            .await
        } else {
            self.create_guild_announce(
                self.token_required()?,
                guild_id,
                &announce.channel_id,
                &announce.message_id,
            )
            .await
        }
    }

    /// Guild announce delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteGuildAnnounces(&self, guild_id: &str, message_id: &str) -> Result<()> {
        self.delete_guild_announce(self.token_required()?, guild_id, message_id)
            .await
    }

    /// Guild announces clean API.
    #[allow(non_snake_case)]
    pub async fn CleanGuildAnnounces(&self, guild_id: &str) -> Result<()> {
        self.clean_guild_announces(self.token_required()?, guild_id)
            .await
    }

    /// API permissions list API.
    #[allow(non_snake_case)]
    pub async fn GetAPIPermissions(&self, guild_id: &str) -> Result<APIPermissions> {
        self.get_api_permissions(self.token_required()?, guild_id)
            .await
    }

    /// API permission demand API.
    #[allow(non_snake_case)]
    pub async fn RequireAPIPermissions(
        &self,
        guild_id: &str,
        demand: &APIPermissionDemandToCreate,
    ) -> Result<APIPermissionDemand> {
        self.require_api_permissions(self.token_required()?, guild_id, demand)
            .await
    }

    /// Audio control API.
    #[allow(non_snake_case)]
    pub async fn PostAudio(
        &self,
        channel_id: &str,
        audio_control: &AudioControl,
    ) -> Result<AudioControl> {
        self.post_audio(self.token_required()?, channel_id, audio_control)
            .await
    }

    /// Microphone enable API.
    #[allow(non_snake_case)]
    pub async fn PutMic(&self, channel_id: &str) -> Result<()> {
        self.on_microphone(self.token_required()?, channel_id).await
    }

    /// Microphone disable API.
    #[allow(non_snake_case)]
    pub async fn DeleteMic(&self, channel_id: &str) -> Result<()> {
        self.off_microphone(self.token_required()?, channel_id)
            .await
    }

    /// Setup constructor.
    #[allow(non_snake_case)]
    pub fn Setup(
        bot_app_id: impl Into<String>,
        secret: impl Into<String>,
        in_sandbox: bool,
    ) -> Result<(Self, Token)> {
        Self::setup(bot_app_id, secret, in_sandbox)
    }

    /// OpenAPI version method.
    #[allow(non_snake_case)]
    pub const fn Version(&self) -> APIVersion {
        self.version()
    }

    /// Timeout configuration method.
    #[allow(non_snake_case)]
    pub fn WithTimeout(&self, duration: Duration) -> Result<Self> {
        self.with_timeout(duration)
    }

    /// Debug configuration method.
    #[allow(non_snake_case)]
    pub fn SetDebug(&self, debug: bool) -> Self {
        self.set_debug(debug)
    }

    /// App ID accessor for the v1 OpenAPI implementation.
    #[allow(non_snake_case)]
    pub fn GetAppID(&self) -> &str {
        self.get_app_id()
    }

    /// Transport passthrough.
    #[allow(non_snake_case)]
    pub async fn Transport<B>(&self, method: Method, url: &str, body: Option<&B>) -> Result<Vec<u8>>
    where
        B: Serialize + ?Sized,
    {
        self.transport(self.token_required()?, method, url, body)
            .await
    }

    /// Trace ID accessor.
    #[allow(non_snake_case)]
    pub fn TraceID(&self) -> String {
        self.trace_id()
    }

    /// Channel lookup API.
    #[allow(non_snake_case)]
    pub async fn Channel(&self, channel_id: &str) -> Result<Channel> {
        self.get_channel(self.token_required()?, channel_id).await
    }

    /// Channel list API.
    #[allow(non_snake_case)]
    pub async fn Channels(&self, guild_id: &str) -> Result<Vec<Channel>> {
        self.get_channels(self.token_required()?, guild_id).await
    }

    /// Channel creation API.
    #[allow(non_snake_case)]
    pub async fn PostChannel(&self, guild_id: &str, value: &ChannelValueObject) -> Result<Channel> {
        self.post_channel(self.token_required()?, guild_id, value)
            .await
    }

    /// Channel update API.
    #[allow(non_snake_case)]
    pub async fn PatchChannel(
        &self,
        channel_id: &str,
        value: &ChannelValueObject,
    ) -> Result<Channel> {
        self.patch_channel(self.token_required()?, channel_id, value)
            .await
    }

    /// Channel delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteChannel(&self, channel_id: &str) -> Result<()> {
        self.delete_channel(self.token_required()?, channel_id)
            .await?;
        Ok(())
    }

    /// Private channel creation API.
    #[allow(non_snake_case)]
    pub async fn CreatePrivateChannel(
        &self,
        guild_id: &str,
        value: &ChannelValueObject,
        user_ids: Vec<String>,
    ) -> Result<Channel> {
        self.create_private_channel(self.token_required()?, guild_id, value, user_ids)
            .await
    }

    /// Voice channel member list API.
    #[allow(non_snake_case)]
    pub async fn ListVoiceChannelMembers(&self, channel_id: &str) -> Result<Vec<Member>> {
        self.list_voice_channel_members(self.token_required()?, channel_id)
            .await
    }

    /// Channel permissions API.
    #[allow(non_snake_case)]
    pub async fn ChannelPermissions(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<ChannelPermissions> {
        self.get_channel_user_permissions(self.token_required()?, channel_id, user_id)
            .await
    }

    /// Channel permissions update API.
    #[allow(non_snake_case)]
    pub async fn PutChannelPermissions(
        &self,
        channel_id: &str,
        user_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.put_channel_permissions(self.token_required()?, channel_id, user_id, permissions)
            .await
    }

    /// Channel role permissions API.
    #[allow(non_snake_case)]
    pub async fn ChannelRolesPermissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelRolesPermissions> {
        self.get_channel_role_permissions(self.token_required()?, channel_id, role_id)
            .await
    }

    /// Channel role permissions update API.
    #[allow(non_snake_case)]
    pub async fn PutChannelRolesPermissions(
        &self,
        channel_id: &str,
        role_id: &str,
        permissions: &UpdateChannelPermissions,
    ) -> Result<()> {
        self.put_channel_roles_permissions(self.token_required()?, channel_id, role_id, permissions)
            .await
    }

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

    /// Websocket gateway address API.
    #[allow(non_snake_case)]
    pub async fn WS(
        &self,
        _params: Option<&HashMap<String, String>>,
        _body: Option<&str>,
    ) -> Result<GatewayResponse> {
        self.get_gateway(self.token_required()?).await
    }

    /// Guild lookup API.
    #[allow(non_snake_case)]
    pub async fn Guild(&self, guild_id: &str) -> Result<Guild> {
        self.get_guild(self.token_required()?, guild_id).await
    }

    /// Guild member lookup API.
    #[allow(non_snake_case)]
    pub async fn GuildMember(&self, guild_id: &str, user_id: &str) -> Result<Member> {
        self.get_guild_member(self.token_required()?, guild_id, user_id)
            .await
    }

    /// Guild member list API.
    #[allow(non_snake_case)]
    pub async fn GuildMembers(
        &self,
        guild_id: &str,
        pager: &GuildMembersPager,
    ) -> Result<Vec<Member>> {
        self.get_guild_members_with_pager(self.token_required()?, guild_id, pager)
            .await
    }

    /// Guild role member list API.
    #[allow(non_snake_case)]
    pub async fn GuildRoleMembers(
        &self,
        guild_id: &str,
        role_id: &str,
        pager: &GuildRoleMembersPager,
    ) -> Result<(Vec<Member>, String)> {
        let members = self
            .get_guild_role_members_with_pager(self.token_required()?, guild_id, role_id, pager)
            .await?;
        Ok((members.data, members.next))
    }

    /// Guild member delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteGuildMember(
        &self,
        guild_id: &str,
        user_id: &str,
        options: impl IntoIterator<Item = crate::models::guild::MemberDeleteOption>,
    ) -> Result<()> {
        let mut delete_options = crate::models::guild::MemberDeleteOptions::new();
        for option in options {
            option(&mut delete_options);
        }
        self.delete_member_with_options(self.token_required()?, guild_id, user_id, &delete_options)
            .await
    }

    /// Guild mute API.
    #[allow(non_snake_case)]
    pub async fn GuildMute(&self, guild_id: &str, mute: &UpdateGuildMute) -> Result<()> {
        let token = self.token_required()?;
        let path = resource::guild_mute(guild_id);
        self.http
            .patch(token, &path, None::<&()>, Some(mute))
            .await?;
        Ok(())
    }

    /// Interaction update API.
    #[allow(non_snake_case)]
    pub async fn PutInteraction(&self, interaction_id: &str, body: &str) -> Result<()> {
        self.put_interaction(self.token_required()?, interaction_id, body)
            .await
    }

    /// Current bot user API.
    #[allow(non_snake_case)]
    pub async fn Me(&self) -> Result<User> {
        Ok(self.get_bot_info(self.token_required()?).await?.into())
    }

    /// Current bot guild list API.
    #[allow(non_snake_case)]
    pub async fn MeGuilds(&self, pager: &GuildPager) -> Result<Vec<Guild>> {
        self.get_guilds_with_pager(self.token_required()?, pager)
            .await
    }

    /// Member role add API.
    #[allow(non_snake_case)]
    pub async fn MemberAddRole(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        value: &MemberAddRoleBody,
    ) -> Result<()> {
        self.member_add_role(self.token_required()?, guild_id, role_id, user_id, value)
            .await
    }

    /// Member role delete API.
    #[allow(non_snake_case)]
    pub async fn MemberDeleteRole(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        value: &MemberAddRoleBody,
    ) -> Result<()> {
        self.member_delete_role(self.token_required()?, guild_id, role_id, user_id, value)
            .await
    }

    /// Single member mute API.
    #[allow(non_snake_case)]
    pub async fn MemberMute(
        &self,
        guild_id: &str,
        user_id: &str,
        mute: &UpdateGuildMute,
    ) -> Result<()> {
        let token = self.token_required()?;
        let path = resource::guild_member_mute(guild_id, user_id);
        self.http
            .patch(token, &path, None::<&()>, Some(mute))
            .await?;
        Ok(())
    }

    /// Batch member mute API.
    #[allow(non_snake_case)]
    pub async fn MultiMemberMute(
        &self,
        guild_id: &str,
        mute: &UpdateGuildMute,
    ) -> Result<UpdateGuildMuteResponse> {
        self.multi_member_mute(self.token_required()?, guild_id, mute)
            .await
    }

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
                .recall_message(
                    self.token_required()?,
                    channel_id,
                    message_id,
                    Some(opts.hide_tip),
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

    /// Message reaction add API.
    #[allow(non_snake_case)]
    pub async fn CreateMessageReaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.create_message_reaction(self.token_required()?, channel_id, message_id, emoji)
            .await
    }

    /// Message reaction delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteOwnMessageReaction(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
    ) -> Result<()> {
        self.delete_own_message_reaction(self.token_required()?, channel_id, message_id, emoji)
            .await
    }

    /// Message reaction users API.
    #[allow(non_snake_case)]
    pub async fn GetMessageReactionUsers(
        &self,
        channel_id: &str,
        message_id: &str,
        emoji: &ReactionEmoji,
        pager: &MessageReactionPager,
    ) -> Result<ReactionUsers> {
        self.get_message_reaction_users(
            self.token_required()?,
            channel_id,
            message_id,
            emoji,
            pager,
        )
        .await
    }

    /// Message setting API.
    #[allow(non_snake_case)]
    pub async fn GetMessageSetting(&self, guild_id: &str) -> Result<MessageSetting> {
        self.get_message_setting(self.token_required()?, guild_id)
            .await
    }

    /// Pins add API.
    #[allow(non_snake_case)]
    pub async fn AddPins(&self, channel_id: &str, message_id: &str) -> Result<PinsMessage> {
        self.put_pin(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins delete API.
    #[allow(non_snake_case)]
    pub async fn DeletePins(&self, channel_id: &str, message_id: &str) -> Result<()> {
        self.delete_pin(self.token_required()?, channel_id, message_id)
            .await
    }

    /// Pins clean API.
    #[allow(non_snake_case)]
    pub async fn CleanPins(&self, channel_id: &str) -> Result<()> {
        self.clean_pins(self.token_required()?, channel_id).await
    }

    /// Pins list API.
    #[allow(non_snake_case)]
    pub async fn GetPins(&self, channel_id: &str) -> Result<PinsMessage> {
        self.get_pins(self.token_required()?, channel_id).await
    }

    /// Role list API.
    #[allow(non_snake_case)]
    pub async fn Roles(&self, guild_id: &str) -> Result<GuildRoles> {
        self.get_guild_roles(self.token_required()?, guild_id).await
    }

    /// Role creation API.
    #[allow(non_snake_case)]
    pub async fn PostRole(&self, guild_id: &str, role: &GuildRole) -> Result<UpdateResult> {
        self.create_guild_role_with_update(self.token_required()?, guild_id, role.clone())
            .await
    }

    /// Role update API.
    #[allow(non_snake_case)]
    pub async fn PatchRole(
        &self,
        guild_id: &str,
        role_id: &str,
        role: &GuildRole,
    ) -> Result<UpdateResult> {
        self.update_guild_role_with_update(self.token_required()?, guild_id, role_id, role.clone())
            .await
    }

    /// Role delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteRole(&self, guild_id: &str, role_id: &str) -> Result<()> {
        self.delete_guild_role(self.token_required()?, guild_id, role_id)
            .await
    }

    /// Schedule list API.
    #[allow(non_snake_case)]
    pub async fn ListSchedules(&self, channel_id: &str, since: u64) -> Result<Vec<Schedule>> {
        let since = since.to_string();
        self.get_schedules(self.token_required()?, channel_id, Some(since.as_str()))
            .await
    }

    /// Schedule lookup API.
    #[allow(non_snake_case)]
    pub async fn GetSchedule(&self, channel_id: &str, schedule_id: &str) -> Result<Schedule> {
        self.get_schedule(self.token_required()?, channel_id, schedule_id)
            .await
    }

    /// Schedule creation API.
    #[allow(non_snake_case)]
    pub async fn CreateSchedule(&self, channel_id: &str, schedule: &Schedule) -> Result<Schedule> {
        self.create_schedule_with_model(self.token_required()?, channel_id, schedule)
            .await
    }

    /// Schedule modification API.
    #[allow(non_snake_case)]
    pub async fn ModifySchedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
        schedule: &Schedule,
    ) -> Result<Schedule> {
        self.update_schedule_with_model(self.token_required()?, channel_id, schedule_id, schedule)
            .await
    }

    /// Schedule delete API.
    #[allow(non_snake_case)]
    pub async fn DeleteSchedule(&self, channel_id: &str, schedule_id: &str) -> Result<()> {
        self.delete_schedule(self.token_required()?, channel_id, schedule_id)
            .await?;
        Ok(())
    }

    /// HTTP webhook session creation API.
    #[allow(non_snake_case)]
    pub async fn CreateSession(&self, identity: HttpIdentity) -> Result<HttpReady> {
        self.create_session(self.token_required()?, &identity).await
    }

    /// HTTP webhook session check API.
    #[allow(non_snake_case)]
    pub async fn CheckSessions(&self) -> Result<Vec<HttpSession>> {
        self.check_sessions(self.token_required()?).await
    }

    /// HTTP webhook session list API.
    #[allow(non_snake_case)]
    pub async fn SessionList(&self) -> Result<Vec<HttpSession>> {
        self.session_list(self.token_required()?).await
    }

    /// HTTP webhook session remove API.
    #[allow(non_snake_case)]
    pub async fn RemoveSession(&self, session_id: &str) -> Result<()> {
        self.remove_session(self.token_required()?, session_id)
            .await
    }
}

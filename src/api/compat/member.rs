use crate::api::{resource, BotApi};
use crate::error::Result;
use crate::models::guild::{MemberAddRoleBody, UpdateGuildMute, UpdateGuildMuteResponse};

impl BotApi {
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
}

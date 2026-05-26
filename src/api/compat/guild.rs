use crate::api::{resource, BotApi};
use crate::error::Result;
use crate::models::guild::{
    Guild, GuildMembersPager, GuildRoleMembersPager, Member, MemberDeleteOption,
    MemberDeleteOptions, UpdateGuildMute,
};

impl BotApi {
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
        options: impl IntoIterator<Item = MemberDeleteOption>,
    ) -> Result<()> {
        let mut delete_options = MemberDeleteOptions::new();
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
}

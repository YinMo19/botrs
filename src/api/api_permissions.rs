use super::{BotApi, resource};
use crate::error::Result;
use crate::models::permission::{
    APIPermission, APIPermissionDemand, APIPermissionDemandIdentify, APIPermissionDemandToCreate,
    APIPermissions,
};
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Permission APIs

    /// Gets the list of API permissions for a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID
    ///
    /// # Returns
    ///
    /// List of API permissions.
    pub async fn get_api_permissions(
        &self,
        token: &Token,
        guild_id: &str,
    ) -> Result<APIPermissions> {
        debug!("Getting permissions for guild {}", guild_id);

        let path = resource::api_permission(guild_id);
        let response = self.http.get(token, &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    pub async fn get_permissions(
        &self,
        token: &Token,
        guild_id: &str,
    ) -> Result<Vec<APIPermission>> {
        Ok(self.get_api_permissions(token, guild_id).await?.api_list)
    }

    /// Creates an API permission demand request with a structured body.
    pub async fn require_api_permissions(
        &self,
        token: &Token,
        guild_id: &str,
        demand: &APIPermissionDemandToCreate,
    ) -> Result<APIPermissionDemand> {
        debug!("Creating permission demand in guild {}", guild_id);

        let path = resource::api_permission_demand(guild_id);
        let response = self
            .http
            .post(token, &path, None::<&()>, Some(demand))
            .await?;
        Self::decode_json(response)
    }

    /// Creates an API permission demand request.
    ///
    /// # Arguments
    ///
    /// * `token` - Authentication token
    /// * `guild_id` - The guild ID where permission is requested
    /// * `channel_id` - The channel ID where the request will be sent
    /// * `api_identify` - The API identifier for which permission is requested
    /// * `desc` - Description explaining why the permission is needed
    ///
    /// # Returns
    ///
    /// The created permission demand.
    pub async fn post_permission_demand(
        &self,
        token: &Token,
        guild_id: &str,
        channel_id: &str,
        api_identify: APIPermissionDemandIdentify,
        desc: &str,
    ) -> Result<APIPermissionDemand> {
        debug!("Creating permission demand in guild {}", guild_id);

        let demand = APIPermissionDemandToCreate::new(channel_id, api_identify, desc);
        self.require_api_permissions(token, guild_id, &demand).await
    }
}

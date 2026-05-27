use super::{BotApi, resource};
use crate::error::Result;
use crate::models::permission::{
    APIPermissionDemand, APIPermissionDemandIdentify, APIPermissionDemandToCreate, APIPermissions,
};
use tracing::debug;

impl BotApi {
    // Permission APIs

    /// Lists API permissions available in a guild.
    pub async fn get_api_permissions(&self, guild_id: &str) -> Result<APIPermissions> {
        debug!("Getting permissions for guild {}", guild_id);

        let path = resource::api_permission(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
        Self::decode_json(response)
    }

    /// Creates an API permission demand request with a structured body.
    pub async fn require_api_permissions(
        &self,
        guild_id: &str,
        demand: &APIPermissionDemandToCreate,
    ) -> Result<APIPermissionDemand> {
        debug!("Creating permission demand in guild {}", guild_id);

        let path = resource::api_permission_demand(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(demand))
            .await?;
        Self::decode_json(response)
    }

    /// Creates an API permission demand request from inline fields.
    pub async fn post_permission_demand(
        &self,
        guild_id: &str,
        channel_id: &str,
        api_identify: APIPermissionDemandIdentify,
        desc: &str,
    ) -> Result<APIPermissionDemand> {
        debug!("Creating permission demand in guild {}", guild_id);

        let demand = APIPermissionDemandToCreate::new(channel_id, api_identify, desc);
        self.require_api_permissions(guild_id, &demand).await
    }
}

use crate::api::BotApi;
use crate::error::Result;
use crate::models::permission::{APIPermissionDemand, APIPermissionDemandToCreate, APIPermissions};

impl BotApi {
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
}

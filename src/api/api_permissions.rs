use super::{BotApi, resource};
use crate::error::Result;
use crate::models::Snowflake;
use crate::models::permission::{APIPermissionDemand, APIPermissionDemandIdentify, APIPermissions};
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PermissionDemandBody {
    channel_id: Snowflake,
    api_identify: APIPermissionDemandIdentify,
    desc: String,
}

impl BotApi {
    // Permission APIs

    /// Lists API permissions available in a guild.
    pub async fn get_api_permissions(&self, guild_id: &str) -> Result<APIPermissions> {
        debug!("Getting permissions for guild {}", guild_id);

        let path = resource::api_permission(guild_id);
        let response = self.http.get(self.token(), &path, None::<&()>).await?;
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

        let demand = PermissionDemandBody {
            channel_id: channel_id.to_string(),
            api_identify,
            desc: desc.to_string(),
        };
        let path = resource::api_permission_demand(guild_id);
        let response = self
            .http
            .post(self.token(), &path, None::<&()>, Some(&demand))
            .await?;
        Self::decode_json(response)
    }
}

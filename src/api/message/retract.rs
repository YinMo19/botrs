use crate::api::{BotApi, resource};
use crate::error::Result;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    /// Recalls (deletes) a message.
    pub async fn recall_message(
        &self,
        token: &Token,
        channel_id: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Recalling message {} in channel {}", message_id, channel_id);
        let params = Self::recall_hide_tip_query(hidetip);
        let path = resource::channel_message(channel_id, message_id);
        self.http.delete(token, &path, Some(&params)).await?;
        Ok(())
    }

    /// Recalls a C2C message.
    pub async fn retract_c2c_message(
        &self,
        token: &Token,
        openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!("Retracting C2C message {} for {}", message_id, openid);
        self.retract_open_message(token, resource::c2c_message(openid, message_id), hidetip)
            .await
    }

    /// Recalls a group message.
    pub async fn retract_group_message(
        &self,
        token: &Token,
        group_openid: &str,
        message_id: &str,
        hidetip: Option<bool>,
    ) -> Result<()> {
        debug!(
            "Retracting group message {} for {}",
            message_id, group_openid
        );
        self.retract_open_message(
            token,
            resource::group_message(group_openid, message_id),
            hidetip,
        )
        .await
    }

    pub(crate) async fn retract_open_message(
        &self,
        token: &Token,
        path: String,
        hidetip: Option<bool>,
    ) -> Result<()> {
        let params = Self::hide_tip_query(hidetip.unwrap_or(false));
        self.http.delete(token, &path, params.as_ref()).await?;
        Ok(())
    }
}

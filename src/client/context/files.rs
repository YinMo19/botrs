use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn post_group_file(
        &self,
        group_openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<serde_json::Value> {
        self.api
            .post_group_file(&self.token, group_openid, file_type, url, srv_send_msg)
            .await
    }

    /// Sends a file to a C2C chat.
    ///
    /// # Arguments
    ///
    /// * `openid` - The user's OpenID
    /// * `file_type` - The file type (1=image, 2=video, 3=audio, 4=file)
    /// * `url` - The file URL
    /// * `srv_send_msg` - Whether to send as message
    ///
    /// # Returns
    ///
    /// The file upload response.

    pub async fn post_c2c_file(
        &self,
        openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<serde_json::Value> {
        self.api
            .post_c2c_file(&self.token, openid, file_type, url, srv_send_msg)
            .await
    }
}

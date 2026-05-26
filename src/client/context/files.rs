use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn post_group_file(
        &self,
        group_openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        self.api
            .post_group_file(&self.token, group_openid, file_type, url, srv_send_msg)
            .await
    }

    /// Sends a file message to a C2C chat.
    ///
    /// `file_type` follows the platform numeric values: image, video, audio,
    /// or generic file.
    pub async fn post_c2c_file(
        &self,
        openid: &str,
        file_type: u32,
        url: &str,
        srv_send_msg: Option<bool>,
    ) -> Result<Media> {
        self.api
            .post_c2c_file(&self.token, openid, file_type, url, srv_send_msg)
            .await
    }
}

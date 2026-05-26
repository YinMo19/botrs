use super::Context;
use crate::client::prelude::*;

impl Context {
    /// Lists forum threads in a channel.
    pub async fn get_threads(&self, channel_id: &str) -> Result<ForumRsp> {
        self.api.get_threads(&self.token, channel_id).await
    }

    /// Gets one forum thread's detail.
    pub async fn get_thread_detail(&self, channel_id: &str, thread_id: &str) -> Result<ThreadInfo> {
        self.api
            .get_thread_detail(&self.token, channel_id, thread_id)
            .await
    }

    /// Creates a forum thread from inline fields.
    pub async fn post_thread(
        &self,
        channel_id: &str,
        title: &str,
        content: &str,
        format: Format,
    ) -> Result<PostThreadRsp> {
        self.api
            .post_thread(&self.token, channel_id, title, content, format)
            .await
    }

    /// Creates a forum thread from a structured body.
    pub async fn put_thread(
        &self,
        channel_id: &str,
        thread: &ThreadToCreate,
    ) -> Result<PostThreadRsp> {
        self.api.put_thread(&self.token, channel_id, thread).await
    }

    /// Deletes one forum thread.
    pub async fn delete_thread(&self, channel_id: &str, thread_id: &str) -> Result<()> {
        self.api
            .delete_thread(&self.token, channel_id, thread_id)
            .await
    }
}

use super::Context;
use crate::client::prelude::*;

impl Context {
    pub async fn update_audio(&self, channel_id: &str, audio_control: &AudioControl) -> Result<()> {
        self.api
            .update_audio(&self.token, channel_id, audio_control)
            .await
    }

    /// Updates audio control and returns the submitted control body.
    pub async fn post_audio(
        &self,
        channel_id: &str,
        audio_control: &AudioControl,
    ) -> Result<AudioControl> {
        self.api
            .post_audio(&self.token, channel_id, audio_control)
            .await
    }

    /// Enables the bot microphone in an audio channel.
    pub async fn on_microphone(&self, channel_id: &str) -> Result<()> {
        self.api.on_microphone(&self.token, channel_id).await
    }

    /// Disables the bot microphone in an audio channel.
    pub async fn off_microphone(&self, channel_id: &str) -> Result<()> {
        self.api.off_microphone(&self.token, channel_id).await
    }
}

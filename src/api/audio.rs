use super::{BotApi, resource};
use crate::audio::AudioControl;
use crate::error::Result;
use crate::token::Token;
use tracing::debug;

impl BotApi {
    // Audio APIs

    /// Updates audio control and discards the submitted body.
    pub async fn update_audio(
        &self,
        token: &Token,
        channel_id: &str,
        audio_control: &AudioControl,
    ) -> Result<()> {
        self.post_audio(token, channel_id, audio_control).await?;
        Ok(())
    }

    /// Updates audio control and returns the submitted control body.
    pub async fn post_audio(
        &self,
        token: &Token,
        channel_id: &str,
        audio_control: &AudioControl,
    ) -> Result<AudioControl> {
        debug!("Updating audio in channel {}", channel_id);
        let path = resource::channel_audio(channel_id);
        let _response = self
            .http
            .post(token, &path, None::<&()>, Some(audio_control))
            .await?;
        Ok(audio_control.clone())
    }

    /// Enables the bot microphone in an audio channel.
    pub async fn on_microphone(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Turning on microphone in channel {}", channel_id);
        let path = resource::channel_mic(channel_id);
        self.http
            .put(token, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(())
    }

    /// Disables the bot microphone in an audio channel.
    pub async fn off_microphone(&self, token: &Token, channel_id: &str) -> Result<()> {
        debug!("Turning off microphone in channel {}", channel_id);
        let path = resource::channel_mic(channel_id);
        self.http.delete(token, &path, None::<&()>).await?;
        Ok(())
    }
}

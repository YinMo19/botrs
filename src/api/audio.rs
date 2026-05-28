use super::{BotApi, resource};
use crate::error::Result;
use crate::models::audio::AudioControl;
use tracing::debug;

impl BotApi {
    /// Sends an audio control action to a voice/live channel.
    pub async fn post_audio(
        &self,
        channel_id: &str,
        control: &AudioControl,
    ) -> Result<AudioControl> {
        debug!("Posting audio control to channel {}", channel_id);
        let path = resource::channel_audio(channel_id);
        self.http
            .post(self.token(), &path, None::<&()>, Some(control))
            .await?;
        Ok(control.clone())
    }

    /// Moves the bot onto the microphone in a voice/live channel.
    pub async fn put_mic(&self, channel_id: &str) -> Result<()> {
        debug!("Putting bot on mic in channel {}", channel_id);
        let path = resource::channel_mic(channel_id);
        self.http
            .put(self.token(), &path, None::<&()>, None::<&()>)
            .await?;
        Ok(())
    }

    /// Moves the bot off the microphone in a voice/live channel.
    pub async fn delete_mic(&self, channel_id: &str) -> Result<()> {
        debug!("Deleting bot mic in channel {}", channel_id);
        let path = resource::channel_mic(channel_id);
        self.http.delete(self.token(), &path, None::<&()>).await?;
        Ok(())
    }
}

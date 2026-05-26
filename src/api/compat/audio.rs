use crate::api::BotApi;
use crate::audio::AudioControl;
use crate::error::Result;

impl BotApi {
    /// Audio control API.
    #[allow(non_snake_case)]
    pub async fn PostAudio(
        &self,
        channel_id: &str,
        audio_control: &AudioControl,
    ) -> Result<AudioControl> {
        self.post_audio(self.token_required()?, channel_id, audio_control)
            .await
    }

    /// Microphone enable API.
    #[allow(non_snake_case)]
    pub async fn PutMic(&self, channel_id: &str) -> Result<()> {
        self.on_microphone(self.token_required()?, channel_id).await
    }

    /// Microphone disable API.
    #[allow(non_snake_case)]
    pub async fn DeleteMic(&self, channel_id: &str) -> Result<()> {
        self.off_microphone(self.token_required()?, channel_id)
            .await
    }
}

use async_trait::async_trait;
use diceri_application::ports::SpeechAudio;
use diceri_domain::LanguageCode;

use crate::Caption;

/// Where the pipeline delivers results; implemented by the media transport.
#[async_trait]
pub trait PipelineOutput: Send + Sync {
    /// Publishes a caption to listeners.
    async fn caption(&self, caption: Caption);

    /// Plays synthesized speech on `speaker`'s translated track for `language`.
    /// Returns when playback has been queued completely, so phrases never overlap.
    async fn speak(&self, speaker: &str, language: &LanguageCode, audio: SpeechAudio);
}

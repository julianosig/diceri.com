use async_trait::async_trait;
use diceri_domain::LanguageCode;
use tokio::sync::mpsc;

/// Audio format every synthesizer returns: 24 kHz mono PCM.
pub const TTS_SAMPLE_RATE: u32 = 24_000;

/// Provider-specific voice identifier (a stock voice or a consented clone).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoiceId(pub String);

#[derive(Debug, Clone, Copy)]
pub struct SpeechRequest<'a> {
    pub text: &'a str,
    pub language: &'a LanguageCode,
    pub voice: &'a VoiceId,
}

#[derive(Debug, thiserror::Error)]
pub enum TextToSpeechError {
    #[error("speech synthesis request failed: {0}")]
    Request(String),
}

/// PCM chunks as they are synthesized; the channel closes when speech ends.
pub type SpeechAudio = mpsc::Receiver<Result<Vec<i16>, TextToSpeechError>>;

/// Streaming speech synthesis (e.g. ElevenLabs).
#[async_trait]
pub trait TextToSpeech: Send + Sync {
    async fn synthesize(
        &self,
        request: SpeechRequest<'_>,
    ) -> Result<SpeechAudio, TextToSpeechError>;
}

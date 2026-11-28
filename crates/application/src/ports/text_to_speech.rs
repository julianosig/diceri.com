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
    /// The provider rejected the request (e.g. the voice clone): retrying will not help.
    #[error("speech synthesis request failed: {0}")]
    Request(String),
    /// Busy, rate-limited or unreachable for now (429, 5xx, network): only this phrase is lost.
    #[error("speech synthesis unavailable: {0}")]
    Unavailable(String),
}

impl TextToSpeechError {
    /// The error for a non-success HTTP status. Only the status is kept: provider bodies can
    /// echo the text being spoken, which must never reach the logs.
    #[must_use]
    pub fn from_status(status: u16) -> Self {
        let detail = format!("HTTP {status}");
        if status == 408 || status == 429 || status >= 500 {
            Self::Unavailable(detail)
        } else {
            Self::Request(detail)
        }
    }
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

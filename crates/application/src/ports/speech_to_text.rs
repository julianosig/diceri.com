use async_trait::async_trait;
use diceri_domain::LanguageCode;
use tokio::sync::mpsc;

/// Audio format every speech-to-text stream receives: 16 kHz mono PCM.
pub const STT_SAMPLE_RATE: u32 = 16_000;

/// What the recognizer heard, as it streams in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptEvent {
    /// Provisional text; may still change. Useful for live captions.
    Partial(String),
    /// Text that will never change, with the language the recognizer detected
    /// for it (when language identification is available).
    Final {
        text: String,
        language: Option<LanguageCode>,
    },
    /// The speaker finished a phrase: everything final so far is one segment.
    Endpoint,
}

#[derive(Debug, thiserror::Error)]
pub enum SpeechToTextError {
    #[error("speech-to-text connection failed: {0}")]
    Connection(String),
    #[error("speech-to-text provider error: {0}")]
    Provider(String),
}

/// A live recognition session: push audio in, read transcript events out.
/// Dropping `audio` ends the stream; `events` closes when the provider is done.
#[derive(Debug)]
pub struct SpeechStream {
    pub audio: mpsc::Sender<Vec<i16>>,
    pub events: mpsc::Receiver<Result<TranscriptEvent, SpeechToTextError>>,
}

/// Streaming speech recognition (e.g. Soniox). The spoken language is detected;
/// `hints` are languages the speaker is likely to use (may be empty).
#[async_trait]
pub trait SpeechToText: Send + Sync {
    async fn open(&self, hints: &[LanguageCode]) -> Result<SpeechStream, SpeechToTextError>;
}

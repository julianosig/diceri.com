use async_trait::async_trait;
use diceri_domain::LanguageCode;
use tokio::sync::mpsc;

/// One earlier turn of the call, from anyone in it, so the translation follows the
/// conversation (replies, references, irony) instead of each phrase alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextTurn {
    /// A neutral label per participant ("A", "B", …): never a name or an id.
    pub label: String,
    /// Whether this turn is from the person whose phrase is being translated.
    pub by_speaker: bool,
    pub text: String,
}

/// One phrase to translate, with the recent conversation for context.
#[derive(Debug, Clone, Copy)]
pub struct TranslationRequest<'a> {
    pub text: &'a str,
    pub source: &'a LanguageCode,
    pub target: &'a LanguageCode,
    /// Recent turns of the call from every participant, oldest first.
    pub context: &'a [ContextTurn],
}

#[derive(Debug, thiserror::Error)]
pub enum TranslatorError {
    #[error("translation request failed: {0}")]
    Request(String),
    #[error("translation provider returned no text")]
    Empty,
}

/// Text machine translation (e.g. Gemini). Translates directly, never via a pivot language.
#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, request: TranslationRequest<'_>) -> Result<String, TranslatorError>;

    /// Sends the translation in pieces, in order, as the provider generates it, so speech
    /// can start before it is complete. The default sends it whole, as one piece.
    async fn translate_stream(
        &self,
        request: TranslationRequest<'_>,
        pieces: mpsc::Sender<String>,
    ) -> Result<(), TranslatorError> {
        let text = self.translate(request).await?;
        // The receiver hanging up only means nobody needs the rest.
        let _ = pieces.send(text).await;
        Ok(())
    }
}

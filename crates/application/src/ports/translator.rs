use async_trait::async_trait;
use diceri_domain::LanguageCode;

/// One phrase to translate, with recent phrases for context.
#[derive(Debug, Clone, Copy)]
pub struct TranslationRequest<'a> {
    pub text: &'a str,
    pub source: &'a LanguageCode,
    pub target: &'a LanguageCode,
    /// Previous phrases from the same speaker, oldest first.
    pub context: &'a [String],
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
}

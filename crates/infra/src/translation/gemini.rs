use async_trait::async_trait;
use diceri_application::ports::{TranslationRequest, Translator, TranslatorError};
use secrecy::{ExposeSecret as _, SecretString};

use super::gemini_protocol;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

#[derive(Debug, Clone)]
pub struct GeminiConfig {
    pub api_key: SecretString,
    pub model: String,
}

impl GeminiConfig {
    #[must_use]
    pub fn new(api_key: SecretString) -> Self {
        Self {
            api_key,
            model: "gemini-3.5-flash-lite".to_owned(),
        }
    }
}

/// Gemini translation via `generateContent` with minimal thinking for low latency.
#[derive(Debug, Clone)]
pub struct GeminiTranslator {
    config: GeminiConfig,
    http: reqwest::Client,
}

impl GeminiTranslator {
    #[must_use]
    pub fn new(config: GeminiConfig, http: reqwest::Client) -> Self {
        Self { config, http }
    }
}

#[async_trait]
impl Translator for GeminiTranslator {
    async fn translate(&self, request: TranslationRequest<'_>) -> Result<String, TranslatorError> {
        let failed = |error: &dyn std::fmt::Display| TranslatorError::Request(error.to_string());
        let url = format!("{BASE_URL}/{}:generateContent", self.config.model);
        let response = self
            .http
            .post(url)
            .header("x-goog-api-key", self.config.api_key.expose_secret())
            .json(&gemini_protocol::request_body(&request))
            .send()
            .await
            .map_err(|e| failed(&e))?;
        let status = response.status();
        let body = response.text().await.map_err(|e| failed(&e))?;
        if !status.is_success() {
            return Err(TranslatorError::Request(format!("HTTP {status}: {body}")));
        }
        gemini_protocol::translation(&body)
            .map_err(|e| failed(&e))?
            .ok_or(TranslatorError::Empty)
    }
}

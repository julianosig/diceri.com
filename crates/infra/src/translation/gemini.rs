use async_trait::async_trait;
use diceri_application::ports::{TranslationRequest, Translator, TranslatorError};
use futures_util::StreamExt as _;
use secrecy::{ExposeSecret as _, SecretString};
use tokio::sync::mpsc;

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

/// Gemini translation with minimal thinking for low latency: `generateContent` for a
/// whole answer, `streamGenerateContent` (server-sent events) to speak as it arrives.
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

    /// Posts the request to `method` (`generateContent` or `streamGenerateContent?alt=sse`).
    async fn post(
        &self,
        method: &str,
        request: &TranslationRequest<'_>,
    ) -> Result<reqwest::Response, TranslatorError> {
        let url = format!("{BASE_URL}/{}:{method}", self.config.model);
        let response = self
            .http
            .post(url)
            .header("x-goog-api-key", self.config.api_key.expose_secret())
            .json(&gemini_protocol::request_body(request))
            .send()
            .await
            .map_err(|e| failed(&e))?;
        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let body = response.text().await.unwrap_or_default();
        Err(TranslatorError::Request(format!("HTTP {status}: {body}")))
    }
}

fn failed(error: &dyn std::fmt::Display) -> TranslatorError {
    TranslatorError::Request(error.to_string())
}

#[async_trait]
impl Translator for GeminiTranslator {
    async fn translate(&self, request: TranslationRequest<'_>) -> Result<String, TranslatorError> {
        let response = self.post("generateContent", &request).await?;
        let body = response.text().await.map_err(|e| failed(&e))?;
        gemini_protocol::translation(&body)
            .map_err(|e| failed(&e))?
            .ok_or(TranslatorError::Empty)
    }

    async fn translate_stream(
        &self,
        request: TranslationRequest<'_>,
        pieces: mpsc::Sender<String>,
    ) -> Result<(), TranslatorError> {
        let response = self.post("streamGenerateContent?alt=sse", &request).await?;
        let mut bytes = response.bytes_stream();
        let mut pending = String::new();
        let mut sent_any = false;
        while let Some(chunk) = bytes.next().await {
            pending.push_str(&String::from_utf8_lossy(&chunk.map_err(|e| failed(&e))?));
            // Events are lines `data: {json}`; keep a partial last line for the next chunk.
            while let Some(end) = pending.find('\n') {
                let line: String = pending.drain(..=end).collect();
                let Some(data) = line.trim().strip_prefix("data:") else {
                    continue;
                };
                let piece = gemini_protocol::stream_piece(data.trim()).map_err(|e| failed(&e))?;
                if piece.is_empty() {
                    continue;
                }
                sent_any = true;
                if pieces.send(piece).await.is_err() {
                    return Ok(());
                }
            }
        }
        if sent_any {
            Ok(())
        } else {
            Err(TranslatorError::Empty)
        }
    }
}

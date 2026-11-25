use async_trait::async_trait;
use diceri_application::ports::{
    SpeechAudio, SpeechRequest, TTS_SAMPLE_RATE, TextToSpeech, TextToSpeechError,
};
use futures_util::StreamExt as _;
use secrecy::{ExposeSecret as _, SecretString};
use serde_json::json;
use tokio::sync::mpsc;

use super::pcm::PcmDecoder;

const BASE_URL: &str = "https://api.elevenlabs.io/v1/text-to-speech";
const CHUNK_QUEUE: usize = 32;

#[derive(Debug, Clone)]
pub struct ElevenLabsConfig {
    pub api_key: SecretString,
    pub model: String,
}

impl ElevenLabsConfig {
    #[must_use]
    pub fn new(api_key: SecretString) -> Self {
        Self {
            api_key,
            model: "eleven_v4_turbo".to_owned(),
        }
    }
}

/// ElevenLabs streaming synthesis returning raw 24 kHz PCM as it is generated.
#[derive(Debug, Clone)]
pub struct ElevenLabsTextToSpeech {
    config: ElevenLabsConfig,
    http: reqwest::Client,
}

impl ElevenLabsTextToSpeech {
    #[must_use]
    pub fn new(config: ElevenLabsConfig, http: reqwest::Client) -> Self {
        Self { config, http }
    }
}

#[async_trait]
impl TextToSpeech for ElevenLabsTextToSpeech {
    async fn synthesize(
        &self,
        request: SpeechRequest<'_>,
    ) -> Result<SpeechAudio, TextToSpeechError> {
        let failed = |error: &dyn std::fmt::Display| TextToSpeechError::Request(error.to_string());
        let url = format!(
            "{BASE_URL}/{}/stream?output_format=pcm_{TTS_SAMPLE_RATE}",
            request.voice.0
        );
        let body = json!({
            "text": request.text,
            "model_id": self.config.model,
            "language_code": request.language.primary(),
        });
        let response = self
            .http
            .post(url)
            .header("xi-api-key", self.config.api_key.expose_secret())
            .json(&body)
            .send()
            .await
            .map_err(|e| failed(&e))?;
        let status = response.status();
        if !status.is_success() {
            let detail = response.text().await.unwrap_or_default();
            return Err(TextToSpeechError::Request(format!(
                "HTTP {status}: {detail}"
            )));
        }
        let (tx, rx) = mpsc::channel(CHUNK_QUEUE);
        let mut bytes = response.bytes_stream();
        tokio::spawn(async move {
            let mut decoder = PcmDecoder::default();
            while let Some(chunk) = bytes.next().await {
                let item = chunk
                    .map(|data| decoder.push(&data))
                    .map_err(|error| TextToSpeechError::Request(error.to_string()));
                let stop = item.is_err();
                if tx.send(item).await.is_err() || stop {
                    break;
                }
            }
        });
        Ok(rx)
    }
}

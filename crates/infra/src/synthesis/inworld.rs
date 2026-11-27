//! Inworld TTS-2 streaming synthesis (docs/research/2026-11-inworld-custom-voices).

use async_trait::async_trait;
use base64::Engine as _;
use diceri_application::ports::{
    SpeechAudio, SpeechRequest, TTS_SAMPLE_RATE, TextToSpeech, TextToSpeechError,
};
use futures_util::StreamExt as _;
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;

use super::pcm::PcmDecoder;

const STREAM_URL: &str = "https://api.inworld.ai/tts/v1/voice:stream";
const CHUNK_QUEUE: usize = 32;

/// Inworld account key and model (`inworld-tts-2`, or `inworld-tts-2-flash`).
#[derive(Debug, Clone)]
pub struct InworldConfig {
    pub api_key: SecretString,
    pub model: String,
}

impl InworldConfig {
    #[must_use]
    pub fn new(api_key: SecretString) -> Self {
        Self {
            api_key,
            model: "inworld-tts-2".to_owned(),
        }
    }
}

/// Inworld streaming synthesis: newline-delimited JSON, each line carrying a base64 chunk of
/// headerless 24 kHz 16-bit PCM. An error can arrive mid-stream after HTTP 200.
#[derive(Debug, Clone)]
pub struct InworldTextToSpeech {
    config: InworldConfig,
    http: reqwest::Client,
}

impl InworldTextToSpeech {
    #[must_use]
    pub fn new(config: InworldConfig, http: reqwest::Client) -> Self {
        Self { config, http }
    }
}

#[derive(Deserialize)]
struct StreamLine {
    result: Option<StreamResult>,
    error: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamResult {
    audio_content: Option<String>,
}

/// Splits newline-delimited JSON arriving in arbitrary chunks into complete lines.
#[derive(Default)]
struct Lines {
    pending: Vec<u8>,
}

impl Lines {
    fn push(&mut self, bytes: &[u8]) -> Vec<Vec<u8>> {
        self.pending.extend_from_slice(bytes);
        let mut lines = Vec::new();
        while let Some(end) = self.pending.iter().position(|&byte| byte == b'\n') {
            let line: Vec<u8> = self.pending.drain(..=end).collect();
            lines.push(line);
        }
        lines
    }
}

/// The PCM samples in one stream line, or the provider's error.
fn samples_of(line: &[u8], decoder: &mut PcmDecoder) -> Result<Vec<i16>, TextToSpeechError> {
    let failed = |error: &dyn std::fmt::Display| TextToSpeechError::Request(error.to_string());
    if line.iter().all(u8::is_ascii_whitespace) {
        return Ok(Vec::new());
    }
    let parsed: StreamLine = serde_json::from_slice(line).map_err(|e| failed(&e))?;
    if let Some(error) = parsed.error {
        return Err(TextToSpeechError::Request(error.to_string()));
    }
    let Some(audio) = parsed.result.and_then(|result| result.audio_content) else {
        return Ok(Vec::new());
    };
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(audio)
        .map_err(|e| failed(&e))?;
    Ok(decoder.push(&bytes))
}

#[async_trait]
impl TextToSpeech for InworldTextToSpeech {
    async fn synthesize(
        &self,
        request: SpeechRequest<'_>,
    ) -> Result<SpeechAudio, TextToSpeechError> {
        let failed = |error: &dyn std::fmt::Display| TextToSpeechError::Request(error.to_string());
        let body = json!({
            "text": request.text,
            "voiceId": request.voice.0,
            "modelId": self.config.model,
            // Always set: auto-detection gets short translated turns wrong.
            "language": request.language.as_str(),
            "audioConfig": { "audioEncoding": "PCM", "sampleRateHertz": TTS_SAMPLE_RATE },
        });
        let response = self
            .http
            .post(STREAM_URL)
            .header(
                "Authorization",
                format!("Basic {}", self.config.api_key.expose_secret()),
            )
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
            let mut lines = Lines::default();
            let mut decoder = PcmDecoder::default();
            while let Some(chunk) = bytes.next().await {
                let Ok(chunk) = chunk.inspect_err(|error| {
                    let _ = tx.try_send(Err(TextToSpeechError::Request(error.to_string())));
                }) else {
                    break;
                };
                for line in lines.push(&chunk) {
                    let item = samples_of(&line, &mut decoder);
                    let stop = item.is_err();
                    if tx.send(item).await.is_err() || stop {
                        return;
                    }
                }
            }
        });
        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_lines_across_chunks() {
        let mut lines = Lines::default();
        assert_eq!(lines.push(b"{\"a\":"), Vec::<Vec<u8>>::new());
        assert_eq!(
            lines.push(b"1}\n{\"b\":2}\n{"),
            vec![b"{\"a\":1}\n".to_vec(), b"{\"b\":2}\n".to_vec()]
        );
    }

    #[test]
    fn decodes_audio_and_reports_errors() {
        let mut decoder = PcmDecoder::default();
        // Two samples: 1 and -2, little-endian.
        let audio = base64::engine::general_purpose::STANDARD.encode([1, 0, 0xFE, 0xFF]);
        let line = format!("{{\"result\":{{\"audioContent\":\"{audio}\"}}}}\n");
        assert_eq!(
            samples_of(line.as_bytes(), &mut decoder).unwrap(),
            vec![1, -2]
        );
        assert!(samples_of(b"{\"error\":{\"code\":8}}\n", &mut decoder).is_err());
        assert_eq!(samples_of(b"\n", &mut decoder).unwrap(), Vec::<i16>::new());
    }
}

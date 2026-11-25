use async_trait::async_trait;
use diceri_application::ports::{SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent};
use diceri_domain::LanguageCode;
use futures_util::{SinkExt as _, StreamExt as _};
use secrecy::{ExposeSecret as _, SecretString};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use super::soniox_protocol::{self, ServerMessage};

const URL: &str = "wss://stt-rt.soniox.com/transcribe-websocket";
/// ~2 s of 20 ms frames; older audio is dropped by the caller rather than queued.
const AUDIO_QUEUE: usize = 100;
const EVENT_QUEUE: usize = 64;

#[derive(Debug, Clone)]
pub struct SonioxConfig {
    pub api_key: SecretString,
    pub model: String,
    /// Upper bound on waiting after speech stops before closing a phrase (500–3000).
    pub max_endpoint_delay_ms: u32,
}

impl SonioxConfig {
    #[must_use]
    pub fn new(api_key: SecretString) -> Self {
        Self {
            api_key,
            model: "stt-rt-v6".to_owned(),
            max_endpoint_delay_ms: 1_000,
        }
    }
}

/// Soniox real-time speech-to-text over WebSocket.
#[derive(Debug, Clone)]
pub struct SonioxSpeechToText {
    config: SonioxConfig,
}

impl SonioxSpeechToText {
    #[must_use]
    pub fn new(config: SonioxConfig) -> Self {
        Self { config }
    }
}

type EventSender = mpsc::Sender<Result<TranscriptEvent, SpeechToTextError>>;

#[async_trait]
impl SpeechToText for SonioxSpeechToText {
    async fn open(&self, language: &LanguageCode) -> Result<SpeechStream, SpeechToTextError> {
        let connection =
            |error: &dyn std::fmt::Display| SpeechToTextError::Connection(error.to_string());
        let mut request = URL.into_client_request().map_err(|e| connection(&e))?;
        let bearer = format!("Bearer {}", self.config.api_key.expose_secret());
        request.headers_mut().insert(
            "authorization",
            HeaderValue::from_str(&bearer).map_err(|e| connection(&e))?,
        );
        let (socket, _) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|e| connection(&e))?;
        let (mut sink, mut stream) = socket.split();
        let config = soniox_protocol::config_message(
            &self.config.model,
            language,
            self.config.max_endpoint_delay_ms,
        );
        sink.send(Message::text(config))
            .await
            .map_err(|e| connection(&e))?;

        let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<i16>>(AUDIO_QUEUE);
        let (event_tx, event_rx) = mpsc::channel(EVENT_QUEUE);
        tokio::spawn(async move {
            while let Some(samples) = audio_rx.recv().await {
                if sink
                    .send(Message::binary(soniox_protocol::pcm_bytes(&samples)))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            // An empty text frame asks Soniox to finalize and close the stream.
            let _ = sink.send(Message::text("")).await;
        });
        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                let keep_going = match message {
                    Ok(Message::Text(text)) => forward(&text, &event_tx).await,
                    Ok(Message::Close(_)) => false,
                    Ok(_) => true,
                    Err(error) => {
                        let _ = event_tx
                            .send(Err(SpeechToTextError::Connection(error.to_string())))
                            .await;
                        false
                    }
                };
                if !keep_going {
                    break;
                }
            }
        });
        Ok(SpeechStream {
            audio: audio_tx,
            events: event_rx,
        })
    }
}

/// Sends parsed events on; returns false when the stream should stop.
async fn forward(raw: &str, events: &EventSender) -> bool {
    match soniox_protocol::parse(raw) {
        Ok(ServerMessage::Events(batch)) => {
            for event in batch {
                if events.send(Ok(event)).await.is_err() {
                    return false;
                }
            }
            true
        }
        Ok(ServerMessage::Finished) => false,
        Ok(ServerMessage::Error(message)) => {
            let _ = events.send(Err(SpeechToTextError::Provider(message))).await;
            false
        }
        Err(error) => {
            tracing::warn!(%error, "unparseable Soniox message");
            true
        }
    }
}

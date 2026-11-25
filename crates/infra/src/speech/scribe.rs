use async_trait::async_trait;
use diceri_application::ports::{SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent};
use diceri_domain::LanguageCode;
use futures_util::{SinkExt as _, StreamExt as _};
use secrecy::{ExposeSecret as _, SecretString};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use super::scribe_protocol::{self, ServerMessage};

/// ~2 s of 10–20 ms frames; the caller drops audio rather than queueing more.
const AUDIO_QUEUE: usize = 200;
const EVENT_QUEUE: usize = 64;
/// Scribe recommends ~100 ms chunks: 1,600 samples at 16 kHz.
const CHUNK_SAMPLES: usize = 1_600;
const SESSION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Debug, Clone)]
pub struct ScribeConfig {
    pub api_key: SecretString,
    pub model: String,
    /// Silence that closes a phrase, in seconds.
    pub silence_secs: f32,
    /// Lets one segment mix languages; off by default (see live tests).
    pub multilang: bool,
}

impl ScribeConfig {
    #[must_use]
    pub fn new(api_key: SecretString) -> Self {
        Self {
            api_key,
            model: "scribe_v3_live".to_owned(),
            silence_secs: 0.3,
            multilang: false,
        }
    }
}

/// ElevenLabs Scribe realtime speech-to-text with automatic language detection.
#[derive(Debug, Clone)]
pub struct ScribeSpeechToText {
    config: ScribeConfig,
}

impl ScribeSpeechToText {
    #[must_use]
    pub fn new(config: ScribeConfig) -> Self {
        Self { config }
    }
}

type EventSender = mpsc::Sender<Result<TranscriptEvent, SpeechToTextError>>;

#[async_trait]
impl SpeechToText for ScribeSpeechToText {
    /// Hints are not sent: multilingual mode detects the language per segment.
    async fn open(&self, _hints: &[LanguageCode]) -> Result<SpeechStream, SpeechToTextError> {
        let connection =
            |error: &dyn std::fmt::Display| SpeechToTextError::Connection(error.to_string());
        let url = scribe_protocol::url(
            &self.config.model,
            self.config.silence_secs,
            self.config.multilang,
        );
        let mut request = url.into_client_request().map_err(|e| connection(&e))?;
        let key = HeaderValue::from_str(self.config.api_key.expose_secret())
            .map_err(|e| connection(&e))?;
        request.headers_mut().insert("xi-api-key", key);
        let (socket, _) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|e| connection(&e))?;
        let (mut sink, mut stream) = socket.split();
        wait_for_session(&mut stream).await?;

        let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<i16>>(AUDIO_QUEUE);
        let (event_tx, event_rx) = mpsc::channel(EVENT_QUEUE);
        tokio::spawn(async move {
            let mut pending = Vec::with_capacity(CHUNK_SAMPLES * 2);
            while let Some(samples) = audio_rx.recv().await {
                pending.extend(samples);
                if pending.len() >= CHUNK_SAMPLES {
                    let chunk = scribe_protocol::audio_message(&pending, false);
                    pending.clear();
                    if sink.send(Message::text(chunk)).await.is_err() {
                        return;
                    }
                }
            }
            // Flush and commit whatever was said last, then close.
            let _ = sink
                .send(Message::text(scribe_protocol::audio_message(
                    &pending, true,
                )))
                .await;
            let _ = sink.close().await;
        });
        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                let keep_going = match message {
                    Ok(Message::Text(text)) => forward(&text, &event_tx).await,
                    Ok(Message::Close(_)) => false,
                    Ok(_) => true,
                    Err(error) if is_abrupt_close(&error) => false,
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

/// Audio sent before the session starts can be lost, so wait for `session_started`.
async fn wait_for_session<S>(stream: &mut S) -> Result<(), SpeechToTextError>
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let failed = |reason: String| SpeechToTextError::Connection(reason);
    let first = tokio::time::timeout(SESSION_TIMEOUT, stream.next())
        .await
        .map_err(|_| failed("timed out waiting for session_started".to_owned()))?;
    match first {
        Some(Ok(Message::Text(text))) => match scribe_protocol::parse(&text) {
            Ok(ServerMessage::Error(message)) => Err(SpeechToTextError::Provider(message)),
            _ => Ok(()),
        },
        Some(Ok(other)) => Err(failed(format!("unexpected first message: {other:?}"))),
        Some(Err(error)) => Err(failed(error.to_string())),
        None => Err(failed("closed before session_started".to_owned())),
    }
}

/// The server sometimes drops TCP after its last message without a TLS
/// `close_notify`; everything was already delivered, so it is a normal end.
fn is_abrupt_close(error: &tokio_tungstenite::tungstenite::Error) -> bool {
    use tokio_tungstenite::tungstenite::Error;
    use tokio_tungstenite::tungstenite::error::ProtocolError;
    match error {
        Error::Io(io) => io.kind() == std::io::ErrorKind::UnexpectedEof,
        Error::Protocol(ProtocolError::ResetWithoutClosingHandshake) => true,
        _ => false,
    }
}

/// Sends parsed events on; returns false when the stream should stop.
async fn forward(raw: &str, events: &EventSender) -> bool {
    match scribe_protocol::parse(raw) {
        Ok(ServerMessage::Events(batch)) => {
            for event in batch {
                if events.send(Ok(event)).await.is_err() {
                    return false;
                }
            }
            true
        }
        Ok(ServerMessage::Ignored) => true,
        Ok(ServerMessage::Error(message)) => {
            let _ = events.send(Err(SpeechToTextError::Provider(message))).await;
            false
        }
        Err(error) => {
            tracing::warn!(%error, "unparseable Scribe message");
            true
        }
    }
}

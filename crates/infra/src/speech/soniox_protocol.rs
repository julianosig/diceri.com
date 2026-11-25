//! Soniox real-time WebSocket messages (pure, no I/O).
//! Reference: <https://docs.soniox.com/api-reference/stt/websocket-api>.

use diceri_application::ports::{STT_SAMPLE_RATE, TranscriptEvent};
use diceri_domain::LanguageCode;
use serde::Deserialize;
use serde_json::json;

/// Marker token emitted when endpoint detection closes a phrase.
const ENDPOINT_TOKEN: &str = "<end>";

pub(super) fn config_message(model: &str, language: &LanguageCode, max_endpoint_ms: u32) -> String {
    json!({
        "model": model,
        "audio_format": "pcm_s16le",
        "sample_rate": STT_SAMPLE_RATE,
        "num_channels": 1,
        "language_hints": [language.primary()],
        "enable_endpoint_detection": true,
        "max_endpoint_delay_ms": max_endpoint_ms,
    })
    .to_string()
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    tokens: Vec<Token>,
    #[serde(default)]
    finished: bool,
    error_type: Option<String>,
    error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Token {
    text: String,
    #[serde(default)]
    is_final: bool,
}

/// A parsed server message.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ServerMessage {
    Events(Vec<TranscriptEvent>),
    Finished,
    Error(String),
}

pub(super) fn parse(raw: &str) -> Result<ServerMessage, serde_json::Error> {
    let response: Response = serde_json::from_str(raw)?;
    if let Some(kind) = response.error_type {
        let message = response.error_message.unwrap_or_default();
        return Ok(ServerMessage::Error(format!("{kind}: {message}")));
    }
    if response.finished {
        return Ok(ServerMessage::Finished);
    }
    Ok(ServerMessage::Events(events(&response.tokens)))
}

/// Groups tokens into at most one `Final`, one `Partial` and one `Endpoint`, in that order.
fn events(tokens: &[Token]) -> Vec<TranscriptEvent> {
    let mut finals = String::new();
    let mut partial = String::new();
    let mut endpoint = false;
    for token in tokens {
        if token.text == ENDPOINT_TOKEN {
            endpoint = true;
        } else if token.is_final {
            finals.push_str(&token.text);
        } else {
            partial.push_str(&token.text);
        }
    }
    let mut events = Vec::new();
    if !finals.is_empty() {
        events.push(TranscriptEvent::Final(finals));
    }
    if !partial.is_empty() {
        events.push(TranscriptEvent::Partial(partial));
    }
    if endpoint {
        events.push(TranscriptEvent::Endpoint);
    }
    events
}

/// PCM samples as little-endian bytes, the wire format for `pcm_s16le`.
pub(super) fn pcm_bytes(samples: &[i16]) -> Vec<u8> {
    samples
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_uses_primary_language_and_endpointing() {
        let config = config_message("stt-rt-v6", &"pt-BR".parse().unwrap(), 1500);
        let value: serde_json::Value = serde_json::from_str(&config).unwrap();
        assert_eq!(value["language_hints"][0], "pt");
        assert_eq!(value["enable_endpoint_detection"], true);
        assert_eq!(value["sample_rate"], 16_000);
    }

    #[test]
    fn splits_final_partial_and_endpoint() {
        let raw = r#"{"tokens":[
            {"text":"Olá","is_final":true},{"text":" mundo","is_final":true},
            {"text":"<end>","is_final":true},{"text":" tu","is_final":false}]}"#;
        assert_eq!(
            parse(raw).unwrap(),
            ServerMessage::Events(vec![
                TranscriptEvent::Final("Olá mundo".to_owned()),
                TranscriptEvent::Partial(" tu".to_owned()),
                TranscriptEvent::Endpoint,
            ])
        );
    }

    #[test]
    fn recognizes_finished_and_errors() {
        assert_eq!(
            parse(r#"{"tokens":[],"finished":true}"#).unwrap(),
            ServerMessage::Finished
        );
        let error =
            parse(r#"{"error_code":401,"error_type":"unauthenticated","error_message":"bad key"}"#);
        assert_eq!(
            error.unwrap(),
            ServerMessage::Error("unauthenticated: bad key".to_owned())
        );
    }

    #[test]
    fn encodes_little_endian_pcm() {
        assert_eq!(pcm_bytes(&[1, -2]), vec![1, 0, 0xFE, 0xFF]);
    }
}

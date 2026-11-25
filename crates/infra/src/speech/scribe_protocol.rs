//! ElevenLabs Scribe realtime WebSocket messages (pure, no I/O).
//! Reference: <https://elevenlabs.io/docs/api-reference/speech-to-text/v-1-speech-to-text-realtime>.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use diceri_application::ports::{STT_SAMPLE_RATE, TranscriptEvent};
use diceri_domain::LanguageCode;
use serde::Deserialize;
use serde_json::json;

/// Realtime endpoint; parameters go in the query string.
pub(super) fn url(model: &str, silence_secs: f32, multilang: bool) -> String {
    format!(
        "wss://api.elevenlabs.io/v1/speech-to-text/realtime?model_id={model}\
         &audio_format=pcm_{STT_SAMPLE_RATE}&commit_strategy=vad\
         &vad_silence_threshold_secs={silence_secs}&include_timestamps=true\
         &include_language_detection=true&multilang_mode={multilang}"
    )
}

/// One audio chunk; `commit` forces the current segment to finalize.
pub(super) fn audio_message(samples: &[i16], commit: bool) -> String {
    let bytes: Vec<u8> = samples
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect();
    json!({
        "message_type": "input_audio_chunk",
        "audio_base_64": STANDARD.encode(bytes),
        "commit": commit,
        "sample_rate": STT_SAMPLE_RATE,
    })
    .to_string()
}

#[derive(Debug, Deserialize)]
struct Frame {
    message_type: String,
    text: Option<String>,
    language_code: Option<String>,
    error: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ServerMessage {
    Events(Vec<TranscriptEvent>),
    Ignored,
    Error(String),
}

pub(super) fn parse(raw: &str) -> Result<ServerMessage, serde_json::Error> {
    let message: Frame = serde_json::from_str(raw)?;
    let text = message.text.unwrap_or_default();
    Ok(match message.message_type.as_str() {
        "partial_transcript" if !text.trim().is_empty() => {
            ServerMessage::Events(vec![TranscriptEvent::Partial(text)])
        }
        // With timestamps on, this is the committed segment that carries the language;
        // the plain `committed_transcript` duplicate is ignored.
        "committed_transcript_with_timestamps" if !text.trim().is_empty() => {
            let language = message.language_code.as_deref().and_then(to_language);
            ServerMessage::Events(vec![
                TranscriptEvent::Final { text, language },
                TranscriptEvent::Endpoint,
            ])
        }
        "warning"
        | "session_started"
        | "partial_transcript"
        | "committed_transcript"
        | "committed_transcript_with_timestamps" => ServerMessage::Ignored,
        kind => ServerMessage::Error(format!("{kind}: {}", message.error.unwrap_or_default())),
    })
}

/// Scribe reports ISO 639-3 (`por`); the rest of the system uses 639-1 (`pt`).
fn to_language(code: &str) -> Option<LanguageCode> {
    let short = match code {
        "por" => "pt",
        "eng" => "en",
        "spa" => "es",
        "fra" => "fr",
        "deu" => "de",
        "ita" => "it",
        "nld" => "nl",
        "pol" => "pl",
        "rus" => "ru",
        "ukr" => "uk",
        "tur" => "tr",
        "ara" => "ar",
        "hin" => "hi",
        "cmn" | "zho" => "zh",
        "jpn" => "ja",
        "kor" => "ko",
        "vie" => "vi",
        "ind" => "id",
        other => other,
    };
    short.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_requests_vad_commits_and_language_detection() {
        let url = url("scribe_v3_live", 0.5, false);
        assert!(url.contains("model_id=scribe_v3_live"));
        assert!(url.contains("audio_format=pcm_16000"));
        assert!(url.contains("commit_strategy=vad"));
        assert!(url.contains("include_language_detection=true"));
    }

    #[test]
    fn audio_is_base64_little_endian_pcm() {
        let message: serde_json::Value =
            serde_json::from_str(&audio_message(&[1, -2], false)).unwrap();
        assert_eq!(message["audio_base_64"], "AQD+/w==");
        assert_eq!(message["sample_rate"], 16_000);
    }

    #[test]
    fn committed_segment_becomes_a_final_phrase_with_language() {
        let raw = r#"{"message_type":"committed_transcript_with_timestamps","text":"Olá","language_code":"por","words":[]}"#;
        assert_eq!(
            parse(raw).unwrap(),
            ServerMessage::Events(vec![
                TranscriptEvent::Final {
                    text: "Olá".to_owned(),
                    language: Some("pt".parse().unwrap())
                },
                TranscriptEvent::Endpoint,
            ])
        );
    }

    #[test]
    fn partials_pass_through_and_duplicates_are_ignored() {
        let partial = r#"{"message_type":"partial_transcript","text":"Ol"}"#;
        assert_eq!(
            parse(partial).unwrap(),
            ServerMessage::Events(vec![TranscriptEvent::Partial("Ol".to_owned())])
        );
        let plain = r#"{"message_type":"committed_transcript","text":"Olá"}"#;
        assert_eq!(parse(plain).unwrap(), ServerMessage::Ignored);
    }

    #[test]
    fn error_messages_are_reported() {
        let raw = r#"{"message_type":"quota_exceeded","error":"out of credits"}"#;
        assert_eq!(
            parse(raw).unwrap(),
            ServerMessage::Error("quota_exceeded: out of credits".to_owned())
        );
    }
}

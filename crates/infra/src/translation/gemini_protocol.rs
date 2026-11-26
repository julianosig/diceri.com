//! Gemini `generateContent` request and response bodies (pure, no I/O).

use std::fmt::Write as _;

use diceri_application::ports::TranslationRequest;
use serde::Deserialize;
use serde_json::{Value, json};

/// English names help the model with less common codes.
fn language_name(code: &str) -> &str {
    match code {
        "pt" => "Portuguese",
        "en" => "English",
        "es" => "Spanish",
        "fr" => "French",
        "de" => "German",
        "it" => "Italian",
        "ja" => "Japanese",
        "ko" => "Korean",
        "zh" => "Chinese",
        "ar" => "Arabic",
        "hi" => "Hindi",
        "ru" => "Russian",
        _ => code,
    }
}

fn describe(code: &diceri_domain::LanguageCode) -> String {
    format!("{} ({})", language_name(code.primary()), code.as_str())
}

pub(super) fn request_body(request: &TranslationRequest<'_>) -> Value {
    let target = describe(request.target);
    let instruction = format!(
        "You are a professional live interpreter in a video call. Render the speaker's latest \
         utterance from {source} into {target} the way a native speaker would naturally say it \
         in conversation: convey the meaning and intent, not the words. Adapt idioms, \
         expressions, slang and cultural references to natural equivalents; never translate \
         them literally. Keep the tone, register and form of address consistent with the \
         conversation. Keep names, numbers and amounts exact. The utterance comes from speech \
         recognition and may contain recognition errors: use the conversation to work out what \
         was meant. Use the conversation only to understand; translate only the latest \
         utterance. Output only the translation, ready to be spoken aloud: no quotes, notes, \
         or explanations.",
        source = describe(request.source),
    );
    json!({
        "systemInstruction": { "parts": [{ "text": instruction }] },
        "contents": [{ "role": "user", "parts": [{ "text": prompt(request) }] }],
        "generationConfig": { "temperature": 0, "thinkingConfig": { "thinkingLevel": "minimal" } }
    })
}

/// The conversation so far (each participant under a neutral label), then the utterance.
fn prompt(request: &TranslationRequest<'_>) -> String {
    if request.context.is_empty() {
        return request.text.to_owned();
    }
    let speaker = request
        .context
        .iter()
        .find(|turn| turn.by_speaker)
        .map_or("the speaker", |turn| turn.label.as_str());
    let mut prompt = String::from("Conversation so far, oldest first:\n");
    for turn in request.context {
        let role = if turn.by_speaker { " (speaker)" } else { "" };
        // Writing to a String cannot fail.
        let _ = writeln!(prompt, "{}{role}: {}", turn.label, turn.text);
    }
    let _ = write!(prompt, "\nLatest utterance from {speaker}, to translate:\n");
    prompt.push_str(request.text);
    prompt
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    #[serde(default)]
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: Option<String>,
}

/// The translated text, or `None` when the model returned nothing usable.
pub(super) fn translation(raw: &str) -> Result<Option<String>, serde_json::Error> {
    let response: Response = serde_json::from_str(raw)?;
    let text: String = response
        .candidates
        .into_iter()
        .filter_map(|candidate| candidate.content)
        .flat_map(|content| content.parts)
        .filter_map(|part| part.text)
        .collect();
    let trimmed = text.trim();
    Ok((!trimmed.is_empty()).then(|| trimmed.to_owned()))
}

/// The text of one streamed event (`data: {...}` of `streamGenerateContent?alt=sse`),
/// untrimmed: the spaces between pieces belong to the translation.
pub(super) fn stream_piece(data: &str) -> Result<String, serde_json::Error> {
    let response: Response = serde_json::from_str(data)?;
    Ok(response
        .candidates
        .into_iter()
        .filter_map(|candidate| candidate.content)
        .flat_map(|content| content.parts)
        .filter_map(|part| part.text)
        .collect())
}

#[cfg(test)]
mod tests {
    use diceri_domain::LanguageCode;

    use super::*;

    fn lang(code: &str) -> LanguageCode {
        code.parse().unwrap()
    }

    #[test]
    fn request_names_languages_and_includes_context() {
        let context = vec![
            diceri_application::ports::ContextTurn {
                label: "A".to_owned(),
                by_speaker: true,
                text: "Bom dia.".to_owned(),
            },
            diceri_application::ports::ContextTurn {
                label: "B".to_owned(),
                by_speaker: false,
                text: "Good morning!".to_owned(),
            },
        ];
        let request = TranslationRequest {
            text: "Tudo bem?",
            source: &lang("pt-BR"),
            target: &lang("en"),
            context: &context,
        };
        let body = request_body(&request);
        let instruction = body["systemInstruction"]["parts"][0]["text"]
            .as_str()
            .unwrap();
        assert!(instruction.contains("from Portuguese (pt-BR) into English (en)"));
        assert!(instruction.contains("never translate them literally"));
        let prompt = body["contents"][0]["parts"][0]["text"].as_str().unwrap();
        assert!(prompt.contains("A (speaker): Bom dia.\nB: Good morning!\n"));
        assert!(prompt.ends_with("Latest utterance from A, to translate:\nTudo bem?"));
        assert_eq!(
            body["generationConfig"]["thinkingConfig"]["thinkingLevel"],
            "minimal"
        );
    }

    #[test]
    fn extracts_and_trims_translation() {
        let raw = r#"{"candidates":[{"content":{"parts":[{"text":" How are you? \n"}]}}]}"#;
        assert_eq!(translation(raw).unwrap().as_deref(), Some("How are you?"));
        assert_eq!(translation(r#"{"candidates":[]}"#).unwrap(), None);
    }

    #[test]
    fn keeps_the_spaces_of_streamed_pieces() {
        let piece = r#"{"candidates":[{"content":{"parts":[{"text":" are you?"}]}}]}"#;
        assert_eq!(stream_piece(piece).unwrap(), " are you?");
    }
}

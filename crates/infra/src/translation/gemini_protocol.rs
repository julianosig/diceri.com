//! Gemini `generateContent` request and response bodies (pure, no I/O).

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
    let instruction = format!(
        "You are a live interpreter in a voice call. Translate the speaker's latest utterance \
         from {} to {}. Output only the translation, ready to be spoken aloud: no quotes, \
         notes, or explanations. Keep the meaning, tone, and register. Keep names, numbers, \
         and amounts exact. Previous utterances are context only; do not translate them.",
        describe(request.source),
        describe(request.target),
    );
    let mut prompt = String::new();
    if !request.context.is_empty() {
        prompt.push_str("Previous utterances (context only):\n");
        for line in request.context {
            prompt.push_str("- ");
            prompt.push_str(line);
            prompt.push('\n');
        }
        prompt.push_str("\nLatest utterance to translate:\n");
    }
    prompt.push_str(request.text);
    json!({
        "systemInstruction": { "parts": [{ "text": instruction }] },
        "contents": [{ "role": "user", "parts": [{ "text": prompt }] }],
        "generationConfig": { "temperature": 0, "thinkingConfig": { "thinkingLevel": "minimal" } }
    })
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

#[cfg(test)]
mod tests {
    use diceri_domain::LanguageCode;

    use super::*;

    fn lang(code: &str) -> LanguageCode {
        code.parse().unwrap()
    }

    #[test]
    fn request_names_languages_and_includes_context() {
        let context = vec!["Bom dia.".to_owned()];
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
        assert!(instruction.contains("from Portuguese (pt-BR) to English (en)"));
        let prompt = body["contents"][0]["parts"][0]["text"].as_str().unwrap();
        assert!(prompt.contains("- Bom dia.") && prompt.ends_with("Tudo bem?"));
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
}

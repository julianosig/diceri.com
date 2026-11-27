//! Inworld instant voice cloning: a stored voice per participant, created at pre-join and
//! deleted when they leave. Inworld counts voices held at once, not voices created, and a
//! deleted voice frees its slot at once (docs/research/2026-11-inworld-custom-voices).

use async_trait::async_trait;
use base64::Engine as _;
use diceri_application::ports::{VoiceCloneError, VoiceCloner, VoiceId, VoiceSample};
use reqwest::StatusCode;
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;
use serde_json::json;

const VOICES_URL: &str = "https://api.inworld.ai/voices/v1/voices";

#[derive(Debug, Clone)]
pub struct InworldVoiceCloner {
    api_key: SecretString,
    http: reqwest::Client,
}

impl InworldVoiceCloner {
    #[must_use]
    pub fn new(api_key: SecretString, http: reqwest::Client) -> Self {
        Self { api_key, http }
    }

    fn auth(&self) -> String {
        format!("Basic {}", self.api_key.expose_secret())
    }
}

#[derive(Deserialize)]
struct Created {
    voice: CreatedVoice,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatedVoice {
    voice_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceList {
    #[serde(default)]
    voices: Vec<ListedVoice>,
    #[serde(default)]
    next_page_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListedVoice {
    voice_id: String,
    #[serde(default)]
    display_name: String,
}

/// A refused sample is the participant's to redo; rate limits and outages are not.
fn clone_failure(status: StatusCode, body: String) -> VoiceCloneError {
    if status.is_client_error() && status != StatusCode::TOO_MANY_REQUESTS {
        VoiceCloneError::Rejected(body)
    } else {
        VoiceCloneError::Request(format!("HTTP {status}: {body}"))
    }
}

#[async_trait]
impl VoiceCloner for InworldVoiceCloner {
    fn provider(&self) -> &'static str {
        "inworld"
    }

    async fn create(&self, name: &str, sample: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
        let failed = |error: &dyn std::fmt::Display| VoiceCloneError::Request(error.to_string());
        let body = json!({
            "displayName": name,
            // The spoken language is detected; the clone speaks every target language.
            "langCode": "AUTO",
            "voiceSamples": [{
                "audioData": base64::engine::general_purpose::STANDARD.encode(&sample.bytes),
            }],
        });
        let response = self
            .http
            .post(format!("{VOICES_URL}:clone"))
            .header("Authorization", self.auth())
            .json(&body)
            .send()
            .await
            .map_err(|e| failed(&e))?;
        let status = response.status();
        let text = response.text().await.map_err(|e| failed(&e))?;
        if !status.is_success() {
            return Err(clone_failure(status, text));
        }
        let created: Created = serde_json::from_str(&text).map_err(|e| failed(&e))?;
        Ok(VoiceId(created.voice.voice_id))
    }

    async fn delete(&self, voice: &VoiceId) -> Result<(), VoiceCloneError> {
        let response = self
            .http
            .delete(format!("{VOICES_URL}/{}", voice.0))
            .header("Authorization", self.auth())
            .send()
            .await
            .map_err(|error| VoiceCloneError::Request(error.to_string()))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(VoiceCloneError::Request(format!(
                "delete answered HTTP {}",
                response.status()
            )))
        }
    }

    async fn delete_all_named(&self, prefix: &str) -> Result<usize, VoiceCloneError> {
        let failed = |error: &dyn std::fmt::Display| VoiceCloneError::Request(error.to_string());
        let mut ours = Vec::new();
        let mut page_token = String::new();
        loop {
            let url = reqwest::Url::parse_with_params(
                VOICES_URL,
                &[
                    ("filter", "source=\"IVC\""),
                    ("pageSize", "1000"),
                    ("pageToken", page_token.as_str()),
                ],
            )
            .map_err(|e| failed(&e))?;
            let list: VoiceList = self
                .http
                .get(url)
                .header("Authorization", self.auth())
                .send()
                .await
                .map_err(|e| failed(&e))?
                .json()
                .await
                .map_err(|e| failed(&e))?;
            ours.extend(
                list.voices
                    .into_iter()
                    .filter(|voice| voice.display_name.starts_with(prefix))
                    .map(|voice| voice.voice_id),
            );
            if list.next_page_token.is_empty() {
                break;
            }
            page_token = list.next_page_token;
        }
        // Pages are offsets over a changing list, so a voice can appear twice.
        ours.sort_unstable();
        ours.dedup();
        for voice in &ours {
            self.delete(&VoiceId(voice.clone())).await?;
        }
        Ok(ours.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_bad_samples_are_the_participants_to_redo() {
        let refused = clone_failure(StatusCode::BAD_REQUEST, "bad audio".to_owned());
        assert!(matches!(refused, VoiceCloneError::Rejected(_)));
        let busy = clone_failure(StatusCode::TOO_MANY_REQUESTS, String::new());
        assert!(matches!(busy, VoiceCloneError::Request(_)));
        let down = clone_failure(StatusCode::BAD_GATEWAY, String::new());
        assert!(matches!(down, VoiceCloneError::Request(_)));
    }
}

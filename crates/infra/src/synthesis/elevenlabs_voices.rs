use async_trait::async_trait;
use diceri_application::ports::{VoiceCloneError, VoiceCloner, VoiceId, VoiceSample};
use reqwest::multipart::{Form, Part};
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;

const VOICES_URL: &str = "https://api.elevenlabs.io/v1/voices";

/// ElevenLabs instant voice cloning (create from a short sample, delete on leave).
#[derive(Debug, Clone)]
pub struct ElevenLabsVoiceCloner {
    api_key: SecretString,
    http: reqwest::Client,
}

impl ElevenLabsVoiceCloner {
    #[must_use]
    pub fn new(api_key: SecretString, http: reqwest::Client) -> Self {
        Self { api_key, http }
    }
}

#[derive(Deserialize)]
struct Created {
    voice_id: String,
}

#[derive(Deserialize)]
struct VoiceList {
    voices: Vec<ListedVoice>,
}

#[derive(Deserialize)]
struct ListedVoice {
    voice_id: String,
    name: String,
}

/// Every clone (personal voice) is a name prefix away from being ours.
const LIST_URL: &str = "https://api.elevenlabs.io/v2/voices?voice_type=personal&page_size=100";

fn extension(content_type: &str) -> &'static str {
    match content_type.split(';').next().unwrap_or_default() {
        "audio/wav" | "audio/x-wav" => "wav",
        "audio/mp4" | "audio/m4a" => "m4a",
        "audio/mpeg" => "mp3",
        "audio/ogg" => "ogg",
        _ => "webm",
    }
}

#[async_trait]
impl VoiceCloner for ElevenLabsVoiceCloner {
    fn provider(&self) -> &'static str {
        "elevenlabs"
    }

    async fn create(&self, name: &str, sample: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
        let failed = |error: &dyn std::fmt::Display| VoiceCloneError::Request(error.to_string());
        let file = Part::bytes(sample.bytes)
            .file_name(format!("sample.{}", extension(&sample.content_type)))
            .mime_str(&sample.content_type)
            .map_err(|e| failed(&e))?;
        let form = Form::new()
            .text("name", name.to_owned())
            .text(
                "description",
                "Diceri call voice, deleted when the participant leaves",
            )
            .text("remove_background_noise", "true")
            .part("files", file);
        let response = self
            .http
            .post(format!("{VOICES_URL}/add"))
            .header("xi-api-key", self.api_key.expose_secret())
            .multipart(form)
            .send()
            .await
            .map_err(|e| failed(&e))?;
        let status = response.status();
        let body = response.text().await.map_err(|e| failed(&e))?;
        if status.is_client_error() {
            return Err(VoiceCloneError::Rejected(body));
        }
        if !status.is_success() {
            return Err(VoiceCloneError::Request(format!("HTTP {status}: {body}")));
        }
        let created: Created = serde_json::from_str(&body).map_err(|e| failed(&e))?;
        Ok(VoiceId(created.voice_id))
    }

    async fn delete(&self, voice: &VoiceId) -> Result<(), VoiceCloneError> {
        let response = self
            .http
            .delete(format!("{VOICES_URL}/{}", voice.0))
            .header("xi-api-key", self.api_key.expose_secret())
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
        let list: VoiceList = self
            .http
            .get(LIST_URL)
            .header("xi-api-key", self.api_key.expose_secret())
            .send()
            .await
            .map_err(|e| failed(&e))?
            .json()
            .await
            .map_err(|e| failed(&e))?;
        let mut deleted = 0;
        for voice in list
            .voices
            .into_iter()
            .filter(|v| v.name.starts_with(prefix))
        {
            self.delete(&VoiceId(voice.voice_id)).await?;
            deleted += 1;
        }
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_extension_follows_the_content_type() {
        assert_eq!(extension("audio/webm;codecs=opus"), "webm");
        assert_eq!(extension("audio/wav"), "wav");
        assert_eq!(extension("audio/mp4"), "m4a");
    }
}

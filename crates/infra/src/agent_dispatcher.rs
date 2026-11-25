use async_trait::async_trait;
use diceri_application::ports::{DispatchError, TranslationDispatcher};
use diceri_domain::RoomId;
use secrecy::{ExposeSecret as _, SecretString};
use serde_json::json;

/// Header the media agent requires on control requests.
pub const AGENT_KEY_HEADER: &str = "x-diceri-agent-key";

/// Starts translation by calling the media agent's control API over HTTP.
#[derive(Debug, Clone)]
pub struct HttpTranslationDispatcher {
    base_url: String,
    key: SecretString,
    http: reqwest::Client,
}

impl HttpTranslationDispatcher {
    #[must_use]
    pub fn new(base_url: String, key: SecretString, http: reqwest::Client) -> Self {
        Self {
            base_url,
            key,
            http,
        }
    }
}

#[async_trait]
impl TranslationDispatcher for HttpTranslationDispatcher {
    async fn start(&self, room: &RoomId) -> Result<(), DispatchError> {
        let response = self
            .http
            .post(format!("{}/sessions", self.base_url.trim_end_matches('/')))
            .header(AGENT_KEY_HEADER, self.key.expose_secret())
            .json(&json!({ "room": room.as_str() }))
            .send()
            .await
            .map_err(|error| DispatchError(error.to_string()))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(DispatchError(format!(
                "agent answered HTTP {}",
                response.status()
            )))
        }
    }
}

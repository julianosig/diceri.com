use std::time::Duration;

use diceri_application::ports::MediaAccessError;
use livekit_api::access_token::{AccessToken, VideoGrants};
use secrecy::{ExposeSecret as _, SecretString};
use serde::de::DeserializeOwned;

const TOKEN_TTL: Duration = Duration::from_secs(60);

/// Minimal client for LiveKit's server `RoomService` (Twirp over HTTP + JSON);
/// avoids pulling the SDK's service stack and its licenses.
#[derive(Debug, Clone)]
pub(crate) struct TwirpClient {
    http_url: String,
    api_key: String,
    api_secret: SecretString,
    http: reqwest::Client,
}

impl TwirpClient {
    /// `server_url` is the client URL (`wss://…`); the API uses its HTTPS form.
    pub(crate) fn new(
        server_url: &str,
        api_key: String,
        api_secret: SecretString,
        http: reqwest::Client,
    ) -> Self {
        Self {
            http_url: http_url(server_url),
            api_key,
            api_secret,
            http,
        }
    }

    /// Calls `livekit.RoomService/<method>` with a short-lived token carrying `grants`.
    pub(crate) async fn call<T: DeserializeOwned>(
        &self,
        method: &str,
        grants: VideoGrants,
        body: &serde_json::Value,
    ) -> Result<T, MediaAccessError> {
        let failed = |error: &dyn std::fmt::Display| MediaAccessError(error.to_string());
        let token = AccessToken::with_api_key(&self.api_key, self.api_secret.expose_secret())
            .with_ttl(TOKEN_TTL)
            .with_grants(grants)
            .to_jwt()
            .map_err(|e| failed(&e))?;
        let response = self
            .http
            .post(format!(
                "{}/twirp/livekit.RoomService/{method}",
                self.http_url
            ))
            .bearer_auth(token)
            .json(body)
            .send()
            .await
            .map_err(|e| failed(&e))?;
        if !response.status().is_success() {
            return Err(MediaAccessError(format!(
                "{method} answered HTTP {}",
                response.status()
            )));
        }
        response.json().await.map_err(|e| failed(&e))
    }
}

fn http_url(server_url: &str) -> String {
    let url = server_url.trim_end_matches('/');
    url.strip_prefix("wss://")
        .map(|rest| format!("https://{rest}"))
        .or_else(|| {
            url.strip_prefix("ws://")
                .map(|rest| format!("http://{rest}"))
        })
        .unwrap_or_else(|| url.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_url_is_the_https_form_of_the_client_url() {
        assert_eq!(http_url("wss://x.livekit.cloud"), "https://x.livekit.cloud");
        assert_eq!(http_url("ws://localhost:7880/"), "http://localhost:7880");
    }
}

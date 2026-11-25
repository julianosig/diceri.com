use std::time::Duration;

use async_trait::async_trait;
use diceri_application::ports::{MediaAccessError, MediaRoomCloser};
use diceri_domain::RoomId;
use livekit_api::access_token::{AccessToken, VideoGrants};
use secrecy::{ExposeSecret as _, SecretString};
use serde_json::json;

const TOKEN_TTL: Duration = Duration::from_secs(60);

/// Ends a LiveKit room for everyone via the server `RoomService/DeleteRoom` API
/// (a single Twirp call; avoids pulling the SDK's service stack).
#[derive(Debug, Clone)]
pub struct LiveKitRoomCloser {
    http_url: String,
    api_key: String,
    api_secret: SecretString,
    http: reqwest::Client,
}

impl LiveKitRoomCloser {
    /// `server_url` is the client URL (`wss://…`); the API uses its HTTPS form.
    #[must_use]
    pub fn new(
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

#[async_trait]
impl MediaRoomCloser for LiveKitRoomCloser {
    async fn close(&self, room: &RoomId) -> Result<(), MediaAccessError> {
        let failed = |error: &dyn std::fmt::Display| MediaAccessError(error.to_string());
        let token = AccessToken::with_api_key(&self.api_key, self.api_secret.expose_secret())
            .with_ttl(TOKEN_TTL)
            .with_grants(VideoGrants {
                room_create: true,
                ..VideoGrants::default()
            })
            .to_jwt()
            .map_err(|e| failed(&e))?;
        let response = self
            .http
            .post(format!(
                "{}/twirp/livekit.RoomService/DeleteRoom",
                self.http_url
            ))
            .bearer_auth(token)
            .json(&json!({ "room": room.as_str() }))
            .send()
            .await
            .map_err(|e| failed(&e))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(MediaAccessError(format!(
                "DeleteRoom answered HTTP {}",
                response.status()
            )))
        }
    }
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

use async_trait::async_trait;
use diceri_application::ports::{MediaAccessError, MediaRoomCloser};
use diceri_domain::RoomId;
use livekit_api::access_token::VideoGrants;
use secrecy::SecretString;
use serde_json::json;

use super::twirp::TwirpClient;

/// Ends a LiveKit room for everyone via the server `RoomService/DeleteRoom` API.
#[derive(Debug, Clone)]
pub struct LiveKitRoomCloser {
    twirp: TwirpClient,
}

impl LiveKitRoomCloser {
    /// `server_url` is the client URL (`wss://…`).
    #[must_use]
    pub fn new(
        server_url: &str,
        api_key: String,
        api_secret: SecretString,
        http: reqwest::Client,
    ) -> Self {
        Self {
            twirp: TwirpClient::new(server_url, api_key, api_secret, http),
        }
    }
}

#[async_trait]
impl MediaRoomCloser for LiveKitRoomCloser {
    async fn close(&self, room: &RoomId) -> Result<(), MediaAccessError> {
        let grants = VideoGrants {
            room_create: true,
            ..VideoGrants::default()
        };
        self.twirp
            .call::<serde::de::IgnoredAny>("DeleteRoom", grants, &json!({ "room": room.as_str() }))
            .await
            .map(|_| ())
    }
}

use std::time::Duration;

use diceri_application::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole,
};
use diceri_domain::RoomId;
use livekit_api::access_token::{AccessToken, VideoGrants};
use secrecy::{ExposeSecret as _, SecretString};

/// Join tokens are only needed to connect; LiveKit keeps the session alive after.
const TOKEN_TTL: Duration = Duration::from_mins(10);

/// Issues LiveKit JWTs scoped to a single room.
#[derive(Debug, Clone)]
pub struct LiveKitAccessIssuer {
    server_url: String,
    api_key: String,
    api_secret: SecretString,
}

impl LiveKitAccessIssuer {
    #[must_use]
    pub fn new(server_url: String, api_key: String, api_secret: SecretString) -> Self {
        Self {
            server_url,
            api_key,
            api_secret,
        }
    }
}

fn grants_for(room: &RoomId, role: MediaRole) -> VideoGrants {
    let speaker = role == MediaRole::Speaker;
    VideoGrants {
        room_join: true,
        room: room.to_string(),
        can_publish: Some(speaker),
        can_subscribe: Some(true),
        can_publish_data: Some(speaker),
        can_publish_sources: if speaker {
            vec!["microphone".to_owned()]
        } else {
            vec![]
        },
        hidden: !speaker,
        ..VideoGrants::default()
    }
}

impl MediaAccessIssuer for LiveKitAccessIssuer {
    fn issue(
        &self,
        room: &RoomId,
        participant: MediaParticipant<'_>,
    ) -> Result<MediaGrant, MediaAccessError> {
        let mut token = AccessToken::with_api_key(&self.api_key, self.api_secret.expose_secret())
            .with_identity(participant.identity)
            .with_ttl(TOKEN_TTL)
            .with_grants(grants_for(room, participant.role));
        if let Some(name) = participant.name {
            token = token.with_name(name.as_str());
        }
        let token = token
            .to_jwt()
            .map_err(|error| MediaAccessError(error.to_string()))?;
        Ok(MediaGrant {
            server_url: self.server_url.clone(),
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room() -> RoomId {
        RoomId::from_random_bytes([5; 16])
    }

    #[test]
    fn speaker_can_publish_microphone_only() {
        let grants = grants_for(&room(), MediaRole::Speaker);
        assert_eq!(grants.can_publish, Some(true));
        assert_eq!(grants.can_publish_sources, vec!["microphone".to_owned()]);
        assert!(!grants.hidden);
    }

    #[test]
    fn monitor_is_hidden_and_cannot_publish() {
        let grants = grants_for(&room(), MediaRole::Monitor);
        assert_eq!(grants.can_publish, Some(false));
        assert!(grants.hidden);
    }

    #[test]
    fn issues_a_jwt_for_the_configured_server() {
        let issuer = LiveKitAccessIssuer::new(
            "wss://example.livekit.cloud".to_owned(),
            "key".to_owned(),
            SecretString::from("secret-that-is-long-enough-for-hs256"),
        );
        let name = "Ana".parse().unwrap();
        let participant = MediaParticipant {
            identity: "p-test",
            name: Some(&name),
            role: MediaRole::Speaker,
        };
        let grant = issuer.issue(&room(), participant).unwrap();
        assert_eq!(grant.server_url, "wss://example.livekit.cloud");
        assert_eq!(grant.token.split('.').count(), 3);
    }
}

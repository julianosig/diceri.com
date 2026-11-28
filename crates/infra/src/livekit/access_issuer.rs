use std::time::Duration;

use diceri_application::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole,
};
use diceri_domain::RoomId;
use livekit_api::access_token::{AccessToken, VideoGrants};
use secrecy::{ExposeSecret as _, SecretString};

/// Join tokens are only needed to connect; LiveKit keeps the session alive after.
/// Only to connect (LiveKit renews it inside the session); as long as a seat is held for
/// someone connecting (`SEAT_GRACE_MS` in the domain), so an unused token never outlives it.
const TOKEN_TTL: Duration = Duration::from_mins(2);

/// A hidden listener connects at once and is removed when its person leaves: its token lasts
/// only long enough to connect, so it cannot be used to come back much later.
const LISTENER_TOKEN_TTL: Duration = Duration::from_secs(30);

fn ttl_for(role: MediaRole) -> Duration {
    if role == MediaRole::Monitor {
        LISTENER_TOKEN_TTL
    } else {
        TOKEN_TTL
    }
}

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

/// Participant attribute carrying the language a participant wants to hear everyone in.
pub const OUTPUT_LANGUAGE_ATTRIBUTE: &str = "output_language";
/// Participant attribute with the id of their consented voice clone, if any.
pub const VOICE_ATTRIBUTE: &str = "voice_id";

fn grants_for(room: &RoomId, role: MediaRole) -> VideoGrants {
    let base = VideoGrants {
        room_join: true,
        room: room.to_string(),
        can_subscribe: Some(true),
        ..VideoGrants::default()
    };
    match role {
        MediaRole::Speaker => VideoGrants {
            can_publish: Some(true),
            // Captions come only from the translator: a person sending data could forge them.
            can_publish_data: Some(false),
            // Voice and camera; never screen share, which nothing in the app offers.
            can_publish_sources: vec!["microphone".to_owned(), "camera".to_owned()],
            // Name, language and voice are set by the server in the token: a person who could
            // edit them could point `voice_id` at someone else's clone (visible in the room).
            can_update_own_metadata: Some(false),
            ..base
        },
        MediaRole::Monitor => VideoGrants {
            can_publish: Some(false),
            can_publish_data: Some(false),
            hidden: true,
            ..base
        },
        MediaRole::Translator => VideoGrants {
            can_publish: Some(true),
            can_publish_data: Some(true),
            agent: true,
            ..base
        },
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
            .with_ttl(ttl_for(participant.role))
            .with_grants(grants_for(room, participant.role));
        if participant.role == MediaRole::Translator {
            token = token.with_kind("agent");
        }
        if let Some(profile) = participant.profile {
            let mut attributes =
                vec![(OUTPUT_LANGUAGE_ATTRIBUTE, profile.output_language.as_str())];
            if let Some(voice) = &profile.voice_id {
                attributes.push((VOICE_ATTRIBUTE, voice.as_str()));
            }
            token = token
                .with_name(profile.name.as_str())
                .with_attributes(attributes);
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
    use diceri_domain::ParticipantProfile;

    use super::*;

    fn room() -> RoomId {
        RoomId::from_parts(5, [5; 10])
    }

    #[test]
    fn speaker_can_publish_microphone_and_camera_only() {
        let grants = grants_for(&room(), MediaRole::Speaker);
        assert_eq!(grants.can_publish, Some(true));
        assert_eq!(
            grants.can_publish_sources,
            vec!["microphone".to_owned(), "camera".to_owned()]
        );
        assert_eq!(grants.can_update_own_metadata, Some(false));
        assert_eq!(grants.can_publish_data, Some(false));
        assert!(!grants.hidden);
    }

    #[test]
    fn translator_is_an_agent_that_publishes_any_source() {
        let grants = grants_for(&room(), MediaRole::Translator);
        assert!(grants.agent);
        assert_eq!(grants.can_publish, Some(true));
        assert_eq!(grants.can_publish_sources, Vec::<String>::new());
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
        let profile = ParticipantProfile {
            name: "Ana".parse().unwrap(),
            output_language: "pt-BR".parse().unwrap(),
            voice_id: None,
        };
        let participant = MediaParticipant {
            identity: "p-test",
            profile: Some(&profile),
            role: MediaRole::Speaker,
        };
        let grant = issuer.issue(&room(), participant).unwrap();
        assert_eq!(grant.server_url, "wss://example.livekit.cloud");
        assert_eq!(grant.token.split('.').count(), 3);
    }
}

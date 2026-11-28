//! LiveKit webhooks (ADR 0018): checked against the project's API key and read as call events.

use diceri_application::ports::{MediaEvent, MediaEventError, MediaEventVerifier};
use diceri_domain::RoomId;
use livekit_api::access_token::TokenVerifier;
use livekit_api::webhooks::{WebhookError, WebhookReceiver};
use secrecy::{ExposeSecret as _, SecretString};

/// Reads LiveKit's signed webhook requests.
pub struct LiveKitWebhooks {
    receiver: WebhookReceiver,
}

impl std::fmt::Debug for LiveKitWebhooks {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LiveKitWebhooks")
            .finish_non_exhaustive()
    }
}

impl LiveKitWebhooks {
    #[must_use]
    pub fn new(api_key: &str, api_secret: &SecretString) -> Self {
        let verifier = TokenVerifier::with_api_key(api_key, api_secret.expose_secret());
        Self {
            receiver: WebhookReceiver::new(verifier),
        }
    }
}

impl MediaEventVerifier for LiveKitWebhooks {
    fn verify(
        &self,
        body: &str,
        authorization: &str,
    ) -> Result<Option<MediaEvent>, MediaEventError> {
        let token = authorization
            .strip_prefix("Bearer ")
            .unwrap_or(authorization);
        let event = self
            .receiver
            .receive(body, token)
            .map_err(|error| match error {
                WebhookError::InvalidData(_) => MediaEventError::Unreadable,
                _ => MediaEventError::Unsigned,
            })?;
        // Media rooms are named after the room id; anything else is not ours.
        let Some(room) = event
            .room
            .as_ref()
            .and_then(|room| room.name.parse::<RoomId>().ok())
        else {
            return Ok(None);
        };
        let identity = event.participant.map(|participant| participant.identity);
        Ok(match (event.event.as_str(), identity) {
            ("participant_joined", Some(identity)) => Some(MediaEvent::Joined { room, identity }),
            ("participant_left" | "participant_connection_aborted", Some(identity)) => {
                Some(MediaEvent::Left { room, identity })
            }
            ("room_finished", _) => Some(MediaEvent::Finished { room }),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine as _;
    use livekit_api::access_token::AccessToken;
    use sha2::{Digest as _, Sha256};

    use super::*;

    const KEY: &str = "devkey";
    const SECRET: &str = "a-secret-long-enough-for-hs256-signing";

    fn signed(body: &str, secret: &str) -> String {
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(body));
        AccessToken::with_api_key(KEY, secret)
            .with_sha256(&hash)
            .to_jwt()
            .unwrap()
    }

    fn webhooks() -> LiveKitWebhooks {
        LiveKitWebhooks::new(KEY, &SecretString::from(SECRET))
    }

    fn joined(room: &str) -> String {
        format!(
            r#"{{"event":"participant_joined","room":{{"name":"{room}"}},"participant":{{"identity":"p-1"}}}}"#
        )
    }

    const ROOM: &str = "019ac3a4-0000-7000-8000-000000000001";

    #[test]
    fn reads_a_signed_join() {
        let body = joined(ROOM);
        let event = webhooks().verify(&body, &signed(&body, SECRET)).unwrap();
        assert_eq!(
            event,
            Some(MediaEvent::Joined {
                room: ROOM.parse().unwrap(),
                identity: "p-1".to_owned()
            })
        );
    }

    #[test]
    fn refuses_another_key_or_another_body() {
        let body = joined(ROOM);
        let other_key = signed(&body, "another-secret-long-enough-for-hs256");
        assert!(matches!(
            webhooks().verify(&body, &other_key),
            Err(MediaEventError::Unsigned)
        ));
        let for_other_body = signed(&joined("x"), SECRET);
        assert!(matches!(
            webhooks().verify(&body, &for_other_body),
            Err(MediaEventError::Unsigned)
        ));
    }

    #[test]
    fn ignores_rooms_not_named_after_a_room_id() {
        let body = joined("someone-elses-room");
        assert_eq!(
            webhooks().verify(&body, &signed(&body, SECRET)).unwrap(),
            None
        );
    }
}

use diceri_application::use_cases::{JoinTicket, Knock, KnockOutcome};
use diceri_domain::{JoinRequest, ParticipantProfile, Room};
use serde::{Deserialize, Serialize};

use crate::ApiError;

/// Public representation of a room.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoomResponse {
    pub id: String,
    pub max_participants: u8,
}

impl From<&Room> for RoomResponse {
    fn from(room: &Room) -> Self {
        Self {
            id: room.id.to_string(),
            max_participants: room.max_participants,
        }
    }
}

/// Returned once to the creator: includes the secret host key.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreatedRoomResponse {
    #[serde(flatten)]
    pub room: RoomResponse,
    pub host_key: String,
}

/// Body carrying how the participant presents themselves.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProfileRequest {
    pub name: String,
    /// BCP 47 tag of the language the participant wants to hear everyone in, e.g. `pt-BR`.
    pub output_language: String,
    /// Voice clone recorded in this room (`POST /rooms/{room}/voices`), if any.
    #[serde(default)]
    pub voice_id: Option<String>,
}

impl ProfileRequest {
    pub(crate) fn profile(&self) -> Result<ParticipantProfile, ApiError> {
        let name = self
            .name
            .parse()
            .map_err(|error| ApiError::Validation(format!("{error}")))?;
        let output_language = self
            .output_language
            .parse()
            .map_err(|error| ApiError::Validation(format!("{error}")))?;
        Ok(ParticipantProfile {
            name,
            output_language,
            voice_id: self.voice_id.clone(),
        })
    }
}

/// Credentials for the browser to connect to the media server.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JoinResponse {
    pub identity: String,
    pub server_url: String,
    pub token: String,
}

impl From<JoinTicket> for JoinResponse {
    fn from(ticket: JoinTicket) -> Self {
        Self {
            identity: ticket.identity,
            server_url: ticket.grant.server_url,
            token: ticket.grant.token,
        }
    }
}

/// Returned to a guest who asks to join.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KnockResponse {
    pub request_id: String,
    pub request_key: String,
}

impl From<Knock> for KnockResponse {
    fn from(knock: Knock) -> Self {
        Self {
            request_id: knock.request_id.to_string(),
            request_key: knock.key.expose().to_owned(),
        }
    }
}

/// A guest waiting in the lobby, as shown to the host.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PendingRequestResponse {
    pub id: String,
    pub name: String,
    pub output_language: String,
}

impl From<JoinRequest> for PendingRequestResponse {
    fn from(request: JoinRequest) -> Self {
        Self {
            id: request.id.to_string(),
            name: request.profile.name.to_string(),
            output_language: request.profile.output_language.to_string(),
        }
    }
}

/// Polled by the waiting guest.
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub(crate) enum OutcomeResponse {
    Pending,
    Denied,
    Admitted { credentials: JoinResponse },
}

impl From<KnockOutcome> for OutcomeResponse {
    fn from(outcome: KnockOutcome) -> Self {
        match outcome {
            KnockOutcome::Pending => Self::Pending,
            KnockOutcome::Denied => Self::Denied,
            KnockOutcome::Admitted(ticket) => Self::Admitted {
                credentials: ticket.into(),
            },
        }
    }
}

/// Returned after recording a voice: attach `voiceId` to the join profile.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VoiceResponse {
    pub voice_id: String,
}

use diceri_application::use_cases::{JoinTicket, Knock, KnockOutcome};
use diceri_domain::{EntryPolicy, JoinRequest, Mascot, ParticipantProfile, Room, RoomSettings};
use serde::{Deserialize, Serialize};

use crate::ApiError;

/// How people get in: `open` (straight in) or `approval` (the host decides).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum EntryDto {
    Open,
    #[default]
    Approval,
}

impl From<EntryPolicy> for EntryDto {
    fn from(entry: EntryPolicy) -> Self {
        match entry {
            EntryPolicy::Open => Self::Open,
            EntryPolicy::Approval => Self::Approval,
        }
    }
}

impl From<EntryDto> for EntryPolicy {
    fn from(entry: EntryDto) -> Self {
        match entry {
            EntryDto::Open => Self::Open,
            EntryDto::Approval => Self::Approval,
        }
    }
}

/// `POST /rooms`: the host names the room and picks who sees it and who gets in.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateRoomRequest {
    pub title: String,
    /// Show on the public live board; private by default.
    #[serde(default)]
    pub listed: bool,
    #[serde(default)]
    pub entry: EntryDto,
    /// One of the mascot names (`wizard`, `robot`, …); `wizard` when omitted.
    #[serde(default)]
    pub mascot: Option<String>,
}

impl CreateRoomRequest {
    pub(crate) fn settings(&self) -> Result<RoomSettings, ApiError> {
        let title = self
            .title
            .parse()
            .map_err(|error| ApiError::Validation(format!("{error}")))?;
        let mascot = self
            .mascot
            .as_deref()
            .map_or(Ok(Mascot::default()), str::parse)
            .map_err(|error| ApiError::Validation(format!("{error}")))?;
        Ok(RoomSettings {
            title,
            listed: self.listed,
            entry: self.entry.into(),
            mascot,
        })
    }
}

/// Public representation of a room (anyone with the link sees it).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoomResponse {
    /// UUIDv7; identifies the room, grants nothing.
    pub id: String,
    /// The shareable link code (`/r/<code>`): whoever has it can reach the room.
    pub code: String,
    pub max_participants: u8,
    pub title: String,
    /// Username of the account that owns (hosts) the room.
    pub owner: String,
    pub entry: EntryDto,
    /// The room's animated symbol (`/mascots/<name>.gif`).
    pub mascot: &'static str,
}

impl From<&Room> for RoomResponse {
    fn from(room: &Room) -> Self {
        Self {
            id: room.id.to_string(),
            code: room.code.to_string(),
            max_participants: room.max_participants,
            title: room.settings.title.to_string(),
            owner: room.owner.to_string(),
            entry: room.settings.entry.into(),
            mascot: room.settings.mascot.as_str(),
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

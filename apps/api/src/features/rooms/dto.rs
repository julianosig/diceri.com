use diceri_application::ports::MediaRole;
use diceri_application::use_cases::JoinTicket;
use diceri_domain::Room;
use serde::{Deserialize, Serialize};

/// Public representation of a room.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RoomResponse {
    pub id: String,
    pub max_participants: u8,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id.to_string(),
            max_participants: room.max_participants,
        }
    }
}

/// Body of `POST /rooms/{id}/join`. Defaults to joining as a speaker.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JoinRequest {
    #[serde(default)]
    pub role: JoinRole,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum JoinRole {
    #[default]
    Speaker,
    Monitor,
}

impl From<JoinRole> for MediaRole {
    fn from(role: JoinRole) -> Self {
        match role {
            JoinRole::Speaker => Self::Speaker,
            JoinRole::Monitor => Self::Monitor,
        }
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

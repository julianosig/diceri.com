use diceri_domain::Room;
use serde::Serialize;

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

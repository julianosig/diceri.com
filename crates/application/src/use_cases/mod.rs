//! Application use cases, one per file.

mod join_room;
mod rooms;

pub use join_room::{JoinRoomError, JoinRoomService, JoinTicket};
pub use rooms::{RoomError, RoomService};

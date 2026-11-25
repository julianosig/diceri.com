//! Pure domain types for Diceri.
//!
//! This crate must stay free of I/O, async runtimes and web frameworks.

mod language;
mod room;
mod room_id;

pub use language::{LanguageCode, LanguageCodeError};
pub use room::Room;
pub use room_id::{RoomId, RoomIdError};

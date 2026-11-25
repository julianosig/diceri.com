//! Application use cases, one per file.

mod admission;
mod rooms;
mod voices;

pub use admission::{AdmissionError, AdmissionService, JoinTicket, Knock, KnockOutcome};
pub use rooms::{RoomError, RoomService};
pub use voices::{CLONE_NAME_PREFIX, VoiceError, VoiceService};

//! Application use cases, one per file.

mod admission;
mod presence;
mod rooms;
mod voices;

pub use admission::{AdmissionError, AdmissionService, JoinTicket, Knock, KnockOutcome};
pub use presence::{PresenceService, PresenceSnapshot, RANKING_SIZE, RankedRoom};
pub use rooms::{RoomError, RoomService};
pub use voices::{VoiceError, VoiceService, clone_name_prefix};

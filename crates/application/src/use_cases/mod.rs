//! Application use cases, one per file.

mod admission;
mod rooms;

pub use admission::{AdmissionError, AdmissionService, JoinTicket, Knock, KnockOutcome};
pub use rooms::{RoomError, RoomService};

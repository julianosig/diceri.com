//! Application use cases, one per file.

mod accounts;
mod admission;
mod call_events;
mod metering;
mod plan_limits;
mod presence;
mod rooms;
mod speech_check;
mod voice_sample;
mod voices;

pub use accounts::{
    AccountError, AccountService, BetaSpots, Registration, SESSION_TTL_MS, SignedIn,
};
pub use admission::{AdmissionError, AdmissionService, JoinTicket, Knock, KnockOutcome};
pub use call_events::CallEventsService;
pub use metering::MeteringService;
pub use plan_limits::{Allowance, Capacity, LimitError, PlanCatalogue};
pub use presence::{PresenceService, PresenceSnapshot, RANKING_SIZE, RankedRoom};
pub use rooms::{RoomError, RoomService};
pub use voices::{VoiceError, VoiceService, account_clone_prefix, clone_name_prefix};

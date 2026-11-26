//! Pure domain types for Diceri.
//!
//! This crate must stay free of I/O, async runtimes, randomness and web frameworks.

mod access_key;
mod base62;
mod display_name;
mod join_request;
mod language;
mod participant_profile;
mod room;
mod room_id;
mod short_text;

pub use access_key::{AccessKey, AccessKeyError};
pub use display_name::{DisplayName, DisplayNameError};
pub use join_request::{JoinRequest, RequestId, RequestIdError, RequestStatus};
pub use language::{LanguageCode, LanguageCodeError};
pub use participant_profile::ParticipantProfile;
pub use room::{Room, RoomRuleError};
pub use room_id::{RoomId, RoomIdError};

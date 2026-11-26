//! Pure domain types for Diceri.
//!
//! This crate must stay free of I/O, async runtimes, randomness and web frameworks.

mod access_key;
mod base62;
mod display_name;
mod ids;
mod join_request;
mod language;
mod mascot;
mod participant_profile;
mod room;
mod room_code;
mod room_settings;
mod short_text;
mod username;

pub use access_key::{AccessKey, AccessKeyError};
pub use display_name::{DisplayName, DisplayNameError};
pub use ids::{AccountId, IdError, ParticipantId, RequestId, RoomId};
pub use join_request::{JoinRequest, RequestStatus};
pub use language::{LanguageCode, LanguageCodeError};
pub use mascot::{Mascot, MascotError};
pub use participant_profile::ParticipantProfile;
pub use room::{Room, RoomRuleError};
pub use room_code::{RoomCode, RoomCodeError};
pub use room_settings::{EntryPolicy, RoomSettings, RoomTitle, RoomTitleError};
pub use username::{Username, UsernameError};

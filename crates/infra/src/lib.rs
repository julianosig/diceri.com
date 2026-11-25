//! Adapters implementing `diceri-application` ports.
//!
//! One adapter per module; provider adapters are grouped by capability:
//! `speech/` (speech-to-text), `translation/`, `synthesis/` (text-to-speech).

mod livekit_access_issuer;
mod memory_room_repository;
mod os_random;
pub mod speech;
pub mod synthesis;
pub mod translation;

pub use livekit_access_issuer::LiveKitAccessIssuer;
pub use memory_room_repository::MemoryRoomRepository;
pub use os_random::OsRandom;

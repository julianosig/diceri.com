//! Adapters implementing `diceri-application` ports.
//!
//! One adapter per module; provider adapters are grouped by capability:
//! `speech/` (speech-to-text), `translation/`, `synthesis/` (text-to-speech).

mod agent_dispatcher;
mod livekit_access_issuer;
mod memory_room_repository;
mod os_random;
pub mod speech;
pub mod synthesis;
pub mod translation;

pub use agent_dispatcher::{AGENT_KEY_HEADER, HttpTranslationDispatcher};
pub use livekit_access_issuer::{LANGUAGE_ATTRIBUTE, LiveKitAccessIssuer};
pub use memory_room_repository::MemoryRoomRepository;
pub use os_random::OsRandom;

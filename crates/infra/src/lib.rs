//! Adapters implementing `diceri-application` ports.
//!
//! One adapter per module; provider adapters are grouped by capability:
//! `speech/` (speech-to-text), `translation/`, `synthesis/` (text-to-speech).

mod agent_dispatcher;
mod livekit;
mod memory_room_repository;
mod os_random;
pub mod speech;
pub mod synthesis;
pub mod translation;

pub use agent_dispatcher::{AGENT_KEY_HEADER, HttpTranslationDispatcher};
pub use livekit::{
    LiveKitAccessIssuer, LiveKitPresence, LiveKitRoomCloser, OUTPUT_LANGUAGE_ATTRIBUTE,
    VOICE_ATTRIBUTE,
};
pub use memory_room_repository::MemoryRoomRepository;
pub use os_random::OsRandom;

/// Selects the process-wide TLS crypto provider. Dependencies enable both
/// `ring` and `aws-lc-rs`, so rustls cannot pick one by itself; call this once
/// at the start of every binary, before any TLS connection.
pub fn install_tls_provider() {
    // Fails only if a provider is already installed, which is fine.
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}

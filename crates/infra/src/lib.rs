//! Adapters implementing `diceri-application` ports.
//!
//! One adapter per module; provider adapters are grouped by capability:
//! `speech/` (speech-to-text), `translation/`, `synthesis/` (text-to-speech).

#[cfg(test)]
mod account_contract;
mod agent_dispatcher;
mod argon2_hasher;
mod livekit;
mod memory_accounts;
mod memory_room_repository;
mod os_random;
pub mod speech;
mod sqlite_accounts;
pub mod synthesis;
pub mod translation;

pub use agent_dispatcher::{AGENT_KEY_HEADER, HttpTranslationDispatcher};
pub use argon2_hasher::Argon2Hasher;
pub use livekit::{
    LiveKitAccessIssuer, LiveKitPresence, LiveKitRoomCloser, OUTPUT_LANGUAGE_ATTRIBUTE,
    VOICE_ATTRIBUTE,
};
pub use memory_accounts::MemoryAccountRepository;
pub use memory_room_repository::MemoryRoomRepository;
pub use os_random::OsRandom;
pub use sqlite_accounts::SqliteAccountRepository;

/// Selects the process-wide TLS crypto provider. Dependencies enable both
/// `ring` and `aws-lc-rs`, so rustls cannot pick one by itself; call this once
/// at the start of every binary, before any TLS connection.
pub fn install_tls_provider() {
    // Fails only if a provider is already installed, which is fine.
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}

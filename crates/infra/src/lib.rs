//! Adapters implementing `diceri-application` ports.
//!
//! One adapter per module. Provider adapters (STT, MT, TTS) will live in
//! feature folders such as `stt/`, `translation/` and `tts/`.

mod livekit_access_issuer;
mod memory_room_repository;
mod random_room_id_generator;

pub use livekit_access_issuer::LiveKitAccessIssuer;
pub use memory_room_repository::MemoryRoomRepository;
pub use random_room_id_generator::RandomRoomIdGenerator;

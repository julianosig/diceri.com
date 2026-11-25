//! Ports (driven side) that adapters must implement.

mod media_access;
mod room_id_generator;
mod room_repository;

pub use media_access::{MediaAccessError, MediaAccessIssuer, MediaGrant, MediaRole};
pub use room_id_generator::RoomIdGenerator;
pub use room_repository::{RoomRepository, RoomRepositoryError};

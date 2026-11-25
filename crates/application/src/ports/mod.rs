//! Ports (driven side) that adapters must implement.

mod media_access;
mod random_source;
mod room_repository;

pub use media_access::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole,
};
pub use random_source::RandomSource;
pub use room_repository::{RoomRepository, RoomRepositoryError};

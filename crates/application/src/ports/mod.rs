//! Ports (driven side) that adapters must implement.

mod room_id_generator;
mod room_repository;

pub use room_id_generator::RoomIdGenerator;
pub use room_repository::{RoomRepository, RoomRepositoryError};

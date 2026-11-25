use diceri_domain::RoomId;

/// Source of unguessable room identifiers.
pub trait RoomIdGenerator: Send + Sync {
    fn generate(&self) -> RoomId;
}

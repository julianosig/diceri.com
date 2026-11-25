use diceri_application::ports::RoomIdGenerator;
use diceri_domain::RoomId;
use rand::Rng as _;

/// Generates room ids from the operating system CSPRNG-seeded thread RNG.
#[derive(Debug, Default, Clone, Copy)]
pub struct RandomRoomIdGenerator;

impl RoomIdGenerator for RandomRoomIdGenerator {
    fn generate(&self) -> RoomId {
        let mut bytes = [0_u8; 16];
        rand::rng().fill_bytes(&mut bytes);
        RoomId::from_random_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_distinct_ids() {
        let generator = RandomRoomIdGenerator;
        assert_ne!(generator.generate(), generator.generate());
    }
}

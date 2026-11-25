use diceri_application::ports::RandomSource;
use rand::Rng as _;

/// Randomness from the thread-local CSPRNG, seeded by the operating system.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsRandom;

impl RandomSource for OsRandom {
    fn random_128(&self) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        rand::rng().fill_bytes(&mut bytes);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_distinct_values() {
        assert_ne!(OsRandom.random_128(), OsRandom.random_128());
    }
}

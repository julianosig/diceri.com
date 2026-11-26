use std::time::{SystemTime, UNIX_EPOCH};

use diceri_application::ports::{Clock, IdGenerator, IdParts, RandomSource};
use rand::Rng as _;

/// The operating system's CSPRNG and clock: randomness, UUIDv7 parts (clock plus
/// randomness) and the current time.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsRandom;

impl RandomSource for OsRandom {
    fn random_128(&self) -> [u8; 16] {
        let mut bytes = [0_u8; 16];
        rand::rng().fill_bytes(&mut bytes);
        bytes
    }
}

impl Clock for OsRandom {
    fn now_ms(&self) -> u64 {
        unix_ms()
    }
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
        })
}

impl IdGenerator for OsRandom {
    /// All 74 non-time bits are fresh randomness (no counter), so ids minted in the
    /// same millisecond are unordered among themselves but never collide in practice.
    fn next_parts(&self) -> IdParts {
        let mut random = [0_u8; 10];
        rand::rng().fill_bytes(&mut random);
        IdParts {
            unix_ms: unix_ms(),
            random,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_distinct_values() {
        assert_ne!(OsRandom.random_128(), OsRandom.random_128());
    }

    #[test]
    fn ids_are_v7_and_follow_the_clock() {
        let first = OsRandom.room_id();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let second = OsRandom.room_id();
        assert!(first < second);
        assert_eq!(first.as_uuid().get_version_num(), 7);
    }
}

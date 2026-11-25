/// Cryptographically secure randomness for ids and secret keys.
pub trait RandomSource: Send + Sync {
    fn random_128(&self) -> [u8; 16];
}

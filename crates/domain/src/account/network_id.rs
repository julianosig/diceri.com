use std::fmt;

/// The network an account was signed up from, as a keyed hash (HMAC-SHA256) of its address
/// (an IPv4 address or an IPv6 /64). Only one account may be signed up per network; the
/// raw address is never kept, and the hash is personal data (`docs/data-inventory.md`).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkId([u8; 32]);

impl NetworkId {
    /// Wraps a hash computed by the composition root (the key never reaches the domain).
    #[must_use]
    pub const fn from_hash(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for NetworkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("NetworkId(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_shows_the_hash() {
        assert_eq!(
            format!("{:?}", NetworkId::from_hash([7; 32])),
            "NetworkId(***)"
        );
    }
}

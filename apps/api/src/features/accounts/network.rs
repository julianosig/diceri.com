//! A few accounts per network: the sign-up's client address, reduced to its network (an IPv4
//! address, or an IPv6 /64, which one household or phone usually gets) and hashed with a
//! server key, so the raw address is never stored and the hash cannot be reversed by
//! trying every address without the key.

use std::net::IpAddr;

use axum::http::{HeaderMap, HeaderName};
use diceri_application::ports::SignupNetwork;
use diceri_domain::NetworkId;
use secrecy::{ExposeSecret as _, SecretString};
use sha2::{Digest as _, Sha256};

/// SHA-256 block size, for HMAC (RFC 2104).
const BLOCK: usize = 64;

/// Turns a request into the network it comes from.
#[derive(Clone)]
pub(crate) struct SignupNetworks {
    key: SecretString,
    /// Accounts one network may sign up.
    max_accounts: u32,
    /// Header holding the real client IP behind a proxy (e.g. `cf-connecting-ip`).
    ip_header: Option<HeaderName>,
}

impl std::fmt::Debug for SignupNetworks {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SignupNetworks(***)")
    }
}

impl SignupNetworks {
    pub(crate) fn new(key: SecretString, max_accounts: u32, ip_header: Option<HeaderName>) -> Self {
        Self {
            key,
            max_accounts,
            ip_header,
        }
    }

    /// The sign-up's network, or `None` when the client address is unknown.
    pub(crate) fn of(&self, headers: &HeaderMap, peer: Option<IpAddr>) -> Option<SignupNetwork> {
        let ip = crate::rate_limit::client_ip(headers, peer, self.ip_header.as_ref())?;
        let id = NetworkId::from_hash(hmac(
            self.key.expose_secret().as_bytes(),
            &network_bytes(ip),
        ));
        Some(SignupNetwork {
            id,
            max_accounts: self.max_accounts,
        })
    }
}

/// The address that names a network: IPv4 whole (IPv4-mapped IPv6 included), IPv6 by /64.
fn network_bytes(ip: IpAddr) -> Vec<u8> {
    match ip {
        IpAddr::V4(v4) => v4.octets().to_vec(),
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or_else(
            || v6.octets().iter().take(8).copied().collect(),
            |v4| v4.octets().to_vec(),
        ),
    }
}

/// HMAC-SHA256 (RFC 2104).
fn hmac(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut block = [0_u8; BLOCK];
    if key.len() > BLOCK {
        block
            .iter_mut()
            .zip(Sha256::digest(key))
            .for_each(|(slot, byte)| *slot = byte);
    } else {
        block
            .iter_mut()
            .zip(key)
            .for_each(|(slot, byte)| *slot = *byte);
    }
    let pad = |with: u8| block.map(|byte| byte ^ with);
    let inner = Sha256::new()
        .chain_update(pad(0x36))
        .chain_update(message)
        .finalize();
    Sha256::new()
        .chain_update(pad(0x5c))
        .chain_update(inner)
        .finalize()
        .into()
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::*;

    #[test]
    fn matches_the_rfc_4231_test_vector() {
        // RFC 4231, test case 2.
        let mac = hmac(b"Jefe", b"what do ya want for nothing?");
        let hex = mac.iter().fold(String::new(), |mut out, byte| {
            let _ = write!(out, "{byte:02x}");
            out
        });
        assert_eq!(
            hex,
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn one_ipv6_household_is_one_network() {
        let a: IpAddr = "2001:db8:1:2:aaaa::1".parse().unwrap();
        let b: IpAddr = "2001:db8:1:2:bbbb::9".parse().unwrap();
        let other: IpAddr = "2001:db8:1:3::1".parse().unwrap();
        assert_eq!(network_bytes(a), network_bytes(b));
        assert_ne!(network_bytes(a), network_bytes(other));
        let mapped: IpAddr = "::ffff:203.0.113.9".parse().unwrap();
        assert_eq!(network_bytes(mapped), vec![203, 0, 113, 9]);
    }

    #[test]
    fn the_proxy_header_names_the_network() {
        let networks = SignupNetworks::new(
            SecretString::from("key".to_owned()),
            3,
            Some(HeaderName::from_static("cf-connecting-ip")),
        );
        let mut headers = HeaderMap::new();
        headers.insert("cf-connecting-ip", "203.0.113.9".parse().unwrap());
        let tunnel = Some("127.0.0.1".parse().unwrap());
        assert_eq!(
            networks.of(&headers, tunnel),
            networks.of(&headers, Some("10.0.0.1".parse().unwrap()))
        );
        assert_ne!(
            networks.of(&headers, tunnel),
            networks.of(&HeaderMap::new(), tunnel)
        );
    }
}

//! AES-256-GCM for voice recordings: a fresh random nonce per recording, and the account
//! id as associated data, so a recording cannot be decrypted as another account's.

use aes_gcm::aead::{Aead as _, KeyInit as _, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use diceri_domain::AccountId;
use rand::Rng as _;
use secrecy::{ExposeSecret as _, SecretBox};

/// A 256-bit key for voice recordings (`DICERI_VOICE_KEY`).
pub(crate) struct VoiceKey(SecretBox<[u8; 32]>);

impl VoiceKey {
    pub(crate) fn new(bytes: [u8; 32]) -> Self {
        Self(SecretBox::new(Box::new(bytes)))
    }

    fn cipher(&self) -> Aes256Gcm {
        Aes256Gcm::new(self.0.expose_secret().into())
    }

    /// Encrypts `audio`; returns the nonce and the ciphertext (with its tag).
    pub(crate) fn seal(&self, account: &AccountId, audio: &[u8]) -> Option<([u8; 12], Vec<u8>)> {
        let mut nonce = [0_u8; 12];
        rand::rng().fill_bytes(&mut nonce);
        let id = account.as_uuid();
        let payload = Payload {
            msg: audio,
            aad: id.as_bytes(),
        };
        let sealed = self.cipher().encrypt(&Nonce::from(nonce), payload).ok()?;
        Some((nonce, sealed))
    }

    /// Decrypts; `None` when the key, nonce or account do not match (or it was altered).
    pub(crate) fn open(
        &self,
        account: &AccountId,
        nonce: [u8; 12],
        sealed: &[u8],
    ) -> Option<Vec<u8>> {
        let id = account.as_uuid();
        let payload = Payload {
            msg: sealed,
            aad: id.as_bytes(),
        };
        self.cipher().decrypt(&Nonce::from(nonce), payload).ok()
    }
}

impl std::fmt::Debug for VoiceKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("VoiceKey(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_same_key_and_account_open_a_recording() {
        let key = VoiceKey::new([7; 32]);
        let ana = AccountId::from_parts(1, [1; 10]);
        let bob = AccountId::from_parts(2, [2; 10]);
        let (nonce, sealed) = key.seal(&ana, b"hello voice").unwrap();
        assert_ne!(sealed.as_slice(), b"hello voice".as_slice());
        assert_eq!(key.open(&ana, nonce, &sealed).unwrap(), b"hello voice");
        assert!(key.open(&bob, nonce, &sealed).is_none());
        assert!(VoiceKey::new([8; 32]).open(&ana, nonce, &sealed).is_none());
    }
}

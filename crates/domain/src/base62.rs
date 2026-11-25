//! Fixed-length base62 encoding of 128-bit random values.

const ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Length of an encoded 128-bit value.
pub(crate) const ENCODED_LEN: usize = 22;

pub(crate) fn encode_128(bytes: [u8; 16]) -> String {
    let mut value = u128::from_be_bytes(bytes);
    let mut encoded = [b'0'; ENCODED_LEN];
    for slot in encoded.iter_mut().rev() {
        let digit = usize::try_from(value % 62).unwrap_or_default();
        *slot = ALPHABET.get(digit).copied().unwrap_or(b'0');
        value /= 62;
    }
    encoded.iter().map(|&byte| char::from(byte)).collect()
}

/// Validation failure for an encoded token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenFormatError {
    Length,
    Character,
}

pub(crate) fn validate(raw: &str) -> Result<(), TokenFormatError> {
    if raw.len() != ENCODED_LEN {
        return Err(TokenFormatError::Length);
    }
    if !raw.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(TokenFormatError::Character);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_to_fixed_length() {
        assert_eq!(encode_128([255; 16]).len(), ENCODED_LEN);
        assert_eq!(encode_128([0; 16]).len(), ENCODED_LEN);
    }

    #[test]
    fn validates_length_and_alphabet() {
        assert_eq!(validate(&encode_128([7; 16])), Ok(()));
        assert_eq!(validate("abc"), Err(TokenFormatError::Length));
        assert_eq!(
            validate("abcdefghijklmnopqrstu-"),
            Err(TokenFormatError::Character)
        );
    }
}

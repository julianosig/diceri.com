/// Reassembles little-endian 16-bit samples from arbitrarily split byte chunks.
#[derive(Debug, Default)]
pub(super) struct PcmDecoder {
    carry: Option<u8>,
}

impl PcmDecoder {
    pub(super) fn push(&mut self, bytes: &[u8]) -> Vec<i16> {
        let mut samples = Vec::with_capacity(bytes.len() / 2 + 1);
        let mut iter = bytes.iter().copied();
        if let Some(low) = self.carry.take() {
            match iter.next() {
                Some(high) => samples.push(i16::from_le_bytes([low, high])),
                None => self.carry = Some(low),
            }
        }
        loop {
            match (iter.next(), iter.next()) {
                (Some(low), Some(high)) => samples.push(i16::from_le_bytes([low, high])),
                (Some(low), None) => {
                    self.carry = Some(low);
                    break;
                }
                _ => break,
            }
        }
        samples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_samples_split_across_chunks() {
        let mut decoder = PcmDecoder::default();
        assert_eq!(decoder.push(&[1, 0, 0xFE]), vec![1]);
        assert_eq!(decoder.push(&[0xFF, 3]), vec![-2]);
        assert_eq!(decoder.push(&[0]), vec![3]);
    }
}

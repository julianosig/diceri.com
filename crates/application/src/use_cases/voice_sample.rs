//! Free checks on a voice recording, before any paid provider sees it: it must be
//! 16 kHz mono 16-bit WAV (what speech-to-text reads) and loud enough for long enough
//! to hold speech at all.

use crate::ports::STT_SAMPLE_RATE;

/// 20 ms analysis frames at 16 kHz.
const FRAME_SAMPLES: usize = 320;
const FRAME_MS: u32 = 20;
/// Frame loudness (RMS of 16-bit samples) from which a frame counts as voiced, about
/// -36 dBFS: quiet room noise stays well below, a voice near the microphone above.
const VOICED_RMS: f64 = 500.0;
/// Voiced audio a recording needs before it is worth transcribing.
pub(crate) const MIN_VOICED_MS: u32 = 1_500;
/// Audio transcribed to tell speech apart: a few seconds hold plenty of words.
const SPEECH_WINDOW_SAMPLES: usize = 6 * 16_000;

/// The samples of a 16 kHz mono 16-bit PCM WAV, or `None` for anything else.
pub(crate) fn wav_samples(bytes: &[u8]) -> Option<Vec<i16>> {
    if bytes.get(0..4)? != b"RIFF" || bytes.get(8..12)? != b"WAVE" {
        return None;
    }
    let mut at = 12;
    let mut format_ok = false;
    while let Some(id) = bytes.get(at..at + 4) {
        let size = usize::try_from(le_u32(bytes, at + 4)?).ok()?;
        let body = bytes.get(at + 8..at + 8 + size)?;
        match id {
            b"fmt " => format_ok = is_speech_pcm(body)?,
            b"data" if format_ok => return Some(pcm16(body)),
            b"data" => return None,
            _ => {}
        }
        // Chunks are word-aligned: an odd size is followed by one padding byte.
        at += 8 + size + (size & 1);
    }
    None
}

/// Milliseconds of `samples` loud enough to be someone speaking.
pub(crate) fn voiced_ms(samples: &[i16]) -> u32 {
    let threshold = VOICED_RMS * VOICED_RMS;
    let voiced = samples
        .as_chunks::<FRAME_SAMPLES>()
        .0
        .iter()
        .filter(|frame| mean_square(frame.as_slice()) >= threshold)
        .count();
    u32::try_from(voiced)
        .unwrap_or(u32::MAX)
        .saturating_mul(FRAME_MS)
}

/// Up to six seconds from the first voiced frame: what is worth transcribing.
pub(crate) fn speech_window(samples: &[i16]) -> &[i16] {
    let threshold = VOICED_RMS * VOICED_RMS;
    let first = samples
        .as_chunks::<FRAME_SAMPLES>()
        .0
        .iter()
        .position(|frame| mean_square(frame.as_slice()) >= threshold)
        .unwrap_or(0);
    let start = first * FRAME_SAMPLES;
    let end = samples.len().min(start + SPEECH_WINDOW_SAMPLES);
    samples.get(start..end).unwrap_or_default()
}

fn is_speech_pcm(format: &[u8]) -> Option<bool> {
    let pcm = le_u16(format, 0)? == 1;
    let mono = le_u16(format, 2)? == 1;
    let rate = le_u32(format, 4)? == STT_SAMPLE_RATE;
    let bits = le_u16(format, 14)? == 16;
    Some(pcm && mono && rate && bits)
}

fn pcm16(data: &[u8]) -> Vec<i16> {
    data.as_chunks::<2>()
        .0
        .iter()
        .map(|pair| i16::from_le_bytes(*pair))
        .collect()
}

fn mean_square(frame: &[i16]) -> f64 {
    let sum: f64 = frame.iter().map(|&s| f64::from(s) * f64::from(s)).sum();
    sum / f64::from(u16::try_from(frame.len()).unwrap_or(u16::MAX))
}

fn le_u16(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(bytes.get(at..at + 2)?.try_into().ok()?))
}

fn le_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// A 16 kHz mono 16-bit WAV of `samples`, for tests here and in the voice use case.
#[cfg(test)]
pub(crate) fn wav(samples: &[i16]) -> Vec<u8> {
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let data_len = u32::try_from(data.len()).unwrap_or(u32::MAX);
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16_u32.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&STT_SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(STT_SAMPLE_RATE * 2).to_le_bytes());
    out.extend_from_slice(&2_u16.to_le_bytes());
    out.extend_from_slice(&16_u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    out.extend_from_slice(&data);
    out
}

/// `ms` of a loud square wave (voiced) or of near silence, for tests.
#[cfg(test)]
pub(crate) fn tone(ms: usize, loud: bool) -> Vec<i16> {
    let amplitude = if loud { 4_000 } else { 30 };
    (0..ms * 16)
        .map(|i| {
            if (i / 20) % 2 == 0 {
                amplitude
            } else {
                -amplitude
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_16khz_mono_pcm_wav() {
        let samples = tone(100, true);
        assert_eq!(wav_samples(&wav(&samples)), Some(samples));
    }

    #[test]
    fn rejects_other_formats() {
        assert_eq!(wav_samples(b"\x1aE\xdf\xa3 webm bytes"), None);
        let mut stereo = wav(&tone(100, true));
        stereo[22] = 2;
        assert_eq!(wav_samples(&stereo), None);
        let mut other_rate = wav(&tone(100, true));
        other_rate[24..28].copy_from_slice(&44_100_u32.to_le_bytes());
        assert_eq!(wav_samples(&other_rate), None);
    }

    #[test]
    fn transcribes_a_few_seconds_from_the_first_voice() {
        let mut samples = tone(2_000, false);
        samples.extend(tone(10_000, true));
        let window = speech_window(&samples);
        assert_eq!(window.len(), 6 * 16_000);
        assert_eq!(voiced_ms(window), 6_000);
        assert_eq!(speech_window(&tone(500, true)).len(), 8_000);
    }

    #[test]
    fn measures_only_loud_frames_as_voiced() {
        let mut samples = tone(1_000, false);
        samples.extend(tone(600, true));
        assert_eq!(voiced_ms(&samples), 600);
        assert_eq!(voiced_ms(&tone(3_000, false)), 0);
    }
}

//! Checks before a recording reaches the cloning provider: the room's call is open and has
//! room for a voice, and the recording is clear speech.

use diceri_domain::{Room, RoomCode, RoomRuleError};

use super::{MAX_SAMPLE_BYTES, MIN_SAMPLE_BYTES, VoiceError, VoiceService};
use crate::ports::VoiceSample;
use crate::use_cases::speech_check::{MIN_LETTERS, letters_heard};
use crate::use_cases::voice_sample::{MIN_VOICED_MS, speech_window, voiced_ms, wav_samples};

impl VoiceService {
    pub(super) async fn find_room(&self, code: &RoomCode) -> Result<Room, VoiceError> {
        self.rooms
            .find_by_code(code)
            .await?
            .ok_or(VoiceError::NotFound)
    }

    /// The room as it is now, if its call is open and has room for one more voice.
    pub(super) async fn open_room_with_space(&self, code: &RoomCode) -> Result<Room, VoiceError> {
        let room = self.find_room(code).await?;
        room.ensure_open()?;
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        Ok(room)
    }

    /// The room, once `sample` is known to be clear speech and the room has room for a voice.
    pub(super) async fn checked_room(
        &self,
        code: &RoomCode,
        sample: &VoiceSample,
    ) -> Result<Room, VoiceError> {
        if !(MIN_SAMPLE_BYTES..=MAX_SAMPLE_BYTES).contains(&sample.bytes.len()) {
            return Err(VoiceError::InvalidSample);
        }
        // Free checks first: the format, then enough loud audio to be speech at all.
        let samples = wav_samples(&sample.bytes).ok_or(VoiceError::InvalidSample)?;
        if voiced_ms(&samples) < MIN_VOICED_MS {
            return Err(VoiceError::NoSpeech);
        }
        // Check before the paid recognizer and provider; both are checked again afterwards.
        let room = self.open_room_with_space(code).await?;
        // The recognizer must hear words: noise or music that passed the loudness check
        // still never reaches the cloning provider.
        let started = std::time::Instant::now();
        let letters = letters_heard(self.recognizer.as_ref(), speech_window(&samples)).await?;
        let elapsed_ms = started.elapsed().as_millis();
        tracing::info!(room_id = %room.id, elapsed_ms, speech = letters >= MIN_LETTERS, "voice sample checked");
        if letters < MIN_LETTERS {
            return Err(VoiceError::NoSpeech);
        }
        Ok(room)
    }
}

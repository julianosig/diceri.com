use std::sync::Arc;

use diceri_domain::{AccountId, RoomCode, RoomRuleError};

use super::speech_check::{MIN_LETTERS, letters_heard};
use super::voice_sample::{MIN_VOICED_MS, speech_window, voiced_ms, wav_samples};
use crate::ports::{
    Clock, RoomRepository, RoomRepositoryError, SpeechToText, SpeechToTextError, VoiceCloneError,
    VoiceCloner, VoiceId, VoiceSample, VoiceSampleStore, VoiceSampleStoreError,
};

/// Every clone is named `diceri-<deployment>-…` so each deployment (prod, dev, e2e)
/// can find and delete its own orphans without touching the others' live clones.
#[must_use]
pub fn clone_name_prefix(deployment: &str) -> String {
    format!("diceri-{deployment}-")
}

/// Bounds on an uploaded sample: 5–15 s of 16 kHz mono 16-bit WAV is 160–480 KiB.
const MIN_SAMPLE_BYTES: usize = 8 * 1024;
const MAX_SAMPLE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum VoiceError {
    #[error("room not found")]
    NotFound,
    #[error("voice sample must be 16 kHz mono 16-bit WAV between 8 KiB and 5 MiB")]
    InvalidSample,
    /// The recording holds no clear speech (silence, noise, music): nothing is cloned.
    #[error("no clear speech in the recording")]
    NoSpeech,
    /// Speech could not be checked, so nothing is sent to the cloning provider.
    #[error(transparent)]
    SpeechCheck(#[from] SpeechToTextError),
    #[error(transparent)]
    Rule(#[from] RoomRuleError),
    #[error(transparent)]
    Clone(#[from] VoiceCloneError),
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
    /// The account has no saved recording to clone again.
    #[error("no saved voice")]
    NoSavedVoice,
    #[error(transparent)]
    Samples(#[from] VoiceSampleStoreError),
}

/// Turns a few seconds of consented speech into a voice clone owned by the room. Only a
/// recording that is clearly someone speaking reaches the cloning provider.
#[derive(Clone)]
pub struct VoiceService {
    rooms: Arc<dyn RoomRepository>,
    cloner: Arc<dyn VoiceCloner>,
    recognizer: Arc<dyn SpeechToText>,
    name_prefix: String,
    /// Where each account's recording is kept (ADR 0015); `None`: not kept.
    samples: Option<(Arc<dyn VoiceSampleStore>, Arc<dyn Clock>)>,
}

impl std::fmt::Debug for VoiceService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VoiceService")
            .finish_non_exhaustive()
    }
}

impl VoiceService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        cloner: Arc<dyn VoiceCloner>,
        recognizer: Arc<dyn SpeechToText>,
        deployment: &str,
    ) -> Self {
        Self {
            rooms,
            cloner,
            recognizer,
            name_prefix: clone_name_prefix(deployment),
            samples: None,
        }
    }

    /// Keeps each account's recording in `samples` so it can be cloned again later.
    #[must_use]
    pub fn with_samples(
        mut self,
        samples: Arc<dyn VoiceSampleStore>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        self.samples = Some((samples, clock));
        self
    }

    /// Keeps no recordings (no voice key configured): they are used for the call only.
    #[must_use]
    pub fn without_samples(mut self) -> Self {
        self.samples = None;
        self
    }

    /// Clones `sample` for the room like `clone_voice`, then keeps it as `account`'s
    /// recording (replacing an older one). The call goes on even if keeping it fails.
    pub async fn record_for(
        &self,
        code: &RoomCode,
        account: &AccountId,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        let voice = self.clone_voice(code, sample.clone()).await?;
        if let Some((samples, clock)) = &self.samples
            && let Err(error) = samples.save(account, &sample, clock.now_ms()).await
        {
            tracing::warn!(%error, account_id = %account, "voice recording not kept");
        }
        Ok(voice)
    }

    /// Clones `account`'s kept recording for the room (it passed the speech checks when
    /// it was recorded).
    pub async fn clone_saved(
        &self,
        code: &RoomCode,
        account: &AccountId,
    ) -> Result<VoiceId, VoiceError> {
        let (samples, _) = self.samples.as_ref().ok_or(VoiceError::NoSavedVoice)?;
        let sample = samples
            .find(account)
            .await?
            .ok_or(VoiceError::NoSavedVoice)?;
        let mut room = self
            .rooms
            .find_by_code(code)
            .await?
            .ok_or(VoiceError::NotFound)?;
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        let voice = self
            .cloner
            .create(&format!("{}{}", self.name_prefix, room.id), sample)
            .await?;
        room.register_voice(voice.0.clone())?;
        self.rooms.update(room).await?;
        Ok(voice)
    }

    /// Whether `account` has a kept recording.
    pub async fn has_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        match &self.samples {
            Some((samples, _)) => Ok(samples.has(account).await?),
            None => Ok(false),
        }
    }

    /// Deletes `account`'s kept recording ("Delete my voice"); `false` when there was none.
    pub async fn delete_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        match &self.samples {
            Some((samples, _)) => Ok(samples.delete(account).await?),
            None => Ok(false),
        }
    }

    pub async fn clone_voice(
        &self,
        code: &RoomCode,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        if !(MIN_SAMPLE_BYTES..=MAX_SAMPLE_BYTES).contains(&sample.bytes.len()) {
            return Err(VoiceError::InvalidSample);
        }
        // Free checks first: the format, then enough loud audio to be speech at all.
        let samples = wav_samples(&sample.bytes).ok_or(VoiceError::InvalidSample)?;
        if voiced_ms(&samples) < MIN_VOICED_MS {
            return Err(VoiceError::NoSpeech);
        }
        let mut room = self
            .rooms
            .find_by_code(code)
            .await?
            .ok_or(VoiceError::NotFound)?;
        // Check before the paid provider call; registering re-checks afterwards.
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        // The recognizer must hear words: noise or music that passed the loudness check
        // still never reaches the cloning provider.
        let started = std::time::Instant::now();
        let letters = letters_heard(self.recognizer.as_ref(), speech_window(&samples)).await?;
        let elapsed_ms = started.elapsed().as_millis();
        tracing::info!(room_id = %room.id, elapsed_ms, speech = letters >= MIN_LETTERS, "voice sample checked");
        if letters < MIN_LETTERS {
            return Err(VoiceError::NoSpeech);
        }
        // Named by the room id: the provider never sees the room's link code.
        let voice = self
            .cloner
            .create(&format!("{}{}", self.name_prefix, room.id), sample)
            .await?;
        room.register_voice(voice.0.clone())?;
        self.rooms.update(room).await?;
        Ok(voice)
    }
}

#[cfg(test)]
#[path = "voices_tests.rs"]
mod tests;

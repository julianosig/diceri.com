use std::sync::Arc;

use diceri_domain::{AccountId, Room, RoomCode, RoomId, RoomRuleError};

use super::speech_check::{MIN_LETTERS, letters_heard};
use super::voice_sample::{MIN_VOICED_MS, speech_window, voiced_ms, wav_samples};
use crate::ports::{
    Clock, KeptClone, RoomChangeError, RoomRepository, RoomRepositoryError, SpeechToText,
    SpeechToTextError, VoiceCloneError, VoiceCloner, VoiceId, VoiceSample, VoiceSampleStore,
    VoiceSampleStoreError,
};

/// Every clone is named `diceri-<deployment>-…` so each deployment (prod, dev, e2e)
/// can find and delete its own orphans without touching the others' live clones.
#[must_use]
pub fn clone_name_prefix(deployment: &str) -> String {
    format!("diceri-{deployment}-")
}

/// Account clones are named `diceri-account-<deployment>-…`: outside the per-call prefix, so
/// the startup sweep of orphaned call clones never deletes them.
#[must_use]
pub fn account_clone_prefix(deployment: &str) -> String {
    format!("diceri-account-{deployment}-")
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

impl From<RoomChangeError> for VoiceError {
    fn from(error: RoomChangeError) -> Self {
        match error {
            RoomChangeError::Rule(rule) => Self::Rule(rule),
            RoomChangeError::Repository(repository) => Self::Repository(repository),
        }
    }
}

/// Turns a few seconds of consented speech into a voice clone owned by the room. Only a
/// recording that is clearly someone speaking reaches the cloning provider.
#[derive(Clone)]
pub struct VoiceService {
    rooms: Arc<dyn RoomRepository>,
    cloner: Arc<dyn VoiceCloner>,
    recognizer: Arc<dyn SpeechToText>,
    name_prefix: String,
    /// Account clones (ADR 0017) are named apart, so the per-call orphan sweep leaves them.
    account_prefix: String,
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
            account_prefix: account_clone_prefix(deployment),
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

    /// Records `account`'s voice in this room: checks it is speech and clones it. With kept
    /// recordings (ADR 0015) the recording is kept and the clone becomes the account's, reused
    /// in later calls and replacing an older one (ADR 0017); otherwise it lasts the call.
    pub async fn record_for(
        &self,
        code: &RoomCode,
        account: &AccountId,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        let room = self.checked_room(code, &sample).await?;
        let Some((samples, clock)) = &self.samples else {
            let voice = self.cloner.create(&self.call_name(&room), sample).await?;
            // A call's clone that cannot join the room (it ended meanwhile) would be orphaned.
            if let Err(error) = self.register(room.id, account, &voice).await {
                self.delete_at_provider(&voice).await;
                return Err(error);
            }
            return Ok(voice);
        };
        let previous = samples.kept_clone(account).await?;
        let voice = self
            .cloner
            .create(&self.account_name(account), sample.clone())
            .await?;
        if let Err(error) = samples.save(account, &sample, clock.now_ms()).await {
            warn_not_kept(&error, account);
        }
        // Kept first: the account's clone is tracked even if the call ended meanwhile.
        self.keep(account, &voice).await;
        if let Some(old) = previous.filter(|old| self.is_ours(old) && old.voice != voice) {
            self.delete_at_provider(&old.voice).await;
        }
        self.register(room.id, account, &voice).await?;
        Ok(voice)
    }

    /// The account's voice for this room: its kept clone when the provider holds one
    /// (instant), otherwise a clone made once from the kept recording and kept from then on.
    pub async fn clone_saved(
        &self,
        code: &RoomCode,
        account: &AccountId,
    ) -> Result<VoiceId, VoiceError> {
        let (samples, _) = self.samples.as_ref().ok_or(VoiceError::NoSavedVoice)?;
        let room = self
            .rooms
            .find_by_code(code)
            .await?
            .ok_or(VoiceError::NotFound)?;
        if let Some(kept) = samples
            .kept_clone(account)
            .await?
            .filter(|kept| self.is_ours(kept))
        {
            self.register(room.id, account, &kept.voice).await?;
            return Ok(kept.voice);
        }
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        let sample = samples
            .find(account)
            .await?
            .ok_or(VoiceError::NoSavedVoice)?;
        let voice = self
            .cloner
            .create(&self.account_name(account), sample)
            .await?;
        self.keep(account, &voice).await;
        self.register(room.id, account, &voice).await?;
        Ok(voice)
    }

    /// Whether `account` has a kept recording.
    pub async fn has_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        match &self.samples {
            Some((samples, _)) => Ok(samples.has(account).await?),
            None => Ok(false),
        }
    }

    /// "Delete my voice": the kept recording and the account's clone at the provider go;
    /// `false` when there was no recording.
    pub async fn delete_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        let Some((samples, _)) = &self.samples else {
            return Ok(false);
        };
        let kept = samples.kept_clone(account).await?;
        let deleted = samples.delete(account).await?;
        if let Some(kept) = kept.filter(|kept| self.is_ours(kept)) {
            self.delete_at_provider(&kept.voice).await;
        }
        Ok(deleted)
    }

    /// The room, once `sample` is known to be clear speech and the room has room for a voice.
    async fn checked_room(
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
        let room = self
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
        Ok(room)
    }

    /// Registers `voice` in the room as `account`'s, on the room as it is now: cloning takes
    /// seconds, and a copy read before it would undo what happened meanwhile (the host ending
    /// the call, a guest let in). Rejoining (a reload) reuses the clone: registered once.
    async fn register(
        &self,
        room: RoomId,
        account: &AccountId,
        voice: &VoiceId,
    ) -> Result<(), VoiceError> {
        let (account, voice_id) = (*account, voice.0.clone());
        let change = move |room: &mut Room| {
            room.ensure_open()?;
            if room.has_voice_of(&account, &voice_id) {
                return Ok(());
            }
            room.register_voice(account, voice_id)
        };
        self.rooms.modify(&room, Box::new(change)).await?;
        Ok(())
    }

    /// Named by the room id: the provider never sees the room's link code.
    fn call_name(&self, room: &Room) -> String {
        format!("{}{}", self.name_prefix, room.id)
    }

    fn account_name(&self, account: &AccountId) -> String {
        format!("{}{}", self.account_prefix, account)
    }

    /// Clones held by another provider cannot be used (or deleted) through this one.
    fn is_ours(&self, kept: &KeptClone) -> bool {
        kept.provider == self.cloner.provider()
    }

    async fn keep(&self, account: &AccountId, voice: &VoiceId) {
        let Some((samples, clock)) = &self.samples else {
            return;
        };
        let kept = KeptClone {
            provider: self.cloner.provider().to_owned(),
            voice: voice.clone(),
        };
        if let Err(error) = samples.keep_clone(account, &kept, clock.now_ms()).await {
            tracing::warn!(%error, account_id = %account, "account voice clone not kept");
        }
    }

    async fn delete_at_provider(&self, voice: &VoiceId) {
        if let Err(error) = self.cloner.delete(voice).await {
            tracing::warn!(%error, "account voice clone not deleted at the provider");
        }
    }
}

fn warn_not_kept(error: &VoiceSampleStoreError, account: &AccountId) {
    tracing::warn!(%error, account_id = %account, "voice recording not kept");
}

#[cfg(test)]
#[path = "voices_tests.rs"]
mod tests;

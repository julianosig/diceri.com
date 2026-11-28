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
    /// The operation's task stopped before finishing (a panic).
    #[error("voice operation stopped unexpectedly")]
    Interrupted,
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
    /// One voice operation at a time per account: a recording finishing while "Delete my
    /// voice" runs (another tab) must not leave a clone nobody tracks.
    locks: Arc<AccountLocks>,
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
            locks: Arc::default(),
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
        let (this, account) = (self.clone(), *account);
        // Waiting for its turn and checking the recording can still be cut short; once the
        // provider is asked for a clone, it always finishes (kept or deleted, never lost).
        let turn = self.locks.lock(&account).await;
        let room = self.checked_room(code, &sample).await?;
        to_completion(async move {
            let _turn = turn;
            this.record_in(room, &account, sample).await
        })
        .await
    }

    async fn record_in(
        &self,
        room: Room,
        account: &AccountId,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        let Some((samples, clock)) = &self.samples else {
            let voice = self.cloner.create(&self.call_name(&room), sample).await?;
            // A call's clone that cannot join the room (it ended meanwhile) would be orphaned.
            if let Err(error) = self.register(room.id, account, &voice).await {
                self.discard(&voice);
                return Err(error);
            }
            return Ok(voice);
        };
        let previous = samples.kept_clone(account).await?;
        let voice = self
            .cloner
            .create(&self.account_name(account), sample.clone())
            .await?;
        // In the room first: if the call ended or filled up meanwhile, the new clone goes and
        // the account keeps the voice it had.
        if let Err(error) = self.register(room.id, account, &voice).await {
            self.discard(&voice);
            return Err(error);
        }
        if let Err(error) = samples.save(account, &sample, clock.now_ms()).await {
            warn_not_kept(&error, account);
        }
        self.keep(account, &voice).await;
        if let Some(old) = previous.filter(|old| self.is_ours(old) && old.voice != voice) {
            self.discard(&old.voice);
        }
        Ok(voice)
    }

    /// The account's voice for this room: its kept clone when the provider holds one
    /// (instant), otherwise a clone made once from the kept recording and kept from then on.
    pub async fn clone_saved(
        &self,
        code: &RoomCode,
        account: &AccountId,
    ) -> Result<VoiceId, VoiceError> {
        let (this, code, account) = (self.clone(), code.clone(), *account);
        let turn = self.locks.lock(&account).await;
        to_completion(async move {
            let _turn = turn;
            this.reuse_saved(&code, &account).await
        })
        .await
    }

    async fn reuse_saved(
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
        // Checked before the paid provider call; registering checks again afterwards.
        room.ensure_open()?;
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
        // In the room first: a clone for a call that ended meanwhile is not kept.
        if let Err(error) = self.register(room.id, account, &voice).await {
            self.discard(&voice);
            return Err(error);
        }
        self.keep(account, &voice).await;
        Ok(voice)
    }

    /// Whether `account` has a kept recording.
    pub async fn has_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        match &self.samples {
            Some((samples, _)) => Ok(samples.has(account).await?),
            None => Ok(false),
        }
    }

    /// "Delete my voice": the account's clone at the provider, then the kept recording;
    /// `false` when there was no recording.
    pub async fn delete_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        let (this, account) = (self.clone(), *account);
        let turn = self.locks.lock(&account).await;
        to_completion(async move {
            let _turn = turn;
            this.forget_saved(&account).await
        })
        .await
    }

    async fn forget_saved(&self, account: &AccountId) -> Result<bool, VoiceError> {
        let Some((samples, _)) = &self.samples else {
            return Ok(false);
        };
        // The provider first: if it fails, nothing is deleted and the person can try again,
        // instead of losing the only reference to a clone that still exists.
        if let Some(kept) = samples
            .kept_clone(account)
            .await?
            .filter(|kept| self.is_ours(kept))
        {
            self.cloner.delete(&kept.voice).await?;
        }
        Ok(samples.delete(account).await?)
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
        room.ensure_open()?;
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

    /// Deletes a clone nobody will track (its call ended meanwhile, or a newer recording
    /// replaced it), trying a few times, on its own task: a request cut short by its time
    /// limit must not stop the cleanup. Once this gives up, only the logged id finds it.
    fn discard(&self, voice: &VoiceId) {
        let (cloner, voice) = (self.cloner.clone(), voice.clone());
        tokio::spawn(async move { discard_with_retries(cloner.as_ref(), &voice).await });
    }
}

fn warn_not_kept(error: &VoiceSampleStoreError, account: &AccountId) {
    tracing::warn!(%error, account_id = %account, "voice recording not kept");
}

mod lifecycle;

use lifecycle::{AccountLocks, discard_with_retries, to_completion};

#[cfg(test)]
mod failure_tests;
#[cfg(test)]
mod tests;

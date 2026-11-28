use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use diceri_application::ports::{
    SpeechToTextError, TextToSpeech, TranscriptEvent, Translator, VoiceId,
};
use diceri_domain::LanguageCode;
use tokio::sync::{mpsc, watch};

use crate::conversation::Conversation;
use crate::segmenter::{SegmentUpdate, Segmenter};
use crate::target_worker::{PhraseJob, TargetWorker, WorkerDeps};
use crate::voice_choice::VoiceChoice;
use crate::{Caption, CaptionKind, PipelineOutput};

/// Who is speaking and how their translations should sound.
#[derive(Debug, Clone)]
pub struct SpeakerConfig {
    pub speaker: String,
    /// Assumed source language when the recognizer reports none for a phrase.
    pub fallback_language: LanguageCode,
    /// The speaker's trained (consented) voice; without one, translations are captions only.
    pub voice: Option<VoiceId>,
}

/// Per-stream mutable state of a running pipeline.
#[derive(Default)]
struct StreamState {
    segmenter: Segmenter,
    /// When the recognizer last updated the phrase in progress.
    last_progress: Option<Instant>,
    workers: HashMap<LanguageCode, TargetWorker>,
    phrase: u64,
}

/// Translates one speaker into every other language present in the room.
pub struct SpeakerPipeline {
    deps: Arc<WorkerDeps>,
    /// The whole call's recent turns, shared with the other speakers' pipelines.
    conversation: Arc<Conversation>,
}

impl std::fmt::Debug for SpeakerPipeline {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SpeakerPipeline")
            .field("speaker", &self.deps.speaker)
            .finish_non_exhaustive()
    }
}

type Events = mpsc::Receiver<Result<TranscriptEvent, SpeechToTextError>>;

impl SpeakerPipeline {
    #[must_use]
    pub fn new(
        config: SpeakerConfig,
        translator: Arc<dyn Translator>,
        tts: Arc<dyn TextToSpeech>,
        output: Arc<dyn PipelineOutput>,
        conversation: Arc<Conversation>,
    ) -> Self {
        let deps = WorkerDeps {
            speaker: config.speaker,
            source: config.fallback_language,
            voice: VoiceChoice::new(config.voice),
            translator,
            tts,
            output,
        };
        Self {
            deps: Arc::new(deps),
            conversation,
        }
    }

    /// Runs until the transcript stream ends, then lets queued phrases finish.
    /// `targets` holds the languages listeners want and may change at any time. Phrases are
    /// numbered from `first_phrase`: a stream reopened for the same speaker must start above
    /// the last one, or its captions would replace earlier lines of the transcript.
    pub async fn run(
        self,
        mut events: Events,
        targets: watch::Receiver<Vec<LanguageCode>>,
        first_phrase: u64,
    ) {
        let mut state = StreamState {
            phrase: first_phrase,
            ..StreamState::default()
        };
        while let Some(event) = events.recv().await {
            match event {
                Ok(event) => self.on_event(&mut state, event, &targets).await,
                Err(error) => {
                    tracing::warn!(%error, speaker = %self.deps.speaker, "transcription error");
                    break;
                }
            }
        }
        for (_, worker) in state.workers.drain() {
            worker.finish().await;
        }
    }

    async fn on_event(
        &self,
        state: &mut StreamState,
        event: TranscriptEvent,
        targets: &watch::Receiver<Vec<LanguageCode>>,
    ) {
        match state.segmenter.push(event) {
            Some(SegmentUpdate::Progress(text)) => {
                state.last_progress = Some(Instant::now());
                self.caption(CaptionKind::Partial, state.phrase, text).await;
            }
            Some(SegmentUpdate::Phrase { text, language }) => {
                // Trust what was detected over what the participant declared.
                let source = language.unwrap_or_else(|| self.deps.source.clone());
                self.caption_in(CaptionKind::Original, state.phrase, text.clone(), &source)
                    .await;
                let targets = Self::targets(targets, &source);
                self.dispatch(state, &text, &source, targets);
                self.conversation.record(&self.deps.speaker, &text);
                state.phrase += 1;
            }
            None => {}
        }
    }

    /// Queues the phrase on each target's worker, dropping it if that queue is full.
    fn dispatch(
        &self,
        state: &mut StreamState,
        text: &str,
        source: &LanguageCode,
        targets: Vec<LanguageCode>,
    ) {
        let context = self.conversation.context_for(&self.deps.speaker);
        let ended_at = Instant::now();
        let stt_wait_ms = state
            .last_progress
            .take()
            .map_or(0, |at| ended_at.duration_since(at).as_millis());
        for target in targets {
            let worker = state
                .workers
                .entry(target.clone())
                .or_insert_with(|| TargetWorker::spawn(self.deps.clone(), target.clone()));
            let job = PhraseJob {
                phrase: state.phrase,
                text: text.to_owned(),
                source: source.clone(),
                context: context.clone(),
                ended_at,
                stt_wait_ms,
            };
            if worker.jobs.try_send(job).is_err() {
                tracing::warn!(%target, "dropping phrase: listener queue full");
            }
        }
    }

    /// Target languages other than the phrase's own (same base language is skipped).
    fn targets(
        targets: &watch::Receiver<Vec<LanguageCode>>,
        source: &LanguageCode,
    ) -> Vec<LanguageCode> {
        let source = source.primary();
        targets
            .borrow()
            .iter()
            .filter(|t| t.primary() != source)
            .cloned()
            .collect()
    }

    async fn caption(&self, kind: CaptionKind, phrase: u64, text: String) {
        let language = self.deps.source.clone();
        self.caption_in(kind, phrase, text, &language).await;
    }

    async fn caption_in(
        &self,
        kind: CaptionKind,
        phrase: u64,
        text: String,
        language: &LanguageCode,
    ) {
        let caption = Caption {
            speaker: self.deps.speaker.clone(),
            language: language.clone(),
            kind,
            phrase,
            text,
        };
        self.deps.output.caption(caption).await;
    }
}

#[cfg(test)]
#[path = "speaker_tests.rs"]
mod tests;

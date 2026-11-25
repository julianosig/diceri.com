use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use diceri_application::ports::{
    SpeechToTextError, TextToSpeech, TranscriptEvent, Translator, VoiceId,
};
use diceri_domain::LanguageCode;
use tokio::sync::{mpsc, watch};

use crate::context::RollingContext;
use crate::segmenter::{SegmentUpdate, Segmenter};
use crate::target_worker::{PhraseJob, TargetWorker, WorkerDeps};
use crate::{Caption, CaptionKind, PipelineOutput};

/// Phrases of context given to the translator.
const CONTEXT_PHRASES: usize = 3;

/// Who is speaking and how their translations should sound.
#[derive(Debug, Clone)]
pub struct SpeakerConfig {
    pub speaker: String,
    /// Assumed source language when the recognizer reports none for a phrase.
    pub fallback_language: LanguageCode,
    pub voice: VoiceId,
}

/// Per-stream mutable state of a running pipeline.
struct StreamState {
    segmenter: Segmenter,
    context: RollingContext,
    workers: HashMap<LanguageCode, TargetWorker>,
    phrase: u64,
}

impl Default for StreamState {
    fn default() -> Self {
        Self {
            segmenter: Segmenter::default(),
            context: RollingContext::new(CONTEXT_PHRASES),
            workers: HashMap::new(),
            phrase: 0,
        }
    }
}

/// Translates one speaker into every other language present in the room.
pub struct SpeakerPipeline {
    deps: Arc<WorkerDeps>,
}

impl std::fmt::Debug for SpeakerPipeline {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SpeakerPipeline")
            .field("speaker", &self.deps.speaker)
            .finish()
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
    ) -> Self {
        let deps = WorkerDeps {
            speaker: config.speaker,
            source: config.fallback_language,
            voice: config.voice,
            translator,
            tts,
            output,
        };
        Self {
            deps: Arc::new(deps),
        }
    }

    /// Runs until the transcript stream ends, then lets queued phrases finish.
    /// `targets` holds the languages listeners want and may change at any time.
    pub async fn run(self, mut events: Events, targets: watch::Receiver<Vec<LanguageCode>>) {
        let mut state = StreamState::default();
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
            drop(worker.jobs);
            let _ = worker.handle.await;
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
                self.caption(CaptionKind::Partial, state.phrase, text).await;
            }
            Some(SegmentUpdate::Phrase { text, language }) => {
                // Trust what was detected over what the participant declared.
                let source = language.unwrap_or_else(|| self.deps.source.clone());
                self.caption_in(CaptionKind::Original, state.phrase, text.clone(), &source)
                    .await;
                let targets = Self::targets(targets, &source);
                self.dispatch(state, &text, &source, targets);
                state.context.push(text);
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
        let context = state.context.snapshot();
        let ended_at = Instant::now();
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

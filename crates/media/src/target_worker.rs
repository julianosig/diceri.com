use std::sync::Arc;
use std::time::Instant;

use diceri_application::ports::{
    SpeechRequest, TextToSpeech, TranslationRequest, Translator, VoiceId,
};
use diceri_domain::LanguageCode;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::{Caption, CaptionKind, PipelineOutput};

/// Phrases waiting per target language; beyond this the speaker is far ahead
/// of playback and new phrases are dropped instead of piling up delay.
const QUEUE_DEPTH: usize = 4;

/// One phrase to translate and speak in a target language.
#[derive(Debug)]
pub(crate) struct PhraseJob {
    pub phrase: u64,
    pub text: String,
    pub context: Vec<String>,
    /// When the speaker finished the phrase; the origin of latency metrics.
    pub ended_at: Instant,
}

/// Everything a worker needs, shared by all of a speaker's workers.
pub(crate) struct WorkerDeps {
    pub speaker: String,
    pub source: LanguageCode,
    pub voice: VoiceId,
    pub translator: Arc<dyn Translator>,
    pub tts: Arc<dyn TextToSpeech>,
    pub output: Arc<dyn PipelineOutput>,
}

/// Translates and speaks phrases for one target language, strictly in order.
pub(crate) struct TargetWorker {
    pub jobs: mpsc::Sender<PhraseJob>,
    pub handle: JoinHandle<()>,
}

impl TargetWorker {
    pub(crate) fn spawn(deps: Arc<WorkerDeps>, target: LanguageCode) -> Self {
        let (jobs, rx) = mpsc::channel(QUEUE_DEPTH);
        let handle = tokio::spawn(run(deps, target, rx));
        Self { jobs, handle }
    }
}

async fn run(deps: Arc<WorkerDeps>, target: LanguageCode, mut jobs: mpsc::Receiver<PhraseJob>) {
    while let Some(job) = jobs.recv().await {
        process(&deps, &target, job).await;
    }
}

async fn process(deps: &WorkerDeps, target: &LanguageCode, job: PhraseJob) {
    let Some(translated) = translate(deps, target, &job).await else {
        return;
    };
    let translated_ms = job.ended_at.elapsed().as_millis();
    deps.output
        .caption(Caption {
            speaker: deps.speaker.clone(),
            language: target.clone(),
            kind: CaptionKind::Translation,
            phrase: job.phrase,
            text: translated.clone(),
        })
        .await;
    speak(deps, target, &job, &translated, translated_ms).await;
}

async fn translate(deps: &WorkerDeps, target: &LanguageCode, job: &PhraseJob) -> Option<String> {
    let request = TranslationRequest {
        text: &job.text,
        source: &deps.source,
        target,
        context: &job.context,
    };
    deps.translator
        .translate(request)
        .await
        .inspect_err(|error| tracing::warn!(%error, %target, "translation failed"))
        .ok()
}

async fn speak(
    deps: &WorkerDeps,
    target: &LanguageCode,
    job: &PhraseJob,
    text: &str,
    translated_ms: u128,
) {
    let request = SpeechRequest {
        text,
        language: target,
        voice: &deps.voice,
    };
    let Some(audio) = deps
        .tts
        .synthesize(request)
        .await
        .inspect_err(|error| tracing::warn!(%error, %target, "synthesis failed"))
        .ok()
    else {
        return;
    };
    log_latency(target, job, translated_ms);
    deps.output.speak(&deps.speaker, target, audio).await;
}

/// Milliseconds from the end of the phrase to translation and to first speech.
fn log_latency(target: &LanguageCode, job: &PhraseJob, translated_ms: u128) {
    let speech_ms = job.ended_at.elapsed().as_millis();
    tracing::info!(%target, phrase = job.phrase, translated_ms, speech_ms, "phrase latency");
}

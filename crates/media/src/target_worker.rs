use std::sync::Arc;
use std::time::Instant;

use diceri_application::ports::{
    ContextTurn, SpeechAudio, SpeechRequest, TextToSpeech, TextToSpeechError, TranslationRequest,
    Translator,
};
use diceri_domain::LanguageCode;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::clauses::Clauses;
use crate::voice_choice::VoiceChoice;
use crate::{Caption, CaptionKind, PipelineOutput};

/// Phrases waiting per target language; beyond this the speaker is far ahead
/// of playback and new phrases are dropped instead of piling up delay.
const QUEUE_DEPTH: usize = 4;
/// Translation pieces in flight between the translator and speech.
const PIECES: usize = 64;

/// One phrase to translate and speak in a target language.
#[derive(Debug)]
pub(crate) struct PhraseJob {
    pub phrase: u64,
    pub text: String,
    /// Detected language of this phrase (falls back to the declared one).
    pub source: LanguageCode,
    /// The recent conversation of the call, from everyone in it.
    pub context: Vec<ContextTurn>,
    /// When the speaker finished the phrase; the origin of latency metrics.
    pub ended_at: Instant,
    /// How long the recognizer took to close the phrase after its last partial text.
    pub stt_wait_ms: u128,
}

/// Everything a worker needs, shared by all of a speaker's workers.
pub(crate) struct WorkerDeps {
    pub speaker: String,
    pub source: LanguageCode,
    pub voice: VoiceChoice,
    pub translator: Arc<dyn Translator>,
    pub tts: Arc<dyn TextToSpeech>,
    pub output: Arc<dyn PipelineOutput>,
}

/// Translates and speaks phrases for one target language, strictly in order. Dropping it
/// stops the work at once (a pipeline cancelled because its microphone was replaced must not
/// keep speaking); finishing the queue is waiting on `handle` after closing `jobs`.
pub(crate) struct TargetWorker {
    pub jobs: mpsc::Sender<PhraseJob>,
    pub handle: JoinHandle<()>,
}

impl Drop for TargetWorker {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl TargetWorker {
    pub(crate) fn spawn(deps: Arc<WorkerDeps>, target: LanguageCode) -> Self {
        let (jobs, rx) = mpsc::channel(QUEUE_DEPTH);
        let handle = tokio::spawn(run(deps, target, rx));
        Self { jobs, handle }
    }

    /// Lets the phrases already queued finish, then ends.
    pub(crate) async fn finish(mut self) {
        // Closing the queue (its sender is swapped for one that leads nowhere) ends the loop.
        drop(std::mem::replace(&mut self.jobs, mpsc::channel(1).0));
        let _ = (&mut self.handle).await;
    }
}

async fn run(deps: Arc<WorkerDeps>, target: LanguageCode, mut jobs: mpsc::Receiver<PhraseJob>) {
    while let Some(job) = jobs.recv().await {
        process(&deps, &target, job).await;
    }
}

/// Milliseconds from the end of the phrase to each stage, for one target language.
#[derive(Debug, Default)]
struct Timing {
    first_piece_ms: Option<u128>,
    translated_ms: Option<u128>,
    first_audio_ms: Option<u128>,
    parts: usize,
}

/// Translates as a stream and speaks each part as soon as it is complete, so the
/// listener hears the first sentence while the rest is still being translated.
async fn process(deps: &WorkerDeps, target: &LanguageCode, job: PhraseJob) {
    let (pieces, received) = mpsc::channel(PIECES);
    let request = TranslationRequest {
        text: &job.text,
        source: &job.source,
        target,
        context: &job.context,
    };
    let translate = async {
        if let Err(error) = deps.translator.translate_stream(request, pieces).await {
            tracing::warn!(%error, %target, "translation failed");
        }
    };
    let ((), timing) = tokio::join!(translate, speak_as_translated(deps, target, &job, received));
    if timing.parts > 0 {
        log_latency(target, &job, &timing);
    }
}

async fn speak_as_translated(
    deps: &WorkerDeps,
    target: &LanguageCode,
    job: &PhraseJob,
    mut received: mpsc::Receiver<String>,
) -> Timing {
    let mut timing = Timing::default();
    let mut clauses = Clauses::default();
    let mut translated = String::new();
    while let Some(piece) = received.recv().await {
        timing
            .first_piece_ms
            .get_or_insert(job.ended_at.elapsed().as_millis());
        translated.push_str(&piece);
        for part in clauses.push(&piece) {
            caption(deps, target, job, &translated).await;
            speak(deps, target, job, &part, &mut timing).await;
        }
    }
    timing.translated_ms = Some(job.ended_at.elapsed().as_millis());
    if translated.trim().is_empty() {
        return timing;
    }
    caption(deps, target, job, &translated).await;
    let Some(rest) = clauses.finish() else {
        return timing;
    };
    // A misdetected short phrase ("Alô.") can come back unchanged; repeating the
    // speaker's own words adds nothing, so only the caption is kept.
    if timing.parts == 0 && same_words(&translated, &job.text) {
        return timing;
    }
    speak(deps, target, job, &rest, &mut timing).await;
    timing
}

/// The translation so far, as one caption line that grows until the phrase is done.
async fn caption(deps: &WorkerDeps, target: &LanguageCode, job: &PhraseJob, text: &str) {
    deps.output
        .caption(Caption {
            speaker: deps.speaker.clone(),
            language: target.clone(),
            kind: CaptionKind::Translation,
            phrase: job.phrase,
            text: text.trim().to_owned(),
        })
        .await;
}

/// Equal ignoring case, punctuation and spacing.
fn same_words(a: &str, b: &str) -> bool {
    let normalize = |text: &str| -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect()
    };
    normalize(a) == normalize(b)
}

async fn speak(
    deps: &WorkerDeps,
    target: &LanguageCode,
    job: &PhraseJob,
    text: &str,
    timing: &mut Timing,
) {
    let Some(audio) = synthesize(deps, target, text).await else {
        return;
    };
    timing
        .first_audio_ms
        .get_or_insert(job.ended_at.elapsed().as_millis());
    timing.parts += 1;
    deps.output.speak(&deps.speaker, target, audio).await;
}

/// Synthesizes in the speaker's own trained voice. Without one nothing is spoken (the
/// caption still shows); if the provider refuses the clone (e.g. it needs verification),
/// the speaker stays captions-only for the rest of the call.
async fn synthesize(deps: &WorkerDeps, target: &LanguageCode, text: &str) -> Option<SpeechAudio> {
    let voice = deps.voice.current()?;
    let request = SpeechRequest {
        text,
        language: target,
        voice,
    };
    match deps.tts.synthesize(request).await {
        Ok(audio) => Some(audio),
        // Busy or unreachable: this phrase stays a caption, the next one tries again.
        Err(error @ TextToSpeechError::Unavailable(_)) => {
            tracing::warn!(%error, %target, "speech synthesis unavailable, phrase captioned only");
            None
        }
        Err(error) => {
            if deps.voice.refuse() {
                warn_clone_refused(&error, target);
            }
            None
        }
    }
}

fn warn_clone_refused(error: &TextToSpeechError, target: &LanguageCode) {
    tracing::warn!(%error, %target, "voice clone refused, captions only for this speaker");
}

/// One line per phrase and language with every stage, measured from the end of the
/// phrase: recognizer wait, first and last translation piece, first audio, all audio.
fn log_latency(target: &LanguageCode, job: &PhraseJob, timing: &Timing) {
    tracing::info!(
        %target,
        phrase = job.phrase,
        stt_wait_ms = job.stt_wait_ms,
        first_piece_ms = timing.first_piece_ms,
        translated_ms = timing.translated_ms,
        first_audio_ms = timing.first_audio_ms,
        speech_ms = job.ended_at.elapsed().as_millis(),
        parts = timing.parts,
        "phrase latency"
    );
}

#[cfg(test)]
mod tests {
    use super::same_words;

    #[test]
    fn same_words_ignores_case_punctuation_and_spacing() {
        assert!(same_words("Alô.", "alô"));
        assert!(same_words("Tudo bem?", "tudo  bem"));
        assert!(!same_words("Hello.", "Olá."));
    }
}

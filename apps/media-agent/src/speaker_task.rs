use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};

use diceri_application::ports::{
    STT_SAMPLE_RATE, SpeechStream, SpeechToTextError, TranscriptEvent, VoiceId,
};
use diceri_domain::LanguageCode;
use diceri_media::{Conversation, PipelineOutput, SpeakerConfig, SpeakerPipeline};
use futures_util::StreamExt as _;
use livekit::track::RemoteAudioTrack;
use livekit::webrtc::audio_stream::native::NativeAudioStream;
use tokio::sync::{mpsc, watch};

use crate::providers::Providers;

/// Attempts at opening speech-to-text, and the pause between them (about a minute).
const OPEN_ATTEMPTS: u32 = 20;
const OPEN_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(3);
/// Pause before reopening a stream that closed mid-call.
const REOPEN_DELAY: std::time::Duration = std::time::Duration::from_secs(1);
/// A stream that lived this long was working; closing after it is a drop, not a fault.
const STABLE_STREAM: std::time::Duration = std::time::Duration::from_secs(30);
/// Consecutive streams closing before `STABLE_STREAM` after which the speaker is given up.
const MAX_QUICK_DROPS: u32 = 5;
/// Transcript events waiting for the pipeline.
const TRANSCRIPT_QUEUE: usize = 64;
/// Phrase numbers set aside per track (far more than one track ever says).
const PHRASES_PER_TRACK: u64 = 1_000_000;

/// Who is speaking on a subscribed microphone track. Their spoken language is
/// detected; `hints` are the languages present in the room when they joined.
pub(crate) struct Speaker {
    pub identity: String,
    pub hints: Vec<LanguageCode>,
    /// Assumed source when a phrase has no detected language: their output language.
    pub fallback_language: LanguageCode,
    /// The speaker's consented voice clone, if they recorded one.
    pub voice: Option<VoiceId>,
    pub track: RemoteAudioTrack,
}

/// Tries to open speech-to-text for this speaker: providers cap concurrent streams, so a
/// refusal is retried for about a minute (a seat frees when another call ends) before the
/// speaker is given up on.
async fn open_speech_to_text(providers: &Providers, speaker: &Speaker) -> Option<SpeechStream> {
    for attempt in 1..=OPEN_ATTEMPTS {
        match providers.stt.open(&speaker.hints).await {
            Ok(stream) => return Some(stream),
            Err(error) if attempt < OPEN_ATTEMPTS => {
                warn_retrying(&error, &speaker.identity, attempt);
                tokio::time::sleep(OPEN_RETRY_DELAY).await;
            }
            Err(error) => warn_given_up(&error, &speaker.identity),
        }
    }
    None
}

fn warn_retrying(error: &SpeechToTextError, speaker: &str, attempt: u32) {
    tracing::warn!(%error, speaker, attempt, "speech-to-text refused, retrying");
}

fn warn_given_up(error: &SpeechToTextError, speaker: &str) {
    tracing::error!(%error, speaker, "could not open speech-to-text");
}

/// Transcribes one speaker and runs their translation pipeline until the track ends. One
/// pipeline lives as long as the track; a speech-to-text stream that drops mid-call (network,
/// provider session limit) is reopened and feeds the same pipeline, so the speaker is never
/// silently left untranslated, reopening never waits for queued speech, and phrase numbers
/// keep counting up.
pub(crate) async fn run(
    speaker: Speaker,
    providers: Arc<Providers>,
    output: Arc<dyn PipelineOutput>,
    conversation: Arc<Conversation>,
    targets: watch::Receiver<Vec<LanguageCode>>,
) {
    let config = SpeakerConfig {
        voice: speaker.voice.clone(),
        speaker: speaker.identity.clone(),
        fallback_language: speaker.fallback_language.clone(),
    };
    log_start(&config);
    let (events, transcript) = mpsc::channel(TRANSCRIPT_QUEUE);
    let pipeline = SpeakerPipeline::new(
        config,
        providers.translator.clone(),
        providers.tts.clone(),
        output,
        conversation,
    );
    // Stopped with this task: a replaced or departed microphone's queued speech never plays
    // over the new one's on the shared track.
    let mut translating = AbortOnDrop(tokio::spawn(pipeline.run(
        transcript,
        targets,
        first_phrase(),
    )));
    transcribe(&speaker, &providers, &events).await;
    // The track ended on its own: the pipeline finishes the phrases already queued.
    drop(events);
    let _ = (&mut translating.0).await;
}

/// Feeds `events` from speech-to-text until the track ends, reopening a stream that closes.
async fn transcribe(speaker: &Speaker, providers: &Providers, events: &Transcript) {
    let track_ended = Arc::new(AtomicBool::new(false));
    let mut quick_drops = 0;
    while let Some(stream) = open_speech_to_text(providers, speaker).await {
        let started = tokio::time::Instant::now();
        // Stopped as soon as this stream closes (or the whole speaker does): no audio is
        // sent to a failed stream while reconnecting.
        let pump = AbortOnDrop(pump_audio(
            &speaker.track,
            stream.audio,
            track_ended.clone(),
        ));
        let pipeline_open = forward(stream.events, events).await;
        drop(pump);
        if !pipeline_open {
            return;
        }
        // The stream closed: what it heard is one phrase, never spliced to the next stream's.
        if events.send(Ok(TranscriptEvent::Endpoint)).await.is_err()
            || track_ended.load(Ordering::Relaxed)
        {
            return;
        }
        // A stream that keeps closing at once is broken, not dropped: stop retrying.
        quick_drops = if started.elapsed() < STABLE_STREAM {
            quick_drops + 1
        } else {
            0
        };
        if quick_drops >= MAX_QUICK_DROPS {
            warn_given_up_reopening(&speaker.identity);
            return;
        }
        warn_reopening(&speaker.identity);
        tokio::time::sleep(REOPEN_DELAY).await;
    }
}

type Transcript = mpsc::Sender<Result<TranscriptEvent, SpeechToTextError>>;

/// Passes one stream's events on until it closes or fails; `false` once the pipeline is gone.
async fn forward(
    mut stream: mpsc::Receiver<Result<TranscriptEvent, SpeechToTextError>>,
    events: &Transcript,
) -> bool {
    while let Some(event) = stream.recv().await {
        match event {
            Ok(event) => {
                if events.send(Ok(event)).await.is_err() {
                    return false;
                }
            }
            Err(error) => {
                tracing::warn!(%error, "transcription error");
                return true;
            }
        }
    }
    true
}

/// Forwards 16 kHz mono frames from the track to speech-to-text, until either ends;
/// `track_ended` tells the two apart.
fn pump_audio(
    track: &RemoteAudioTrack,
    audio: mpsc::Sender<Vec<i16>>,
    track_ended: Arc<AtomicBool>,
) -> tokio::task::JoinHandle<()> {
    let sample_rate = i32::try_from(STT_SAMPLE_RATE).unwrap_or(16_000);
    let mut frames = NativeAudioStream::new(track.rtc_track(), sample_rate, 1);
    tokio::spawn(async move {
        loop {
            // Ends with its stream too, even while the microphone sends nothing.
            let frame = tokio::select! {
                frame = frames.next() => frame,
                () = audio.closed() => break,
            };
            let Some(frame) = frame else {
                track_ended.store(true, Ordering::Relaxed);
                break;
            };
            // A late frame is worthless: drop it instead of building up delay.
            if audio.try_send(frame.data.into_owned()).is_err() && audio.is_closed() {
                break;
            }
        }
    })
}

/// Aborts a task when dropped, so it never outlives what started it.
struct AbortOnDrop(tokio::task::JoinHandle<()>);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// Phrases of one track are numbered from here, counting up by one. Seeded from the current
/// time in milliseconds and never below an earlier track's range in this process, so a
/// speaker who rejoins (same identity) never reuses a number, even if the clock steps back.
fn first_phrase() -> u64 {
    static NEXT: Mutex<u64> = Mutex::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| {
            u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
        });
    let mut next = NEXT.lock().unwrap_or_else(PoisonError::into_inner);
    let first = now.max(*next);
    *next = first.saturating_add(PHRASES_PER_TRACK);
    first
}

fn warn_reopening(speaker: &str) {
    tracing::warn!(speaker, "speech-to-text closed mid-call, reopening");
}

fn warn_given_up_reopening(speaker: &str) {
    tracing::error!(
        speaker,
        "speech-to-text keeps closing, speaker left untranslated"
    );
}

fn log_start(config: &SpeakerConfig) {
    tracing::info!(speaker = %config.speaker, "translating speaker");
}

#[cfg(test)]
mod tests {
    use super::first_phrase;

    #[test]
    fn each_track_numbers_its_phrases_above_every_earlier_one() {
        let first = first_phrase();
        let second = first_phrase();
        assert!(second >= first + super::PHRASES_PER_TRACK);
    }
}

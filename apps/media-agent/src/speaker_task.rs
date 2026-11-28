use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use diceri_application::ports::{STT_SAMPLE_RATE, SpeechStream, SpeechToTextError, VoiceId};
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

/// Transcribes one speaker and runs their translation pipeline until the track ends. A
/// speech-to-text stream that drops mid-call (network, provider session limit) is reopened
/// while the microphone is still live, so the speaker is never silently left untranslated.
pub(crate) async fn run(
    speaker: Speaker,
    providers: Arc<Providers>,
    output: Arc<dyn PipelineOutput>,
    conversation: Arc<Conversation>,
    targets: watch::Receiver<Vec<LanguageCode>>,
) {
    let track_ended = Arc::new(AtomicBool::new(false));
    let config = SpeakerConfig {
        voice: speaker.voice.clone(),
        speaker: speaker.identity.clone(),
        fallback_language: speaker.fallback_language.clone(),
    };
    log_start(&config);
    let mut quick_drops = 0;
    while let Some(stream) = open_speech_to_text(&providers, &speaker).await {
        let started = tokio::time::Instant::now();
        pump_audio(&speaker.track, stream.audio, track_ended.clone());
        SpeakerPipeline::new(
            config.clone(),
            providers.translator.clone(),
            providers.tts.clone(),
            output.clone(),
            conversation.clone(),
        )
        .run(stream.events, targets.clone())
        .await;
        if track_ended.load(Ordering::Relaxed) {
            return;
        }
        // A stream that keeps closing at once is broken, not dropped: stop retrying.
        quick_drops = if started.elapsed() < STABLE_STREAM {
            quick_drops + 1
        } else {
            0
        };
        if quick_drops >= MAX_QUICK_DROPS {
            warn_given_up_reopening(&config.speaker);
            return;
        }
        warn_reopening(&config.speaker);
        tokio::time::sleep(REOPEN_DELAY).await;
    }
}

/// Forwards 16 kHz mono frames from the track to speech-to-text, until either ends;
/// `track_ended` tells the two apart.
fn pump_audio(
    track: &RemoteAudioTrack,
    audio: mpsc::Sender<Vec<i16>>,
    track_ended: Arc<AtomicBool>,
) {
    let sample_rate = i32::try_from(STT_SAMPLE_RATE).unwrap_or(16_000);
    let mut frames = NativeAudioStream::new(track.rtc_track(), sample_rate, 1);
    tokio::spawn(async move {
        loop {
            let Some(frame) = frames.next().await else {
                track_ended.store(true, Ordering::Relaxed);
                break;
            };
            // A late frame is worthless: drop it instead of building up delay.
            if audio.try_send(frame.data.into_owned()).is_err() && audio.is_closed() {
                break;
            }
        }
    });
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

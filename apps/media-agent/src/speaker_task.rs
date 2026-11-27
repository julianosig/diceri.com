use std::sync::Arc;

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

/// Transcribes one speaker and runs their translation pipeline until the track ends.
pub(crate) async fn run(
    speaker: Speaker,
    providers: Arc<Providers>,
    output: Arc<dyn PipelineOutput>,
    conversation: Arc<Conversation>,
    targets: watch::Receiver<Vec<LanguageCode>>,
) {
    let Some(stream) = open_speech_to_text(&providers, &speaker).await else {
        return;
    };
    pump_audio(&speaker.track, stream.audio);
    let config = SpeakerConfig {
        voice: speaker.voice,
        speaker: speaker.identity,
        fallback_language: speaker.fallback_language,
    };
    log_start(&config);
    SpeakerPipeline::new(
        config,
        providers.translator.clone(),
        providers.tts.clone(),
        output,
        conversation,
    )
    .run(stream.events, targets)
    .await;
}

/// Forwards 16 kHz mono frames from the track to speech-to-text.
fn pump_audio(track: &RemoteAudioTrack, audio: mpsc::Sender<Vec<i16>>) {
    let sample_rate = i32::try_from(STT_SAMPLE_RATE).unwrap_or(16_000);
    let mut frames = NativeAudioStream::new(track.rtc_track(), sample_rate, 1);
    tokio::spawn(async move {
        while let Some(frame) = frames.next().await {
            // A late frame is worthless: drop it instead of building up delay.
            if audio.try_send(frame.data.into_owned()).is_err() && audio.is_closed() {
                break;
            }
        }
    });
}

fn log_start(config: &SpeakerConfig) {
    tracing::info!(speaker = %config.speaker, "translating speaker");
}

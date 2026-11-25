use std::sync::Arc;

use diceri_application::ports::STT_SAMPLE_RATE;
use diceri_domain::LanguageCode;
use diceri_media::{PipelineOutput, SpeakerConfig, SpeakerPipeline};
use futures_util::StreamExt as _;
use livekit::track::RemoteAudioTrack;
use livekit::webrtc::audio_stream::native::NativeAudioStream;
use tokio::sync::{mpsc, watch};

use crate::providers::Providers;
use crate::voices::voice_for;

/// Who is speaking on a subscribed microphone track.
pub(crate) struct Speaker {
    pub identity: String,
    pub language: LanguageCode,
    pub track: RemoteAudioTrack,
}

/// Transcribes one speaker and runs their translation pipeline until the track ends.
pub(crate) async fn run(
    speaker: Speaker,
    providers: Arc<Providers>,
    output: Arc<dyn PipelineOutput>,
    targets: watch::Receiver<Vec<LanguageCode>>,
) {
    let stream = match providers.stt.open(&speaker.language).await {
        Ok(stream) => stream,
        Err(error) => {
            tracing::error!(%error, speaker = %speaker.identity, "could not open speech-to-text");
            return;
        }
    };
    pump_audio(&speaker.track, stream.audio);
    let config = SpeakerConfig {
        voice: voice_for(&speaker.identity, &providers.voices),
        speaker: speaker.identity,
        language: speaker.language,
    };
    log_start(&config);
    SpeakerPipeline::new(
        config,
        providers.translator.clone(),
        providers.tts.clone(),
        output,
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
    tracing::info!(speaker = %config.speaker, language = %config.language, "translating speaker");
}

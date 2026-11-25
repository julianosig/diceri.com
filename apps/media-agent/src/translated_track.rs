use diceri_application::ports::{SpeechAudio, TTS_SAMPLE_RATE};
use livekit::options::TrackPublishOptions;
use livekit::participant::LocalParticipant;
use livekit::track::{LocalAudioTrack, LocalTrack, TrackSource};
use livekit::webrtc::audio_frame::AudioFrame;
use livekit::webrtc::audio_source::native::NativeAudioSource;
use livekit::webrtc::audio_source::{AudioSourceOptions, RtcAudioSource};

/// 10 ms of 24 kHz mono audio: the frame size WebRTC expects.
const FRAME_SAMPLES: usize = 240;
/// Audio buffered inside the source; `capture_frame` waits when it is full,
/// which paces playback in real time.
const SOURCE_QUEUE_MS: u32 = 200;

/// A published audio track carrying one speaker translated into one language.
pub(crate) struct TranslatedTrack {
    source: NativeAudioSource,
    _track: LocalAudioTrack,
}

impl TranslatedTrack {
    pub(crate) async fn publish(
        participant: &LocalParticipant,
        name: &str,
    ) -> anyhow::Result<Self> {
        let source = NativeAudioSource::new(
            AudioSourceOptions::default(),
            TTS_SAMPLE_RATE,
            1,
            SOURCE_QUEUE_MS,
        );
        let track =
            LocalAudioTrack::create_audio_track(name, RtcAudioSource::Native(source.clone()));
        let options = TrackPublishOptions {
            source: TrackSource::Unknown,
            dtx: false,
            ..Default::default()
        };
        participant
            .publish_track(LocalTrack::Audio(track.clone()), options)
            .await?;
        Ok(Self {
            source,
            _track: track,
        })
    }

    /// Streams synthesized speech into the track, returning when it has all been queued.
    pub(crate) async fn play(&self, mut audio: SpeechAudio) {
        let mut pending: Vec<i16> = Vec::with_capacity(FRAME_SAMPLES * 4);
        while let Some(chunk) = audio.recv().await {
            match chunk {
                Ok(samples) => pending.extend(samples),
                Err(error) => {
                    tracing::warn!(%error, "speech stream failed mid-phrase");
                    break;
                }
            }
            let whole = pending.len() - pending.len() % FRAME_SAMPLES;
            let ready: Vec<i16> = pending.drain(..whole).collect();
            self.capture(&ready).await;
        }
        if !pending.is_empty() {
            pending.resize(pending.len().next_multiple_of(FRAME_SAMPLES), 0);
            self.capture(&pending).await;
        }
    }

    async fn capture(&self, samples: &[i16]) {
        let (frames, _) = samples.as_chunks::<FRAME_SAMPLES>();
        for frame in frames {
            let frame = AudioFrame {
                data: frame.as_slice().into(),
                sample_rate: TTS_SAMPLE_RATE,
                num_channels: 1,
                samples_per_channel: u32::try_from(FRAME_SAMPLES).unwrap_or(240),
            };
            if let Err(error) = self.source.capture_frame(&frame).await {
                tracing::warn!(%error, "failed to capture translated audio frame");
                return;
            }
        }
    }
}

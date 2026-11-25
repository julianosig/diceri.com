use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context as _;
use diceri_application::ports::{MediaAccessIssuer as _, MediaParticipant, MediaRole};
use diceri_domain::{LanguageCode, RoomId};
use diceri_infra::LANGUAGE_ATTRIBUTE;
use diceri_media::PipelineOutput;
use livekit::prelude::{
    RemoteParticipant, RemoteTrack, RemoteTrackPublication, Room, RoomEvent, RoomOptions,
};
use livekit::track::TrackSource;
use tokio::sync::{mpsc, watch};

use crate::output::LiveKitOutput;
use crate::providers::Providers;
use crate::speaker_task::{self, Speaker};
use crate::track_names::AGENT_IDENTITY;

/// Leave a room after it has had no human participants for this long.
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const IDLE_CHECK: Duration = Duration::from_secs(5);

/// Joins one room and translates everyone in it until the room empties.
pub(crate) async fn run(room_id: RoomId, providers: Arc<Providers>) -> anyhow::Result<()> {
    let participant = MediaParticipant {
        identity: AGENT_IDENTITY,
        profile: None,
        role: MediaRole::Translator,
    };
    let grant = providers
        .issuer
        .issue(&room_id, participant)
        .context("agent token")?;
    let (room, events) =
        Room::connect(&providers.livekit_url, &grant.token, RoomOptions::default())
            .await
            .context("connect to room")?;
    tracing::info!(%room_id, "agent joined room");
    serve(&room, events, &providers).await;
    room.close().await.context("leave room")
}

/// Reacts to room events until disconnected or idle for [`IDLE_TIMEOUT`].
async fn serve(
    room: &Room,
    mut events: mpsc::UnboundedReceiver<RoomEvent>,
    providers: &Arc<Providers>,
) {
    let output: Arc<dyn PipelineOutput> = Arc::new(LiveKitOutput::new(room.local_participant()));
    let (targets, _) = watch::channel(listener_languages(room));
    let mut last_human = Instant::now();
    let mut idle_check = tokio::time::interval(IDLE_CHECK);
    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    None | Some(RoomEvent::Disconnected { .. }) => return,
                    Some(event) => handle(event, providers, &output, &targets),
                }
                targets.send_replace(listener_languages(room));
            }
            _ = idle_check.tick() => {
                if !listener_languages(room).is_empty() {
                    last_human = Instant::now();
                } else if last_human.elapsed() >= IDLE_TIMEOUT {
                    return;
                }
            }
        }
    }
}

fn handle(
    event: RoomEvent,
    providers: &Arc<Providers>,
    output: &Arc<dyn PipelineOutput>,
    targets: &watch::Sender<Vec<LanguageCode>>,
) {
    if let RoomEvent::TrackSubscribed {
        track: RemoteTrack::Audio(track),
        publication,
        participant,
    } = event
        && let Some(speaker) = speaker(track, &publication, &participant)
    {
        tokio::spawn(speaker_task::run(
            speaker,
            providers.clone(),
            output.clone(),
            targets.subscribe(),
        ));
    }
}

/// A human's microphone track with a declared language; anything else is ignored.
fn speaker(
    track: livekit::track::RemoteAudioTrack,
    publication: &RemoteTrackPublication,
    participant: &RemoteParticipant,
) -> Option<Speaker> {
    if publication.source() != TrackSource::Microphone {
        return None;
    }
    let language = participant
        .attributes()
        .get(LANGUAGE_ATTRIBUTE)?
        .parse()
        .ok()?;
    Some(Speaker {
        identity: participant.identity().to_string(),
        language,
        track,
    })
}

/// Distinct languages of the humans in the room: the languages to translate into.
fn listener_languages(room: &Room) -> Vec<LanguageCode> {
    let mut languages: Vec<LanguageCode> = room
        .remote_participants()
        .values()
        .filter_map(|p| p.attributes().get(LANGUAGE_ATTRIBUTE)?.parse().ok())
        .collect();
    languages.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    languages.dedup();
    languages
}

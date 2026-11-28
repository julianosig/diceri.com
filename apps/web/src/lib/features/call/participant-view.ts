import {
	LocalVideoTrack,
	RemoteVideoTrack,
	Track,
	type ConnectionQuality,
	type Participant as RoomParticipant,
	type Room,
	type VideoTrack
} from 'livekit-client';
import { AGENT_IDENTITY, OUTPUT_LANGUAGE_ATTRIBUTE } from '#lib/features/translation/index.ts';

export interface Participant {
	identity: string;
	name: string;
	speaking: boolean;
	muted: boolean;
	quality: ConnectionQuality;
	isLocal: boolean;
	/** Language the participant hears everyone in (BCP 47). */
	outputLanguage: string | undefined;
	/** Their camera, while it is on (and, for others, received). */
	video: VideoTrack | undefined;
}

/** How this caller appears to themselves: muted as they chose, hearing their language. */
export interface LocalView {
	muted: boolean;
	outputLanguage: string;
}

/** The people in `room` as the call screen shows them (the translator is not one). */
export function participantsOf(room: Room, local: LocalView): Participant[] {
	const me = room.localParticipant;
	const humans = [...room.remoteParticipants.values()].filter(
		(participant) => participant.identity !== AGENT_IDENTITY
	);
	return [me, ...humans].map((participant) => ({
		identity: participant.identity,
		name: participant.name || 'Guest',
		speaking: participant.isSpeaking,
		muted: participant === me ? local.muted : !participant.isMicrophoneEnabled,
		quality: participant.connectionQuality,
		isLocal: participant === me,
		outputLanguage:
			participant === me ? local.outputLanguage : participant.attributes[OUTPUT_LANGUAGE_ATTRIBUTE],
		video: cameraOf(participant)
	}));
}

/** A participant's camera track while it is on and (for others) received. */
export function cameraOf(participant: RoomParticipant): VideoTrack | undefined {
	const publication = participant.getTrackPublication(Track.Source.Camera);
	const track = publication?.track;
	const video = track instanceof LocalVideoTrack || track instanceof RemoteVideoTrack;
	return video && !publication?.isMuted ? track : undefined;
}

import {
	LocalVideoTrack,
	RemoteVideoTrack,
	Track,
	type Participant,
	type VideoTrack
} from 'livekit-client';

/** A participant's camera track while it is on and (for others) received. */
export function cameraOf(participant: Participant): VideoTrack | undefined {
	const publication = participant.getTrackPublication(Track.Source.Camera);
	const track = publication?.track;
	const video = track instanceof LocalVideoTrack || track instanceof RemoteVideoTrack;
	return video && !publication?.isMuted ? track : undefined;
}

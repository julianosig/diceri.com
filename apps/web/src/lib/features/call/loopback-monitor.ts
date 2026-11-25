import { Room, RoomEvent, Track, type RemoteTrack } from 'livekit-client';
import type { MediaCredentials } from '#lib/features/room/index.ts';

/**
 * Hears your own microphone after a full round trip through the media server,
 * using a hidden subscribe-only participant. This validates real transmission:
 * codec, network, jitter buffer and server routing.
 */
export class LoopbackMonitor {
	#room = new Room({ adaptiveStream: false, dynacast: false });
	#elements: HTMLMediaElement[] = [];

	constructor(private readonly speakerIdentity: string) {}

	async start(credentials: MediaCredentials): Promise<void> {
		this.#room.on(RoomEvent.TrackSubscribed, (track: RemoteTrack, _publication, participant) => {
			if (participant.identity !== this.speakerIdentity || track.kind !== Track.Kind.Audio) return;
			const element = track.attach();
			element.hidden = true;
			document.body.append(element);
			this.#elements.push(element);
		});
		await this.#room.connect(credentials.serverUrl, credentials.token, { autoSubscribe: true });
		await this.#room.startAudio();
	}

	async stop(): Promise<void> {
		for (const element of this.#elements) element.remove();
		this.#elements = [];
		await this.#room.disconnect();
	}
}

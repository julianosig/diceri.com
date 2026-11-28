import { DisconnectReason, Room, RoomEvent, Track, type RemoteTrack } from 'livekit-client';
import type { MediaCredentials } from '#lib/features/room/index.ts';

/**
 * Hears your own microphone after a full round trip through the media server,
 * using a hidden subscribe-only participant. This validates real transmission:
 * codec, network, jitter buffer and server routing.
 */
export class LoopbackMonitor {
	#room = new Room({ adaptiveStream: false, dynacast: false });
	#elements: HTMLMediaElement[] = [];

	constructor(
		private readonly speakerIdentity: string,
		/** The server ended it (for example, removed as its person was not seen in the call). */
		private readonly onEnded: () => void = () => undefined
	) {}

	async start(credentials: MediaCredentials): Promise<void> {
		this.#room.on(RoomEvent.TrackSubscribed, (track: RemoteTrack, _publication, participant) => {
			if (participant.identity !== this.speakerIdentity || track.kind !== Track.Kind.Audio) return;
			const element = track.attach();
			element.hidden = true;
			document.body.append(element);
			this.#elements.push(element);
		});
		this.#room.on(RoomEvent.Disconnected, (reason?: DisconnectReason) => {
			// However it ends, its hidden players go with it.
			this.#detach();
			if (reason !== DisconnectReason.CLIENT_INITIATED) this.onEnded();
		});
		await this.#room.connect(credentials.serverUrl, credentials.token, { autoSubscribe: true });
		await this.#room.startAudio();
	}

	async stop(): Promise<void> {
		this.#detach();
		await this.#room.disconnect();
	}

	#detach(): void {
		for (const element of this.#elements) element.remove();
		this.#elements = [];
	}
}

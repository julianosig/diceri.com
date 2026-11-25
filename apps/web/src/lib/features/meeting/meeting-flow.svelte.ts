import { CallSession } from '#lib/features/call/index.ts';
import {
	GuestKnock,
	HostRequests,
	hostSession,
	monitorSession,
	saveDisplayName,
	saveLanguage,
	type Profile
} from '#lib/features/lobby/index.ts';
import type { MediaCredentials } from '#lib/features/room/index.ts';

export type MeetingPhase = 'prejoin' | 'joining' | 'waiting' | 'call' | 'left';

/**
 * Drives one visit to a meeting: pre-join, then either straight into the call
 * (host) or knocking until admitted (guest), then the call, then leaving.
 */
export class MeetingFlow {
	phase = $state<MeetingPhase>('prejoin');
	error = $state<string | undefined>();
	session = $state<CallSession | undefined>();
	readonly knock: GuestKnock;
	readonly hostRequests: HostRequests | undefined;
	#deviceId: string | undefined;
	#language = 'en';

	constructor(
		private readonly roomId: string,
		private readonly hostKey: string | undefined
	) {
		this.knock = new GuestKnock(roomId, (credentials, request) => {
			void this.#startCall(credentials, request.requestKey);
		});
		this.hostRequests = hostKey ? new HostRequests(roomId, hostKey) : undefined;
	}

	get isHost(): boolean {
		return this.hostKey !== undefined;
	}

	async join(profile: Profile, deviceId: string | undefined): Promise<void> {
		saveDisplayName(profile.name);
		saveLanguage(profile.language);
		this.#deviceId = deviceId;
		this.#language = profile.language;
		this.error = undefined;
		if (!this.hostKey) {
			this.phase = 'waiting';
			await this.knock.ask(profile);
			return;
		}
		this.phase = 'joining';
		try {
			const credentials = await hostSession(this.roomId, this.hostKey, profile);
			await this.#startCall(credentials, this.hostKey);
		} catch (cause) {
			this.phase = 'prejoin';
			this.error = cause instanceof Error ? cause.message : String(cause);
		}
	}

	cancelKnock(): void {
		this.knock.cancel();
		this.phase = 'prejoin';
	}

	async leave(): Promise<void> {
		this.hostRequests?.stop();
		await this.session?.disconnect();
		this.session = undefined;
		this.phase = 'left';
	}

	async #startCall(credentials: MediaCredentials, accessKey: string): Promise<void> {
		const roomId = this.roomId;
		this.session = new CallSession({
			credentials,
			language: this.#language,
			deviceId: this.#deviceId,
			monitorCredentials: () => monitorSession(roomId, accessKey)
		});
		this.phase = 'call';
		this.hostRequests?.start();
		await this.session.connect();
	}
}

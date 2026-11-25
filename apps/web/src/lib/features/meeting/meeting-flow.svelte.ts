import { CallSession } from '#lib/features/call/index.ts';
import type { TranscriptLog } from '#lib/features/translation/index.ts';
import {
	endCall,
	GuestKnock,
	HostRequests,
	hostSession,
	monitorSession,
	saveDisplayName,
	saveLanguage,
	type Profile
} from '#lib/features/lobby/index.ts';
import type { MediaCredentials } from '#lib/features/room/index.ts';

export type MeetingPhase = 'prejoin' | 'joining' | 'waiting' | 'call' | 'left' | 'ended';

/**
 * Drives one visit to a meeting: pre-join, then either straight into the call
 * (host) or knocking until admitted (guest), then the call, then leaving.
 */
export class MeetingFlow {
	phase = $state<MeetingPhase>('prejoin');
	error = $state<string | undefined>();
	session = $state<CallSession | undefined>();
	/** Transcript of the last call, kept after leaving so it can be read and downloaded. */
	transcript = $state<TranscriptLog | undefined>();
	/** Who ended the call, for the end screen. */
	endedBy = $state<'host' | 'me' | undefined>();
	readonly knock: GuestKnock;
	readonly hostRequests: HostRequests | undefined;
	#deviceId: string | undefined;
	#outputLanguage = 'en';

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
		saveLanguage(profile.outputLanguage);
		this.#deviceId = deviceId;
		this.#outputLanguage = profile.outputLanguage;
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
		await this.#close('left');
	}

	/** Host only: ends the meeting for everyone, then shows the transcript. */
	async endForEveryone(): Promise<void> {
		if (!this.hostKey) return;
		this.endedBy = 'me';
		await endCall(this.roomId, this.hostKey);
		await this.#close('ended');
	}

	async #close(phase: 'left' | 'ended'): Promise<void> {
		this.hostRequests?.stop();
		this.transcript = this.session?.transcript ?? this.transcript;
		await this.session?.disconnect();
		this.session = undefined;
		this.phase = phase;
	}

	async #startCall(credentials: MediaCredentials, accessKey: string): Promise<void> {
		const roomId = this.roomId;
		this.session = new CallSession({
			credentials,
			outputLanguage: this.#outputLanguage,
			deviceId: this.#deviceId,
			monitorCredentials: () => monitorSession(roomId, accessKey),
			onEndedByHost: () => {
				this.endedBy ??= 'host';
				void this.#close('ended');
			}
		});
		this.phase = 'call';
		this.hostRequests?.start();
		await this.session.connect();
	}
}

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
	type JoinDevices,
	type Profile
} from '#lib/features/lobby/index.ts';
import { limitText } from '#lib/features/plans/index.ts';
import type { MediaCredentials } from '#lib/features/room/index.ts';
import { i18n } from '#lib/i18n/index.ts';
import { forgetInCall, markInCall, wasInCall } from './rejoin.ts';

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
	/** Who ended the call, for the end screen; `limit` when the plan's time ran out. */
	endedBy = $state<'host' | 'me' | 'limit' | undefined>();
	/** The call dropped on its own (network): the end screen offers to rejoin. */
	lostConnection = $state(false);
	/** When the owner's plan ends this call (ms since epoch), once it has started. */
	deadline = $state<number | undefined>();
	readonly knock: GuestKnock;
	readonly hostRequests: HostRequests | undefined;
	#devices: JoinDevices = {};
	#outputLanguage = 'en';

	constructor(
		private readonly roomId: string,
		private readonly hostKey: string | undefined,
		/** Minutes the owner's plan leaves for this call, if the API said. */
		private readonly minutesLeft?: number
	) {
		this.knock = new GuestKnock(
			roomId,
			(credentials, request) => {
				void this.#startCall(credentials, request.requestKey);
			},
			(cause) => limitText(cause, i18n.t)
		);
		this.hostRequests = hostKey ? new HostRequests(roomId, hostKey) : undefined;
	}

	/** This tab was in this call before a reload: pre-join joins again by itself. */
	get resuming(): boolean {
		return this.phase === 'prejoin' && wasInCall(this.roomId);
	}

	get isHost(): boolean {
		return this.hostKey !== undefined;
	}

	async join(profile: Profile, devices: JoinDevices): Promise<void> {
		saveDisplayName(profile.name);
		saveLanguage(profile.outputLanguage);
		this.#devices = devices;
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
			this.error =
				limitText(cause, i18n.t) ?? (cause instanceof Error ? cause.message : String(cause));
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
		forgetInCall();
	}

	/** The API meters once a minute, so a call ends within about a minute of the deadline. */
	#reachedDeadline(): boolean {
		return this.deadline !== undefined && Date.now() >= this.deadline - 90_000;
	}

	async #startCall(credentials: MediaCredentials, accessKey: string): Promise<void> {
		const roomId = this.roomId;
		this.session = new CallSession({
			credentials,
			outputLanguage: this.#outputLanguage,
			deviceId: this.#devices.microphone,
			camera: this.#devices.camera,
			monitorCredentials: () => monitorSession(roomId, accessKey),
			onEndedByHost: () => {
				this.endedBy ??= this.#reachedDeadline() ? 'limit' : 'host';
				void this.#close('ended');
			},
			onDisconnected: () => {
				this.lostConnection = true;
				void this.#close('left');
			}
		});
		this.lostConnection = false;
		this.phase = 'call';
		markInCall(this.roomId);
		// Minutes count only while someone is in the call, so the clock starts now.
		if (this.minutesLeft !== undefined) {
			this.deadline = Date.now() + this.minutesLeft * 60_000;
		}
		this.hostRequests?.start();
		await this.session.connect();
	}
}

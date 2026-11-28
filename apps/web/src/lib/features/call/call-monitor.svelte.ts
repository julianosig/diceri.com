import type { MediaCredentials } from '#lib/features/room/index.ts';
import { LocalMonitor } from './local-monitor.ts';
import { LoopbackMonitor } from './loopback-monitor.ts';

export type MonitorMode = 'off' | 'local' | 'loopback';

export interface CallMonitorOptions {
	/** The caller's own media identity, so the round trip plays only their microphone. */
	identity: string;
	/** Fetches hidden-listener credentials for the round-trip monitor. */
	credentials: () => Promise<MediaCredentials>;
	/** Whether the call is being left: nothing starts after that. */
	isClosing: () => boolean;
	/** The round trip could not start (the caller may try again). */
	onFailed: () => void;
}

/**
 * Hearing yourself during a call: your microphone played back locally, or round-trip through
 * the server. Only the latest change takes effect; one finishing late never does.
 */
export class CallMonitor {
	mode = $state<MonitorMode>('off');
	#local: LocalMonitor | undefined;
	#loopback: LoopbackMonitor | undefined;
	#request = 0;
	/** Set while the microphone is being changed: choices are kept, nothing starts. */
	#paused = false;

	constructor(private readonly options: CallMonitorOptions) {}

	/** Switches to `mode`; `outgoing` is the audio the call sends (for the local monitor). */
	async set(mode: MonitorMode, outgoing: MediaStreamTrack | undefined): Promise<void> {
		// The choice counts at once, even while paused or while the old monitor closes.
		this.mode = mode;
		const request = await this.#stop();
		// A newer choice, a pause, or leaving came while the old monitor closed: it wins.
		if (request !== this.#request || this.#paused || this.options.isClosing()) return;
		if (mode === 'local' && outgoing) this.#local = new LocalMonitor(outgoing);
		if (mode === 'loopback') await this.#startLoopback(request);
	}

	/** Stops hearing yourself for a moment (the microphone is being changed); keeps the choice. */
	async pause(): Promise<void> {
		this.#paused = true;
		await this.#stop();
	}

	/** Back to the latest choice, which may have changed during the pause. */
	async resume(outgoing: MediaStreamTrack | undefined): Promise<void> {
		this.#paused = false;
		await this.set(this.mode, outgoing);
	}

	/** Closes whatever monitor runs (detached first, so it is never closed twice). */
	async #stop(): Promise<number> {
		const request = ++this.#request;
		const [local, loopback] = [this.#local, this.#loopback];
		this.#local = undefined;
		this.#loopback = undefined;
		await Promise.allSettled([local?.close(), loopback?.stop()]);
		return request;
	}

	/** Superseded or failed attempts never linger. */
	async #startLoopback(request: number): Promise<void> {
		const current = () => request === this.#request && !this.options.isClosing();
		let loopback: LoopbackMonitor | undefined;
		try {
			const credentials = await this.options.credentials();
			if (!current()) return;
			loopback = new LoopbackMonitor(this.options.identity);
			this.#loopback = loopback;
			await loopback.start(credentials);
			if (!current()) await loopback.stop();
		} catch {
			// A listener that connected before failing never stays behind.
			await loopback?.stop().catch(() => undefined);
			if (this.#loopback === loopback) this.#loopback = undefined;
			if (!current()) return;
			this.mode = 'off';
			this.options.onFailed();
		}
	}
}

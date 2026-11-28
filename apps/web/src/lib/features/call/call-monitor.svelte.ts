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
}

/**
 * Hearing yourself during a call: your microphone played back locally, or round-trip through
 * the server.
 */
export class CallMonitor {
	mode = $state<MonitorMode>('off');
	#local: LocalMonitor | undefined;
	#loopback: LoopbackMonitor | undefined;

	constructor(private readonly options: CallMonitorOptions) {}

	/** Switches to `mode`; `outgoing` is the audio the call sends (for the local monitor). */
	async set(mode: MonitorMode, outgoing: MediaStreamTrack | undefined): Promise<void> {
		// Detached first, so two callers never close the same monitor twice.
		const [local, loopback] = [this.#local, this.#loopback];
		this.#local = undefined;
		this.#loopback = undefined;
		await Promise.allSettled([local?.close(), loopback?.stop()]);
		this.mode = mode;
		if (mode === 'local' && outgoing) this.#local = new LocalMonitor(outgoing);
		if (mode === 'loopback') {
			const credentials = await this.options.credentials();
			// Left while the credentials came: no hidden listener joins after the call.
			if (this.options.isClosing()) return;
			this.#loopback = new LoopbackMonitor(this.options.identity);
			await this.#loopback.start(credentials);
		}
	}
}

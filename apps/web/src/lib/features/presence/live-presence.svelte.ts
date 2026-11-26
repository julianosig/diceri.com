import { getPresence } from './presence-api.ts';
import type { Presence } from './types.ts';

/** Matches the server's snapshot window; polling faster would only repeat it. */
export const POLL_INTERVAL_MS = 15_000;

/** Polls who is online while the page is visible; keeps the last snapshot on errors. */
export class LivePresence {
	snapshot = $state<Presence | null>(null);
	readonly #load: () => Promise<Presence>;
	readonly #intervalMs: number;

	constructor(load: () => Promise<Presence> = getPresence, intervalMs = POLL_INTERVAL_MS) {
		this.#load = load;
		this.#intervalMs = intervalMs;
	}

	/** Starts polling; returns the stop function (fits `$effect`). */
	start(): () => void {
		void this.refresh();
		const timer = setInterval(() => {
			if (document.visibilityState === 'visible') void this.refresh();
		}, this.#intervalMs);
		return () => {
			clearInterval(timer);
		};
	}

	async refresh(): Promise<void> {
		try {
			this.snapshot = await this.#load();
		} catch {
			// Presence is decoration: a failed poll keeps what is on screen.
		}
	}
}

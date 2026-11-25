import type { MediaCredentials } from '#lib/features/room/index.ts';
import { knock, knockOutcome, type Knock, type Profile } from './lobby-api.ts';

const POLL_MS = 1500;

export type KnockStatus = 'idle' | 'waiting' | 'admitted' | 'denied' | 'error';

/** A guest asking the host to be let in, polling until a decision. */
export class GuestKnock {
	status = $state<KnockStatus>('idle');
	error = $state<string | undefined>();
	credentials = $state<MediaCredentials | undefined>();
	request: Knock | undefined;
	#timer: ReturnType<typeof setTimeout> | undefined;

	constructor(
		private readonly roomId: string,
		private readonly onAdmitted: (credentials: MediaCredentials, request: Knock) => void
	) {}

	async ask(profile: Profile): Promise<void> {
		this.status = 'waiting';
		this.error = undefined;
		try {
			this.request = await knock(this.roomId, profile);
			this.#schedule();
		} catch (cause) {
			this.#fail(cause);
		}
	}

	cancel(): void {
		clearTimeout(this.#timer);
		this.status = 'idle';
	}

	#schedule(): void {
		this.#timer = setTimeout(() => {
			void this.#poll();
		}, POLL_MS);
	}

	async #poll(): Promise<void> {
		if (!this.request || this.status !== 'waiting') return;
		try {
			const outcome = await knockOutcome(this.roomId, this.request);
			if (outcome.status === 'admitted') {
				this.credentials = outcome.credentials;
				this.status = 'admitted';
				this.onAdmitted(outcome.credentials, this.request);
			} else if (outcome.status === 'denied') {
				this.status = 'denied';
			} else {
				this.#schedule();
			}
		} catch (cause) {
			this.#fail(cause);
		}
	}

	#fail(cause: unknown): void {
		this.status = 'error';
		this.error = cause instanceof Error ? cause.message : String(cause);
	}
}

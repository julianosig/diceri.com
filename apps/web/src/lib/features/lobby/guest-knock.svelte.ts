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
	/** Bumped on every ask and cancel: an answer to an older one is ignored. */
	#attempt = 0;

	constructor(
		private readonly roomId: string,
		private readonly onAdmitted: (credentials: MediaCredentials, request: Knock) => void,
		/** Words a known refusal (a full call, a plan limit) in the visitor's language. */
		private readonly explain: (cause: unknown) => string | undefined = () => undefined
	) {}

	async ask(profile: Profile): Promise<void> {
		const attempt = ++this.#attempt;
		this.status = 'waiting';
		this.error = undefined;
		try {
			const request = await knock(this.roomId, profile);
			if (attempt !== this.#attempt) return;
			this.request = request;
			this.#schedule(attempt);
		} catch (cause) {
			if (attempt === this.#attempt) this.#fail(cause);
		}
	}

	/** Stops waiting: an answer still on its way (even "let in") is ignored. */
	cancel(): void {
		this.#attempt += 1;
		clearTimeout(this.#timer);
		this.status = 'idle';
	}

	#schedule(attempt: number): void {
		this.#timer = setTimeout(() => {
			void this.#poll(attempt);
		}, POLL_MS);
	}

	async #poll(attempt: number): Promise<void> {
		if (!this.request || this.status !== 'waiting' || attempt !== this.#attempt) return;
		try {
			const outcome = await knockOutcome(this.roomId, this.request);
			// Cancelled (or asked again) while the answer was on its way: never join.
			if (attempt !== this.#attempt) return;
			if (outcome.status === 'admitted') {
				this.credentials = outcome.credentials;
				this.status = 'admitted';
				this.onAdmitted(outcome.credentials, this.request);
			} else if (outcome.status === 'denied') {
				this.status = 'denied';
			} else {
				this.#schedule(attempt);
			}
		} catch (cause) {
			if (attempt === this.#attempt) this.#fail(cause);
		}
	}

	#fail(cause: unknown): void {
		this.status = 'error';
		this.error = this.explain(cause) ?? (cause instanceof Error ? cause.message : String(cause));
	}
}

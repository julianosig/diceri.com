import { checkAvailability } from './account-api.ts';
import { isValidUsername, normalizeUsername } from './username.ts';

/** Where the typed username stands during onboarding. */
export type UsernameStatus = 'empty' | 'invalid' | 'checking' | 'available' | 'taken' | 'error';

/** Waits this long after the last keystroke before asking the server. */
export const CHECK_DELAY_MS = 350;

/**
 * Checks a username's availability as it is typed (debounced). Only the answer for
 * the latest input counts, so slow responses never overwrite newer ones.
 */
export class UsernameCheck {
	status = $state<UsernameStatus>('empty');
	readonly #check: typeof checkAvailability;
	readonly #delayMs: number;
	#timer: ReturnType<typeof setTimeout> | undefined;
	#latest = '';

	constructor(check: typeof checkAvailability = checkAvailability, delayMs = CHECK_DELAY_MS) {
		this.#check = check;
		this.#delayMs = delayMs;
	}

	/** Call on every change of the input. */
	update(raw: string): void {
		clearTimeout(this.#timer);
		const username = normalizeUsername(raw);
		this.#latest = username;
		if (username.length === 0) {
			this.status = 'empty';
			return;
		}
		if (!isValidUsername(username)) {
			this.status = 'invalid';
			return;
		}
		this.status = 'checking';
		this.#timer = setTimeout(() => {
			void this.#run(username);
		}, this.#delayMs);
	}

	async #run(username: string): Promise<void> {
		try {
			const { available } = await this.#check(username);
			if (this.#latest === username) this.status = available ? 'available' : 'taken';
		} catch {
			if (this.#latest === username) this.status = 'error';
		}
	}

	dispose(): void {
		clearTimeout(this.#timer);
	}
}

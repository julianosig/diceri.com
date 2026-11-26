import { currentAccount, register, signIn, signOut } from './account-api.ts';
import { normalizeUsername } from './username.ts';

/** Who is signed in on this browser; `undefined` until the first check finishes. */
export class AccountSession {
	username = $state<string | null | undefined>(undefined);
	/** Shown once right after sign-up so the owner can save it. */
	newKey = $state<string | null>(null);

	async load(): Promise<void> {
		try {
			this.username = (await currentAccount())?.username ?? null;
		} catch {
			this.username = null;
		}
	}

	async register(raw: string): Promise<void> {
		const account = await register(normalizeUsername(raw));
		this.username = account.username;
		this.newKey = account.key;
	}

	async signIn(key: string): Promise<void> {
		this.username = (await signIn(key.trim())).username;
	}

	async signOut(): Promise<void> {
		await signOut();
		this.username = null;
		this.newKey = null;
	}

	dismissKey(): void {
		this.newKey = null;
	}
}

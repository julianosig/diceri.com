import { forgetHostKeys } from '#lib/features/room/index.ts';
import { currentAccount, register, signIn, signOut } from './account-api.ts';
import type { Account, Credentials, Registration } from './types.ts';
import { normalizeUsername } from './username.ts';

/** Who is signed in on this browser; `undefined` until the first check finishes. */
export class AccountSession {
	account = $state<Account | null | undefined>(undefined);

	async load(): Promise<void> {
		try {
			this.account = await currentAccount();
		} catch {
			this.account = null;
		}
	}

	async register(registration: Registration): Promise<void> {
		this.account = await register({
			...registration,
			username: normalizeUsername(registration.username)
		});
	}

	async signIn(credentials: Credentials): Promise<void> {
		this.account = await signIn({
			...credentials,
			username: normalizeUsername(credentials.username)
		});
	}

	async signOut(): Promise<void> {
		await signOut();
		forgetHostKeys();
		this.account = null;
	}
}

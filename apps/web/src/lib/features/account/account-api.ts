import { ApiError, requestJson, requestVoid } from '#lib/api/http-client.ts';
import type { Account, RegisteredAccount } from './types.ts';

/** The session lives in an HttpOnly cookie the browser sends by itself. */
export function register(username: string, fetchFn?: typeof fetch): Promise<RegisteredAccount> {
	return requestJson<RegisteredAccount>('/accounts', {
		method: 'POST',
		body: { username },
		fetchFn
	});
}

export function signIn(key: string, fetchFn?: typeof fetch): Promise<Account> {
	return requestJson<Account>('/accounts/session', { method: 'POST', body: { key }, fetchFn });
}

/** The signed-in account, or `null` when there is no session. */
export async function currentAccount(fetchFn?: typeof fetch): Promise<Account | null> {
	try {
		return await requestJson<Account>('/accounts/me', { fetchFn });
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 401) return null;
		throw cause;
	}
}

/** Answers 204 No Content and expires the session cookie. */
export function signOut(fetchFn?: typeof fetch): Promise<void> {
	return requestVoid('/accounts/sign-out', { method: 'POST', fetchFn });
}

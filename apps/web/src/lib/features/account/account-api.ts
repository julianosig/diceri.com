import { ApiError, requestJson, requestVoid } from '#lib/api/http-client.ts';
import type { Account, Credentials, Registration } from './types.ts';

/** The session lives in an HttpOnly cookie the browser sends by itself. */
export function register(registration: Registration, fetchFn?: typeof fetch): Promise<Account> {
	return requestJson<Account>('/accounts', { method: 'POST', body: registration, fetchFn });
}

export function signIn(credentials: Credentials, fetchFn?: typeof fetch): Promise<Account> {
	return requestJson<Account>('/accounts/session', {
		method: 'POST',
		body: credentials,
		fetchFn
	});
}

/** Whether a username is still free; `username` comes back as it would be stored. */
export function checkAvailability(
	username: string,
	fetchFn?: typeof fetch
): Promise<{ username: string; available: boolean }> {
	return requestJson(`/accounts/available?username=${encodeURIComponent(username)}`, {
		fetchFn
	});
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

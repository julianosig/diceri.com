import { describe, expect, it, vi } from 'vitest';
import { currentAccount, register, signIn, signOut } from './account-api.ts';

function fakeFetch(status: number, body: unknown) {
	return vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body, { status })));
}

const account = { username: 'ana', mascot: 'dragon' };

describe('account api', () => {
	it('signs up with username, password and mascot', async () => {
		const fetchFn = fakeFetch(201, account);
		const registration = { username: 'ana', password: 'correct horse', mascot: 'dragon' };
		await expect(register(registration, fetchFn)).resolves.toEqual(account);
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/accounts',
			expect.objectContaining({ method: 'POST', body: JSON.stringify(registration) })
		);
	});

	it('signs in with username and password', async () => {
		const fetchFn = fakeFetch(200, account);
		await expect(signIn({ username: 'ana', password: 'x'.repeat(8) }, fetchFn)).resolves.toEqual(
			account
		);
		expect(fetchFn).toHaveBeenCalledWith('/api/v1/accounts/session', expect.anything());
	});

	it('reports no account instead of failing when signed out', async () => {
		const fetchFn = vi.fn<typeof fetch>(() => Promise.resolve(new Response(null, { status: 204 })));
		await expect(currentAccount(fetchFn)).resolves.toBeNull();
	});

	it('signs out on a 204 with no body', async () => {
		const fetchFn = vi.fn<typeof fetch>(() => Promise.resolve(new Response(null, { status: 204 })));
		await expect(signOut(fetchFn)).resolves.toBeUndefined();
	});
});

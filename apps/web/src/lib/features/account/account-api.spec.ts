import { describe, expect, it, vi } from 'vitest';
import { currentAccount, register, signOut } from './account-api.ts';

function fakeFetch(status: number, body: unknown) {
	return vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body, { status })));
}

describe('account api', () => {
	it('claims a username with POST /api/v1/accounts', async () => {
		const fetchFn = fakeFetch(201, { username: 'ana', key: 'k' });
		await expect(register('ana', fetchFn)).resolves.toEqual({ username: 'ana', key: 'k' });
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/accounts',
			expect.objectContaining({ method: 'POST', body: JSON.stringify({ username: 'ana' }) })
		);
	});

	it('reports no account instead of failing when signed out', async () => {
		const fetchFn = fakeFetch(401, { title: 'Unauthorized', status: 401 });
		await expect(currentAccount(fetchFn)).resolves.toBeNull();
	});

	it('signs out on a 204 with no body', async () => {
		const fetchFn = vi.fn<typeof fetch>(() => Promise.resolve(new Response(null, { status: 204 })));
		await expect(signOut(fetchFn)).resolves.toBeUndefined();
	});
});

import { describe, expect, it, vi } from 'vitest';
import { getPresence } from './presence-api.ts';

describe('presence api', () => {
	it('reads GET /api/v1/presence', async () => {
		const body = { online: 2, liveRooms: 1, ranking: [{ people: 2, languages: ['en'] }] };
		const fetchFn = vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body)));
		await expect(getPresence(fetchFn)).resolves.toEqual(body);
		expect(fetchFn).toHaveBeenCalledWith('/api/v1/presence', expect.anything());
	});
});

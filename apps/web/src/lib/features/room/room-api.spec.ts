import { describe, expect, it, vi } from 'vitest';
import { ApiError } from '#lib/api/http-client.ts';
import { getUserRoom, openRoom } from './room-api.ts';

const room = {
	id: '01936f2e-7b4c-7d2a-9c1e-3f5a6b7c8d9e',
	code: 'abcdefghijklmnopqrstuv',
	maxParticipants: 2,
	owner: 'ana',
	entry: 'approval',
	mascot: 'wizard'
};

function fakeFetch(status: number, body: unknown) {
	return vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body, { status })));
}

describe('room api', () => {
	it('opens the signed-in person’s room with POST /api/v1/rooms', async () => {
		const fetchFn = fakeFetch(201, { ...room, hostKey: 'k' });
		const options = { listed: true, entry: 'open' } as const;
		await expect(openRoom(options, fetchFn)).resolves.toEqual({ ...room, hostKey: 'k' });
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/rooms',
			expect.objectContaining({ method: 'POST', body: JSON.stringify(options) })
		);
	});

	it('finds a live room by its owner and reports when it is not open', async () => {
		await expect(getUserRoom('ana', fakeFetch(200, room))).resolves.toEqual(room);
		const missing = fakeFetch(404, { title: 'Not Found', status: 404 });
		await expect(getUserRoom('ana', missing)).rejects.toBeInstanceOf(ApiError);
	});
});

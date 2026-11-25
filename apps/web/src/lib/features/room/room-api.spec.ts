import { describe, expect, it, vi } from 'vitest';
import { ApiError } from '#lib/api/http-client.ts';
import { createRoom, getRoom } from './room-api.ts';

const room = { id: 'abcdefghijklmnopqrstuv', maxParticipants: 2 };

function fakeFetch(status: number, body: unknown) {
	return vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body, { status })));
}

describe('room api', () => {
	it('creates a room with POST /api/v1/rooms', async () => {
		const fetchFn = fakeFetch(201, { ...room, hostKey: 'k' });
		await expect(createRoom(fetchFn)).resolves.toEqual({ ...room, hostKey: 'k' });
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/rooms',
			expect.objectContaining({ method: 'POST' })
		);
	});

	it('throws ApiError with the problem title on failure', async () => {
		const fetchFn = fakeFetch(404, { title: 'Not Found', status: 404 });
		await expect(getRoom(room.id, fetchFn)).rejects.toMatchObject({
			status: 404,
			title: 'Not Found'
		});
		await expect(getRoom(room.id, fetchFn)).rejects.toBeInstanceOf(ApiError);
	});
});

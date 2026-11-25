import { describe, expect, it, vi } from 'vitest';
import { ApiError } from '#lib/api/http-client.ts';
import { createRoom, getRoom, joinRoom } from './room-api.ts';

const room = { id: 'abcdefghijklmnopqrstuv', maxParticipants: 2 };

function fakeFetch(status: number, body: unknown) {
	return vi.fn(() => Promise.resolve(Response.json(body, { status })));
}

describe('room api', () => {
	it('creates a room with POST /api/v1/rooms', async () => {
		const fetchFn = fakeFetch(201, room);
		await expect(createRoom(fetchFn)).resolves.toEqual(room);
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/rooms',
			expect.objectContaining({ method: 'POST' })
		);
	});

	it('throws ApiError with the problem title on failure', async () => {
		const fetchFn = fakeFetch(404, { title: 'Not Found', status: 404 });
		await expect(getRoom(room.id, fetchFn)).rejects.toEqual(new ApiError(404, 'Not Found'));
	});

	it('joins as monitor with a JSON body', async () => {
		const credentials = { identity: 'monitor-x', serverUrl: 'wss://m', token: 't' };
		const fetchFn = fakeFetch(200, credentials);
		await expect(joinRoom(room.id, 'monitor', fetchFn)).resolves.toEqual(credentials);
		expect(fetchFn).toHaveBeenCalledWith(
			`/api/v1/rooms/${room.id}/join`,
			expect.objectContaining({ method: 'POST', body: '{"role":"monitor"}' })
		);
	});
});

import { describe, expect, it, vi } from 'vitest';
import { ApiError } from '#lib/api/http-client.ts';
import { createRoom, getRoom } from './room-api.ts';

const room = {
	id: '01936f2e-7b4c-7d2a-9c1e-3f5a6b7c8d9e',
	code: 'abcdefghijklmnopqrstuv',
	maxParticipants: 2,
	title: 'Coffee chat',
	owner: 'ana',
	mascot: 'wizard',
	entry: 'approval'
};
const newRoom = {
	title: 'Coffee chat',
	listed: false,
	entry: 'approval',
	mascot: 'wizard'
} as const;

function fakeFetch(status: number, body: unknown) {
	return vi.fn<typeof fetch>(() => Promise.resolve(Response.json(body, { status })));
}

describe('room api', () => {
	it('creates a room with POST /api/v1/rooms', async () => {
		const fetchFn = fakeFetch(201, { ...room, hostKey: 'k' });
		await expect(createRoom(newRoom, fetchFn)).resolves.toEqual({ ...room, hostKey: 'k' });
		expect(fetchFn).toHaveBeenCalledWith(
			'/api/v1/rooms',
			expect.objectContaining({ method: 'POST', body: JSON.stringify(newRoom) })
		);
	});

	it('throws ApiError with the problem title on failure', async () => {
		const fetchFn = fakeFetch(404, { title: 'Not Found', status: 404 });
		await expect(getRoom(room.code, fetchFn)).rejects.toMatchObject({
			status: 404,
			title: 'Not Found'
		});
		await expect(getRoom(room.code, fetchFn)).rejects.toBeInstanceOf(ApiError);
	});
});

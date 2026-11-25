import { requestJson } from '#lib/api/http-client.ts';
import type { JoinRole, MediaCredentials, Room } from './types.ts';

export function createRoom(fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>('/rooms', { method: 'POST' }, fetchFn);
}

export function getRoom(id: string, fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>(`/rooms/${encodeURIComponent(id)}`, {}, fetchFn);
}

export function joinRoom(
	id: string,
	role: JoinRole = 'speaker',
	fetchFn?: typeof fetch
): Promise<MediaCredentials> {
	return requestJson<MediaCredentials>(
		`/rooms/${encodeURIComponent(id)}/join`,
		{
			method: 'POST',
			headers: { 'content-type': 'application/json' },
			body: JSON.stringify({ role })
		},
		fetchFn
	);
}

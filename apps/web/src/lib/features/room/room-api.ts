import { requestJson } from '#lib/api/http-client.ts';
import type { CreatedRoom, Room } from './types.ts';

export function createRoom(fetchFn?: typeof fetch): Promise<CreatedRoom> {
	return requestJson<CreatedRoom>('/rooms', { method: 'POST', fetchFn });
}

export function getRoom(id: string, fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>(`/rooms/${encodeURIComponent(id)}`, { fetchFn });
}

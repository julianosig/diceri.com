import { requestJson } from '#lib/api/http-client.ts';
import type { CreatedRoom, NewRoom, Room } from './types.ts';

export function createRoom(room: NewRoom, fetchFn?: typeof fetch): Promise<CreatedRoom> {
	return requestJson<CreatedRoom>('/rooms', { method: 'POST', body: room, fetchFn });
}

export function getRoom(id: string, fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>(`/rooms/${encodeURIComponent(id)}`, { fetchFn });
}

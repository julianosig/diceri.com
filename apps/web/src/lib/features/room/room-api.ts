import { requestJson } from '#lib/api/http-client.ts';
import type { OpenedRoom, Room, RoomOptions } from './types.ts';

/** Opens (or reopens with new options) the signed-in person's room. */
export function openRoom(options: RoomOptions, fetchFn?: typeof fetch): Promise<OpenedRoom> {
	return requestJson<OpenedRoom>('/rooms', { method: 'POST', body: options, fetchFn });
}

/** The live room at `/@username` (404 while its owner has not opened it). */
export function getUserRoom(username: string, fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>(`/users/${encodeURIComponent(username)}/room`, { fetchFn });
}

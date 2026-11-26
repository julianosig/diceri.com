import { ApiError, requestJson } from '#lib/api/http-client.ts';
import type { OpenedRoom, Room, RoomOptions, UserProfile } from './types.ts';

/** Opens (or reopens with new options) the signed-in person's room. */
export function openRoom(options: RoomOptions, fetchFn?: typeof fetch): Promise<OpenedRoom> {
	return requestJson<OpenedRoom>('/rooms', { method: 'POST', body: options, fetchFn });
}

/** The live room at `/@username` (404 while its owner has not opened it). */
export function getUserRoom(username: string, fetchFn?: typeof fetch): Promise<Room> {
	return requestJson<Room>(`/users/${encodeURIComponent(username)}/room`, { fetchFn });
}

/** The handle and mascot behind `/@username`, or `null` while nobody has claimed it. */
export async function getUserProfile(
	username: string,
	fetchFn?: typeof fetch
): Promise<UserProfile | null> {
	try {
		return await requestJson<UserProfile>(`/users/${encodeURIComponent(username)}`, { fetchFn });
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 404) return null;
		throw cause;
	}
}

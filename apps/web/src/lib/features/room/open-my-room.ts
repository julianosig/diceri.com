import { ApiError } from '#lib/api/http-client.ts';
import { saveHostKey } from './host-keys.ts';
import { getUserRoom, openRoom } from './room-api.ts';
import type { Room, RoomEntry } from './types.ts';

/** The live room's entry rule, so reopening keeps it; a new room asks for approval. */
async function currentEntry(username: string): Promise<RoomEntry> {
	try {
		return (await getUserRoom(username)).entry;
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 404) return 'approval';
		throw cause;
	}
}

/**
 * Opens (or reopens) the signed-in person's room and keeps its host key in this browser.
 * Returns the live room as visitors see it (with the minutes its owner has left).
 */
export async function openMyRoom(username: string): Promise<{ room: Room; hostKey: string }> {
	// Rooms stay off the live board while it is hidden; the API keeps the option.
	const opened = await openRoom({ listed: false, entry: await currentEntry(username) });
	saveHostKey(opened.code, opened.hostKey);
	return { room: await getUserRoom(username), hostKey: opened.hostKey };
}

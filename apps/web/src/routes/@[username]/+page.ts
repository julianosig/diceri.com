import { ApiError } from '#lib/api/http-client.ts';
import { getUserProfile, getUserRoom } from '#lib/features/room/index.ts';
import type { PageLoad } from './$types';

/** The live room, or — while there is none — who the address belongs to (if anyone). */
export const load: PageLoad = async ({ params, fetch }) => {
	const username = params.username.toLowerCase();
	try {
		return { username, room: await getUserRoom(username, fetch), profile: null };
	} catch (cause) {
		if (!(cause instanceof ApiError && cause.status === 404)) throw cause;
		return { username, room: null, profile: await getUserProfile(username, fetch) };
	}
};
